// Partial execution via Markov sampling of instruction sequences
// Sample depth-N instruction chains from functions
// Map signature → byte patterns → find fixed points

use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;
use anyhow::Result;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct InstructionPattern {
    bytes: Vec<u8>,
    depth: usize,
}

#[derive(Debug)]
struct FunctionSignature {
    name: String,
    addr: u64,
    size: u64,
    instruction_chains: Vec<InstructionPattern>,
}

fn main() -> Result<()> {
    println!("🔬 Markov instruction sampling - finding fixed points");
    
    let depth = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(4);
    
    println!("📊 Sampling depth: {}", depth);
    
    // Load libraries from our study (they actually exist)
    let build_data = fs::read_to_string("data/build_analysis/real_build_1768332029_libraries.json")?;
    let json: serde_json::Value = serde_json::from_str(&build_data)?;
    
    let mut all_patterns: HashMap<InstructionPattern, u64> = HashMap::new();
    let mut signature_to_patterns: HashMap<String, Vec<InstructionPattern>> = HashMap::new();
    
    if let Some(libs) = json["libraries"].as_array() {
        for lib in libs.iter().take(5) { // Sample first 5 libraries
            if let Some(path) = lib.as_str() {
                if let Ok(patterns) = sample_function_patterns(path, depth) {
                    println!("  📦 {} - {} patterns", 
                        std::path::Path::new(path).file_name().unwrap().to_str().unwrap(),
                        patterns.len());
                    
                    for (sig, pats) in patterns {
                        signature_to_patterns.entry(sig.clone()).or_default().extend(pats.clone());
                        for pat in pats {
                            *all_patterns.entry(pat).or_insert(0) += 1;
                        }
                    }
                }
            }
        }
    }
    
    println!("\n✅ Total unique patterns: {}", all_patterns.len());
    
    // Find fixed points - patterns that appear frequently
    let mut ranked: Vec<_> = all_patterns.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("\n🎯 Top 30 fixed point patterns (most common):\n");
    for (i, (pattern, count)) in ranked.iter().take(30).enumerate() {
        let hex: String = pattern.bytes.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{:3}. [depth {}] {} (count: {})", 
            i+1, pattern.depth, hex, count);
    }
    
    // Find signature convergence - signatures with similar patterns
    println!("\n🔗 Signature convergence (top 10):\n");
    let mut sig_counts: Vec<_> = signature_to_patterns.iter()
        .map(|(sig, pats)| (sig, pats.len()))
        .collect();
    sig_counts.sort_by(|a, b| b.1.cmp(&a.1));
    
    for (i, (sig, count)) in sig_counts.iter().take(10).enumerate() {
        println!("{:3}. {} - {} patterns", i+1, sig, count);
    }
    
    // Save results
    let output = serde_json::json!({
        "depth": depth,
        "total_patterns": ranked.len(),
        "fixed_points": ranked.iter().take(100).map(|(pat, count)| {
            serde_json::json!({
                "bytes": pat.bytes,
                "depth": pat.depth,
                "count": count
            })
        }).collect::<Vec<_>>()
    });
    
    let output_path = format!("data/perf_rankings/markov_patterns_depth{}.json", depth);
    fs::write(&output_path, serde_json::to_string_pretty(&output)?)?;
    println!("\n💾 Saved to: {}", output_path);
    
    Ok(())
}

fn sample_function_patterns(path: &str, depth: usize) -> Result<HashMap<String, Vec<InstructionPattern>>> {
    let buffer = fs::read(path)?;
    let elf = Elf::parse(&buffer)?;
    
    let mut patterns = HashMap::new();
    
    // Sample from dynamic symbols
    for sym in elf.dynsyms.iter() {
        if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
            if !name.is_empty() && sym.st_value != 0 && sym.st_size > 0 {
                let offset = sym.st_value as usize;
                let size = sym.st_size as usize;
                
                if offset < buffer.len() && offset + size <= buffer.len() {
                    let func_bytes = &buffer[offset..offset + size.min(256)]; // Max 256 bytes
                    let chains = sample_instruction_chains(func_bytes, depth);
                    patterns.insert(name.to_string(), chains);
                }
            }
        }
    }
    
    Ok(patterns)
}

fn sample_instruction_chains(bytes: &[u8], depth: usize) -> Vec<InstructionPattern> {
    let mut chains = Vec::new();
    
    // Sample every N bytes as potential instruction start
    let step = 4; // x86_64 instructions are variable length, sample every 4 bytes
    
    for start in (0..bytes.len()).step_by(step) {
        for d in 1..=depth {
            let end = (start + d * step).min(bytes.len());
            if end > start {
                chains.push(InstructionPattern {
                    bytes: bytes[start..end].to_vec(),
                    depth: d,
                });
            }
        }
    }
    
    chains
}
