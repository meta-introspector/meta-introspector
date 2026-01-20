use std::collections::HashMap;
use arrow::array::{BinaryArray, StringArray, UInt64Array, UInt8Array};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use std::fs::File;
use std::sync::Arc;

const MAX_CONST_SIZE: usize = 71;

fn main() {
    println!("🔢 Collecting constants up to size {}", MAX_CONST_SIZE);
    
    // Extract from /bin
    let bin_consts = extract_constants_from_bin("/bin");
    println!("📦 Found {} constants in /bin", bin_consts.len());
    
    // Extract from /nix/store
    let nix_consts = extract_constants_from_nix("/nix/store");
    println!("❄️  Found {} constants in /nix/store", nix_consts.len());
    
    // Combine and deduplicate
    let all_consts = deduplicate(bin_consts, nix_consts);
    println!("🔗 Total unique constants: {}", all_consts.len());
    
    // Group by size
    let by_size = group_by_size(&all_consts);
    for size in 1..=MAX_CONST_SIZE {
        if let Some(consts) = by_size.get(&size) {
            println!("  Size {}: {} constants", size, consts.len());
        }
    }
    
    // Save to parquet
    save_const71(&all_consts, "zos/layer0/const71.parquet");
}

fn extract_constants_from_bin(bin_dir: &str) -> Vec<Const71> {
    use goblin::Object;
    use std::fs;
    
    let mut constants = Vec::new();
    
    for entry in fs::read_dir(bin_dir).unwrap().flatten() {
        let path = entry.path();
        if let Ok(buffer) = fs::read(&path) {
            if let Ok(Object::Elf(elf)) = Object::parse(&buffer) {
                // Extract from .rodata
                for section in elf.section_headers.iter() {
                    if section.sh_type == goblin::elf::section_header::SHT_PROGBITS {
                        let offset = section.sh_offset as usize;
                        let size = section.sh_size as usize;
                        let data = &buffer[offset..offset + size];
                        
                        // Scan for constants up to size 71
                        for i in 0..data.len() {
                            for len in 1..=MAX_CONST_SIZE.min(data.len() - i) {
                                let value = &data[i..i + len];
                                
                                constants.push(Const71 {
                                    value: value.to_vec(),
                                    size: len as u8,
                                    binary_path: path.to_str().unwrap().to_string(),
                                    binary_offset: (offset + i) as u64,
                                    source_file: String::new(),
                                    source_line: 0,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    
    constants
}

fn extract_constants_from_nix(nix_dir: &str) -> Vec<Const71> {
    // Similar to extract_constants_from_bin
    vec![]
}

fn deduplicate(mut bin_consts: Vec<Const71>, nix_consts: Vec<Const71>) -> Vec<Const71> {
    let mut seen = HashMap::new();
    let mut unique = Vec::new();
    
    bin_consts.extend(nix_consts);
    
    for const_val in bin_consts {
        let key = const_val.value.clone();
        if !seen.contains_key(&key) {
            seen.insert(key, true);
            unique.push(const_val);
        }
    }
    
    unique
}

fn group_by_size(consts: &[Const71]) -> HashMap<usize, Vec<&Const71>> {
    let mut groups = HashMap::new();
    
    for c in consts {
        groups.entry(c.size as usize)
            .or_insert(Vec::new())
            .push(c);
    }
    
    groups
}

fn save_const71(consts: &[Const71], output: &str) {
    let schema = Schema::new(vec![
        Field::new("value", DataType::Binary, false),
        Field::new("size", DataType::UInt8, false),
        Field::new("binary_path", DataType::Utf8, false),
        Field::new("binary_offset", DataType::UInt64, false),
        Field::new("source_file", DataType::Utf8, false),
        Field::new("source_line", DataType::UInt64, false),
    ]);
    
    let values: Vec<Vec<u8>> = consts.iter().map(|c| c.value.clone()).collect();
    let sizes: Vec<u8> = consts.iter().map(|c| c.size).collect();
    let binary_paths: Vec<String> = consts.iter().map(|c| c.binary_path.clone()).collect();
    let binary_offsets: Vec<u64> = consts.iter().map(|c| c.binary_offset).collect();
    let source_files: Vec<String> = consts.iter().map(|c| c.source_file.clone()).collect();
    let source_lines: Vec<u64> = consts.iter().map(|c| c.source_line).collect();
    
    let batch = RecordBatch::try_new(
        Arc::new(schema.clone()),
        vec![
            Arc::new(BinaryArray::from(values)),
            Arc::new(UInt8Array::from(sizes)),
            Arc::new(StringArray::from(binary_paths)),
            Arc::new(UInt64Array::from(binary_offsets)),
            Arc::new(StringArray::from(source_files)),
            Arc::new(UInt64Array::from(source_lines)),
        ],
    ).unwrap();
    
    let file = File::create(output).unwrap();
    let mut writer = ArrowWriter::try_new(file, Arc::new(schema), None).unwrap();
    writer.write(&batch).unwrap();
    writer.close().unwrap();
    
    println!("💾 Saved {} constants to {}", consts.len(), output);
}

#[derive(Debug, Clone)]
struct Const71 {
    value: Vec<u8>,
    size: u8,
    binary_path: String,
    binary_offset: u64,
    source_file: String,
    source_line: u64,
}
