// Direct test driver for build functions
// Bypasses server to test build logic directly

mod error_store;
use error_store::*;

use std::process::Command;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: test_driver <binary_name> [--nix] [--perf]");
        std::process::exit(1);
    }
    
    let target = &args[1];
    let use_nix = args.contains(&"--nix".to_string());
    let use_perf = args.contains(&"--perf".to_string());
    
    println!("🔨 Testing build: {}", target);
    if use_nix {
        println!("📦 Using nix develop");
    }
    if use_perf {
        println!("📊 Recording perf data");
    }
    println!("");
    
    let start = Instant::now();
    
    // Start perf recording if requested
    let perf_file = format!("/tmp/build_{}_{}.perf", target, std::process::id());
    let mut perf_child = if use_perf {
        println!("🔍 Starting perf record...");
        Some(Command::new("perf")
            .args(["record", "-o", &perf_file, "-a", "-g"])
            .spawn()
            .ok())
    } else {
        None
    };
    
    // Give perf time to start
    if use_perf {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
    
    // Run cargo build (with or without nix)
    let output = if use_nix {
        Command::new("nix")
            .args(["develop", "-c", "cargo", "build", "--bin", target])
            .output()
            .expect("Failed to run nix develop")
    } else {
        Command::new("cargo")
            .args(["build", "--bin", target])
            .output()
            .expect("Failed to run cargo")
    };
    
    let duration = start.elapsed();
    
    // Stop perf
    if let Some(Some(mut child)) = perf_child {
        std::thread::sleep(std::time::Duration::from_millis(100));
        let _ = child.kill();
        let _ = child.wait();
        
        if std::path::Path::new(&perf_file).exists() {
            println!("📊 Perf data saved to: {}", perf_file);
            
            // Generate perf report
            let report = Command::new("perf")
                .args(["report", "-i", &perf_file, "--stdio", "-n"])
                .output();
            
            if let Ok(output) = report {
                let report_text = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = report_text.lines().take(20).collect();
                if !lines.is_empty() {
                    println!("\n📈 Top functions:");
                    for line in lines {
                        if line.contains('%') {
                            println!("   {}", line.trim());
                        }
                    }
                }
            }
        }
    }
    
    println!("⏱️  Build took: {:.2}s", duration.as_secs_f64());
    println!("");
    
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    
    // Parse errors
    let errors = parse_errors(&stderr);
    
    println!("📊 Found {} errors", errors.len());
    println!("");
    
    // Store and suggest fixes
    for error in &errors {
        let build_error = BuildError {
            bin: target.to_string(),
            error_type: error.error_type.clone(),
            message: error.message.clone(),
            file: Some(error.file.clone()),
            line: error.line,
            suggestion: suggest_fix(&BuildError {
                bin: target.to_string(),
                error_type: error.error_type.clone(),
                message: error.message.clone(),
                file: Some(error.file.clone()),
                line: error.line,
                suggestion: None,
            }),
        };
        
        add_error(build_error.clone());
        
        println!("❌ {} in {}:{}", 
            build_error.error_type,
            build_error.file.as_ref().unwrap_or(&"unknown".to_string()),
            build_error.line.unwrap_or(0)
        );
        println!("   {}", build_error.message);
        
        // Show lines around error
        if let Some(ref file) = build_error.file {
            if let Some(line_num) = build_error.line {
                if let Ok(content) = std::fs::read_to_string(file) {
                    let lines: Vec<&str> = content.lines().collect();
                    let start = (line_num as usize).saturating_sub(2);
                    let end = ((line_num as usize) + 2).min(lines.len());
                    
                    println!("");
                    for (i, line) in lines[start..end].iter().enumerate() {
                        let line_no = start + i + 1;
                        let marker = if line_no == line_num as usize { "→" } else { " " };
                        println!("   {} {:4} | {}", marker, line_no, line);
                    }
                    println!("");
                }
            }
        }
        
        if let Some(suggestion) = &build_error.suggestion {
            println!("   💡 {}", suggestion);
        }
        println!("");
    }
    
    // Print report
    if let Some(report) = get_report() {
        println!("📈 Error Summary:");
        for (error_type, errors) in &report.by_type {
            println!("  {} x{}", error_type, errors.len());
        }
    }
}

fn parse_errors(stderr: &str) -> Vec<CompileError> {
    let mut errors = Vec::new();
    let lines: Vec<&str> = stderr.lines().collect();
    
    for (i, line) in lines.iter().enumerate() {
        if line.contains("error[E") {
            let error_type = line.split("error[")
                .nth(1)
                .and_then(|s| s.split(']').next())
                .unwrap_or("unknown")
                .to_string();
            
            let message = line.split("]: ")
                .nth(1)
                .unwrap_or(line)
                .to_string();
            
            // Parse file and line from next line (format: " --> file.rs:123:45")
            let (file, line_num) = if i + 1 < lines.len() {
                let next = lines[i + 1];
                if next.contains(" --> ") {
                    let parts: Vec<&str> = next.split(" --> ").collect();
                    if parts.len() > 1 {
                        let location = parts[1];
                        let file_parts: Vec<&str> = location.split(':').collect();
                        let file = file_parts[0].to_string();
                        let line = file_parts.get(1)
                            .and_then(|s| s.parse::<u32>().ok());
                        (file, line)
                    } else {
                        ("unknown".to_string(), None)
                    }
                } else {
                    ("unknown".to_string(), None)
                }
            } else {
                ("unknown".to_string(), None)
            };
            
            errors.push(CompileError {
                error_type: format!("E{}", error_type),
                message,
                file,
                line: line_num,
            });
        }
    }
    
    errors
}

#[derive(Debug, Clone)]
struct CompileError {
    error_type: String,
    message: String,
    file: String,
    line: Option<u32>,
}
