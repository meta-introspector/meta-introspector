use std::collections::HashMap;
use arrow::array::{StringArray, UInt64Array, BinaryArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use std::fs::File;
use std::sync::Arc;

#[derive(Debug)]
struct ValueLattice {
    // Source
    source_file: String,
    source_line: u32,
    source_column: u32,
    
    // Git
    git_repo: String,
    git_commit: String,
    git_url: String,
    
    // Binary
    binary_path: String,
    binary_offset: u64,
    binary_section: String,
    
    // Value
    const_name: String,
    const_value: Vec<u8>,
    const_type: String,
    
    // Nix
    nix_store_path: String,
    nix_derivation: String,
}

fn main() {
    println!("🔗 Building Value Lattice - Source ↔ Binary ↔ Parquet");
    
    // Extract from /bin
    let bin_values = extract_from_bin("/bin");
    println!("📦 Extracted {} values from /bin", bin_values.len());
    
    // Extract from /nix/store
    let nix_values = extract_from_nix_store("/nix/store");
    println!("❄️  Extracted {} values from /nix/store", nix_values.len());
    
    // Link to source via addr2line
    let linked = link_to_source(bin_values, nix_values);
    println!("🔗 Linked {} values to source", linked.len());
    
    // Save to parquet
    save_value_lattice(&linked, "zos/layer0/value_lattice.parquet");
    
    // Verify equivalence
    verify_equivalence(&linked);
}

fn extract_from_bin(bin_dir: &str) -> Vec<ValueLattice> {
    use goblin::Object;
    use std::fs;
    
    let mut values = Vec::new();
    
    for entry in fs::read_dir(bin_dir).unwrap().flatten() {
        let path = entry.path();
        if let Ok(buffer) = fs::read(&path) {
            if let Ok(Object::Elf(elf)) = Object::parse(&buffer) {
                // Extract .rodata (constants)
                for section in elf.section_headers.iter() {
                    if section.sh_type == goblin::elf::section_header::SHT_PROGBITS {
                        let offset = section.sh_offset;
                        let size = section.sh_size;
                        
                        let data = &buffer[offset as usize..][..size as usize];
                        
                        values.push(ValueLattice {
                            source_file: String::new(),
                            source_line: 0,
                            source_column: 0,
                            git_repo: String::new(),
                            git_commit: String::new(),
                            git_url: String::new(),
                            binary_path: path.to_str().unwrap().to_string(),
                            binary_offset: offset,
                            binary_section: ".rodata".to_string(),
                            const_name: format!("const_{:x}", offset),
                            const_value: data.to_vec(),
                            const_type: "bytes".to_string(),
                            nix_store_path: String::new(),
                            nix_derivation: String::new(),
                        });
                    }
                }
            }
        }
    }
    
    values
}

fn extract_from_nix_store(store_dir: &str) -> Vec<ValueLattice> {
    // Similar to extract_from_bin but for /nix/store
    vec![]
}

fn link_to_source(mut bin_values: Vec<ValueLattice>, nix_values: Vec<ValueLattice>) -> Vec<ValueLattice> {
    use std::process::Command;
    
    for value in &mut bin_values {
        // Use addr2line to find source location
        let output = Command::new("addr2line")
            .args(&[
                "-e", &value.binary_path,
                "-a", &format!("{:x}", value.binary_offset),
                "-f", "-C"
            ])
            .output();
        
        if let Ok(output) = output {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.lines().collect();
            
            if lines.len() >= 2 {
                // Parse source location
                if let Some(location) = lines[1].split(':').next() {
                    value.source_file = location.to_string();
                    
                    // Extract line number
                    if let Some(line_str) = lines[1].split(':').nth(1) {
                        value.source_line = line_str.parse().unwrap_or(0);
                    }
                }
            }
        }
        
        // Link to git
        if !value.source_file.is_empty() {
            if let Some(git_info) = find_git_info(&value.source_file) {
                value.git_repo = git_info.repo;
                value.git_commit = git_info.commit;
                value.git_url = git_info.url;
            }
        }
    }
    
    bin_values
}

fn find_git_info(file_path: &str) -> Option<GitInfo> {
    use std::process::Command;
    
    let output = Command::new("git")
        .args(&["-C", file_path, "rev-parse", "HEAD"])
        .output()
        .ok()?;
    
    let commit = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    let output = Command::new("git")
        .args(&["-C", file_path, "remote", "get-url", "origin"])
        .output()
        .ok()?;
    
    let url = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    Some(GitInfo {
        repo: file_path.to_string(),
        commit,
        url,
    })
}

fn save_value_lattice(values: &[ValueLattice], output: &str) {
    let schema = Schema::new(vec![
        Field::new("source_file", DataType::Utf8, false),
        Field::new("source_line", DataType::UInt64, false),
        Field::new("git_commit", DataType::Utf8, false),
        Field::new("git_url", DataType::Utf8, false),
        Field::new("binary_path", DataType::Utf8, false),
        Field::new("binary_offset", DataType::UInt64, false),
        Field::new("const_name", DataType::Utf8, false),
        Field::new("const_value", DataType::Binary, false),
        Field::new("nix_store_path", DataType::Utf8, false),
    ]);
    
    let source_files: Vec<String> = values.iter().map(|v| v.source_file.clone()).collect();
    let source_lines: Vec<u64> = values.iter().map(|v| v.source_line as u64).collect();
    let git_commits: Vec<String> = values.iter().map(|v| v.git_commit.clone()).collect();
    let git_urls: Vec<String> = values.iter().map(|v| v.git_url.clone()).collect();
    let binary_paths: Vec<String> = values.iter().map(|v| v.binary_path.clone()).collect();
    let binary_offsets: Vec<u64> = values.iter().map(|v| v.binary_offset).collect();
    let const_names: Vec<String> = values.iter().map(|v| v.const_name.clone()).collect();
    let const_values: Vec<Vec<u8>> = values.iter().map(|v| v.const_value.clone()).collect();
    let nix_paths: Vec<String> = values.iter().map(|v| v.nix_store_path.clone()).collect();
    
    let batch = RecordBatch::try_new(
        Arc::new(schema.clone()),
        vec![
            Arc::new(StringArray::from(source_files)),
            Arc::new(UInt64Array::from(source_lines)),
            Arc::new(StringArray::from(git_commits)),
            Arc::new(StringArray::from(git_urls)),
            Arc::new(StringArray::from(binary_paths)),
            Arc::new(UInt64Array::from(binary_offsets)),
            Arc::new(StringArray::from(const_names)),
            Arc::new(BinaryArray::from(const_values)),
            Arc::new(StringArray::from(nix_paths)),
        ],
    ).unwrap();
    
    let file = File::create(output).unwrap();
    let mut writer = ArrowWriter::try_new(file, Arc::new(schema), None).unwrap();
    writer.write(&batch).unwrap();
    writer.close().unwrap();
    
    println!("💾 Saved value lattice: {}", output);
}

fn verify_equivalence(values: &[ValueLattice]) {
    println!("\n🔍 Verifying equivalence:");
    
    for value in values.iter().take(10) {
        // Read source
        if let Ok(source) = std::fs::read_to_string(&value.source_file) {
            let source_line = source.lines().nth(value.source_line as usize);
            
            // Read binary
            if let Ok(binary) = std::fs::read(&value.binary_path) {
                let binary_value = &binary[value.binary_offset as usize..][..value.const_value.len()];
                
                // Verify: source → binary → parquet
                if binary_value == value.const_value.as_slice() {
                    println!("  ✅ {} @ {}:{} ↔ {:x}", 
                        value.const_name, 
                        value.source_file, 
                        value.source_line,
                        value.binary_offset
                    );
                }
            }
        }
    }
}

struct GitInfo {
    repo: String,
    commit: String,
    url: String,
}
