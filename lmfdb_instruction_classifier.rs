// LMFDB classification of instruction patterns
// Assign: orbit, weight, level, conductor, modular forms

use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;
use anyhow::Result;

#[derive(Debug, Clone)]
struct InstructionLMFDB {
    pattern: Vec<u8>,
    orbit: u32,        // Equivalence class of similar patterns
    weight: u32,       // Complexity measure
    level: u32,        // Depth in call graph
    conductor: u32,    // Importance score
    modular_form: String, // Semantic classification
}

fn main() -> Result<()> {
    println!("🔬 LMFDB classification of instruction patterns");
    
    // Sample more from nix store
    println!("📊 Sampling /nix/store...");
    let samples = sample_nix_store(50)?;
    println!("✅ Found {} libraries", samples.len());
    
    let mut all_patterns: HashMap<Vec<u8>, u64> = HashMap::new();
    let mut name_contexts: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
    
    for (i, path) in samples.iter().enumerate() {
        if let Ok(patterns) = extract_patterns(path) {
            println!("  {:3}. {} - {} patterns", 
                i+1,
                std::path::Path::new(&path).file_name().unwrap().to_str().unwrap(),
                patterns.len());
            
            for (pat, names) in patterns {
                *all_patterns.entry(pat.clone()).or_insert(0) += 1;
                name_contexts.entry(pat).or_default().extend(names);
            }
        }
    }
    
    println!("\n✅ Total unique patterns: {}", all_patterns.len());
    
    // Compute LMFDB invariants
    let mut classified: Vec<InstructionLMFDB> = all_patterns.iter()
        .map(|(pat, count)| {
            let names = name_contexts.get(pat).cloned().unwrap_or_default();
            classify_pattern(pat, *count, &names)
        })
        .collect();
    
    // Sort by conductor (importance)
    classified.sort_by(|a, b| b.conductor.cmp(&a.conductor));
    
    println!("\n🎯 Top 50 patterns by LMFDB conductor:\n");
    for (i, inst) in classified.iter().take(50).enumerate() {
        let hex: String = inst.pattern.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join(" ");
        println!("{:3}. {} | orbit:{} weight:{} level:{} conductor:{} form:{}",
            i+1, hex, inst.orbit, inst.weight, inst.level, inst.conductor, inst.modular_form);
    }
    
    // Group by modular form
    let mut by_form: HashMap<String, Vec<&InstructionLMFDB>> = HashMap::new();
    for inst in &classified {
        by_form.entry(inst.modular_form.clone()).or_default().push(inst);
    }
    
    println!("\n📊 Distribution by modular form:\n");
    let mut forms: Vec<_> = by_form.iter().collect();
    forms.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    
    for (form, insts) in forms.iter().take(20) {
        let avg_conductor: u32 = insts.iter().map(|i| i.conductor).sum::<u32>() / insts.len() as u32;
        println!("  {:20} - {:6} patterns (avg conductor: {})", 
            form, insts.len(), avg_conductor);
    }
    
    // Save results
    let output = serde_json::json!({
        "total_patterns": classified.len(),
        "total_samples": samples.len(),
        "lmfdb_classified": classified.iter().take(500).map(|inst| {
            serde_json::json!({
                "pattern": inst.pattern,
                "orbit": inst.orbit,
                "weight": inst.weight,
                "level": inst.level,
                "conductor": inst.conductor,
                "modular_form": inst.modular_form
            })
        }).collect::<Vec<_>>()
    });
    
    let output_path = "data/perf_rankings/lmfdb_instruction_classification.json";
    fs::write(output_path, serde_json::to_string_pretty(&output)?)?;
    println!("\n💾 Saved to: {}", output_path);
    
    Ok(())
}

fn sample_nix_store(n: usize) -> Result<Vec<String>> {
    let output = std::process::Command::new("find")
        .args(["/nix/store", "-maxdepth", "3", "-type", "f", "-name", "*.so*"])
        .output()?;
    
    let paths: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .take(n)
        .map(|s| s.to_string())
        .collect();
    
    Ok(paths)
}

fn extract_patterns(path: &str) -> Result<HashMap<Vec<u8>, Vec<String>>> {
    let buffer = fs::read(path)?;
    let elf = Elf::parse(&buffer)?;
    
    let mut patterns = HashMap::new();
    
    for sym in elf.dynsyms.iter() {
        if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
            if !name.is_empty() && sym.st_value != 0 && sym.st_size > 0 {
                let offset = sym.st_value as usize;
                let size = sym.st_size as usize;
                
                if offset < buffer.len() && offset + size <= buffer.len() {
                    let func_bytes = &buffer[offset..offset + size.min(32)];
                    
                    // Sample 4-byte patterns
                    for start in (0..func_bytes.len().min(16)).step_by(4) {
                        let end = (start + 4).min(func_bytes.len());
                        if end > start {
                            let pat = func_bytes[start..end].to_vec();
                            patterns.entry(pat).or_insert_with(Vec::new).push(name.to_string());
                        }
                    }
                }
            }
        }
    }
    
    Ok(patterns)
}

fn classify_pattern(pattern: &[u8], count: u64, names: &[String]) -> InstructionLMFDB {
    // Orbit: hash of pattern modulo 1000
    let orbit = pattern.iter().fold(0u32, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as u32)) % 1000;
    
    // Weight: number of non-zero bytes
    let weight = pattern.iter().filter(|&&b| b != 0).count() as u32;
    
    // Level: based on count (frequency)
    let level = match count {
        0..=10 => 1,
        11..=100 => 2,
        101..=1000 => 3,
        _ => 4,
    };
    
    // Conductor: LMFDB-style scoring
    let base = 3000;
    let length_score = pattern.len() as u32 * 10;
    let weight_score = weight * 100;
    let frequency_score = (count as u32).min(1000);
    let name_score = names.len() as u32 * 50;
    
    let conductor = base + length_score + weight_score + frequency_score + name_score;
    
    // Modular form: semantic classification
    let modular_form = if pattern.starts_with(&[0xf3, 0x0f]) {
        "endbr64".to_string()
    } else if pattern.starts_with(&[0xc3]) {
        "ret".to_string()
    } else if pattern.starts_with(&[0x48, 0x89]) {
        "mov_r64".to_string()
    } else if pattern.starts_with(&[0x48, 0x8b]) {
        "mov_load".to_string()
    } else if pattern.starts_with(&[0x41, 0x57]) || pattern.starts_with(&[0x41, 0x55]) {
        "prologue".to_string()
    } else if pattern.starts_with(&[0x0f, 0x1f]) {
        "nop_pad".to_string()
    } else if pattern.iter().all(|&b| b == 0) {
        "zero_pad".to_string()
    } else if weight == 0 {
        "zero_pad".to_string()
    } else if weight == pattern.len() as u32 {
        "dense".to_string()
    } else {
        "mixed".to_string()
    };
    
    InstructionLMFDB {
        pattern: pattern.to_vec(),
        orbit,
        weight,
        level,
        conductor,
        modular_form,
    }
}
