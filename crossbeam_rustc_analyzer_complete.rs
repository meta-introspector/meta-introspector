use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use crossbeam::channel::{bounded, Receiver, Sender};
use std::thread;
use syn::{parse_file, visit::Visit};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileAnalysis {
    filepath: String,
    filename_parts: Vec<String>,
    structs: Vec<String>,
    enums: Vec<String>,
    functions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct AggregatedReport {
    total_files: u32,
    filename_patterns: HashMap<String, u32>,
    content_patterns: HashMap<String, u32>,
    correlations: Vec<(String, String, f32)>,
    skipped_files: Vec<(String, String)>,
}

struct CrossbeamAnalyzer {
    sender: Sender<PathBuf>,
    results: Arc<Mutex<Vec<FileAnalysis>>>,
    skipped_files: Arc<Mutex<Vec<(String, String)>>>,
}

impl CrossbeamAnalyzer {
    fn new() -> Self {
        let (sender, receiver) = bounded(1000);
        let results = Arc::new(Mutex::new(Vec::new()));
        let skipped_files = Arc::new(Mutex::new(Vec::new()));
        
        for worker_id in 0..20 {
            let rx = receiver.clone();
            let results_clone = Arc::clone(&results);
            let skipped_clone = Arc::clone(&skipped_files);
            
            thread::spawn(move || {
                Self::worker(worker_id, rx, results_clone, skipped_clone);
            });
        }
        
        Self { sender, results, skipped_files }
    }

    fn worker(worker_id: usize, receiver: Receiver<PathBuf>, results: Arc<Mutex<Vec<FileAnalysis>>>, skipped_files: Arc<Mutex<Vec<(String, String)>>>) {
        while let Ok(filepath) = receiver.recv() {
            // Log the file being processed
            println!("Worker {}: Processing {}", worker_id, filepath.display());
            
            match Self::analyze_file(&filepath, &skipped_files) {
                Ok(analysis) => {
                    if let Ok(mut results_guard) = results.lock() {
                        results_guard.push(analysis);
                        
                        if results_guard.len() % 1000 == 0 {
                            println!("Worker {}: {} files analyzed", worker_id, results_guard.len());
                        }
                    }
                },
                Err(e) => {
                    eprintln!("ERROR processing {}: {}", filepath.display(), e);
                    if let Ok(mut skipped) = skipped_files.lock() {
                        skipped.push((filepath.to_string_lossy().to_string(), format!("Analysis error: {}", e)));
                    }
                }
            }
        }
    }

    fn analyze_file(path: &Path, skipped_files: &Arc<Mutex<Vec<(String, String)>>>) -> Result<FileAnalysis, Box<dyn std::error::Error>> {
        let path_str = path.to_string_lossy();
        
        // Skip recursive paths and test directories
        if path_str.contains("/zombie_driver2/meta-introspector/data/repos/zombie_driver2/") ||
           path_str.contains("/tests/") || 
           path_str.contains("/test/") ||
           path_str.contains("rustdoc") ||
           path_str.contains("pathological") ||
           path_str.len() > 500 { // Skip extremely long paths
            let reason = "Recursive/test/long path excluded".to_string();
            if let Ok(mut skipped) = skipped_files.lock() {
                skipped.push((path.to_string_lossy().to_string(), reason));
            }
            return Ok(FileAnalysis {
                filepath: path.to_string_lossy().to_string(),
                filename_parts: vec!["SKIPPED_RECURSIVE".to_string()],
                structs: vec!["SKIPPED_RECURSIVE".to_string()],
                enums: vec![],
                functions: vec![],
            });
        }
        
        let filename = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        let filename_parts: Vec<String> = filename
            .split('_')
            .map(|s| s.to_string())
            .collect();
        
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                let reason = format!("Read error: {}", e);
                if let Ok(mut skipped) = skipped_files.lock() {
                    skipped.push((path.to_string_lossy().to_string(), reason));
                }
                return Ok(FileAnalysis {
                    filepath: path.to_string_lossy().to_string(),
                    filename_parts,
                    structs: vec!["SKIPPED_READ_ERROR".to_string()],
                    enums: vec![],
                    functions: vec![],
                });
            }
        };
        
        if content.len() > 1_000_000 {
            let reason = format!("File too large: {} bytes", content.len());
            if let Ok(mut skipped) = skipped_files.lock() {
                skipped.push((path.to_string_lossy().to_string(), reason));
            }
            return Ok(FileAnalysis {
                filepath: path.to_string_lossy().to_string(),
                filename_parts,
                structs: vec!["SKIPPED_LARGE".to_string()],
                enums: vec![],
                functions: vec![],
            });
        }
        
        match std::panic::catch_unwind(|| parse_file(&content)) {
            Ok(Ok(syntax_tree)) => {
                match std::panic::catch_unwind(|| {
                    let mut visitor = ContentVisitor::new();
                    visitor.visit_file(&syntax_tree);
                    visitor
                }) {
                    Ok(visitor) => {
                        Ok(FileAnalysis {
                            filepath: path.to_string_lossy().to_string(),
                            filename_parts,
                            structs: visitor.structs,
                            enums: visitor.enums,
                            functions: visitor.functions,
                        })
                    },
                    Err(_) => {
                        let reason = "Visitor panic (stack overflow)".to_string();
                        eprintln!("VISITOR_PANIC: {}: {}", path.display(), reason);
                        if let Ok(mut skipped) = skipped_files.lock() {
                            skipped.push((path.to_string_lossy().to_string(), reason));
                        }
                        Ok(FileAnalysis {
                            filepath: path.to_string_lossy().to_string(),
                            filename_parts,
                            structs: vec!["SKIPPED_VISITOR_PANIC".to_string()],
                            enums: vec![],
                            functions: vec![],
                        })
                    }
                }
            },
            Ok(Err(e)) => {
                let reason = format!("Parse error: {}", e);
                if let Ok(mut skipped) = skipped_files.lock() {
                    skipped.push((path.to_string_lossy().to_string(), reason));
                }
                Ok(FileAnalysis {
                    filepath: path.to_string_lossy().to_string(),
                    filename_parts,
                    structs: vec!["SKIPPED_PARSE_ERROR".to_string()],
                    enums: vec![],
                    functions: vec![],
                })
            },
            Err(_) => {
                let reason = "Parse panic (stack overflow)".to_string();
                eprintln!("PARSE_PANIC: {}: {}", path.display(), reason);
                if let Ok(mut skipped) = skipped_files.lock() {
                    skipped.push((path.to_string_lossy().to_string(), reason));
                }
                Ok(FileAnalysis {
                    filepath: path.to_string_lossy().to_string(),
                    filename_parts,
                    structs: vec!["SKIPPED_PARSE_PANIC".to_string()],
                    enums: vec![],
                    functions: vec![],
                })
            }
        }
    }

    fn process_directory(&self, start_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut dirs_to_process = vec![(start_dir.to_path_buf(), 0)]; // (path, depth)
        let mut files_queued = 0;
        let max_depth = 10;
        
        while let Some((current_dir, depth)) = dirs_to_process.pop() {
            if depth > max_depth {
                continue;
            }
            
            if let Ok(entries) = fs::read_dir(&current_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    
                    if path.is_dir() {
                        dirs_to_process.push((path, depth + 1));
                    } else if path.extension().is_some_and(|ext| ext == "rs") {
                        self.sender.send(path)?;
                        files_queued += 1;
                        
                        if files_queued % 5000 == 0 {
                            println!("Queued {} files for analysis", files_queued);
                        }
                    }
                }
            }
        }
        
        println!("Total files queued: {}", files_queued);
        Ok(())
    }

    fn generate_report(&self) -> AggregatedReport {
        let results = self.results.lock().unwrap();
        let skipped = self.skipped_files.lock().unwrap();
        let mut filename_patterns = HashMap::new();
        let mut content_patterns = HashMap::new();
        
        for analysis in results.iter() {
            for part in &analysis.filename_parts {
                *filename_patterns.entry(part.clone()).or_default() += 1;
            }
            
            for item in &analysis.structs {
                if item != "SKIPPED_LARGE" && item != "SKIPPED_PARSE_ERROR" && item != "SKIPPED_TEST" && item != "SKIPPED_RECURSIVE" {
                    *content_patterns.entry(item.clone()).or_default() += 1;
                }
            }
            for item in &analysis.enums {
                *content_patterns.entry(item.clone()).or_default() += 1;
            }
        }

        let mut correlations = Vec::new();
        for filename_part in filename_patterns.keys() {
            for content_item in content_patterns.keys() {
                let correlation = Self::calculate_correlation(&results, filename_part, content_item);
                if correlation > 0.2 {
                    correlations.push((filename_part.clone(), content_item.clone(), correlation));
                }
            }
        }
        
        correlations.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

        AggregatedReport {
            total_files: results.len() as u32,
            filename_patterns,
            content_patterns,
            correlations,
            skipped_files: skipped.clone(),
        }
    }

    fn calculate_correlation(results: &[FileAnalysis], filename_part: &str, content_item: &str) -> f32 {
        let mut matches = 0;
        let mut total = 0;

        for analysis in results {
            let has_filename_part = analysis.filename_parts.contains(&filename_part.to_string());
            let has_content = analysis.structs.contains(&content_item.to_string()) || 
                             analysis.enums.contains(&content_item.to_string());
            
            if has_filename_part && has_content {
                matches += 1;
            }
            if has_filename_part || has_content {
                total += 1;
            }
        }

        if total > 0 { matches as f32 / total as f32 } else { 0.0 }
    }
}

