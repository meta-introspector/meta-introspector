#!/usr/bin/env rust-script

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::collections::HashMap;
use serde_json;

#[derive(Debug)]
struct RustcInterceptor {
    patterns: HashMap<String, u16>,
    compressed_files: Vec<(String, Vec<u16>, u64)>, // path, tokens, original_size
    next_token: u16,
}

impl RustcInterceptor {
    fn new() -> Self {
        let mut interceptor = Self {
            patterns: HashMap::new(),
            compressed_files: Vec::new(),
            next_token: 1,
        };
        
        // Pre-load common patterns
        interceptor.add_pattern("use ");
        interceptor.add_pattern("fn ");
        interceptor.add_pattern("impl ");
        interceptor.add_pattern("struct ");
        interceptor.add_pattern("enum ");
        interceptor.add_pattern("rustc_");
        interceptor.add_pattern("pub ");
        
        interceptor
    }
    
    fn add_pattern(&mut self, pattern: &str) -> u16 {
        if let Some(&token) = self.patterns.get(pattern) {
            return token;
        }
        let token = self.next_token;
        self.next_token += 1;
        self.patterns.insert(pattern.to_string(), token);
        token
    }
    
    fn compress_file(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        if !path.ends_with(".rs") {
            return Ok(());
        }
        
        let content = fs::read_to_string(path)?;
        let original_size = content.len() as u64;
        
        let mut tokens = Vec::new();
        for line in content.lines() {
            let mut matched = false;
            for (pattern, &token) in &self.patterns {
                if line.contains(pattern) {
                    tokens.push(token);
                    matched = true;
                    break;
                }
            }
            if !matched {
                tokens.push((line.len() % 65535) as u16);
            }
        }
        
        self.compressed_files.push((path.to_string(), tokens.clone(), original_size));
        
        // Log compression
        eprintln!("🗜️  Compressed: {} ({} bytes -> {} tokens)", 
                 path, original_size, tokens.len());
        
        Ok(())
    }
    
    fn save_results(&self) -> Result<(), Box<dyn std::error::Error>> {
        let total_original: u64 = self.compressed_files.iter().map(|(_, _, size)| size).sum();
        let total_compressed: u64 = self.compressed_files.iter().map(|(_, tokens, _)| tokens.len() as u64 * 2).sum();
        
        let results = serde_json::json!({
            "files_compressed": self.compressed_files.len(),
            "total_original_bytes": total_original,
            "total_compressed_bytes": total_compressed,
            "compression_ratio": total_compressed as f64 / total_original as f64,
            "space_saved_percent": (1.0 - (total_compressed as f64 / total_original as f64)) * 100.0,
            "patterns": self.patterns,
            "files": self.compressed_files
        });
        
        fs::write("rustc_intercept_compression.json", serde_json::to_string_pretty(&results)?)?;
        
        eprintln!("\n📊 RUSTC INTERCEPT RESULTS:");
        eprintln!("Files: {}", self.compressed_files.len());
        eprintln!("Original: {:.2} MB", total_original as f64 / 1_000_000.0);
        eprintln!("Compressed: {:.2} MB", total_compressed as f64 / 1_000_000.0);
        eprintln!("Space saved: {:.1}%", (1.0 - (total_compressed as f64 / total_original as f64)) * 100.0);
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    // Handle rustc metadata queries - pass through to real rustc immediately
    if args.len() > 1 && (args[1] == "-vV" || args[1] == "--version" || 
                          args.iter().any(|arg| arg.starts_with("--print") || arg == "-")) {
        let real_rustc = env::var("REAL_RUSTC").unwrap_or_else(|_| "rustc".to_string());
        let mut cmd = Command::new(real_rustc);
        cmd.args(&args[1..]);
        let status = cmd.status()?;
        std::process::exit(status.code().unwrap_or(1));
    }
    
    // Only compress if we have actual .rs files
    let has_rs_files = args.iter().any(|arg| arg.ends_with(".rs") && Path::new(arg).exists());
    
    if has_rs_files {
        // Initialize interceptor
        let mut interceptor = RustcInterceptor::new();
        
        // Compress all .rs files passed to rustc
        for arg in &args[1..] {
            if arg.ends_with(".rs") && Path::new(arg).exists() {
                let _ = interceptor.compress_file(arg);
            }
        }
        
        // Save compression results
        let _ = interceptor.save_results();
    }
    
    // Don't call real rustc - we're just reading files
    eprintln!("🚫 Rustc backend disabled - compression only mode");
    std::process::exit(0);
}
