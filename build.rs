// 🔥 UNIFIED BUILD.RS - MACRO PROCESSOR, NIX INTEGRATION, AND TELEMETRY
// Combines: ldd2wrap + nix calls + crossbeam + telemetry_lib + macro processing

use std::env;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use crossbeam::channel;
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

// Import our telemetry system
#[path = "build_telemetry_lib.rs"]
mod telemetry_lib;
use telemetry_lib::telemetry_lib::*;

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=data/build_analysis/");
    
    let project = env::var("PROJECT_NAME").unwrap_or_else(|_| "build_system".to_string());
    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    
    println!("🔥 UNIFIED BUILD SYSTEM");
    println!("📊 Project: {}", project);
    println!("⏰ Timestamp: {}", timestamp);
    
    // Create telemetry entry for build start
    let entry = TelemetryEntry {
        r#type: "build_start".to_string(),
        message: "Unified build system starting".to_string(),
        timestamp,
        project: project.clone(),
        binaries: 0,
        libraries: 0,
        symbols: 0,
    };
    
    let log_file = get_log_file(&project, timestamp);
    let _ = write_telemetry_entry(&entry, &log_file);
    
    // Step 1: Process macros and autodiscovery
    let discovered_data = process_autodiscovery(&project, timestamp);
    
    // Step 2: Use crossbeam to process binaries/libraries in parallel
    let wrappers = process_with_crossbeam(discovered_data, &project, timestamp);
    
    // Step 3: Generate unified wrapper code
    generate_unified_wrappers(wrappers, &project, timestamp);
    
    // Final telemetry
    let final_entry = TelemetryEntry {
        r#type: "build_complete".to_string(),
        message: "Unified build system completed".to_string(),
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        project: project.clone(),
        binaries: 0, // Will be filled by actual counts
        libraries: 0,
        symbols: 0,
    };
    
    let _ = write_telemetry_entry(&final_entry, &log_file);
    println!("✅ Unified build completed");
}

#[derive(Debug, Clone)]
struct DiscoveredData {
    binaries: Vec<String>,
    libraries: Vec<String>,
    ldd_deps: Vec<String>,
}

fn process_autodiscovery(project: &str, timestamp: u64) -> DiscoveredData {
    println!("🔍 Processing autodiscovery...");
    
    let mut data = DiscoveredData {
        binaries: Vec::new(),
        libraries: Vec::new(),
        ldd_deps: Vec::new(),
    };
    
    // Load from existing strace data if available
    if let Ok(binaries_json) = fs::read_to_string("data/build_analysis/real_build_1768332029_binaries.json") {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&binaries_json) {
            if let Some(bins) = parsed["binaries"].as_array() {
                for bin in bins {
                    if let Some(path) = bin.as_str() {
                        data.binaries.push(path.to_string());
                    }
                }
            }
        }
    }
    
    // Load libraries
    if let Ok(libs_json) = fs::read_to_string("data/build_analysis/real_build_1768332029_libraries.json") {
        if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&libs_json) {
            if let Some(libs) = parsed["libraries"].as_array() {
                for lib in libs {
                    if let Some(path) = lib.as_str() {
                        if path.ends_with(".so") || path.contains(".so.") {
                            data.libraries.push(path.to_string());
                        }
                    }
                }
            }
        }
    }
    
    // Log discovery results
    let entry = TelemetryEntry {
        r#type: "autodiscovery".to_string(),
        message: format!("Discovered {} binaries, {} libraries", data.binaries.len(), data.libraries.len()),
        timestamp,
        project: project.to_string(),
        binaries: data.binaries.len() as u32,
        libraries: data.libraries.len() as u32,
        symbols: 0,
    };
    
    let log_file = get_log_file(project, timestamp);
    let _ = write_telemetry_entry(&entry, &log_file);
    
    data
}

#[derive(Debug)]
struct WrapperResult {
    name: String,
    symbols: Vec<String>,
    wrapper_code: String,
}

