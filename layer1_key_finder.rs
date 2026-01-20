use std::collections::HashMap;
use parquet::arrow::ParquetRecordBatchReaderBuilder;
use std::fs::File;

fn main() {
    println!("🔑 Layer 1 as Universal Key");
    
    // Compile Layer 1
    let layer1_signature = compile_and_extract_signature("zos/layer1/layer1.rs");
    println!("🔐 Layer 1 signature: {} bytes", layer1_signature.len());
    
    // Load 3M files
    let files = load_file_index("indexes/files.parquet");
    println!("📊 Scanning {} files", files.len());
    
    // Find all programs containing Layer 1 patterns
    let matches = find_layer1_matches(&files, &layer1_signature);
    println!("✅ Found {} programs containing Layer 1", matches.len());
    
    // Save matches
    save_matches(&matches, "layer1_matches.parquet");
    
    // Analyze distribution
    analyze_distribution(&matches);
}

fn compile_and_extract_signature(path: &str) -> Vec<u8> {
    use std::process::Command;
    
    println!("🔨 Compiling Layer 1...");
    
    // Compile to binary
    Command::new("rustc")
        .args(&[path, "-o", "/tmp/layer1"])
        .status()
        .expect("Failed to compile Layer 1");
    
    // Extract signature
    let binary = std::fs::read("/tmp/layer1").expect("Failed to read binary");
    
    // Extract key patterns:
    // - Function signatures
    // - Constant values
    // - Type layouts
    extract_signature(&binary)
}

fn extract_signature(binary: &[u8]) -> Vec<u8> {
    use goblin::Object;
    
    let obj = Object::parse(binary).expect("Failed to parse binary");
    
    let mut signature = Vec::new();
    
    match obj {
        Object::Elf(elf) => {
            // Extract symbol table
            for sym in elf.syms.iter() {
                signature.extend_from_slice(&sym.st_value.to_le_bytes());
                signature.extend_from_slice(&sym.st_size.to_le_bytes());
            }
            
            // Extract .rodata (constants)
            for section in elf.section_headers.iter() {
                if section.sh_type == goblin::elf::section_header::SHT_PROGBITS {
                    let data = &binary[section.sh_offset as usize..][..section.sh_size as usize];
                    signature.extend_from_slice(&data[..data.len().min(256)]);
                }
            }
        }
        _ => {}
    }
    
    // Hash signature
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    signature.hash(&mut hasher);
    hasher.finish().to_le_bytes().to_vec()
}

fn load_file_index(path: &str) -> Vec<String> {
    let mut files = Vec::new();
    
    let file = File::open(path).expect("Failed to open parquet");
    let builder = ParquetRecordBatchReaderBuilder::try_new(file).unwrap();
    let reader = builder.build().unwrap();
    
    for batch in reader.flatten() {
        // Extract file paths
    }
    
    files
}

fn find_layer1_matches(files: &[String], signature: &[u8]) -> Vec<Match> {
    use rayon::prelude::*;
    
    files.par_iter()
        .filter_map(|file| {
            if file.ends_with(".rs") {
                check_file_for_layer1(file, signature)
            } else {
                None
            }
        })
        .collect()
}

fn check_file_for_layer1(file: &str, signature: &[u8]) -> Option<Match> {
    let content = std::fs::read_to_string(file).ok()?;
    
    // Check for Layer 1 patterns
    let mut pattern_count = 0;
    
    // Constants
    if content.contains("const ") {
        pattern_count += content.matches("const ").count();
    }
    
    // Type aliases
    if content.contains("type ") {
        pattern_count += content.matches("type ").count();
    }
    
    // Function signatures
    if content.contains("fn ") {
        pattern_count += content.matches("fn ").count();
    }
    
    // Calculate similarity to Layer 1
    let similarity = calculate_similarity(&content, signature);
    
    if similarity > 0.5 {
        Some(Match {
            file_path: file.to_string(),
            pattern_count,
            similarity,
        })
    } else {
        None
    }
}

fn calculate_similarity(content: &str, signature: &[u8]) -> f64 {
    // Simple similarity: ratio of Layer 1 patterns
    let total_lines = content.lines().count() as f64;
    let pattern_lines = content.lines()
        .filter(|l| {
            let t = l.trim();
            t.starts_with("const ") || t.starts_with("type ") || t.starts_with("fn ")
        })
        .count() as f64;
    
    pattern_lines / total_lines.max(1.0)
}

fn save_matches(matches: &[Match], output: &str) {
    use arrow::array::{Float64Array, StringArray, UInt64Array};
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow::record_batch::RecordBatch;
    use parquet::arrow::ArrowWriter;
    use std::sync::Arc;
    
    let schema = Schema::new(vec![
        Field::new("file_path", DataType::Utf8, false),
        Field::new("pattern_count", DataType::UInt64, false),
        Field::new("similarity", DataType::Float64, false),
    ]);
    
    let file_paths: Vec<String> = matches.iter().map(|m| m.file_path.clone()).collect();
    let pattern_counts: Vec<u64> = matches.iter().map(|m| m.pattern_count as u64).collect();
    let similarities: Vec<f64> = matches.iter().map(|m| m.similarity).collect();
    
    let batch = RecordBatch::try_new(
        Arc::new(schema.clone()),
        vec![
            Arc::new(StringArray::from(file_paths)),
            Arc::new(UInt64Array::from(pattern_counts)),
            Arc::new(Float64Array::from(similarities)),
        ],
    ).unwrap();
    
    let file = File::create(output).unwrap();
    let mut writer = ArrowWriter::try_new(file, Arc::new(schema), None).unwrap();
    writer.write(&batch).unwrap();
    writer.close().unwrap();
    
    println!("💾 Saved {} matches to {}", matches.len(), output);
}

fn analyze_distribution(matches: &[Match]) {
    println!("\n📊 Distribution Analysis:");
    
    let high_similarity = matches.iter().filter(|m| m.similarity > 0.8).count();
    let medium_similarity = matches.iter().filter(|m| m.similarity > 0.5 && m.similarity <= 0.8).count();
    
    println!("  High similarity (>0.8): {}", high_similarity);
    println!("  Medium similarity (0.5-0.8): {}", medium_similarity);
    
    let avg_similarity: f64 = matches.iter().map(|m| m.similarity).sum::<f64>() / matches.len() as f64;
    println!("  Average similarity: {:.2}", avg_similarity);
}

#[derive(Debug)]
struct Match {
    file_path: String,
    pattern_count: usize,
    similarity: f64,
}
