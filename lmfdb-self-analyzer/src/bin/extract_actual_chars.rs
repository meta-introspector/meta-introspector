use goblin::elf::Elf;
use lmfdb_rust_mapping::grammar_extraction::GrammarExtractor;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Extracting actual characters from binary DFAs...\n");
    
    // Pick a few representative binaries
    let test_files = [
        "/nix/store/*/lib/libssl.so*",
        "/nix/store/*/lib/libcrypto.so*",
        "/nix/store/*/lib/libz.so*",
    ];
    
    let mut all_chars: HashMap<char, usize> = HashMap::new();
    let mut char_sequences: Vec<String> = Vec::new();
    let mut extractor = GrammarExtractor::new();
    
    for pattern in &test_files {
        let paths: Vec<_> = glob::glob(pattern)
            .ok()
            .and_then(|g| g.take(1).collect::<Result<Vec<_>, _>>().ok())
            .unwrap_or_default();
        
        for path in paths {
            println!("📂 Analyzing: {}", path.display());
            
            let binary_data = fs::read(&path)?;
            if let Ok(elf) = Elf::parse(&binary_data) {
                if let Some(text_section) = elf.section_headers.iter()
                    .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text") {
                    
                    let text_start = text_section.sh_offset as usize;
                    let text_size = (text_section.sh_size as usize).min(100_000);
                    
                    if text_start + text_size <= binary_data.len() {
                        let text_bytes = &binary_data[text_start..text_start + text_size];
                        
                        // Extract DFA states
                        let states = extractor.extract_dfa(text_bytes);
                        
                        // Collect all characters being checked
                        for state in &states {
                            let mut seq = String::new();
                            for (&byte, _) in &state.transitions {
                                if byte >= 32 && byte <= 126 {
                                    let ch = byte as char;
                                    *all_chars.entry(ch).or_insert(0) += 1;
                                    seq.push(ch);
                                }
                            }
                            if !seq.is_empty() {
                                char_sequences.push(seq);
                            }
                        }
                        
                        println!("   Found {} DFA states\n", states.len());
                    }
                }
            }
        }
    }
    
    println!("\n📊 Character Frequency (ASCII printable):\n");
    let mut freq_vec: Vec<_> = all_chars.iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(a.1));
    
    for (ch, count) in freq_vec.iter().take(50) {
        println!("  '{}': {} checks", ch.escape_default(), count);
    }
    
    println!("\n🔤 Character Sequences Found:\n");
    for seq in char_sequences.iter().take(20) {
        println!("  {}", seq);
    }
    
    // Look for specific words
    println!("\n🔍 Searching for keyword patterns:\n");
    let keywords = ["enum", "struct", "else", "if", "for"];
    for keyword in &keywords {
        let found: Vec<_> = char_sequences.iter()
            .filter(|s| s.to_lowercase().contains(keyword))
            .take(3)
            .collect();
        
        if !found.is_empty() {
            println!("  '{}' found in:", keyword);
            for seq in found {
                println!("    {}", seq);
            }
        }
    }
    
    Ok(())
}
