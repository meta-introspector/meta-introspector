use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use crossbeam::channel::{bounded, Receiver, Sender};
use std::thread;
use syn::{parse_file, visit::Visit, ItemStruct, ItemEnum, ItemFn};
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
}

struct CrossbeamAnalyzer {
    sender: Sender<PathBuf>,
    results: Arc<Mutex<Vec<FileAnalysis>>>,
}

impl CrossbeamAnalyzer {
    fn new() -> Self {
        let (sender, receiver) = bounded(1000);
        let results = Arc::new(Mutex::new(Vec::new()));
        
        // Spawn 20 worker threads
        for worker_id in 0..20 {
            let rx = receiver.clone();
            let results_clone = Arc::clone(&results);
            
            thread::spawn(move || {
                Self::worker(worker_id, rx, results_clone);
            });
        }
        
        Self { sender, results }
    }

    fn worker(worker_id: usize, receiver: Receiver<PathBuf>, results: Arc<Mutex<Vec<FileAnalysis>>>) {
        while let Ok(filepath) = receiver.recv() {
            if let Ok(analysis) = Self::analyze_file(&filepath) {
                if let Ok(mut results_guard) = results.lock() {
                    results_guard.push(analysis);
                    
                    if results_guard.len() % 100 == 0 {
                        println!("Worker {}: {} files analyzed", worker_id, results_guard.len());
                    }
                }
            }
        }
    }

    fn analyze_file(path: &Path) -> Result<FileAnalysis, Box<dyn std::error::Error>> {
        let filename = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        let filename_parts: Vec<String> = filename
            .split('_')
            .map(|s| s.to_string())
            .collect();
        
        let content = fs::read_to_string(path)?;
        
        // No file size limit - process all files
        // if content.len() > 1_000_000 { // Removed limit
        
        match parse_file(&content) {
            Ok(syntax_tree) => {
                let mut visitor = ContentVisitor::new();
                visitor.visit_file(&syntax_tree);
                
                Ok(FileAnalysis {
                    filepath: path.to_string_lossy().to_string(),
                    filename_parts,
                    structs: visitor.structs,
                    enums: visitor.enums,
                    functions: visitor.functions,
                })
            },
            Err(e) => {
                eprintln!("PARSE_ERROR: Failed to parse {}: {}", path.display(), e);
                Ok(FileAnalysis {
                    filepath: path.to_string_lossy().to_string(),
                    filename_parts,
                    structs: vec!["PARSE_ERROR".to_string()],
                    enums: vec![],
                    functions: vec![],
                })
            }
        }
    }

    fn process_directory(&self, dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut dirs_to_process = vec![dir.to_path_buf()];
        let mut files_queued = 0;
        
        while let Some(current_dir) = dirs_to_process.pop() {
            if let Ok(entries) = fs::read_dir(&current_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    
                    if path.is_dir() {
                        dirs_to_process.push(path);
                    } else if path.extension().map_or(false, |ext| ext == "rs") {
                        self.sender.send(path)?;
                        files_queued += 1;
                        
                        if files_queued % 500 == 0 {
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
        let mut filename_patterns = HashMap::new();
        let mut content_patterns = HashMap::new();
        
        for analysis in results.iter() {
            for part in &analysis.filename_parts {
                *filename_patterns.entry(part.clone()).or_default() += 1;
            }
            
            for item in &analysis.structs {
                *content_patterns.entry(item.clone()).or_default() += 1;
            }
            for item in &analysis.enums {
                *content_patterns.entry(item.clone()).or_default() += 1;
            }
        }

        // Calculate correlations
        let mut correlations = Vec::new();
        for (filename_part, _) in &filename_patterns {
            for (content_item, _) in &content_patterns {
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
    fn visit_item_struct(&mut self, node: &'ast ItemStruct) {
        self.structs.push(node.ident.to_string());
    }

    fn visit_item_enum(&mut self, node: &'ast ItemEnum) {
        self.enums.push(node.ident.to_string());
    }

    fn visit_item_fn(&mut self, node: &'ast ItemFn) {
        self.functions.push(node.sig.ident.to_string());
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let target_dir = std::env::args().nth(1)
        .unwrap_or_else(|| "~/nix/vendor/rust/cargo2nix/submodules/rust-build".to_string());

    println!("⚡ CROSSBEAM 20-CPU RUSTC ANALYSIS");
    println!("=================================");
    println!("Target: {}", target_dir);

    let analyzer = CrossbeamAnalyzer::new();
    let path = Path::new(&target_dir);
    
    println!("🚀 Starting parallel analysis with 20 workers...");
    analyzer.process_directory(path)?;
    
    // Wait for workers to finish
    thread::sleep(std::time::Duration::from_secs(2));
    
    let report = analyzer.generate_report();
    
    println!("\n📊 CROSSBEAM ANALYSIS RESULTS:");
    println!("Files analyzed: {}", report.total_files);
    println!("Filename patterns: {}", report.filename_patterns.len());
    println!("Content patterns: {}", report.content_patterns.len());
    println!("Correlations found: {}", report.correlations.len());
    
    println!("\n🔥 TOP CORRELATIONS:");
    for (filename_part, content_item, correlation) in report.correlations.iter().take(15) {
        println!("  '{}' ↔ '{}': {:.3}", filename_part, content_item, correlation);
    }

    let json_data = serde_json::to_string_pretty(&report)?;
    fs::write("crossbeam_rustc_analysis.json", json_data)?;

    println!("\n💾 Analysis saved to crossbeam_rustc_analysis.json");
    Ok(())
}
