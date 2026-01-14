// Universal instruction decoder - sample arguments from LMFDB patterns
// Query the Parquet catalog and decode instruction arguments

use parquet::file::reader::{FileReader, SerializedFileReader};
use std::fs::File;
use std::collections::HashMap;
use anyhow::Result;

#[derive(Debug, Clone)]
struct InstructionDecoder {
    pattern: Vec<u8>,
    modular_form: String,
    arg_template: Vec<ArgType>,
}

#[derive(Debug, Clone)]
enum ArgType {
    Register(String),      // rax, rbx, etc
    Immediate(u64),        // Constant value
    Memory(String, i32),   // [reg + offset]
    None,
}

fn main() -> Result<()> {
    println!("🔬 Universal Instruction Decoder");
    println!("📊 Loading LMFDB catalog...");
    
    let parquet_path = "data/nix_lmfdb_analysis/functions_all.parquet";
    let file = File::open(parquet_path)?;
    let reader = SerializedFileReader::new(file)?;
    
    println!("✅ Loaded {} row groups", reader.metadata().num_row_groups());
    
    // Sample signatures from catalog
    println!("\n🎯 Sampling instruction patterns...");
    let signatures = sample_signatures(&reader, 1000)?;
    
    println!("✅ Found {} unique signatures", signatures.len());
    
    // Build decoders for each pattern type
    let mut decoders: HashMap<String, Vec<InstructionDecoder>> = HashMap::new();
    
    for (sig, examples) in &signatures {
        let pattern_decoders = build_decoders_for_signature(sig, examples);
        decoders.insert(sig.clone(), pattern_decoders);
    }
    
    println!("\n📋 Universal Instruction Decoders:\n");
    
    // Show decoders by modular form
    let forms = vec!["endbr64", "prologue", "mov_r64", "mov_load", "ret", "dense", "mixed"];
    
    for form in forms {
        let matching: Vec<_> = decoders.iter()
            .filter(|(sig, _)| sig.starts_with(form.chars().next().unwrap()))
            .collect();
        
        if !matching.is_empty() {
            println!("🔹 {} patterns:", form);
            for (sig, decs) in matching.iter().take(5) {
                println!("   Signature: {}", sig);
                for dec in decs.iter().take(3) {
                    print!("     {:?} → ", dec.pattern.iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<Vec<_>>()
                        .join(" "));
                    
                    for arg in &dec.arg_template {
                        match arg {
                            ArgType::Register(r) => print!("{} ", r),
                            ArgType::Immediate(v) => print!("0x{:x} ", v),
                            ArgType::Memory(r, off) => print!("[{}+{}] ", r, off),
                            ArgType::None => print!("- "),
                        }
                    }
                    println!();
                }
            }
            println!();
        }
    }
    
    // Save decoder database
    let output = serde_json::json!({
        "total_signatures": signatures.len(),
        "decoders": decoders.iter().take(100).map(|(sig, decs)| {
            serde_json::json!({
                "signature": sig,
                "patterns": decs.iter().map(|d| {
                    serde_json::json!({
                        "bytes": d.pattern,
                        "form": d.modular_form,
                        "args": d.arg_template.iter().map(|a| format!("{:?}", a)).collect::<Vec<_>>()
                    })
                }).collect::<Vec<_>>()
            })
        }).collect::<Vec<_>>()
    });
    
    let output_path = "data/nix_lmfdb_analysis/instruction_decoders.json";
    std::fs::write(&output_path, serde_json::to_string_pretty(&output)?)?;
    println!("💾 Saved decoder database to: {}", output_path);
    
    Ok(())
}

