#!/usr/bin/env rust
//! Binary Byte Attribution System
//! 
//! Traces every byte in the final binary back to:
//! - Source line that caused it
//! - Git commit that introduced it
//! - Author who wrote it
//! - Cost (instructions) of that byte

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
struct BinaryByteAttribution {
    language: String,
    binary_path: String,
    total_bytes: usize,
    byte_map: Vec<ByteSource>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ByteSource {
    offset: usize,
    byte_value: u8,
    source_file: String,
    source_line: usize,
    source_code: String,
    git_commit: String,
    git_author: String,
    commit_date: String,
    instruction_cost: u64,
}

impl BinaryByteAttribution {
    fn new(language: String, binary_path: String) -> Self {
        Self {
            language,
            binary_path,
            total_bytes: 0,
            byte_map: Vec::new(),
        }
    }
    
    /// Read binary and map each byte to source
    fn analyze_binary(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Read binary bytes
        let bytes = fs::read(&self.binary_path)?;
        self.total_bytes = bytes.len();
        
        println!("Analyzing {} bytes from {}", self.total_bytes, self.binary_path);
        
        // Use objdump to get disassembly with source lines
        let objdump = Command::new("objdump")
            .args(["-d", "-l", "-S", &self.binary_path])
            .output()?;
        
        let disasm = String::from_utf8_lossy(&objdump.stdout);
        
        // Parse objdump output to map addresses to source
        let mut addr_to_source = HashMap::new();
        let mut current_file = String::new();
        let mut current_line = 0;
        
        for line in disasm.lines() {
            // Match source file:line annotations
            if line.contains(".rs:") || line.contains(".c:") {
                if let Some((file, rest)) = line.split_once(':') {
                    current_file = file.trim().to_string();
                    if let Some(line_num) = rest.split_whitespace().next() {
                        current_line = line_num.parse().unwrap_or(0);
                    }
                }
            }
            
            // Match instruction addresses
            if let Some(addr_str) = line.split_whitespace().next() {
                if addr_str.ends_with(':') {
                    let addr = u64::from_str_radix(addr_str.trim_end_matches(':'), 16).ok();
                    if let Some(a) = addr {
                        addr_to_source.insert(a, (current_file.clone(), current_line));
                    }
                }
            }
        }
        
        // Map each byte to source
        for (offset, &byte) in bytes.iter().enumerate() {
            let addr = offset as u64;
            
            if let Some((file, line)) = addr_to_source.get(&addr) {
                let (commit, author, date, code) = self.get_git_blame(file, *line);
                
                self.byte_map.push(ByteSource {
                    offset,
                    byte_value: byte,
                    source_file: file.clone(),
                    source_line: *line,
                    source_code: code,
                    git_commit: commit,
                    git_author: author,
                    commit_date: date,
                    instruction_cost: 1, // Simplified: 1 byte ≈ 1 instruction
                });
            }
        }
        
        Ok(())
    }
    
    /// Get git blame for a specific line
    fn get_git_blame(&self, file: &str, line: usize) -> (String, String, String, String) {
        let output = Command::new("git")
            .args(["blame", "-L", &format!("{},{}", line, line), "--porcelain", file])
            .output()
            .ok();
        
        if let Some(out) = output {
            let text = String::from_utf8_lossy(&out.stdout);
            let mut commit = String::new();
            let mut author = String::new();
            let mut date = String::new();
            let mut code = String::new();
            
            for line in text.lines() {
                if line.len() == 40 && line.chars().all(|c| c.is_ascii_hexdigit()) {
                    commit = line[..8].to_string();
                }
                if line.starts_with("author ") {
                    author = line.strip_prefix("author ").unwrap_or("").to_string();
                }
                if line.starts_with("author-time ") {
                    if let Some(ts) = line.strip_prefix("author-time ") {
                        date = ts.to_string();
                    }
                }
                if line.starts_with('\t') {
                    code = line.trim().to_string();
                }
            }
            
            (commit, author, date, code)
        } else {
            ("unknown".to_string(), "unknown".to_string(), "0".to_string(), "".to_string())
        }
    }
    
