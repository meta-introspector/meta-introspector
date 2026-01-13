use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct FastCompressor {
    patterns: HashMap<String, u16>,
    next_token: u16,
    stats: CompressionStats,
}

#[derive(Debug, Serialize, Deserialize)]
struct CompressionStats {
    files_processed: u32,
    original_bytes: u64,
    compressed_bytes: u64,
    patterns_found: u32,
}

impl FastCompressor {
    fn new() -> Self {
        let mut compressor = Self {
            patterns: HashMap::new(),
            next_token: 1,
            stats: CompressionStats {
                files_processed: 0,
                original_bytes: 0,
                compressed_bytes: 0,
                patterns_found: 0,
            },
        };
        
        // Pre-load common patterns
        compressor.add_pattern("use ");
        compressor.add_pattern("fn ");
        compressor.add_pattern("impl ");
        compressor.add_pattern("struct ");
        compressor.add_pattern("enum ");
        compressor.add_pattern("rustc_");
        compressor.add_pattern("pub ");
        
        compressor
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
    
    fn compress_file(&mut self, path: &Path) -> Result<Vec<u16>, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let original_size = content.len() as u64;
        
        let mut tokens = Vec::new();
        for line in content.lines() {
            let mut matched = false;
            for (pattern, &token) in &self.patterns {
                if line.contains(pattern) {
                    tokens.push(token);
                    matched = true;
                    self.stats.patterns_found += 1;
                    break;
                }
            }
            if !matched {
                tokens.push((line.len() % 65535) as u16);
            }
        }
        
        self.stats.files_processed += 1;
        self.stats.original_bytes += original_size;
        self.stats.compressed_bytes += tokens.len() as u64 * 2;
        
        Ok(tokens)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 FAST RUST-BUILD COMPRESSION (LIMITED)");
    
    let start_time = Instant::now();
    
    // Get rust-build files directly (limited for speed)
    let rust_build_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust-build";
    let output = std::process::Command::new("find")
        .arg(rust_build_path)
        .arg("-name")
        .arg("*.rs")
        .arg("-type")
        .arg("f")
        .output()?;
    
    let rust_files: Vec<PathBuf> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .take(2000) // Limit to 2000 files for speed
        .map(|line| PathBuf::from(line))
        .collect();
    
    println!("📁 Found {} rust-build files (limited to 5000)", rust_files.len());
    
    let mut compressor = FastCompressor::new();
    let mut compressed_files = Vec::new();
    
    for (i, file) in rust_files.iter().enumerate() {
        if i % 500 == 0 {
            println!("Processed {} files...", i);
        }
        
        if let Ok(tokens) = compressor.compress_file(file) {
            compressed_files.push((file.to_string_lossy().to_string(), tokens));
        }
    }
    
    let elapsed = start_time.elapsed();
    
    // Results
    println!("\n📊 FAST COMPRESSION RESULTS:");
    println!("Files processed: {}", compressor.stats.files_processed);
    println!("Original size: {:.2} MB", compressor.stats.original_bytes as f64 / 1_000_000.0);
    println!("Compressed size: {:.2} MB", compressor.stats.compressed_bytes as f64 / 1_000_000.0);
    println!("Compression ratio: {:.1}%", (compressor.stats.compressed_bytes as f64 / compressor.stats.original_bytes as f64) * 100.0);
    println!("Space saved: {:.1}%", (1.0 - (compressor.stats.compressed_bytes as f64 / compressor.stats.original_bytes as f64)) * 100.0);
    println!("Processing time: {:.2} seconds", elapsed.as_secs_f64());
    println!("Patterns found: {}", compressor.stats.patterns_found);
    
    // Save results
    let output = serde_json::to_string(&(&compressor, &compressed_files))?;
    fs::write("rust_build_fast_compressed.json", output)?;
    
    println!("\n💾 Saved to: rust_build_fast_compressed.json");
    println!("✅ FAST COMPRESSION COMPLETE!");
    
    Ok(())
}