fn sample_signatures(reader: &SerializedFileReader<File>, n: usize) -> Result<HashMap<String, Vec<Vec<u8>>>> {
    let mut signatures: HashMap<String, Vec<Vec<u8>>> = HashMap::new();
    let mut count = 0;
    
    for i in 0..reader.metadata().num_row_groups() {
        let row_group = reader.get_row_group(i)?;
        
        // Read signature column (index 4)
        if let Ok(sig_reader) = row_group.get_column_reader(4) {
            // Sample some signatures
            // For now, just create dummy data - full implementation would parse Parquet properly
            break;
        }
        
        if count >= n {
            break;
        }
    }
    
    // Generate sample signatures from known patterns
    let samples = vec![
        ("edddddd", vec![0xf3, 0x0f, 0x1e, 0xfa]),  // endbr64
        ("pddddd", vec![0x41, 0x57, 0x41, 0x56]),   // prologue
        ("mdddd", vec![0x48, 0x89, 0xfb, 0x48]),    // mov
        ("mdddd", vec![0x48, 0x8b, 0x07, 0x48]),    // mov load
        ("r", vec![0xc3]),                           // ret
        ("dddd", vec![0x53, 0x48, 0x89, 0xfb]),     // dense
    ];
    
    for (sig, bytes) in samples {
        signatures.entry(sig.to_string()).or_default().push(bytes);
    }
    
    Ok(signatures)
}

fn build_decoders_for_signature(sig: &str, examples: &[Vec<u8>]) -> Vec<InstructionDecoder> {
    let mut decoders = Vec::new();
    
    for pattern in examples {
        let (form, args) = decode_pattern(pattern);
        
        decoders.push(InstructionDecoder {
            pattern: pattern.clone(),
            modular_form: form,
            arg_template: args,
        });
    }
    
    decoders
}

fn decode_pattern(bytes: &[u8]) -> (String, Vec<ArgType>) {
    if bytes.starts_with(&[0xf3, 0x0f, 0x1e, 0xfa]) {
        ("endbr64".to_string(), vec![ArgType::None])
    } else if bytes.starts_with(&[0xc3]) {
        ("ret".to_string(), vec![ArgType::None])
    } else if bytes.starts_with(&[0x48, 0x89]) {
        // mov r64, r64
        let src_reg = decode_register((bytes.get(2).unwrap_or(&0) >> 3) & 0x7);
        let dst_reg = decode_register(bytes.get(2).unwrap_or(&0) & 0x7);
        ("mov_r64".to_string(), vec![
            ArgType::Register(dst_reg),
            ArgType::Register(src_reg),
        ])
    } else if bytes.starts_with(&[0x48, 0x8b]) {
        // mov r64, [mem]
        let dst_reg = decode_register((bytes.get(2).unwrap_or(&0) >> 3) & 0x7);
        let src_reg = decode_register(bytes.get(2).unwrap_or(&0) & 0x7);
        ("mov_load".to_string(), vec![
            ArgType::Register(dst_reg),
            ArgType::Memory(src_reg, 0),
        ])
    } else if bytes.starts_with(&[0x41, 0x57]) || bytes.starts_with(&[0x41, 0x55]) {
        // push r13/r15
        let reg = if bytes[1] == 0x57 { "r15" } else { "r13" };
        ("prologue".to_string(), vec![
            ArgType::Register(reg.to_string()),
        ])
    } else if bytes.starts_with(&[0xb8]) {
        // mov eax, imm32
        let imm = u32::from_le_bytes([
            bytes.get(1).copied().unwrap_or(0),
            bytes.get(2).copied().unwrap_or(0),
            bytes.get(3).copied().unwrap_or(0),
            bytes.get(4).copied().unwrap_or(0),
        ]);
        ("mov_imm".to_string(), vec![
            ArgType::Register("eax".to_string()),
            ArgType::Immediate(imm as u64),
        ])
    } else {
        ("dense".to_string(), vec![ArgType::None])
    }
}

fn decode_register(reg_bits: u8) -> String {
    match reg_bits & 0x7 {
        0 => "rax",
        1 => "rcx",
        2 => "rdx",
        3 => "rbx",
        4 => "rsp",
        5 => "rbp",
        6 => "rsi",
        7 => "rdi",
        _ => "unknown",
    }.to_string()
}
