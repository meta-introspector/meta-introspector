//! Analyze perf traces for duplicates
//! Detects duplicate code execution via eBPF signatures

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::Write;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Duplicate {
    signature: u64,
    count: u64,
    locations: Vec<Location>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    file: String,
    line: u32,
    function: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DuplicateReport {
    total_instructions: u64,
    unique_instructions: u64,
    duplicates: Vec<Duplicate>,
    duplication_rate: f64,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: analyze-duplicates <perf.data> [perf.data...]");
        std::process::exit(1);
    }
    
    let mut signatures: HashMap<u64, Vec<Location>> = HashMap::new();
    let mut total_instructions = 0u64;
    
    // Parse all perf traces
    for perf_file in &args[1..] {
        parse_perf_trace(perf_file, &mut signatures, &mut total_instructions);
    }
    
    // Find duplicates
    let duplicates: Vec<Duplicate> = signatures.iter()
        .filter(|(_, locs)| locs.len() > 1)
        .map(|(sig, locs)| Duplicate {
            signature: *sig,
            count: locs.len() as u64,
            locations: locs.clone(),
        })
        .collect();
    
    let unique_instructions = signatures.len() as u64;
    let duplication_rate = if total_instructions > 0 {
        (total_instructions - unique_instructions) as f64 / total_instructions as f64
    } else {
        0.0
    };
    
    let report = DuplicateReport {
        total_instructions,
        unique_instructions,
        duplicates,
        duplication_rate,
    };
    
    // Output JSON
    let json = serde_json::to_string_pretty(&report).unwrap();
    println!("{}", json);
    
    // Exit with error if duplicates found
    if !report.duplicates.is_empty() {
        eprintln!("❌ Found {} duplicate signatures", report.duplicates.len());
        std::process::exit(1);
    }
}

fn parse_perf_trace(path: &str, signatures: &mut HashMap<u64, Vec<Location>>, total: &mut u64) {
    // Parse perf.data using perf script
    use std::process::Command;
    
    let output = Command::new("perf")
        .args(&["script", "-i", path])
        .output()
        .expect("Failed to run perf script");
    
    let content = String::from_utf8_lossy(&output.stdout);
    
    for line in content.lines() {
        if line.trim().is_empty() {
            continue;
        }
        
        // Parse: "command  pid [cpu] timestamp: ip symbol (file:line)"
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            continue;
        }
        
        // Extract instruction pointer
        let ip_str = parts.iter()
            .find(|p| p.starts_with("0x") || p.parse::<u64>().is_ok())
            .unwrap_or(&"0");
        
        let ip = u64::from_str_radix(ip_str.trim_start_matches("0x"), 16)
            .unwrap_or(0);
        
        if ip == 0 {
            continue;
        }
        
        *total += 1;
        
        // Compute signature (FNV-1a hash of IP)
        let signature = compute_signature(ip);
        
        // Extract location
        let location = extract_location(line);
        
        signatures.entry(signature)
            .or_insert_with(Vec::new)
            .push(location);
    }
}

fn compute_signature(ip: u64) -> u64 {
    const FNV_PRIME: u64 = 0x100000001b3;
    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    
    let mut hash = FNV_OFFSET;
    hash ^= ip;
    hash = hash.wrapping_mul(FNV_PRIME);
    hash
}

fn extract_location(line: &str) -> Location {
    // Try to extract file:line and function name
    let file = line.split('(')
        .nth(1)
        .and_then(|s| s.split(')').next())
        .unwrap_or("unknown")
        .to_string();
    
    let line_num = file.split(':')
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);
    
    let function = line.split_whitespace()
        .last()
        .unwrap_or("unknown")
        .to_string();
    
    Location {
        file: file.split(':').next().unwrap_or("unknown").to_string(),
        line: line_num,
        function,
    }
}
