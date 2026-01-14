use lmfdb_rust_mapping::grammar_extraction::*;
use goblin::elf::Elf;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";
    
    println!("🔬 Complete Grammar Extraction from rustc_driver.so");
    println!("Finding ALL parser/lexer functions...\n");
    
    let binary_data = std::fs::read(binary_path)?;
    let elf = Elf::parse(&binary_data)?;
    
    // Find ALL functions (no filter)
    let all_symbols: Vec<_> = elf.syms.iter()
        .filter(|sym| sym.st_size > 0 && sym.st_value > 0)
        .collect();
    
    println!("✅ Found {} total functions\n", all_symbols.len());
    
    let text_section = elf.section_headers.iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
        .expect("No .text section");
    
    let text_start = text_section.sh_offset as usize;
    let text_size = text_section.sh_size as usize;
    let text_bytes = &binary_data[text_start..text_start + text_size.min(binary_data.len() - text_start)];
    
    let mut extractor = GrammarExtractor::new();
    let mut grammars_by_type: HashMap<String, Vec<Grammar>> = HashMap::new();
    
    println!("📚 Extracting grammars and labeling data types...\n");
    
    for (i, sym) in all_symbols.iter().enumerate() {
        if sym.st_size == 0 || sym.st_value < text_section.sh_addr {
            continue;
        }
        
        let name = elf.strtab.get_at(sym.st_name).unwrap_or("unknown");
        let func_start = (sym.st_value - text_section.sh_addr) as usize;
        let func_size = (sym.st_size as usize).min(1024);
        
        if func_start + func_size <= text_bytes.len() {
            let func_bytes = &text_bytes[func_start..func_start + func_size];
            let states = extractor.extract_dfa(func_bytes);
            
            if !states.is_empty() {
                let pattern = extractor.extract_grammar(states.clone());
                
                // Infer data type from function name
                let data_type = infer_data_type(name);
                
                let grammar = Grammar {
                    function_name: name.to_string(),
                    lmfdb_label: pattern.lmfdb_label.to_string(),
                    states: states.len(),
                    alphabet_size: pattern.alphabet.len(),
                    data_type: data_type.clone(),
                    sample_strings: generate_strings(&states, 3),
                };
                
                grammars_by_type.entry(data_type).or_insert_with(Vec::new).push(grammar);
                
                if i % 10000 == 0 {
                    println!("  Processed {}/{} functions...", i, all_symbols.len());
                }
            }
        }
    }
    
    println!("\n✅ Extracted {} grammars\n", 
        grammars_by_type.values().map(|v| v.len()).sum::<usize>());
    
    // Print by data type
    let mut sorted_types: Vec<_> = grammars_by_type.iter().collect();
    sorted_types.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    
    println!("📊 Grammars by Data Type:\n");
    
    for (data_type, grammars) in sorted_types.iter().take(20) {
        println!("{}. {} ({} grammars)", 
            data_type, 
            data_type,
            grammars.len()
        );
        
        // Show top 5 examples
        for (i, g) in grammars.iter().take(5).enumerate() {
            println!("   {}. {} → LMFDB: {}", 
                i+1,
                g.function_name.chars().take(60).collect::<String>(),
                g.lmfdb_label
            );
            if !g.sample_strings.is_empty() {
                println!("      Accepts: {:?}", g.sample_strings);
            }
        }
        println!();
    }
    
    // Save complete results
    let output = serde_json::json!({
        "total_functions": all_symbols.len(),
        "grammars_extracted": grammars_by_type.values().map(|v| v.len()).sum::<usize>(),
        "data_types": grammars_by_type.keys().collect::<Vec<_>>(),
        "grammars_by_type": grammars_by_type,
    });
    
    std::fs::write("rustc_complete_grammar.json", serde_json::to_string_pretty(&output)?)?;
    println!("💾 Saved complete results to: rustc_complete_grammar.json");
    
    Ok(())
}

#[derive(Debug, Clone, serde::Serialize)]
struct Grammar {
    function_name: String,
    lmfdb_label: String,
    states: usize,
    alphabet_size: usize,
    data_type: String,
    sample_strings: Vec<String>,
}

fn infer_data_type(function_name: &str) -> String {
    if function_name.contains("number") || function_name.contains("digit") || function_name.contains("int") {
        "Number".to_string()
    } else if function_name.contains("string") || function_name.contains("str") {
        "String".to_string()
    } else if function_name.contains("ident") || function_name.contains("name") {
        "Identifier".to_string()
    } else if function_name.contains("type") || function_name.contains("ty") {
        "Type".to_string()
    } else if function_name.contains("expr") {
        "Expression".to_string()
    } else if function_name.contains("stmt") {
        "Statement".to_string()
    } else if function_name.contains("path") {
        "Path".to_string()
    } else if function_name.contains("token") {
        "Token".to_string()
    } else if function_name.contains("attr") {
        "Attribute".to_string()
    } else if function_name.contains("pattern") || function_name.contains("pat") {
        "Pattern".to_string()
    } else if function_name.contains("literal") || function_name.contains("lit") {
        "Literal".to_string()
    } else if function_name.contains("time") || function_name.contains("date") {
        "DateTime".to_string()
    } else if function_name.contains("sign") {
        "Sign".to_string()
    } else if function_name.contains("whitespace") || function_name.contains("space") {
        "Whitespace".to_string()
    } else {
        "Unknown".to_string()
    }
}

fn generate_strings(states: &[lmfdb_rust_mapping::grammar_extraction::DFAState], max_len: usize) -> Vec<String> {
    let mut results = Vec::new();
    
    if states.is_empty() {
        return results;
    }
    
    let start_state = &states[0];
    
    for (char, next_state_id) in start_state.transitions.iter().take(3) {
        if char.is_ascii_graphic() {
            let mut s = String::new();
            s.push(*char as char);
            
            let mut current_id = *next_state_id;
            for _ in 0..max_len {
                if let Some(state) = states.iter().find(|s| s.state_id == current_id) {
                    if let Some((next_char, next_id)) = state.transitions.iter().next() {
                        if next_char.is_ascii_graphic() {
                            s.push(*next_char as char);
                        }
                        current_id = *next_id;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            
            if !s.is_empty() {
                results.push(s);
            }
        }
    }
    
    results
}
