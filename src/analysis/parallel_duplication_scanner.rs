// Parallel Duplication Scanner: Process allrs.txt on 24 CPUs → Parquet
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicationRecord {
    pub fingerprint_ast: String,
    pub fingerprint_structure: String,
    pub fingerprint_markov: String,        // NEW: Markov model fingerprint
    pub markov_transitions: String,        // NEW: Top transitions
    pub file1: String,
    pub file2: String,
    pub function1: String,
    pub function2: String,
    pub similarity: f64,
    pub lines: i32,
}

pub struct ParallelScanner {
    pub results: Arc<Mutex<Vec<DuplicationRecord>>>,
    pub num_cpus: usize,
}

impl ParallelScanner {
    pub fn new(num_cpus: usize) -> Self {
        Self {
            results: Arc::new(Mutex::new(Vec::new())),
            num_cpus,
        }
    }
    
    pub fn scan_file_list(&self, list_path: &str, base_dir: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
        println!("📂 Reading file list: {}", list_path);
        let files: Vec<String> = std::fs::read_to_string(list_path)?
            .lines()
            .filter(|l| l.ends_with(".rs"))
            .map(|s| {
                // Handle relative paths
                if let Some(base) = base_dir {
                    if s.starts_with("./") {
                        format!("{}/{}", base, &s[2..])
                    } else if !s.starts_with("/") {
                        format!("{}/{}", base, s)
                    } else {
                        s.to_string()
                    }
                } else {
                    s.to_string()
                }
            })
            .collect();
        
        println!("📊 Found {} Rust files", files.len());
        println!("🚀 Processing on {} CPUs\n", self.num_cpus);
        
        // Set rayon thread pool
        rayon::ThreadPoolBuilder::new()
            .num_threads(self.num_cpus)
            .build_global()
            .unwrap();
        
        // Process in parallel
        let chunk_size = files.len() / self.num_cpus;
        let chunks: Vec<_> = files.chunks(chunk_size).collect();
        
        chunks.par_iter().enumerate().for_each(|(i, chunk)| {
            println!("  CPU {} processing {} files...", i, chunk.len());
            self.process_chunk(chunk, i);
        });
        
        Ok(())
    }
    
    fn process_chunk(&self, files: &[String], cpu_id: usize) {
        let mut local_results = Vec::new();
        let mut processed = 0;
        
        for file in files {
            if let Ok(records) = self.scan_file(file) {
                local_results.extend(records);
                processed += 1;
                
                if processed % 100 == 0 {
                    println!("    CPU {} processed {} files", cpu_id, processed);
                }
            }
        }
        
        // Merge into global results
        let mut results = self.results.lock().unwrap();
        let dup_count = local_results.len();
        results.extend(local_results);
        
        println!("  ✓ CPU {} complete: {} files, {} duplicates", 
            cpu_id, processed, dup_count);
    }
    
    fn scan_file(&self, file_path: &str) -> Result<Vec<DuplicationRecord>, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(file_path)?;
        
        // Compute Markov model for entire file
        let markov_fp = self.compute_markov_fingerprint(&content);
        
        let syntax = syn::parse_file(&content)?;
        
        let mut records = Vec::new();
        
        // Extract functions and compute fingerprints
        for item in &syntax.items {
            if let syn::Item::Fn(func) = item {
                let fp = self.compute_fingerprint(func);
                
                // Store for comparison (simplified - just create record)
                records.push(DuplicationRecord {
                    fingerprint_ast: fp.0,
                    fingerprint_structure: fp.1,
                    fingerprint_markov: markov_fp.0.clone(),
                    markov_transitions: markov_fp.1.clone(),
                    file1: file_path.to_string(),
                    file2: String::new(), // Will be filled by deduplication
                    function1: func.sig.ident.to_string(),
                    function2: String::new(),
                    similarity: 1.0,
                    lines: 0,
                });
            }
        }
        
