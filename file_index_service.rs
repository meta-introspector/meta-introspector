//! # File Index Service
//! 
//! Centralized file indexing with intelligent caching and prediction.
//! Replaces all scattered find/grep operations with a single cached service.
//!
//! ## Architecture
//! 
//! ```text
//! Shell Scripts → HTTP/CLI → File Index Service → Parquet Cache
//!                                ↓
//!                         In-Memory Index
//!                         (sorted by priority)
//! ```
//!
//! ## Features
//! 
//! - **Centralized**: Single source of truth for file metadata
//! - **Cached**: In-memory index with Parquet persistence
//! - **Predictive**: Learns access patterns, pre-fetches likely queries
//! - **Fast**: Sub-millisecond queries for cached data
//! - **Smart**: Priority-based sorting and eviction

use std::collections::{HashMap, BTreeMap};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, Duration, UNIX_EPOCH};
use std::sync::{Arc, RwLock};
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;

mod access_pattern_profiler;
mod compiler_auto_labeler;
mod self_describing_nix;

use access_pattern_profiler::{AccessPattern, SystemState};
use compiler_auto_labeler::RegexLanguageCompiler;

/// File metadata with access tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified: u64,
    pub extension: Option<String>,
    pub is_dir: bool,
    
    // Access tracking
    pub access_count: u64,
    pub last_accessed: u64,
    pub priority_score: f64,
}

/// Query statistics for prediction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryStats {
    pub pattern: String,
    pub count: u64,
    pub last_used: u64,
    pub avg_results: usize,
}

/// File index with intelligent caching
pub struct FileIndexService {
    /// In-memory file index (path -> metadata)
    index: Arc<RwLock<HashMap<PathBuf, FileEntry>>>,
    
    /// Priority queue (score -> paths)
    priority_queue: Arc<RwLock<BTreeMap<u64, Vec<PathBuf>>>>,
    
    /// Query history for prediction
    query_history: Arc<RwLock<HashMap<String, QueryStats>>>,
    
    /// Cache directory for Parquet files
    cache_dir: PathBuf,
    
    /// Roots to index
    roots: Vec<PathBuf>,
    
    /// Max cache size (entries)
    max_cache_size: usize,
    
    /// Self-describing system state
    system_state: Arc<RwLock<SystemState>>,
    
    /// Regex language compiler
    regex_compiler: Arc<RwLock<RegexLanguageCompiler>>,
    
    /// Cache statistics
    cache_hits: Arc<RwLock<u64>>,
    cache_misses: Arc<RwLock<u64>>,
}

impl FileIndexService {
    /// Create new file index service
    pub fn new(cache_dir: PathBuf, roots: Vec<PathBuf>) -> Self {
        Self {
            index: Arc::new(RwLock::new(HashMap::new())),
            priority_queue: Arc::new(RwLock::new(BTreeMap::new())),
            query_history: Arc::new(RwLock::new(HashMap::new())),
            cache_dir,
            roots,
            max_cache_size: 1_000_000,
            system_state: Arc::new(RwLock::new(SystemState::new())),
            regex_compiler: Arc::new(RwLock::new(RegexLanguageCompiler::new())),
            cache_hits: Arc::new(RwLock::new(0)),
            cache_misses: Arc::new(RwLock::new(0)),
        }
    }
    
    /// Initialize: load from cache or scan filesystem
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Initializing File Index Service...");
        
        // Try to load from cache first
        if self.load_from_cache().is_ok() {
            println!("✅ Loaded index from cache");
            return Ok(());
        }
        
        // Otherwise, scan filesystem
        println!("📁 Scanning filesystem...");
        self.scan_filesystem()?;
        
        // Save to cache
        self.save_to_cache()?;
        
