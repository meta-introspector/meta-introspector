// Map Markov character patterns in function names to instruction patterns
// Find correlations: name substrings → byte sequences

use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;
use anyhow::Result;

#[derive(Debug)]
struct NameToInstructionMapping {
    name_pattern: String,
    instruction_pattern: Vec<u8>,
    correlation_count: u64,
}

fn main() -> Result<()> {
    println!("🔬 Mapping name patterns → instruction patterns");
    
    let depth = 2; // Sample 2-byte instruction patterns
    let name_ngram = 3; // 3-character name patterns
    
    println!("📊 Name n-gram: {}, Instruction depth: {}", name_ngram, depth);
    
    // Load libraries - use nix store samples if available, otherwise build data
    let lib_file = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "data/build_analysis/real_build_1768332029_libraries.json".to_string());
    
    let build_data = fs::read_to_string(&lib_file)?;
    let json: serde_json::Value = serde_json::from_str(&build_data)?;
    
    // Map: name_pattern → instruction_pattern → count
    let mut correlations: HashMap<(String, Vec<u8>), u64> = HashMap::new();
    
    if let Some(libs) = json["libraries"].as_array() {
        for lib in libs.iter().take(10) { // Sample up to 10 libraries
            if let Some(path) = lib.as_str() {
                if let Ok(mappings) = extract_name_instruction_mappings(path, name_ngram, depth) {
                    println!("  📦 {} - {} mappings", 
                        std::path::Path::new(path).file_name().unwrap().to_str().unwrap(),
                        mappings.len());
                    
                    for (name_pat, inst_pat) in mappings {
                        *correlations.entry((name_pat, inst_pat)).or_insert(0) += 1;
                    }
                }
            }
        }
    }
    
    println!("\n✅ Total correlations: {}", correlations.len());
    
    // Find strongest correlations
    let mut ranked: Vec<_> = correlations.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("\n🎯 Top 50 name → instruction correlations:\n");
    for (i, ((name_pat, inst_pat), count)) in ranked.iter().take(50).enumerate() {
        let hex: String = inst_pat.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{:3}. \"{}\" → {} (count: {})", i+1, name_pat, hex, count);
    }
    
    // Group by name pattern
    let mut by_name: HashMap<String, Vec<(Vec<u8>, u64)>> = HashMap::new();
    for ((name_pat, inst_pat), count) in &ranked {
        by_name.entry(name_pat.clone()).or_default().push((inst_pat.clone(), *count));
    }
    
    println!("\n🔗 Name patterns with multiple instruction mappings:\n");
    let mut multi: Vec<_> = by_name.iter()
        .filter(|(_, v)| v.len() > 1)
        .collect();
    multi.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    
    for (i, (name_pat, inst_pats)) in multi.iter().take(20).enumerate() {
        println!("{:3}. \"{}\" → {} different instructions", 
            i+1, name_pat, inst_pats.len());
        for (inst, count) in inst_pats.iter().take(3) {
            let hex: String = inst.iter()
                .map(|b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(" ");
            println!("       {} ({})", hex, count);
        }
    }
    
    // Save results
    let output = serde_json::json!({
        "name_ngram": name_ngram,
        "instruction_depth": depth,
        "total_correlations": ranked.len(),
        "top_mappings": ranked.iter().take(200).map(|((name, inst), count)| {
            serde_json::json!({
                "name_pattern": name,
                "instruction_bytes": inst,
                "count": count
            })
        }).collect::<Vec<_>>()
    });
    
    let output_path = "data/perf_rankings/name_instruction_mappings.json";
    fs::write(output_path, serde_json::to_string_pretty(&output)?)?;
    println!("\n💾 Saved to: {}", output_path);
    
    Ok(())
}

fn extract_name_instruction_mappings(
    path: &str, 
    name_ngram: usize, 
    depth: usize
) -> Result<Vec<(String, Vec<u8>)>> {
    let buffer = fs::read(path)?;
    let elf = Elf::parse(&buffer)?;
    
    let mut mappings = Vec::new();
    
    // Extract from dynamic symbols
    for sym in elf.dynsyms.iter() {
        if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
            if !name.is_empty() && sym.st_value != 0 && sym.st_size > 0 {
                let offset = sym.st_value as usize;
                let size = sym.st_size as usize;
                
                if offset < buffer.len() && offset + size <= buffer.len() {
                    // Extract name n-grams
                    let name_patterns = extract_ngrams(name, name_ngram);
                    
                    // Extract instruction patterns from function start
                    let func_bytes = &buffer[offset..offset + size.min(64)];
                    let inst_patterns = sample_instruction_patterns(func_bytes, depth);
                    
                    // Create all combinations
                    for name_pat in &name_patterns {
                        for inst_pat in &inst_patterns {
                            mappings.push((name_pat.clone(), inst_pat.clone()));
                        }
                    }
                }
            }
        }
    }
    
    Ok(mappings)
}

fn extract_ngrams(s: &str, n: usize) -> Vec<String> {
    let chars: Vec<char> = s.chars().collect();
    if chars.len() < n {
        return vec![s.to_string()];
    }
    
    chars.windows(n)
        .map(|w| w.iter().collect())
        .collect()
}

fn sample_instruction_patterns(bytes: &[u8], depth: usize) -> Vec<Vec<u8>> {
    let mut patterns = Vec::new();
    let step = 4;
    
    for start in (0..bytes.len().min(32)).step_by(step) {
        let end = (start + depth).min(bytes.len());
        if end > start {
            patterns.push(bytes[start..end].to_vec());
        }
    }
    
    patterns
}
