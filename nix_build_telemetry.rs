// 🔧 NIX BUILD TELEMETRY: Wrap nix build with full operation tracking
use std::process::{Command, Stdio};
use std::time::Instant;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("🔧 NIX BUILD TELEMETRY WRAPPER");
    println!("===============================");
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: {} <nix-build-args...>", args[0]);
        println!("Example: {} -E 'with import <nixpkgs> {{}}; hello'", args[0]);
        return;
    }
    
    let nix_args = &args[1..];
    
    // Step 1: Run nix build with telemetry
    let build_result = run_nix_build_with_telemetry(nix_args);
    
    // Step 2: Analyze the built result with ldd
    if let Some(result_path) = build_result {
        analyze_build_result(&result_path);
    }
}

fn run_nix_build_with_telemetry(args: &[String]) -> Option<String> {
    println!("\n🚀 Starting nix build with telemetry...");
    let start = Instant::now();
    
    // Run nix-build with full output capture
    let mut cmd = Command::new("nix-build");
    cmd.args(args);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::piped());
    
    println!("📋 Command: nix-build {}", args.join(" "));
    
    match cmd.output() {
        Ok(output) => {
            let duration = start.elapsed();
            
            println!("📊 Build completed in {:.2}s", duration.as_secs_f64());
            println!("📤 Exit code: {}", output.status.code().unwrap_or(-1));
            
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            if !stdout.is_empty() {
                println!("📋 Build output:");
                for line in stdout.lines() {
                    println!("  {}", line);
                }
            }
            
            if !stderr.is_empty() {
                println!("⚠️  Build stderr:");
                for line in stderr.lines() {
                    println!("  {}", line);
                }
            }
            
            // Extract result path from stdout
            if output.status.success() {
                let result_path = stdout.trim().to_string();
                if Path::new(&result_path).exists() {
                    println!("✅ Build successful: {}", result_path);
                    return Some(result_path);
                }
            }
            
            None
        }
        Err(e) => {
            println!("❌ Failed to run nix-build: {}", e);
            None
        }
    }
}

fn analyze_build_result(result_path: &str) {
    println!("\n🔍 ANALYZING BUILD RESULT");
    println!("=========================");
    println!("📁 Result path: {}", result_path);
    
    // Find all executables in the result
    let executables = find_executables(result_path);
    
    for exe in executables {
        analyze_executable(&exe);
    }
}

fn find_executables(path: &str) -> Vec<String> {
    let mut executables = Vec::new();
    
    // Check if it's a direct executable
    if is_executable(path) {
        executables.push(path.to_string());
        return executables;
    }
    
    // Look in bin/ directory
    let bin_path = format!("{}/bin", path);
    if Path::new(&bin_path).exists() {
        if let Ok(entries) = fs::read_dir(&bin_path) {
            for entry in entries.flatten() {
                let exe_path = entry.path();
                if is_executable(exe_path.to_str().unwrap_or("")) {
                    executables.push(exe_path.to_string_lossy().to_string());
                }
            }
        }
    }
    
    executables
}

fn is_executable(path: &str) -> bool {
    if let Ok(metadata) = fs::metadata(path) {
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            return metadata.is_file() && (metadata.permissions().mode() & 0o111) != 0;
        }
        #[cfg(not(unix))]
        {
            return metadata.is_file();
        }
    }
    false
}

fn analyze_executable(exe_path: &str) {
    println!("\n🔍 Analyzing executable: {}", exe_path);
    
    // Run ldd to get shared library dependencies
    run_ldd_analysis(exe_path);
    
    // Run nm to get symbols
    run_nm_analysis(exe_path);
    
    // Get file info
    get_file_info(exe_path);
}

fn run_ldd_analysis(exe_path: &str) {
    println!("📚 Shared library dependencies (ldd):");
    
    let output = Command::new("ldd")
        .arg(exe_path)
        .output();
    
    match output {
        Ok(output) => {
            let ldd_output = String::from_utf8_lossy(&output.stdout);
            let mut lib_count = 0;
            
            for line in ldd_output.lines() {
                if line.contains(".so") {
                    lib_count += 1;
                    println!("  📦 {}", line.trim());
                }
            }
            
            println!("📊 Total libraries: {}", lib_count);
        }
        Err(e) => {
            println!("  ❌ ldd failed: {}", e);
        }
    }
}

fn run_nm_analysis(exe_path: &str) {
    println!("🔍 Symbol analysis (nm):");
    
    let output = Command::new("nm")
        .args(&["-D", exe_path])
        .output();
    
    match output {
        Ok(output) => {
            let nm_output = String::from_utf8_lossy(&output.stdout);
            let mut symbol_counts = std::collections::HashMap::new();
            
            for line in nm_output.lines() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    let symbol_type = parts[1];
                    *symbol_counts.entry(symbol_type.to_string()).or_insert(0) += 1;
                }
            }
            
            println!("📊 Symbol counts:");
            for (symbol_type, count) in symbol_counts {
                let description = match symbol_type.as_str() {
                    "T" => "Text (code)",
                    "D" => "Data",
                    "B" => "BSS (uninitialized)",
                    "U" => "Undefined (external)",
                    "W" => "Weak",
                    _ => "Other",
                };
                println!("  {} ({}): {}", symbol_type, description, count);
            }
        }
        Err(e) => {
            println!("  ❌ nm failed: {}", e);
        }
    }
}

fn get_file_info(exe_path: &str) {
    println!("📋 File information:");
    
    if let Ok(metadata) = fs::metadata(exe_path) {
        println!("  📏 Size: {} bytes", metadata.len());
        
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            println!("  🔐 Permissions: {:o}", metadata.permissions().mode());
        }
    }
    
    // Run file command for more details
    let output = Command::new("file")
        .arg(exe_path)
        .output();
    
    if let Ok(output) = output {
        let file_output = String::from_utf8_lossy(&output.stdout);
        println!("  🔍 Type: {}", file_output.trim());
    }
}