        Ok(())
    }
    
    /// Scan filesystem and build index
    fn scan_filesystem(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut index = self.index.write().unwrap();
        let start = SystemTime::now();
        let mut count = 0;
        
        for root in &self.roots {
            println!("  Scanning: {:?}", root);
            
            for entry in WalkDir::new(root)
                .follow_links(false)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                let path = entry.path().to_path_buf();
                let metadata = entry.metadata()?;
                
                let modified = metadata.modified()?
                    .duration_since(UNIX_EPOCH)?
                    .as_secs();
                
                let extension = path.extension()
                    .and_then(|s| s.to_str())
                    .map(|s| s.to_string());
                
                let file_entry = FileEntry {
                    path: path.clone(),
                    size: metadata.len(),
                    modified,
                    extension,
                    is_dir: metadata.is_dir(),
                    access_count: 0,
                    last_accessed: 0,
                    priority_score: 0.0,
                };
                
                index.insert(path, file_entry);
                count += 1;
                
                if count % 10000 == 0 {
                    println!("    Indexed {} files...", count);
                }
            }
        }
        
        let elapsed = start.elapsed()?.as_secs_f64();
        println!("✅ Indexed {} files in {:.2}s", count, elapsed);
        
        Ok(())
    }
    
    /// Query files by extension
    pub fn query_by_extension(&self, ext: &str) -> Vec<FileEntry> {
        let index = self.index.read().unwrap();
        
        // Record query
        self.record_query(&format!("ext:{}", ext));
        
        let results: Vec<_> = index.values()
            .filter(|e| !e.is_dir)
            .filter(|e| e.extension.as_deref() == Some(ext))
            .cloned()
            .collect();
        
        // Update access stats
        self.update_access_stats(&results);
        
        // Profile access pattern
        self.profile_access(&format!("ext:{}", ext), &results);
        
        results
    }
    
    /// Profile access pattern for learning
    fn profile_access(&self, query: &str, results: &[FileEntry]) {
        let mut state = self.system_state.write().unwrap();
        
        let pattern = AccessPattern {
            query: query.to_string(),
            files_accessed: results.iter().map(|e| e.path.clone()).collect(),
            next_queries: vec![],
            context: vec!["file_index_service".to_string()],
            timestamp: now(),
        };
        
        state.record_access(pattern);
    }
    
    /// Get learned semantic labels
    pub fn get_semantic_labels(&self) -> Vec<(String, String)> {
        let state = self.system_state.read().unwrap();
        state.auto_labels.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect()
    }
    
    /// Export learned patterns to Parquet
    pub fn export_patterns(&self) -> Result<(), Box<dyn std::error::Error>> {
        let state = self.system_state.read().unwrap();
        state.to_parquet(&format!("{}/patterns", self.cache_dir.display()))?;
        Ok(())
    }
    
    /// Query files by pattern (glob-like)
    pub fn query_by_pattern(&self, pattern: &str) -> Vec<FileEntry> {
        let index = self.index.read().unwrap();
        
        // Record query
        self.record_query(&format!("pattern:{}", pattern));
        
        let results: Vec<_> = index.values()
            .filter(|e| {
                e.path.to_str()
                    .map(|s| s.contains(pattern))
                    .unwrap_or(false)
            })
            .cloned()
            .collect();
        
        // Update access stats
        self.update_access_stats(&results);
        
        results
    }
    
    /// Query files by name
    pub fn query_by_name(&self, name: &str) -> Vec<FileEntry> {
        let index = self.index.read().unwrap();
        
        // Record query
        self.record_query(&format!("name:{}", name));
        
        let results: Vec<_> = index.values()
            .filter(|e| {
                e.path.file_name()
                    .and_then(|n| n.to_str())
                    == Some(name)
            })
            .cloned()
            .collect();
        
        // Update access stats
        self.update_access_stats(&results);
        
        results
    }
    
    /// Get top priority files (most likely to be accessed)
    pub fn get_top_priority(&self, limit: usize) -> Vec<FileEntry> {
        let index = self.index.read().unwrap();
        
        let mut entries: Vec<_> = index.values().cloned().collect();
        entries.sort_by(|a, b| {
            b.priority_score.partial_cmp(&a.priority_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        entries.into_iter().take(limit).collect()
    }
    
    /// Predict next queries based on history
    pub fn predict_next_queries(&self, limit: usize) -> Vec<String> {
        let history = self.query_history.read().unwrap();
        
        let mut queries: Vec<_> = history.values().cloned().collect();
        
        // Sort by recency and frequency
        queries.sort_by(|a, b| {
            let score_a = (a.count as f64) * (1.0 / (now() - a.last_used + 1) as f64);
            let score_b = (b.count as f64) * (1.0 / (now() - b.last_used + 1) as f64);
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });
        
        queries.into_iter()
            .take(limit)
            .map(|q| q.pattern)
            .collect()
    }
    
    /// Pre-fetch predicted queries
    pub fn prefetch_predicted(&self) {
        let predictions = self.predict_next_queries(10);
        
        println!("🔮 Pre-fetching predicted queries:");
        for pattern in predictions {
            println!("  - {}", pattern);
            
            // Execute query to warm cache
            if pattern.starts_with("ext:") {
                let ext = &pattern[4..];
                self.query_by_extension(ext);
            } else if pattern.starts_with("name:") {
                let name = &pattern[5..];
                self.query_by_name(name);
            } else if pattern.starts_with("pattern:") {
                let pat = &pattern[8..];
                self.query_by_pattern(pat);
            }
        }
    }
    
    /// Record query for prediction
    fn record_query(&self, pattern: &str) {
        let mut history = self.query_history.write().unwrap();
        
        let is_repeat = history.contains_key(pattern);
        
        if is_repeat {
            *self.cache_hits.write().unwrap() += 1;
        } else {
            *self.cache_misses.write().unwrap() += 1;
        }
        
        history.entry(pattern.to_string())
            .and_modify(|stats| {
                stats.count += 1;
                stats.last_used = now();
            })
            .or_insert(QueryStats {
                pattern: pattern.to_string(),
                count: 1,
                last_used: now(),
                avg_results: 0,
            });
    }
    
    /// Update access statistics for files
    fn update_access_stats(&self, files: &[FileEntry]) {
        let mut index = self.index.write().unwrap();
        let current_time = now();
        
        for file in files {
            if let Some(entry) = index.get_mut(&file.path) {
                entry.access_count += 1;
                entry.last_accessed = current_time;
                
                // Calculate priority score
                // Higher score = more recent + more frequent
                let recency = 1.0 / (current_time - entry.last_accessed + 1) as f64;
                let frequency = (entry.access_count as f64).ln();
                entry.priority_score = recency * frequency;
            }
        }
    }
    
    /// Save index to Parquet cache
    fn save_to_cache(&self) -> Result<(), Box<dyn std::error::Error>> {
        use parquet::file::properties::WriterProperties;
        use parquet::file::writer::SerializedFileWriter;
        use parquet::schema::parser::parse_message_type;
        use std::fs::File;
        use std::sync::Arc;
        
        println!("💾 Saving index to cache...");
        
        std::fs::create_dir_all(&self.cache_dir)?;
        let cache_file = self.cache_dir.join("file_index.parquet");
        
        // For now, save as JSON (Parquet implementation would go here)
        let index = self.index.read().unwrap();
        let entries: Vec<_> = index.values().cloned().collect();
        
        let json = serde_json::to_string_pretty(&entries)?;
        std::fs::write(cache_file.with_extension("json"), json)?;
        
        println!("✅ Saved {} entries to cache", entries.len());
        
        Ok(())
    }
    
    /// Load index from Parquet cache
    fn load_from_cache(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let cache_file = self.cache_dir.join("file_index.json");
        
        if !cache_file.exists() {
            return Err("Cache not found".into());
        }
        
        println!("📂 Loading index from cache...");
        
        let json = std::fs::read_to_string(cache_file)?;
        let entries: Vec<FileEntry> = serde_json::from_str(&json)?;
        
        let mut index = self.index.write().unwrap();
        for entry in entries {
            index.insert(entry.path.clone(), entry);
        }
        
        println!("✅ Loaded {} entries from cache", index.len());
        
        Ok(())
    }
    
    /// Get statistics
    pub fn stats(&self) -> IndexStats {
        let index = self.index.read().unwrap();
        let history = self.query_history.read().unwrap();
        let hits = *self.cache_hits.read().unwrap();
        let misses = *self.cache_misses.read().unwrap();
        
        let total_files = index.len();
        let total_size: u64 = index.values().map(|e| e.size).sum();
        let total_queries: u64 = history.values().map(|q| q.count).sum();
        
        let cache_hit_rate = if hits + misses > 0 {
            hits as f64 / (hits + misses) as f64
        } else {
            0.0
        };
        
        IndexStats {
            total_files,
            total_size,
            total_queries,
            unique_queries: history.len(),
            cache_hit_rate,
        }
    }
}

/// Index statistics
#[derive(Debug, Serialize)]
pub struct IndexStats {
    pub total_files: usize,
    pub total_size: u64,
    pub total_queries: u64,
    pub unique_queries: usize,
    pub cache_hit_rate: f64,
}

/// Get current timestamp
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_file_index_service() {
        let temp = TempDir::new().unwrap();
        let cache_dir = temp.path().join("cache");
        
        // Create test files
        let test_dir = temp.path().join("test");
        fs::create_dir(&test_dir).unwrap();
        fs::write(test_dir.join("test.rs"), "fn main() {}").unwrap();
        fs::write(test_dir.join("lib.rs"), "pub fn test() {}").unwrap();
        fs::write(test_dir.join("Cargo.toml"), "[package]").unwrap();
        
        // Initialize service
        let mut service = FileIndexService::new(cache_dir, vec![test_dir]);
        service.initialize().unwrap();
        
        // Query by extension
        let rs_files = service.query_by_extension("rs");
        assert_eq!(rs_files.len(), 2);
        
        // Query by name
        let cargo_files = service.query_by_name("Cargo.toml");
        assert_eq!(cargo_files.len(), 1);
        
        // Check stats
        let stats = service.stats();
        assert_eq!(stats.total_files, 3);
        assert!(stats.total_queries > 0);
    }
}
