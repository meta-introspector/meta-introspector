use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crossbeam::channel::{bounded, Receiver, Sender};
use std::thread;
use serde::{Deserialize, Serialize};
use syn::{parse_file, visit::Visit, Expr, Lit};
use clap::Parser; // Add this line

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the Rust project to analyze
    #[arg(short, long, value_name = "DIR")]
    input_path: PathBuf,

    /// Directory to store the analysis results
    #[arg(short, long, value_name = "DIR")]
    output_dir: PathBuf,

    /// Run in plan mode, only print actions without execution
    #[arg(short, long)]
    plan_mode: bool,

    /// Only list Rust files that would be processed, one per line, and exit.
    #[arg(long)]
    list_files_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ValueUsage {
    file_path: String,
    context: String,
    usage_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ValueEntry {
    value: String,
    length: usize,
    total_usages: u32,
    usages: Vec<ValueUsage>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Progress {
    processed_files: Vec<String>,
    total_files: usize,
    current_count: usize,
}

struct ConstantVisitor {
    file_path: String,
    constants: Vec<(String, String, String)>,
}

impl ConstantVisitor {
    fn new(file_path: String) -> Self {
        Self { file_path, constants: Vec::new() }
    }
}

impl<'ast> Visit<'ast> for ConstantVisitor {
    fn visit_expr(&mut self, expr: &'ast Expr) {
        match expr {
            Expr::Lit(lit_expr) => {
                let (value, usage_type) = match &lit_expr.lit {
                    Lit::Int(int_lit) => (int_lit.base10_digits().to_string(), "integer_literal".to_string()),
                    Lit::Float(float_lit) => (float_lit.base10_digits().to_string(), "float_literal".to_string()),
                    Lit::Str(str_lit) => (str_lit.value(), "string_literal".to_string()),
                    Lit::Bool(bool_lit) => (bool_lit.value.to_string(), "boolean_literal".to_string()),
                    _ => return,
                };
                
                self.constants.push((value, "literal_usage".to_string(), usage_type));
            }
            _ => {}
        }
        syn::visit::visit_expr(self, expr);
    }
}

fn load_progress(output_dir: &Path) -> Progress {
    let progress_file = output_dir.join("progress.json");
    if let Ok(content) = fs::read_to_string(&progress_file) {
        if let Ok(progress) = serde_json::from_str(&content) {
            return progress;
        }
    }
    Progress {
        processed_files: Vec::new(),
        total_files: 0,
        current_count: 0,
    }
}

fn save_progress(progress: &Progress, output_dir: &Path) {
    let progress_file = output_dir.join("progress.json");
    if let Ok(json) = serde_json::to_string_pretty(progress) {
        let _ = fs::write(progress_file, json);
    }
}

fn process_file(file_path: String, sender: Sender<(String, Vec<(String, String, String)>)>) {
    if let Ok(source) = fs::read_to_string(&file_path) {
        if let Ok(ast) = parse_file(&source) {
            let mut visitor = ConstantVisitor::new(file_path.clone());
            visitor.visit_file(&ast);
            let _ = sender.send((file_path, visitor.constants));
        }
    }
}

fn collect_rust_files(dir: &Path, files: &mut Vec<String>) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if !matches!(name, "target" | ".git" | "node_modules" | "build" | "dist") {
                        collect_rust_files(&path, files);
                    }
                }
            } else if path.extension().map_or(false, |ext| ext == "rs") {
                files.push(path.to_string_lossy().to_string());
            }
        }
    }
}

