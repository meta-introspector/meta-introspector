// Direct test driver for build functions
// Bypasses server to test build logic directly

mod error_store;
use error_store::*;

use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let target = args.get(1).expect("Usage: test_driver <binary_name>");
    
    println!("🔨 Testing build: {}", target);
    println!("");
    
    // Run cargo build
    let output = Command::new("cargo")
        .args(["build", "--bin", target])
        .output()
        .expect("Failed to run cargo");
    
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
    
    for line in stderr.lines() {
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
            
            errors.push(CompileError {
                error_type: format!("E{}", error_type),
                message,
                file: "unknown".to_string(),
                line: None,
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
