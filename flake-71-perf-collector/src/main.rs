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
    
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    
    println!("🎯 Collecting build + run perf data for 71 flakes");
    println!("Starting with: rust\n");
    
    let lang = "rust";
    let flake_path = base_dir.join(lang);
    
    // Step 1: Build with perf
    println!("📊 Building {} flake at {:?}", lang, flake_path);
    let build_perf = output_dir.join(format!("{}_{}_{}.perf.data", lang, timestamp, "build"));
    
    let build_status = Command::new("perf")
        .args(&["record", "-o", build_perf.to_str().unwrap(), "--call-graph", "dwarf",
                "nix", "build", "--print-build-logs"])
        .current_dir(&flake_path)
        .status()?;
    
    let build_success = build_status.success();
    println!("✅ Build: {}", if build_success { "SUCCESS" } else { "FAILED" });
    
    // Step 2: Run with perf (if build succeeded)
    let (run_success, run_perf_path, run_output) = if build_success {
        println!("🚀 Running {} program with perf", lang);
        let run_perf = output_dir.join(format!("{}_{}_{}.perf.data", lang, timestamp, "run"));
        
        let run_result = Command::new("perf")
            .args(&["record", "-o", run_perf.to_str().unwrap(), "--call-graph", "dwarf",
                    "nix", "run"])
            .current_dir(&flake_path)
            .output()?;
        
        let success = run_result.status.success();
        let output = String::from_utf8_lossy(&run_result.stdout).to_string();
        
        println!("✅ Run: {}", if success { "SUCCESS" } else { "FAILED" });
        println!("📄 Output: {}", output.trim());
        
        (success, Some(run_perf), Some(output))
    } else {
        (false, None, None)
    };
    
    let result = FlakePerfResult {
        language: lang.to_string(),
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
    
    println!("\n📊 Summary:");
    println!("  Build perf: {:?}", result.build_perf_data);
    println!("  Run perf: {:?}", result.run_perf_data);
    println!("  JSON: {:?}", json_output);
    
    Ok(())
}
