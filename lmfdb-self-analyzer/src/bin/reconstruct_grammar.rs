use lmfdb_rust_mapping::grammar_extraction::*;
use goblin::elf::Elf;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binary_path = "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust/compiler/zombie_driver2/target/debug/librustc_driver.so";
    
    println!("🔬 Reconstructing Rust Grammar from rustc_driver.so\n");
    
    let binary_data = std::fs::read(binary_path)?;
    let elf = Elf::parse(&binary_data)?;
    
    // Find parser functions
    let parser_symbols: Vec<_> = elf.syms.iter()
        .filter(|sym| {
            if let Some(name) = elf.strtab.get_at(sym.st_name) {
                name.contains("parse") || name.contains("lex") || name.contains("token")
            } else {
                false
            }
        })
        .take(20)
        .collect();
    
    let text_section = elf.section_headers.iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
        .expect("No .text section");
    
    let text_start = text_section.sh_offset as usize;
    let text_size = text_section.sh_size as usize;
    let text_bytes = &binary_data[text_start..text_start + text_size.min(binary_data.len() - text_start)];
    
    let mut extractor = GrammarExtractor::new();
    
    println!("📚 Extracted Grammars:\n");
    
    for (i, sym) in parser_symbols.iter().enumerate() {
        if sym.st_size == 0 || sym.st_value < text_section.sh_addr {
            continue;
        }
        
        let name = elf.strtab.get_at(sym.st_name).unwrap_or("unknown");
        let func_start = (sym.st_value - text_section.sh_addr) as usize;
        let func_size = (sym.st_size as usize).min(512);
        
        if func_start + func_size <= text_bytes.len() {
            let func_bytes = &text_bytes[func_start..func_start + func_size];
            let states = extractor.extract_dfa(func_bytes);
            
            if !states.is_empty() {
                let pattern = extractor.extract_grammar(states.clone());
                
                println!("{}. Function: {}", i+1, name.chars().take(70).collect::<String>());
                println!("   LMFDB Label: {}", pattern.lmfdb_label.to_string());
                println!("   States: {}", states.len());
                println!("   Alphabet: {} chars", pattern.alphabet.len());
                
                // Show DFA structure
                println!("   DFA Transitions:");
                for (j, state) in states.iter().take(5).enumerate() {
                    println!("     State {}: {} transitions", j, state.transitions.len());
                    for (char, next) in state.transitions.iter().take(3) {
                        let ch = if char.is_ascii_graphic() { 
                            *char as char 
                        } else { 
                            '?' 
                        };
                        println!("       '{}' (0x{:02x}) → state {}", ch, char, next);
                    }
                }
                
                // Generate sample strings
                println!("   Generated Strings:");
                let samples = generate_strings(&states, 5);
                for sample in samples {
                    println!("     \"{}\"", sample);
                }
                
                println!();
            }
        }
    }
    
    Ok(())
}

fn generate_strings(states: &[lmfdb_rust_mapping::grammar_extraction::DFAState], max_len: usize) -> Vec<String> {
    let mut results = Vec::new();
    
    if states.is_empty() {
        return results;
    }
    
    // Start from first state
    let start_state = &states[0];
    
    // Generate strings by following transitions
    for (char, next_state_id) in start_state.transitions.iter().take(3) {
        let mut s = String::new();
        s.push(*char as char);
        
        // Follow chain
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
    
    results
}
