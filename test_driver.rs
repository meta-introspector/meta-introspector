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