        Ok(records)
    }
    
    fn compute_markov_fingerprint(&self, content: &str) -> (String, String) {
        use std::collections::HashMap;
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        // Build character-level Markov chain (3-char window)
        let mut transitions: HashMap<String, HashMap<char, usize>> = HashMap::new();
        let chars: Vec<char> = content.chars().collect();
        
        for i in 0..chars.len().saturating_sub(3) {
            let state: String = chars[i..i+3].iter().collect();
            let next = chars[i+3];
            
            *transitions.entry(state)
                .or_default()
                .entry(next)
                .or_insert(0) += 1;
        }
        
        // Hash the transition matrix
        let mut hasher = DefaultHasher::new();
        let mut sorted_transitions: Vec<_> = transitions.iter().collect();
        sorted_transitions.sort_by_key(|(k, _)| k.as_str());
        
        for (state, nexts) in &sorted_transitions {
            state.hash(&mut hasher);
            let mut sorted_nexts: Vec<_> = nexts.iter().collect();
            sorted_nexts.sort_by_key(|(c, _)| *c);
            for (c, count) in sorted_nexts {
                c.hash(&mut hasher);
                count.hash(&mut hasher);
            }
        }
        
        let markov_hash = format!("{:x}", hasher.finish());
        
        // Get top 10 transitions for debugging
        let mut top_transitions: Vec<_> = transitions.iter()
            .flat_map(|(state, nexts)| {
                nexts.iter().map(move |(c, count)| (state.clone(), *c, *count))
            })
            .collect();
        top_transitions.sort_by_key(|(_, _, count)| std::cmp::Reverse(*count));
        
        let top_10: Vec<String> = top_transitions.iter()
            .take(10)
            .map(|(s, c, cnt)| format!("{}→{}:{}", s, c, cnt))
            .collect();
        
        (markov_hash, top_10.join(","))
    }
    
    fn compute_fingerprint(&self, func: &syn::ItemFn) -> (String, String) {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        // AST hash using quote to convert to string
        let mut ast_hasher = DefaultHasher::new();
        quote::quote!(#func).to_string().hash(&mut ast_hasher);
        let ast_hash = format!("{:x}", ast_hasher.finish());
        
        // Structure hash (simplified)
        let mut struct_hasher = DefaultHasher::new();
        func.sig.inputs.len().hash(&mut struct_hasher);
        let structure_hash = format!("{:x}", struct_hasher.finish());
        
        (ast_hash, structure_hash)
    }
    
    pub fn export_to_parquet(&self, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n💾 Exporting to parquet: {}", output_path);
        
        let results = self.results.lock().unwrap();
        
        // Convert to JSON (parquet crate would be used in production)
        let json = serde_json::to_string_pretty(&*results)?;
        std::fs::write(output_path.replace(".parquet", ".json"), json)?;
        
        println!("  ✓ Exported {} records", results.len());
        println!("  📦 Size: {} MB", results.len() * std::mem::size_of::<DuplicationRecord>() / 1_000_000);
        
        Ok(())
    }
    
    pub fn deduplicate(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n🔍 Deduplicating results...");
        
        let mut results = self.results.lock().unwrap();
        
        // Group by fingerprint
        use std::collections::HashMap;
        let mut by_fingerprint: HashMap<String, Vec<DuplicationRecord>> = HashMap::new();
        
        for record in results.drain(..) {
            by_fingerprint.entry(record.fingerprint_ast.clone())
                .or_default()
                .push(record);
        }
        
        // Find duplicates
        let mut duplicates = Vec::new();
        for (fp, group) in by_fingerprint {
            if group.len() > 1 {
                // Create duplicate pairs
                for i in 0..group.len() {
                    for j in (i+1)..group.len() {
                        duplicates.push(DuplicationRecord {
                            fingerprint_ast: fp.clone(),
                            fingerprint_structure: group[i].fingerprint_structure.clone(),
                            fingerprint_markov: group[i].fingerprint_markov.clone(),
                            markov_transitions: group[i].markov_transitions.clone(),
                            file1: group[i].file1.clone(),
                            file2: group[j].file1.clone(),
                            function1: group[i].function1.clone(),
                            function2: group[j].function1.clone(),
                            similarity: 1.0,
                            lines: 0,
                        });
                    }
                }
            }
        }
        
        *results = duplicates;
        println!("  ✓ Found {} duplicate pairs", results.len());
        
        Ok(())
    }
    
    pub fn compress_results(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("\n🗜️  Compressing results...");
        
        let results = self.results.lock().unwrap();
        
        // Group by structure hash for compression
        use std::collections::HashMap;
        let mut by_structure: HashMap<String, Vec<&DuplicationRecord>> = HashMap::new();
        
        for record in results.iter() {
            by_structure.entry(record.fingerprint_structure.clone())
                .or_default()
                .push(record);
        }
        
        println!("  Original records: {}", results.len());
        println!("  Unique structures: {}", by_structure.len());
        println!("  Compression ratio: {:.2}x", 
            results.len() as f64 / by_structure.len() as f64);
        
        Ok(())
    }
}

pub fn run_parallel_scan(list_path: &str, output_path: &str, num_cpus: usize, base_dir: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let scanner = ParallelScanner::new(num_cpus);
    
    // Scan all files
    scanner.scan_file_list(list_path, base_dir)?;
    
    // Deduplicate
    scanner.deduplicate()?;
    
    // Compress
    scanner.compress_results()?;
    
    // Export to parquet
    scanner.export_to_parquet(output_path)?;
    
    Ok(())
}
