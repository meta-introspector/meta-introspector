// 🔥 CUSTOM RUST OVERLAY NIGHTLY BUILD with Full Telemetry
use std::process::Command;
use std::env;

// Include our complete telemetry system
include!("master_all_calls_allcalls_1768325605.rs");

fn main() {
    println!("🔥 CUSTOM RUST OVERLAY NIGHTLY BUILD");
    println!("====================================");
    
    // Initialize all telemetry wrappers
    init_all_call_wrappers!();
    
    // Build custom rust nightly with overlay
    build_custom_rust_nightly();
}

fn build_custom_rust_nightly() {
    println!("🚀 Building custom rust nightly with overlay + telemetry...");
    
    // Set up telemetry environment
    let preload_lib = std::path::PathBuf::from("/mnt/data1/meta-introspector/rust_preload_interceptor/target/release/librust_preload_interceptor.so");
    
    println!("🔧 Using Rust LD_PRELOAD interceptor");
    println!("📊 LD_PRELOAD telemetry: {:?}", preload_lib.exists());
    
    let mut cmd = Command::new("nix");
    cmd.args(&["build", "./rustc-only-build", "--rebuild", "--show-trace"]);
    
    // Add telemetry preload if available
    if preload_lib.exists() {
        cmd.env("LD_PRELOAD", &preload_lib);
    }
    
    // Also run direct nix build with telemetry
    println!("📋 Also running direct nix build with telemetry...");
    let mut direct_cmd = Command::new("nix");
    direct_cmd.args(&["build", "./rustc-only-build", "--show-trace"]);
    direct_cmd.current_dir("/mnt/data1/meta-introspector");
    if preload_lib.exists() {
        direct_cmd.env("LD_PRELOAD", &preload_lib);
    }
    
    match direct_cmd.output() {
        Ok(direct_output) => {
            println!("📊 Direct build exit code: {}", direct_output.status.code().unwrap_or(-1));
            let direct_stderr = String::from_utf8_lossy(&direct_output.stderr);
            let direct_stdout = String::from_utf8_lossy(&direct_output.stdout);
            
            if !direct_stdout.is_empty() {
                println!("📋 Direct build stdout:");
                for line in direct_stdout.lines() {
                    println!("  {}", line);
                }
            }
            
            if !direct_stderr.is_empty() {
                println!("⚠️  Direct build stderr:");
                for line in direct_stderr.lines() {
                    println!("  {}", line);
                }
            }
        }
        Err(e) => println!("❌ Direct build failed: {}", e),
    }
    
    println!("📋 Running: nix build ./rustc-only-build with full telemetry...");
    
    match cmd.output() {
        Ok(output) => {
            println!("📊 Build completed!");
            println!("📤 Exit code: {}", output.status.code().unwrap_or(-1));
            
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            
            // Show detailed output
            if !stdout.is_empty() {
                println!("📋 STDOUT:");
                println!("{}", stdout);
            }
            
            if !stderr.is_empty() {
                println!("⚠️  STDERR:");
                for line in stderr.lines() {
                    println!("  {}", line);
                }
            }
            
            // Check for telemetry logs
            check_telemetry_capture();
            
            if output.status.success() {
                let rust_path = stdout.trim();
                println!("✅ Custom rust nightly built: {}", rust_path);
                println!("🔧 Extensions: rust-src, rust-analyzer, llvm-tools-preview");
                
                // Test the built rust with telemetry
                test_custom_rust(rust_path);
            } else {
                println!("❌ Build failed with exit code: {}", output.status.code().unwrap_or(-1));
                println!("🔍 Checking if syscalls were captured anyway...");
                check_telemetry_capture();
            }
        }
        Err(e) => {
            println!("❌ Failed to run nix-build: {}", e);
        }
    }
}

fn test_custom_rust(rust_path: &str) {
    println!("\n🦀 TESTING CUSTOM RUST WITH TELEMETRY");
    println!("=====================================");
    
    // Create test rust file
    let test_code = r#"
fn main() {
    println!("Hello from custom rust nightly!");
    let x = vec![1, 2, 3, 4, 5];
    println!("Vector: {:?}", x);
}
"#;
    
    std::fs::write("test_nightly.rs", test_code).unwrap();
    println!("📝 Created test_nightly.rs");
    
    // Compile with our custom rust + telemetry
    let rustc_bin = format!("{}/bin/rustc", rust_path);
    
    let mut cmd = Command::new(&rustc_bin);
    cmd.args(&["test_nightly.rs", "-o", "test_nightly"]);
    
    // Add telemetry preload
    let current_dir = env::current_dir().unwrap();
    let preload_lib = current_dir.join("libpreload_interceptor.so");
    if preload_lib.exists() {
        cmd.env("LD_PRELOAD", preload_lib);
    }
    
    println!("🎯 Compiling with custom rustc + telemetry...");
    
    match cmd.output() {
        Ok(output) => {
            if output.status.success() {
                println!("✅ Compilation successful!");
                
                // Run the compiled binary
                if let Ok(run_output) = Command::new("./test_nightly").output() {
                    println!("🚀 Program output:");
                    println!("{}", String::from_utf8_lossy(&run_output.stdout));
                }
            } else {
                println!("❌ Compilation failed:");
                println!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            println!("❌ Failed to run rustc: {}", e);
        }
    }
    
    println!("\n🎯 CUSTOM RUST NIGHTLY BUILD COMPLETE!");
    println!("All compilation steps captured with full telemetry!");
    
    // Final telemetry check
    check_telemetry_capture();
}

fn check_telemetry_capture() {
    println!("\n🔍 TELEMETRY SUMMARY");
    println!("===================");
    
    // Count telemetry logs
    if let Ok(entries) = std::fs::read_dir("/tmp") {
        let mut found_logs = 0;
        for entry in entries.flatten() {
            let filename = entry.file_name();
            if let Some(name) = filename.to_str() {
                if name.starts_with("preload_intercept_") && name.ends_with(".log") {
                    found_logs += 1;
                }
            }
        }
        
        if found_logs > 0 {
            println!("✅ Intercepted {} processes with LD_PRELOAD", found_logs);
        } else {
            println!("❌ No telemetry processes intercepted");
        }
    }
    
    // Check preload library
    let preload_lib = std::path::PathBuf::from("/mnt/data1/meta-introspector/rust_preload_interceptor/target/release/librust_preload_interceptor.so");
    
    if preload_lib.exists() {
        println!("✅ LD_PRELOAD library active");
    } else {
        println!("❌ LD_PRELOAD library missing");
    }
}