    /// Generate attribution report
    fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str(&format!("# Binary Byte Attribution: {}\n\n", self.language));
        report.push_str(&format!("Binary: {}\n", self.binary_path));
        report.push_str(&format!("Total Bytes: {}\n", self.total_bytes));
        report.push_str(&format!("Attributed Bytes: {}\n\n", self.byte_map.len()));
        
        // Group by author
        let mut author_bytes: HashMap<String, usize> = HashMap::new();
        for byte in &self.byte_map {
            *author_bytes.entry(byte.git_author.clone()).or_insert(0) += 1;
        }
        
        report.push_str("## Bytes by Author\n\n");
        let mut authors: Vec<_> = author_bytes.iter().collect();
        authors.sort_by(|a, b| b.1.cmp(a.1));
        
        for (author, count) in authors.iter().take(10) {
            let pct = (**count as f64 / self.total_bytes as f64) * 100.0;
            report.push_str(&format!("- {}: {} bytes ({:.1}%)\n", author, count, pct));
        }
        
        // Group by commit
        let mut commit_bytes: HashMap<String, usize> = HashMap::new();
        for byte in &self.byte_map {
            *commit_bytes.entry(byte.git_commit.clone()).or_insert(0) += 1;
        }
        
        report.push_str("\n## Bytes by Commit\n\n");
        let mut commits: Vec<_> = commit_bytes.iter().collect();
        commits.sort_by(|a, b| b.1.cmp(a.1));
        
        for (commit, count) in commits.iter().take(10) {
            let pct = (**count as f64 / self.total_bytes as f64) * 100.0;
            report.push_str(&format!("- {}: {} bytes ({:.1}%)\n", commit, count, pct));
        }
        
        // Sample byte details
        report.push_str("\n## Sample Byte Attribution\n\n");
        for byte in self.byte_map.iter().take(20) {
            report.push_str(&format!("### Byte 0x{:04x} = 0x{:02x}\n", byte.offset, byte.byte_value));
            report.push_str(&format!("- Source: {}:{}\n", byte.source_file, byte.source_line));
            report.push_str(&format!("- Code: `{}`\n", byte.source_code));
            report.push_str(&format!("- Commit: {} by {}\n", byte.git_commit, byte.git_author));
            report.push_str(&format!("- Cost: {} instructions\n\n", byte.instruction_cost));
        }
        
        report
    }
    
    fn save_json(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
}

fn main() {
    println!("🔍 Binary Byte Attribution System");
    println!("==================================");
    println!();
    
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 3 {
        eprintln!("Usage: {} <language> <binary_path>", args[0]);
        eprintln!();
        eprintln!("Example:");
        eprintln!("  {} rust /nix/store/.../bin/const71", args[0]);
        std::process::exit(1);
    }
    
    let language = args[1].clone();
    let binary_path = args[2].clone();
    
    let mut attribution = BinaryByteAttribution::new(language.clone(), binary_path);
    
    println!("Analyzing binary...");
    if let Err(e) = attribution.analyze_binary() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
    
    println!("Generating report...");
    let report = attribution.generate_report();
    
    let report_path = format!("data-const71/attribution/{}_bytes.md", language);
    fs::create_dir_all("data-const71/attribution").ok();
    fs::write(&report_path, &report).ok();
    
    let json_path = format!("data-const71/attribution/{}_bytes.json", language);
    attribution.save_json(&json_path).ok();
    
    println!();
    println!("✅ Attribution complete!");
    println!("   Report: {}", report_path);
    println!("   JSON: {}", json_path);
    println!();
    println!("Summary:");
    println!("  Total bytes: {}", attribution.total_bytes);
    println!("  Attributed: {}", attribution.byte_map.len());
    println!("  Coverage: {:.1}%", 
        (attribution.byte_map.len() as f64 / attribution.total_bytes as f64) * 100.0
    );
}