fn main() {
    let args = Args::parse(); // Parse arguments

    if args.list_files_only {
        let mut rust_files = Vec::new();
        collect_rust_files(&args.input_path, &mut rust_files);
        for file in rust_files {
            println!("{}", file);
        }
        return;
    }

    if args.plan_mode {
        println!("--- Plan Mode ---");
        println!("Would analyze project at: {}", args.input_path.display());
        println!("Would save results to: {}", args.output_dir.display());
        let mut rust_files = Vec::new();
        collect_rust_files(&args.input_path, &mut rust_files);
        println!("Would process {} Rust files.", rust_files.len());
        for file in rust_files {
            println!("  - Would analyze file: {}", file);
        }
        println!("--- End Plan Mode ---");
        return;
    }

    println!("🔢 RECOVERABLE 20-CORE VALUE LATTICE INDEXER");
    println!("============================================");

    fs::create_dir_all(&args.output_dir).unwrap(); // Use output_dir
    
    let mut rust_files = Vec::new();
    collect_rust_files(&args.input_path, &mut rust_files); // Use input_path
    
    let mut progress = load_progress(&args.output_dir); // Use output_dir
    
    // Filter out already processed files
    let remaining_files: Vec<String> = rust_files.into_iter()
        .filter(|f| !progress.processed_files.contains(f))
        .collect();
    
    if progress.total_files == 0 {
        progress.total_files = remaining_files.len() + progress.processed_files.len();
    }
    
    println!("🦀 Total files: {}", progress.total_files);
    println!("✅ Already processed: {}", progress.processed_files.len());
    println!("⏳ Remaining: {}", remaining_files.len());
    println!("💻 Using 20 CPU cores");
    
    if remaining_files.is_empty() {
        println!("🎉 All files already processed!");
        return;
    }
    
    let (sender, receiver): (Sender<(String, Vec<(String, String, String)>)>, Receiver<(String, Vec<(String, String, String)>)>) = bounded(1000);
    
    // Spawn 20 worker threads
    let mut handles = Vec::new();
    let chunk_size = (remaining_files.len() + 19) / 20;
    
    for chunk in remaining_files.chunks(chunk_size) {
        let files_chunk = chunk.to_vec();
        let sender_clone = sender.clone();
        
        let handle = thread::spawn(move || {
            for file_path in files_chunk {
                process_file(file_path, sender_clone.clone());
            }
        });
        handles.push(handle);
    }
    
    drop(sender);
    
    // Collect results with progress tracking
    let mut value_map: HashMap<String, ValueEntry> = HashMap::new();
    
    while let Ok((file_path, constants)) = receiver.recv() {
        let file_path_for_progress = file_path.clone(); // Clone for progress tracking
        progress.processed_files.push(file_path_for_progress);
        progress.current_count += 1;
        
        if progress.current_count % 100 == 0 {
            println!("📊 Processed {}/{} files ({:.1}%)", 
                     progress.current_count, 
                     progress.total_files,
                     (progress.current_count as f64 / progress.total_files as f64) * 100.0);
            save_progress(&progress, &args.output_dir); // Use output_dir
        }
        
        for (value, context, usage_type) in constants {
            let entry = value_map.entry(value.clone()).or_insert(ValueEntry {
                value: value.clone(),
                length: value.len(),
                total_usages: 0,
                usages: Vec::new(),
            });
            
            entry.total_usages += 1;
            entry.usages.push(ValueUsage {
                file_path: file_path.clone(), // Use clone here to keep original file_path for later use if needed
                context,
                usage_type,
            });
        }
    }
    
    // Wait for all threads
    for handle in handles {
        let _ = handle.join();
    }
    
    // Final progress save
    save_progress(&progress, &args.output_dir); // Use output_dir
    
    println!("💾 Saving {} unique values...", value_map.len());
    
    // Save results by length in parallel
    let value_lattice_output_dir = args.output_dir.join("value-lattice"); // Use output_dir
    fs::create_dir_all(&value_lattice_output_dir).unwrap();
    
    let entries: Vec<_> = value_map.into_iter().collect();
    let save_chunk_size = (entries.len() + 19) / 20;
    let mut save_handles = Vec::new();
    
    for chunk in entries.chunks(save_chunk_size) {
        let chunk_vec = chunk.to_vec();
        let value_lattice_output_dir_clone = value_lattice_output_dir.clone(); // Clone for each thread
        let handle = thread::spawn(move || {
            for (_, entry) in chunk_vec {
                let length_dir = value_lattice_output_dir_clone.join(format!("length-{}", entry.length)); // Use value_lattice_output_dir_clone
                fs::create_dir_all(&length_dir).unwrap();
                
                let safe_filename = entry.value.chars()
                    .map(|c| if c.is_alphanumeric() || c == '.' || c == '-' { c } else { '_' })
                    .collect::<String>();
                
                let file_path = length_dir.join(format!("{}.json", safe_filename)); // Use length_dir
                let json_data = serde_json::to_string_pretty(&entry).unwrap();
                fs::write(file_path, json_data).unwrap();
            }
        });
        save_handles.push(handle);
    }
    
    // Wait for all save threads
    for handle in save_handles {
        let _ = handle.join();
    }
    
    println!("✅ 20-core recoverable analysis complete!");
    println!("📈 Final: {}/{} files processed", progress.current_count, progress.total_files);
}
