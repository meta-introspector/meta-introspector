use std::path::PathBuf;
use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct FlakePerfResult {
    language: String,
    flake_path: PathBuf,
    timestamp: u64,
    build_success: bool,
    build_perf_data: Option<PathBuf>,
    run_success: bool,
    run_perf_data: Option<PathBuf>,
    run_output: Option<String>,
    derivations_built: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let base_dir = PathBuf::from("/mnt/data1/meta-introspector/const_71_test");
    let output_dir = PathBuf::from("/mnt/data1/meta-introspector/data/71_flakes_perf");
    std::fs::create_dir_all(&output_dir)?;
    
    // Get all language directories
    let mut languages: Vec<String> = std::fs::read_dir(&base_dir)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    languages.sort();
    
    println!("🎯 Collecting build + run perf data for {} flakes\n", languages.len());
    
    let mut results = Vec::new();
    
    for (idx, lang) in languages.iter().enumerate() {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        
        println!("[{}/{}] 📊 Processing: {}", idx + 1, languages.len(), lang);
        
        let flake_path = base_dir.join(lang);
        let build_perf = output_dir.join(format!("{}_{}_{}.perf.data", lang, timestamp, "build"));
        
        // Build with perf
        let build_status = Command::new("perf")
            .args(&["record", "-o", build_perf.to_str().unwrap(), "--call-graph", "dwarf",
                    "nix", "build", "--print-build-logs"])
            .current_dir(&flake_path)
            .status()?;
        
        let build_success = build_status.success();
        println!("  Build: {}", if build_success { "✅" } else { "❌" });
        
        // Run with perf (if build succeeded)
        let (run_success, run_perf_path, run_output) = if build_success {
            let run_perf = output_dir.join(format!("{}_{}_{}.perf.data", lang, timestamp, "run"));
            
            let run_result = Command::new("perf")
                .args(&["record", "-o", run_perf.to_str().unwrap(), 
                        "--call-graph", "dwarf",
                        "-a",  // Record all CPUs (captures child processes)
                        "nix", "run"])
                .current_dir(&flake_path)
                .output()?;
            
            let success = run_result.status.success();
            let output = String::from_utf8_lossy(&run_result.stdout).to_string();
            
            println!("  Run: {} | Output: {}", if success { "✅" } else { "❌" }, output.trim());
            
            (success, Some(run_perf), Some(output))
        } else {
            (false, None, None)
        };
        
        let result = FlakePerfResult {
            language: lang.clone(),
            flake_path: flake_path.clone(),
            timestamp,
            build_success,
            build_perf_data: if build_success { Some(build_perf) } else { None },
            run_success,
            run_perf_data: run_perf_path,
            run_output,
            derivations_built: 0,
        };
        
        let json_output = output_dir.join(format!("{}_{}.json", lang, timestamp));
        let json = serde_json::to_string_pretty(&result)?;
        std::fs::write(&json_output, json)?;
        
        results.push(result);
        println!();
    }
    
    // Summary
    let successful = results.iter().filter(|r| r.build_success && r.run_success).count();
    println!("🎉 Complete: {}/{} flakes successful", successful, results.len());
    
    Ok(())
}
