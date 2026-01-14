use lmfdb_rust_mapping::grammar_extraction::*;
use lmfdb_rust_mapping::markov_conformity::*;
use goblin::elf::Elf;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";
    
    println!("🔬 Extracting Rust Grammar from rustc_driver.so");
    println!("Binary: {}", binary_path);
    println!("Size: 2.8GB\n");
    
    let binary_data = std::fs::read(binary_path)?;
    let elf = Elf::parse(&binary_data)?;
    
    println!("✅ Parsed ELF: {} symbols\n", elf.syms.len());
    
    // Extract grammar from parser functions
    let mut extractor = GrammarExtractor::new();
    let mut all_patterns = Vec::new();
    
    // Find .text section
    let text_section = elf.section_headers.iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
        .expect("No .text section");
    
    let text_start = text_section.sh_offset as usize;
    let text_size = text_section.sh_size as usize;
    let text_bytes = &binary_data[text_start..text_start + text_size.min(binary_data.len() - text_start)];
    
    println!("📊 Analyzing .text section ({} bytes)...\n", text_size);
    
    // Sample parser functions (look for parse/lex in symbol names)
    let parser_symbols: Vec<_> = elf.syms.iter()
        .filter(|sym| {
            if let Some(name) = elf.strtab.get_at(sym.st_name) {
                name.contains("parse") || name.contains("lex") || name.contains("token")
            } else {
                false
            }
        })
        .take(100) // Sample first 100 parser functions
        .collect();
    
    println!("🔍 Found {} parser/lexer functions", parser_symbols.len());
    println!("Extracting DFA patterns...\n");
    
    for (i, sym) in parser_symbols.iter().enumerate() {
        if sym.st_size == 0 || sym.st_value < text_section.sh_addr {
            continue;
        }
        
        let func_start = (sym.st_value - text_section.sh_addr) as usize;
        let func_size = (sym.st_size as usize).min(1024); // Sample first 1KB
        
        if func_start + func_size <= text_bytes.len() {
            let func_bytes = &text_bytes[func_start..func_start + func_size];
            let states = extractor.extract_dfa(func_bytes);
            
            if !states.is_empty() {
                let pattern = extractor.extract_grammar(states);
                
                if i < 10 {
                    let name = elf.strtab.get_at(sym.st_name).unwrap_or("unknown");
                    println!("  {}. {} → Grammar: {} (LMFDB: {})", 
                        i+1,
                        name.chars().take(60).collect::<String>(),
                        pattern.pattern_id,
                        pattern.lmfdb_label.to_string()
                    );
                }
                
                all_patterns.push(pattern);
            }
        }
    }
    
    println!("\n✅ Extracted {} grammar patterns\n", all_patterns.len());
    
    // Find most common patterns
    let mut pattern_counts: HashMap<String, usize> = HashMap::new();
    for pattern in &all_patterns {
        *pattern_counts.entry(pattern.lmfdb_label.to_string()).or_insert(0) += 1;
    }
    
    let mut sorted: Vec<_> = pattern_counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("🔝 Top 10 Grammar Patterns (by LMFDB label):");
    for (i, (label, count)) in sorted.iter().take(10).enumerate() {
        println!("  {}. {} - {} occurrences", i+1, label, count);
    }
    
    // Now do Markov conformity analysis
    println!("\n📈 Markov Conformity Analysis\n");
    
    let analyzer = MarkovAnalyzer::new();
    
    // Get symbol names
    let symbol_names: Vec<String> = elf.syms.iter()
        .filter_map(|sym| elf.strtab.get_at(sym.st_name))
        .map(|s| s.to_string())
        .take(1000)
        .collect();
    
    println!("Building Markov models from:");
    println!("  - Instructions: {} bytes", text_bytes.len());
    println!("  - Symbols: {} names", symbol_names.len());
    
    let inst_model = analyzer.model_from_instructions(&text_bytes[..100000]); // Sample 100KB
    let sym_model = analyzer.model_from_symbols(&symbol_names);
    
    println!("\n✅ Instruction model: {} states, LMFDB: {}", 
        inst_model.states.len(), 
        inst_model.lmfdb_label.to_string()
    );
    println!("✅ Symbol model: {} states, LMFDB: {}", 
        sym_model.states.len(),
        sym_model.lmfdb_label.to_string()
    );
    
    // Save results
    let output = serde_json::json!({
        "binary": binary_path,
        "grammar_patterns": all_patterns.len(),
        "top_patterns": sorted.iter().take(10).collect::<Vec<_>>(),
        "instruction_model": {
            "states": inst_model.states.len(),
            "lmfdb_label": inst_model.lmfdb_label.to_string(),
            "signature": inst_model.signature,
        },
        "symbol_model": {
            "states": sym_model.states.len(),
            "lmfdb_label": sym_model.lmfdb_label.to_string(),
            "signature": sym_model.signature,
        }
    });
    
    std::fs::write("rustc_grammar_extraction.json", serde_json::to_string_pretty(&output)?)?;
    println!("\n💾 Saved to: rustc_grammar_extraction.json");
    
    Ok(())
}