fn process_with_crossbeam(data: DiscoveredData, project: &str, timestamp: u64) -> Vec<WrapperResult> {
    println!("🚀 Processing with crossbeam...");
    
    let (sender, receiver) = channel::bounded(100);
    let results = Arc::new(std::sync::Mutex::new(Vec::new()));
    
    // Spawn worker threads
    let num_workers = 4;
    let mut handles = Vec::new();
    
    for worker_id in 0..num_workers {
        let receiver = receiver.clone();
        let results = Arc::clone(&results);
        let project = project.to_string();
        
        let handle = thread::spawn(move || {
            while let Ok(task) = receiver.recv() {
                match task {
                    Task::ProcessBinary(path) => {
                        if let Some(result) = process_binary_task(&path, &project, timestamp, worker_id) {
                            results.lock().unwrap().push(result);
                        }
                    }
                    Task::ProcessLibrary(path) => {
                        if let Some(result) = process_library_task(&path, &project, timestamp, worker_id) {
                            results.lock().unwrap().push(result);
                        }
                    }
                }
            }
        });
        handles.push(handle);
    }
    
    // Send tasks
    for binary in &data.binaries {
        let _ = sender.send(Task::ProcessBinary(binary.clone()));
    }
    
    for library in &data.libraries {
        let _ = sender.send(Task::ProcessLibrary(library.clone()));
    }
    
    // Close sender and wait for workers
    drop(sender);
    for handle in handles {
        let _ = handle.join();
    }
    
    // Return results
    Arc::try_unwrap(results).unwrap().into_inner().unwrap()
}

#[derive(Debug)]
enum Task {
    ProcessBinary(String),
    ProcessLibrary(String),
}

fn process_binary_task(path: &str, project: &str, timestamp: u64, worker_id: usize) -> Option<WrapperResult> {
    // This is where ldd2wrap logic goes - extract symbols from binary
    println!("🔧 Worker {}: Processing binary {}", worker_id, path);
    
    // Placeholder - would use goblin to extract symbols
    Some(WrapperResult {
        name: format!("binary_{}", Path::new(path).file_name()?.to_str()?),
        symbols: vec!["main".to_string(), "init".to_string()],
        wrapper_code: format!("// Wrapper for {}\n", path),
    })
}

fn process_library_task(path: &str, project: &str, timestamp: u64, worker_id: usize) -> Option<WrapperResult> {
    // This is where library symbol extraction goes
    println!("📚 Worker {}: Processing library {}", worker_id, path);
    
    // Placeholder - would use goblin to extract symbols
    Some(WrapperResult {
        name: format!("lib_{}", Path::new(path).file_stem()?.to_str()?),
        symbols: vec!["malloc".to_string(), "free".to_string()],
        wrapper_code: format!("// Wrapper for {}\n", path),
    })
}

fn generate_unified_wrappers(wrappers: Vec<WrapperResult>, project: &str, timestamp: u64) {
    println!("📝 Generating unified wrappers...");
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let wrapper_file = Path::new(&out_dir).join("unified_wrappers.rs");
    
    let mut content = String::new();
    content.push_str("// 🔥 UNIFIED WRAPPERS - Generated by build.rs\n\n");
    
    // Generate macro that includes all wrappers
    content.push_str("#[macro_export]\n");
    content.push_str("macro_rules! init_all_call_wrappers {\n");
    content.push_str("    () => {\n");
    content.push_str(&format!("        eprintln!(\"🚀 Initialized {} wrappers\");\n", wrappers.len()));
    
    for wrapper in &wrappers {
        content.push_str(&format!("        eprintln!(\"  - {}: {} symbols\");\n", 
                                 wrapper.name, wrapper.symbols.len()));
    }
    
    content.push_str("    };\n");
    content.push_str("}\n");
    
    fs::write(&wrapper_file, content).expect("Failed to write unified wrappers");
    
    // Log generation results
    let entry = TelemetryEntry {
        r#type: "wrapper_generation".to_string(),
        message: format!("Generated {} wrappers", wrappers.len()),
        timestamp,
        project: project.to_string(),
        binaries: wrappers.iter().filter(|w| w.name.starts_with("binary_")).count() as u32,
        libraries: wrappers.iter().filter(|w| w.name.starts_with("lib_")).count() as u32,
        symbols: wrappers.iter().map(|w| w.symbols.len()).sum::<usize>() as u32,
    };
    
    let log_file = get_log_file(project, timestamp);
    let _ = write_telemetry_entry(&entry, &log_file);
    
    println!("✅ Generated unified wrappers at: {:?}", wrapper_file);
}