struct ContentVisitor {
    structs: Vec<String>,
    enums: Vec<String>,
    functions: Vec<String>,
}

impl ContentVisitor {
    fn new() -> Self {
        Self {
            structs: Vec::new(),
            enums: Vec::new(),
            functions: Vec::new(),
        }
    }
}

impl<'ast> Visit<'ast> for ContentVisitor {
    fn visit_item_struct(&mut self, node: &'ast syn::ItemStruct) {
        self.structs.push(node.ident.to_string());
    }

    fn visit_item_enum(&mut self, node: &'ast syn::ItemEnum) {
        self.enums.push(node.ident.to_string());
    }

    fn visit_item_fn(&mut self, node: &'ast syn::ItemFn) {
        self.functions.push(node.sig.ident.to_string());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = std::env::args().nth(1)
        .unwrap_or_else(|| "~/nix/vendor/rust/cargo2nix/submodules/rust-build".to_string());

    println!("⚡ CROSSBEAM 20-CPU RUSTC ANALYSIS (NO TIME LIMIT)");
    println!("=================================================");
    println!("Target: {}", target_dir);

    let analyzer = CrossbeamAnalyzer::new();
    let path = Path::new(&target_dir);
    
    println!("🚀 Starting parallel analysis with 20 workers...");
    analyzer.process_directory(path)?;
    
    println!("⏳ Waiting for workers to complete...");
    thread::sleep(std::time::Duration::from_secs(10));
    
    let report = analyzer.generate_report();
    
    println!("\n📊 CROSSBEAM ANALYSIS RESULTS:");
    println!("Files analyzed: {}", report.total_files);
    println!("Files skipped: {}", report.skipped_files.len());
    println!("Filename patterns: {}", report.filename_patterns.len());
    println!("Content patterns: {}", report.content_patterns.len());
    println!("Correlations found: {}", report.correlations.len());
    
    println!("\n🔥 TOP CORRELATIONS:");
    for (filename_part, content_item, correlation) in report.correlations.iter().take(15) {
        println!("  '{}' ↔ '{}': {:.3}", filename_part, content_item, correlation);
    }

    let json_data = serde_json::to_string_pretty(&report)?;
    fs::write("crossbeam_rustc_analysis_complete.json", json_data)?;

    println!("\n💾 Complete analysis saved to crossbeam_rustc_analysis_complete.json");
    println!("📋 Skipped {} files (large files and parse errors)", report.skipped_files.len());
    
    Ok(())
}
