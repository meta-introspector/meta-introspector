use goblin::elf::Elf;
use lmfdb_rust_mapping::grammar_extraction::GrammarExtractor;
use std::collections::HashMap;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use rustc - it has keyword parsing!
    let path = std::env::args().nth(1)
        .unwrap_or_else(|| "/nix/store/*/lib/librustc_driver*.so".to_string());
    
    let actual_path = if path.contains('*') {
        glob::glob(&path).ok()
            .and_then(|mut g| g.next())
            .and_then(|r| r.ok())
            .expect("No rustc_driver found")
    } else {
        std::path::PathBuf::from(path)
    };
    
    println!("🔍 Extracting characters from: {}\n", actual_path.display());
    
    let binary_data = fs::read(&actual_path)?;
    let elf = Elf::parse(&binary_data)?;
    
    let text_section = elf.section_headers.iter()
        .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text")
        .unwrap();
    
    let text_start = text_section.sh_offset as usize;
    let text_size = (text_section.sh_size as usize).min(1_000_000);
    let text_bytes = &binary_data[text_start..text_start + text_size];
    
    let mut extractor = GrammarExtractor::new();
    let states = extractor.extract_dfa(text_bytes);
    
    println!("Found {} DFA states\n", states.len());
    
    let mut all_chars: HashMap<u8, usize> = HashMap::new();
    
    for state in &states {
        for (&byte, _) in &state.transitions {
            *all_chars.entry(byte).or_insert(0) += 1;
        }
    }
    
    println!("📊 All characters found (byte values):\n");
    let mut chars: Vec<_> = all_chars.iter().collect();
    chars.sort_by_key(|&(b, _)| b);
    
    for (byte, count) in &chars {
        if **byte >= 32 && **byte <= 126 {
            println!("  '{}' (0x{:02x}): {} times", **byte as char, byte, count);
        } else {
            println!("  0x{:02x}: {} times", byte, count);
        }
    }
    
    // Show actual sequences
    println!("\n🔤 Character sequences in DFA states:\n");
    for (i, state) in states.iter().take(10).enumerate() {
        let chars: String = state.transitions.keys()
            .filter(|&&b| b >= 32 && b <= 126)
            .map(|&b| b as char)
            .collect();
        if !chars.is_empty() {
            println!("  State {}: {}", i, chars);
        }
    }
    
    Ok(())
}
