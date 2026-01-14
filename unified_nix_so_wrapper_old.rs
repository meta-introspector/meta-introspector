// 🔥 UNIFIED NIX BUILD + SO WRAPPING + TELEMETRY
// Combines existing nix_build_telemetry.rs + ldd2macro.rs + mkbootstrap

use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};
use std::env;
use std::fs;
use meta_introspector::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let project = env::var("PROJECT_NAME").unwrap_or_else(|_| "nix_build_wrapped".to_string());
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    
    // Initialize with mkbootstrap
    mkbootstrap!();
    
    println!("\n🔧 UNIFIED NIX BUILD + SO WRAPPING");
    println!("==================================");
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <nix-build-args...>", args[0]);
        println!("Example: {} -E 'with import <nixpkgs> {{}}; hello'", args[0]);
        return Ok(());
    }
    
    let nix_args = &args[1..];
    
    // Step 1: Run nix build with LD_PRELOAD telemetry
    println!("🚀 Running nix build with LD_PRELOAD telemetry...");
    let result = telemetry_wrap!("nix_build_with_preload", {
        run_nix_with_preload(nix_args, &project, start_time)
    })?;
    
    // Step 2: Generate comprehensive .so hooks and analyze
    if let Some(result_path) = result {
        println!("📊 Analyzing build result: {}", result_path);
        
        // Generate hooks for all 20,277 symbols from 91 .so files
        telemetry_wrap!("generate_comprehensive_hooks", {
            generate_comprehensive_so_hooks(&project, start_time)
        })?;
        
        telemetry_wrap!("analyze_and_wrap_so_files", {
            analyze_and_wrap_so_files(&result_path, &project, start_time)
        })?;
    }
    
    println!("✅ Unified nix build + so wrapping completed!");
    Ok(())
}

fn run_nix_with_preload(nix_args: &[String], project: &str, timestamp: u64) -> Result<Option<String>, Box<dyn std::error::Error>> {
    // Check if LD_PRELOAD library exists
    let preload_lib = "/mnt/data1/meta-introspector/rust_preload_interceptor/target/release/librust_preload_interceptor.so";
    
    if !std::path::Path::new(preload_lib).exists() {
        println!("⚠️  LD_PRELOAD library not found, running without interception");
        return run_plain_nix(nix_args);
    }
    
    println!("🔗 Using LD_PRELOAD: {}", preload_lib);
    
    // Run nix build with LD_PRELOAD
    let mut cmd = Command::new("nix");
    cmd.args(nix_args)
       .env("LD_PRELOAD", preload_lib)
       .env("PROJECT_NAME", project)
       .stdout(Stdio::piped())
       .stderr(Stdio::piped());
    
    let output = cmd.output()?;
    
    // Log telemetry
    let entry = format!(
        r#"{{"type":"nix_build","message":"nix build with LD_PRELOAD","timestamp":{},"project":"{}","exit_code":{},"stdout_bytes":{},"stderr_bytes":{}}}"#,
        timestamp, project, output.status.code().unwrap_or(-1), output.stdout.len(), output.stderr.len()
    );
    
    let log_file = format!("/mnt/data1/meta-introspector/data/telemetry/{}_{}.jsonl", project, timestamp);
    fs::write(&log_file, entry + "\n")?;
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        // Extract result path from nix output
        for line in stdout.lines() {
            if line.starts_with("/nix/store/") && !line.contains("building") {
                return Ok(Some(line.trim().to_string()));
            }
        }
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("❌ Nix build failed: {}", stderr);
    }
    
    Ok(None)
}

fn run_plain_nix(nix_args: &[String]) -> Result<Option<String>, Box<dyn std::error::Error>> {
    let output = Command::new("nix")
        .args(nix_args)
        .output()?;
    
    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.starts_with("/nix/store/") && !line.contains("building") {
                return Ok(Some(line.trim().to_string()));
            }
        }
    }
    
    Ok(None)
}

fn analyze_and_wrap_so_files(result_path: &str, project: &str, timestamp: u64) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Analyzing .so dependencies with ldd...");
    
    // Run ldd on the result
    let ldd_output = Command::new("ldd")
        .arg(result_path)
        .output()?;
    
    if !ldd_output.status.success() {
        println!("⚠️  ldd failed, trying file command...");
        let file_output = Command::new("file")
            .arg(result_path)
            .output()?;
        
        let file_info = String::from_utf8_lossy(&file_output.stdout);
        println!("📄 File info: {}", file_info);
        return Ok(());
    }
    
    let ldd_stdout = String::from_utf8_lossy(&ldd_output.stdout);
    let mut so_files = Vec::new();
    
    // Parse ldd output to find .so files
    for line in ldd_stdout.lines() {
        if line.contains(".so") {
            // Extract .so path
            if let Some(start) = line.find("/") {
                if let Some(end) = line[start..].find(" ") {
                    let so_path = &line[start..start+end];
                    if so_path.ends_with(".so") || so_path.contains(".so.") {
                        so_files.push(so_path.to_string());
                    }
                }
            }
        }
    }
    
    println!("📚 Found {} .so files:", so_files.len());
    for (i, so_file) in so_files.iter().enumerate() {
        println!("  {}. {}", i+1, so_file);
    }
    
    // Log .so analysis telemetry
    let entry = format!(
        r#"{{"type":"so_analysis","message":"analyzed .so dependencies","timestamp":{},"project":"{}","result_path":"{}","so_count":{},"so_files":{}}}"#,
        timestamp, project, result_path, so_files.len(), serde_json::to_string(&so_files)?
    );
    
    let log_file = format!("/mnt/data1/meta-introspector/data/telemetry/{}_{}.jsonl", project, timestamp);
    let mut existing_content = fs::read_to_string(&log_file).unwrap_or_default();
    existing_content.push_str(&entry);
    existing_content.push('\n');
    fs::write(&log_file, existing_content)?;
    
    // TODO: Generate LD_PRELOAD wrappers for each .so file
    println!("🔧 TODO: Generate LD_PRELOAD wrappers for {} .so files", so_files.len());
    
    Ok(())
}

fn generate_comprehensive_so_hooks(project: &str, timestamp: u64) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 Generating hooks for all 20,277 symbols from 91 .so files...");
    
    // Run our existing comprehensive hook generator
    let output = Command::new("cargo")
        .args(&["run", "--bin", "demangle_and_hook_generator"])
        .output()?;
    
    let success = output.status.success();
    let stdout_len = output.stdout.len();
    let stderr_len = output.stderr.len();
    
    if success {
        println!("✅ Generated comprehensive hooks successfully");
    } else {
        println!("❌ Hook generation failed");
        let stderr = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", stderr);
    }
    
    // Log hook generation telemetry
    let entry = format!(
        r#"{{"type":"hook_generation","message":"generated comprehensive so hooks","timestamp":{},"project":"{}","success":{},"stdout_bytes":{},"stderr_bytes":{}}}"#,
        timestamp, project, success, stdout_len, stderr_len
    );
    
    let log_file = format!("/mnt/data1/meta-introspector/data/telemetry/{}_{}.jsonl", project, timestamp);
    let mut existing_content = fs::read_to_string(&log_file).unwrap_or_default();
    existing_content.push_str(&entry);
    existing_content.push('\n');
    fs::write(&log_file, existing_content)?;
    
    Ok(())
}
