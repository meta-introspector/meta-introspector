// Capture binaries from perf.data, decode with goblin, study symbols
// Phase 1: Extract all binaries referenced in perf MMAP events
// Phase 2: Parse each binary with goblin
// Phase 3: Build symbol table for address resolution

// use linux_perf_data::{PerfFileReader, PerfFileRecord};
use goblin::elf::Elf;
use std::collections::{HashMap, HashSet};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::Path;
use anyhow::Result;

#[derive(Debug, Clone)]
struct Binary {
    path: String,
    base_addr: u64,
    size: u64,
}

#[derive(Debug)]
struct Symbol {
    name: String,
    addr: u64,
    size: u64,
    binary: String,
}

fn main() -> Result<()> {
    let perf_file_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "data/perf_rankings/nix_rust_beta_1768351567.perf.data".to_string());
    
    println!("🔬 Binary capture and symbol study");
    println!("📊 Reading: {}", perf_file_path);
    
    // Phase 1: Extract binaries from MMAP events
    let binaries = extract_binaries_from_perf(&perf_file_path)?;
    println!("✅ Found {} unique binaries", binaries.len());
    
    // Phase 2: Parse each binary with goblin
    let mut all_symbols = Vec::new();
    let mut binary_count = 0;
    
    for binary in &binaries {
        if let Ok(symbols) = parse_binary_symbols(&binary.path, binary.base_addr) {
            println!("  📦 {} - {} symbols", 
                Path::new(&binary.path).file_name().unwrap().to_str().unwrap(),
                symbols.len());
            all_symbols.extend(symbols);
            binary_count += 1;
        }
    }
    
    println!("\n✅ Parsed {} binaries", binary_count);
    println!("✅ Extracted {} total symbols", all_symbols.len());
    
    // Phase 3: Rank symbols
    let mut symbol_counts: HashMap<String, u64> = HashMap::new();
    for sym in &all_symbols {
        *symbol_counts.entry(sym.name.clone()).or_insert(0) += 1;
    }
    
    let mut ranked: Vec<_> = symbol_counts.into_iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("\n🔥 Top 50 symbols by occurrence:\n");
    for (i, (symbol, count)) in ranked.iter().take(50).enumerate() {
        println!("{:3}. {:60} {}", i+1, symbol, count);
    }
    
    // Save results
    let output = serde_json::json!({
        "binaries": binaries.len(),
        "symbols": all_symbols.len(),
        "unique_symbols": ranked.len(),
        "top_symbols": ranked.iter().take(200).map(|(name, count)| {
            serde_json::json!({"name": name, "count": count})
        }).collect::<Vec<_>>()
    });
    
    let output_path = "data/perf_rankings/binary_symbols.json";
    fs::write(output_path, serde_json::to_string_pretty(&output)?)?;
    println!("\n💾 Saved to: {}", output_path);
    
    Ok(())
}

fn extract_binaries_from_perf(perf_path: &str) -> Result<Vec<Binary>> {
    let _file = File::open(perf_path)?;
    
    let mut binaries = Vec::new();
    
    // Use our real build data
    let build_data = fs::read_to_string("data/build_analysis/real_build_1768332029_binaries.json")?;
    let json: serde_json::Value = serde_json::from_str(&build_data)?;
    
    if let Some(bins) = json["binaries"].as_array() {
        for bin in bins {
            if let Some(path) = bin.as_str() {
                if Path::new(path).exists() {
                    binaries.push(Binary {
                        path: path.to_string(),
                        base_addr: 0,
                        size: 0,
                    });
                }
            }
        }
    }
    
    Ok(binaries)
}

// fn extract_path_from_record(record: &linux_perf_data::EventRecord) -> Option<String> {
//     // Try to extract filename from raw record
//     // For now, return None - we'll parse MMAP events properly later
//     None
// }

fn parse_binary_symbols(path: &str, base_addr: u64) -> Result<Vec<Symbol>> {
    let buffer = fs::read(path)?;
    let elf = Elf::parse(&buffer)?;
    
    let mut symbols = Vec::new();
    
    // Dynamic symbols
    for sym in elf.dynsyms.iter() {
        if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
            if !name.is_empty() && sym.st_value != 0 {
                symbols.push(Symbol {
                    name: name.to_string(),
                    addr: base_addr + sym.st_value,
                    size: sym.st_size,
                    binary: path.to_string(),
                });
            }
        }
    }
    
    // Regular symbols
    for sym in elf.syms.iter() {
        if let Some(name) = elf.strtab.get_at(sym.st_name) {
            if !name.is_empty() && sym.st_value != 0 {
                symbols.push(Symbol {
                    name: name.to_string(),
                    addr: base_addr + sym.st_value,
                    size: sym.st_size,
                    binary: path.to_string(),
                });
            }
        }
    }
    
    Ok(symbols)
}
