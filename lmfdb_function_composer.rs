// LMFDB Function Composer - Label and compose functions using LMFDB patterns

use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;
use anyhow::Result;

#[derive(Debug, Clone)]
struct FunctionComposition {
    name: String,
    addr: u64,
    size: u64,
    lmfdb_signature: String,  // Sequence of modular forms
    conductor_sum: u32,        // Total importance
    orbit_sequence: Vec<u32>,  // Pattern orbits
    complexity: u32,           // Derived from weights
}

fn main() -> Result<()> {
    let input_file = std::env::args().nth(1)
        .unwrap_or_else(|| "data/build_analysis/real_build_1768332029_libraries.json".to_string());
    let output_file = std::env::args().nth(2)
        .unwrap_or_else(|| "data/nix_lmfdb_analysis/function_compositions.json".to_string());
    
    println!("🔬 LMFDB Function Composer");
    println!("📊 Input: {}", input_file);
    
    let data = fs::read_to_string(&input_file)?;
    let json: serde_json::Value = serde_json::from_str(&data)?;
    
    let mut all_functions = Vec::new();
    let mut pattern_stats: HashMap<String, u64> = HashMap::new();
    
    if let Some(libs) = json["libraries"].as_array() {
        for (i, lib) in libs.iter().enumerate() {
            if let Some(path) = lib.as_str() {
                if let Ok(functions) = compose_functions(path) {
                    println!("  {:3}. {} - {} functions", 
                        i+1,
                        std::path::Path::new(path).file_name().unwrap().to_str().unwrap(),
                        functions.len());
                    
                    for func in &functions {
                        *pattern_stats.entry(func.lmfdb_signature.clone()).or_insert(0) += 1;
                    }
                    
                    all_functions.extend(functions);
                }
            }
        }
    }
    
    println!("\n✅ Composed {} functions", all_functions.len());
    
    // Sort by conductor (importance)
    all_functions.sort_by(|a, b| b.conductor_sum.cmp(&a.conductor_sum));
    
    println!("\n🎯 Top 30 functions by LMFDB conductor:\n");
    for (i, func) in all_functions.iter().take(30).enumerate() {
        println!("{:3}. {} | conductor:{} sig:{} complexity:{}",
            i+1, 
            func.name.chars().take(50).collect::<String>(),
            func.conductor_sum,
            func.lmfdb_signature,
            func.complexity);
    }
    
    // Most common signatures
    let mut sig_ranked: Vec<_> = pattern_stats.into_iter().collect();
    sig_ranked.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("\n📊 Top 20 LMFDB signatures:\n");
    for (i, (sig, count)) in sig_ranked.iter().take(20).enumerate() {
        println!("{:3}. {} (count: {})", i+1, sig, count);
    }
    
    // Save results
    let output = serde_json::json!({
        "total_functions": all_functions.len(),
        "unique_signatures": sig_ranked.len(),
        "functions": all_functions.iter().take(1000).map(|f| {
            serde_json::json!({
                "name": f.name,
                "addr": f.addr,
                "size": f.size,
                "lmfdb_signature": f.lmfdb_signature,
                "conductor_sum": f.conductor_sum,
                "orbit_sequence": f.orbit_sequence,
                "complexity": f.complexity
            })
        }).collect::<Vec<_>>(),
        "top_signatures": sig_ranked.iter().take(100).map(|(sig, count)| {
            serde_json::json!({"signature": sig, "count": count})
        }).collect::<Vec<_>>()
    });
    
    fs::write(&output_file, serde_json::to_string_pretty(&output)?)?;
    println!("\n💾 Saved to: {}", output_file);
    
    Ok(())
}

fn compose_functions(path: &str) -> Result<Vec<FunctionComposition>> {
    let buffer = fs::read(path)?;
    let elf = Elf::parse(&buffer)?;
    
    let mut functions = Vec::new();
    
    for sym in elf.dynsyms.iter() {
        if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
            if !name.is_empty() && sym.st_value != 0 && sym.st_size > 0 {
                let offset = sym.st_value as usize;
                let size = sym.st_size as usize;
                
                if offset < buffer.len() && offset + size <= buffer.len() {
                    let func_bytes = &buffer[offset..offset + size.min(128)];
                    let composition = analyze_function(name, sym.st_value, sym.st_size, func_bytes);
                    functions.push(composition);
                }
            }
        }
    }
    
    Ok(functions)
}

fn analyze_function(name: &str, addr: u64, size: u64, bytes: &[u8]) -> FunctionComposition {
    let mut signature_parts = Vec::new();
    let mut orbit_sequence = Vec::new();
    let mut conductor_sum = 0u32;
    let mut complexity = 0u32;
    
    // Sample 4-byte patterns
    for start in (0..bytes.len().min(64)).step_by(4) {
        let end = (start + 4).min(bytes.len());
        if end > start {
            let pattern = &bytes[start..end];
            
            // Classify pattern
            let (form, orbit, weight, conductor) = classify_instruction(pattern);
            
            signature_parts.push(form);
            orbit_sequence.push(orbit);
            conductor_sum += conductor;
            complexity += weight;
        }
    }
    
    // Create signature string (first 8 forms)
    let lmfdb_signature = signature_parts.iter()
        .take(8)
        .map(|s| s.chars().next().unwrap_or('?'))
        .collect::<String>();
    
    FunctionComposition {
        name: name.to_string(),
        addr,
        size,
        lmfdb_signature,
        conductor_sum,
        orbit_sequence,
        complexity,
    }
}

fn classify_instruction(pattern: &[u8]) -> (String, u32, u32, u32) {
    let orbit = pattern.iter().fold(0u32, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as u32)) % 1000;
    let weight = pattern.iter().filter(|&&b| b != 0).count() as u32;
    
    let base = 3000;
    let conductor = base + pattern.len() as u32 * 10 + weight * 100;
    
    let form = if pattern.starts_with(&[0xf3, 0x0f]) {
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
    
    (form, orbit, weight, conductor)
}
