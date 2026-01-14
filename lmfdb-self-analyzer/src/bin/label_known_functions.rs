use goblin::elf::Elf;
use lmfdb_rust_mapping::grammar_extraction::GrammarExtractor;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rustc_path = "/nix/store/5r3salsfkfdyyl28c58dyk6sml48vklr-rust-default-1.94.0-nightly-2026-01-09/lib/librustc_driver-b3621a07141c9b94.so";
    
    println!("🏷️  Labeling known functions in rustc_driver\n");
    
    let binary_data = fs::read(rustc_path)?;
    let elf = Elf::parse(&binary_data)?;
    
    // Find text section
    let text_section = elf.section_headers.iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
        .unwrap();
    
    let text_start = text_section.sh_offset as usize;
    let text_bytes = &binary_data[text_start..];
    
    // Known keyword-checking functions (we know these parse keywords)
    let known_labels = vec![
        ("keyword", vec!['e', 'n', 'u', 'm']),  // enum
        ("keyword", vec!['i', 'm', 'p', 'l']),  // impl
        ("keyword", vec!['l', 'o', 'o', 'p']),  // loop
        ("keyword", vec!['t', 'r', 'a', 'i', 't']),  // trait
        ("keyword", vec!['b', 'r', 'e', 'a', 'k']),  // break
    ];
    
    let mut extractor = GrammarExtractor::new();
    let mut labeled_examples = Vec::new();
    
    // Search for functions that might be lexers
    let lexer_patterns = ["lex", "token", "keyword", "parse", "ident"];
    
    for sym in elf.syms.iter() {
        if sym.st_size == 0 { continue; }
        
        let name = elf.strtab.get_at(sym.st_name).unwrap_or("");
        let is_lexer = lexer_patterns.iter().any(|p| name.to_lowercase().contains(p));
        
        if is_lexer && sym.st_value >= text_section.sh_addr {
            let func_start = (sym.st_value - text_section.sh_addr) as usize;
            let func_size = sym.st_size as usize;
            
            if func_start + func_size <= text_bytes.len() {
                let func_bytes = &text_bytes[func_start..func_start + func_size];
                let states = extractor.extract_dfa(func_bytes);
                
                if !states.is_empty() {
                    // Build character sequence by following transitions
                    let mut sequences = Vec::new();
                    
                    for state in &states {
                        for (&byte, &next_state) in &state.transitions {
                            if byte >= 32 && byte <= 126 {
                                let ch = byte as char;
                                // Try to build sequence
                                let mut seq = vec![ch];
                                let mut current = next_state;
                                
                                // Follow up to 10 transitions
                                for _ in 0..10 {
                                    if let Some(next) = states.iter().find(|s| s.state_id == current) {
                                        if let Some((&next_byte, &next_next)) = next.transitions.iter().next() {
                                            if next_byte >= 32 && next_byte <= 126 {
                                                seq.push(next_byte as char);
                                                current = next_next;
                                            } else {
                                                break;
                                            }
                                        } else {
                                            break;
                                        }
                                    } else {
                                        break;
                                    }
                                }
                                
                                if seq.len() > 1 {
                                    sequences.push(seq);
                                }
                            }
                        }
                    }
                    
                    if !sequences.is_empty() {
                        println!("📍 Function: {}", name);
                        println!("   States: {}", states.len());
                        println!("   Sequences: {:?}", sequences.iter().map(|s| s.iter().collect::<String>()).collect::<Vec<_>>());
                        
                        // Check if matches known keyword
                        for (label, keyword) in &known_labels {
                            for seq in &sequences {
                                if seq == keyword {
                                    println!("   ✅ EXACT MATCH: {:?}", keyword.iter().collect::<String>());
                                    labeled_examples.push((name.to_string(), label.to_string(), keyword.clone()));
                                }
                            }
                        }
                        println!();
                    }
                }
            }
        }
    }
    
    println!("\n📊 Labeled Examples: {}", labeled_examples.len());
    for (func, label, chars) in &labeled_examples {
        println!("  {} → {}: {:?}", func, label, chars.iter().collect::<String>());
    }
    
    Ok(())
}
