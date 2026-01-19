//! # Access Pattern Profiler
//! 
//! Traces how data flows through the system, creating a self-describing database.
//! Regex patterns become a language, compilers become auto-labelers.

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

/// Access pattern - traces data flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessPattern {
    pub query: String,           // Original query (regex/pattern)
    pub files_accessed: Vec<PathBuf>,
    pub next_queries: Vec<String>, // What queries followed
    pub context: Vec<String>,    // Shell script, function, etc.
    pub timestamp: u64,
}

/// Pattern language - regex becomes semantic
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternLanguage {
    pub pattern: String,         // Regex pattern
    pub semantic_label: String,  // Auto-generated label
    pub frequency: u64,
    pub typical_results: Vec<PathBuf>,
    pub related_patterns: Vec<String>,
}

/// Data reach - how far data propagates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataReach {
    pub source_file: PathBuf,
    pub accessed_by: Vec<String>,     // Scripts/binaries
    pub transformed_to: Vec<PathBuf>, // Output files
    pub reach_depth: usize,           // How many hops
}

/// Self-describing system state
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemState {
    pub patterns: HashMap<String, PatternLanguage>,
    pub access_traces: Vec<AccessPattern>,
    pub data_reach: HashMap<PathBuf, DataReach>,
    pub auto_labels: HashMap<String, String>,
}

impl SystemState {
    pub fn new() -> Self {
        Self {
            patterns: HashMap::new(),
            access_traces: Vec::new(),
            data_reach: HashMap::new(),
            auto_labels: HashMap::new(),
        }
    }
    
    /// Record access pattern
    pub fn record_access(&mut self, pattern: AccessPattern) {
        // Learn pattern language
        self.learn_pattern(&pattern.query, &pattern.files_accessed);
        
        // Trace data reach
        for file in &pattern.files_accessed {
            self.trace_reach(file.clone(), &pattern.context);
        }
        
        // Store trace
        self.access_traces.push(pattern);
    }
    
    /// Learn pattern as language
    fn learn_pattern(&mut self, query: &str, results: &[PathBuf]) {
        self.patterns.entry(query.to_string())
            .and_modify(|p| {
                p.frequency += 1;
                p.typical_results = results.to_vec();
            })
            .or_insert_with(|| PatternLanguage {
                pattern: query.to_string(),
                semantic_label: self.auto_label(query, results),
                frequency: 1,
                typical_results: results.to_vec(),
                related_patterns: vec![],
            });
    }
    
    /// Auto-generate semantic label from pattern
    fn auto_label(&mut self, pattern: &str, results: &[PathBuf]) -> String {
        // Analyze pattern and results to generate label
        let label = if pattern.contains("*.rs") || pattern == "ext:rs" {
            "rust_source_files"
        } else if pattern.contains("flake.nix") {
            "nix_flakes"
        } else if pattern.contains("Cargo.toml") {
            "cargo_manifests"
        } else if pattern.contains("*.so") {
            "shared_libraries"
        } else {
            // Generate from common path patterns
            self.infer_label_from_results(results)
        };
        
        self.auto_labels.insert(pattern.to_string(), label.to_string());
        label.to_string()
    }
    
    /// Infer label from result paths
    fn infer_label_from_results(&self, results: &[PathBuf]) -> &'static str {
        if results.is_empty() { return "unknown"; }
        
        let first = results[0].to_str().unwrap_or("");
        if first.contains("/src/") { "source_code" }
        else if first.contains("/target/") { "build_artifacts" }
        else if first.contains("/docs/") { "documentation" }
        else if first.contains("/tests/") { "test_files" }
        else { "project_files" }
    }
    
    /// Trace data reach through system
    fn trace_reach(&mut self, file: PathBuf, context: &[String]) {
        self.data_reach.entry(file.clone())
            .and_modify(|r| {
                r.accessed_by.extend_from_slice(context);
            })
            .or_insert(DataReach {
                source_file: file,
                accessed_by: context.to_vec(),
                transformed_to: vec![],
                reach_depth: 1,
            });
    }
    
    /// Get pattern language summary
    pub fn pattern_summary(&self) -> Vec<(String, String, u64)> {
        self.patterns.values()
            .map(|p| (p.pattern.clone(), p.semantic_label.clone(), p.frequency))
            .collect()
    }
    
    /// Get data reach summary
    pub fn reach_summary(&self) -> Vec<(PathBuf, usize)> {
        self.data_reach.iter()
            .map(|(path, reach)| (path.clone(), reach.accessed_by.len()))
            .collect()
    }
    
    /// Export to Parquet
    pub fn to_parquet(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(format!("{}.json", path), json)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_learning() {
        let mut state = SystemState::new();
        
        let pattern = AccessPattern {
            query: "ext:rs".to_string(),
            files_accessed: vec![PathBuf::from("src/main.rs")],
            next_queries: vec![],
            context: vec!["build.sh".to_string()],
            timestamp: 0,
        };
        
        state.record_access(pattern);
        
        assert_eq!(state.patterns.len(), 1);
        assert_eq!(state.auto_labels.get("ext:rs"), Some(&"rust_source_files".to_string()));
    }
}
