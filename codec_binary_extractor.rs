use std::fs;
use std::collections::HashMap;
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::record::RowAccessor;
use goblin::elf::Elf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Extracting codec binary code from modular resonance positions...\n");
    
    // Load moonshine map
    let moonshine_text = fs::read_to_string("elf_moonshine_map.txt")?;
    let mut modulo_patterns: HashMap<u64, Vec<String>> = HashMap::new();
    
    let mut current_modulo = 0u64;
    for line in moonshine_text.lines() {
        if line.starts_with("mod ") {
            if let Some(num_str) = line.split_whitespace().nth(1) {
                current_modulo = num_str.parse().unwrap_or(0);
            }
        } else if line.starts_with("  - ") {
            let pattern = line.trim_start_matches("  - ").to_string();
            modulo_patterns.entry(current_modulo).or_insert_with(Vec::new).push(pattern);
        }
    }
    
    println!("✅ Loaded {} modular positions\n", modulo_patterns.len());
    
    // Load symbols to find files with high modular concentration
    let file = fs::File::open("markov_symbol_scores.parquet")?;
    let reader = SerializedFileReader::new(file)?;
    
    let mut file_modulo_hits: HashMap<String, HashMap<u64, usize>> = HashMap::new();
    
    for row in reader.get_row_iter(None)? {
        let row = row?;
        let file_path = row.get_string(1)?.to_string();
        let cell = row.get_ulong(2)?;
        
        for &modulo in modulo_patterns.keys() {
            if modulo > 0 && cell % modulo == 0 {
                *file_modulo_hits.entry(file_path.clone())
                    .or_insert_with(HashMap::new)
                    .entry(modulo)
                    .or_insert(0) += 1;
            }
        }
    }
    
    // Find files with strongest modular signatures
    let mut codec_candidates: Vec<(String, usize)> = file_modulo_hits.iter()
        .map(|(file, hits)| (file.clone(), hits.values().sum()))
        .collect();
    codec_candidates.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("🎯 Top 30 files with strongest modular signatures (likely codec code):");
    for (i, (file, hits)) in codec_candidates.iter().take(30).enumerate() {
        let basename = file.split('/').last().unwrap_or(file);
        println!("   {}. {} ({} modular hits)", i + 1, basename, hits);
    }
    
    // Extract binary code from top candidates
    println!("\n🔬 Extracting binary code from modular positions...");
    
    let mut codec_extracts = Vec::new();
    
    for (file_path, _) in codec_candidates.iter().take(10) {
        if let Ok(buffer) = fs::read(file_path) {
            if let Ok(elf) = Elf::parse(&buffer) {
                // Find .text section
                for section in &elf.section_headers {
                    if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
                        if name == ".text" {
                            let text_start = section.sh_offset as usize;
                            let text_size = section.sh_size as usize;
                            let text = &buffer[text_start..text_start + text_size];
                            
                            // Extract code at modular positions
                            for (&modulo, patterns) in &modulo_patterns {
                                if modulo > 0 && modulo < text_size as u64 {
                                    let pos = (modulo as usize) % text_size;
                                    let extract_size = 64.min(text_size - pos);
                                    let code_bytes = &text[pos..pos + extract_size];
                                    
                                    codec_extracts.push(CodecExtract {
                                        file: file_path.clone(),
                                        modulo,
                                        position: pos,
                                        bytes: code_bytes.to_vec(),
                                        patterns: patterns.clone(),
                                    });
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
    
    println!("   Extracted {} code segments from modular positions", codec_extracts.len());
    
    // Analyze extracted code for common patterns
    println!("\n📊 Analyzing codec patterns in extracted code...");
    
    let mut opcode_freq: HashMap<u8, usize> = HashMap::new();
    let mut bigram_freq: HashMap<(u8, u8), usize> = HashMap::new();
    
    for extract in &codec_extracts {
        for &byte in &extract.bytes {
            *opcode_freq.entry(byte).or_insert(0) += 1;
        }
        
        for window in extract.bytes.windows(2) {
            *bigram_freq.entry((window[0], window[1])).or_insert(0) += 1;
        }
    }
    
    let mut sorted_opcodes: Vec<_> = opcode_freq.iter().collect();
    sorted_opcodes.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n🎯 Top 20 opcodes in codec regions:");
    for (i, (&opcode, &count)) in sorted_opcodes.iter().take(20).enumerate() {
        let mnemonic = opcode_to_mnemonic(opcode);
        println!("   {}. 0x{:02x} {} ({} occurrences)", i + 1, opcode, mnemonic, count);
    }
    
    let mut sorted_bigrams: Vec<_> = bigram_freq.iter().collect();
    sorted_bigrams.sort_by(|a, b| b.1.cmp(a.1));
    
    println!("\n🎯 Top 20 opcode bigrams in codec regions:");
    for (i, (&(op1, op2), &count)) in sorted_bigrams.iter().take(20).enumerate() {
        println!("   {}. 0x{:02x}{:02x} ({} occurrences)", i + 1, op1, op2, count);
    }
    
    // Save results
    let mut output = String::from("Codec Binary Code Extraction\n\n");
    
    output.push_str("Top 50 files with modular signatures:\n");
    for (i, (file, hits)) in codec_candidates.iter().take(50).enumerate() {
        output.push_str(&format!("{}. {} ({} hits)\n", i + 1, file, hits));
    }
    
    output.push_str("\nExtracted code segments:\n");
    for (i, extract) in codec_extracts.iter().take(100).enumerate() {
        output.push_str(&format!("\n{}. {} @ mod {} (pos {})\n", 
            i + 1, extract.file.split('/').last().unwrap_or("?"), 
            extract.modulo, extract.position));
        output.push_str(&format!("   Patterns: {:?}\n", extract.patterns));
        output.push_str(&format!("   Bytes: {}\n", 
            extract.bytes.iter().take(32).map(|b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ")));
    }
    
    output.push_str("\nTop 100 opcodes:\n");
    for (i, (&opcode, &count)) in sorted_opcodes.iter().take(100).enumerate() {
        output.push_str(&format!("{}. 0x{:02x} {} ({})\n", 
            i + 1, opcode, opcode_to_mnemonic(opcode), count));
    }
    
    fs::write("codec_binary_extraction.txt", output)?;
    println!("\n💾 Saved to codec_binary_extraction.txt");
    
    Ok(())
}

#[derive(Debug, Clone)]
struct CodecExtract {
    file: String,
    modulo: u64,
    position: usize,
    bytes: Vec<u8>,
    patterns: Vec<String>,
}

fn opcode_to_mnemonic(opcode: u8) -> &'static str {
    match opcode {
        0x00 => "add",
        0x48 => "rex.w",
        0x49 => "rex.wb",
        0x4c => "rex.wr",
        0x50..=0x57 => "push",
        0x58..=0x5f => "pop",
        0x74 => "je",
        0x75 => "jne",
        0x83 => "add/sub",
        0x89 => "mov",
        0x8b => "mov",
        0xc3 => "ret",
        0xe8 => "call",
        0xff => "jmp/call",
        _ => "?",
    }
}
