// nix2parquet - Stream LMFDB function data to Parquet (parallel with crossbeam)

use goblin::elf::Elf;
use std::fs::{self, File};
use std::sync::{Arc, Mutex};
use anyhow::Result;
use parquet::file::properties::WriterProperties;
use parquet::arrow::ArrowWriter;
use arrow::array::{StringArray, UInt32Array, UInt64Array, ArrayRef};
use arrow::record_batch::RecordBatch;
use crossbeam::channel::{bounded, Sender, Receiver};
use std::thread;

fn main() -> Result<()> {
    println!("🔬 nix2parquet - LMFDB function data stream (20 cores)");
    
    let output_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "data/nix_lmfdb_analysis/functions_all.parquet".to_string());
    
    println!("📊 Output: {}", output_path);
    
    // Create output directory
    if let Some(parent) = std::path::Path::new(&output_path).parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Find ALL ELF binaries
    println!("🔍 Finding ALL ELF binaries in /nix/store...");
    let binaries = find_all_elf_binaries()?;
    println!("✅ Found {} ELF binaries", binaries.len());
    
    // Define schema
    let schema = Arc::new(arrow::datatypes::Schema::new(vec![
        arrow::datatypes::Field::new("binary", arrow::datatypes::DataType::Utf8, false),
        arrow::datatypes::Field::new("function_name", arrow::datatypes::DataType::Utf8, false),
        arrow::datatypes::Field::new("address", arrow::datatypes::DataType::UInt64, false),
        arrow::datatypes::Field::new("size", arrow::datatypes::DataType::UInt64, false),
        arrow::datatypes::Field::new("lmfdb_signature", arrow::datatypes::DataType::Utf8, false),
        arrow::datatypes::Field::new("conductor", arrow::datatypes::DataType::UInt32, false),
        arrow::datatypes::Field::new("complexity", arrow::datatypes::DataType::UInt32, false),
        arrow::datatypes::Field::new("orbit_hash", arrow::datatypes::DataType::UInt32, false),
    ]));
    
    let file = File::create(&output_path)?;
    let props = WriterProperties::builder()
        .set_compression(parquet::basic::Compression::SNAPPY)
        .build();
    
    let writer = Arc::new(Mutex::new(ArrowWriter::try_new(file, schema.clone(), Some(props))?));
    
    // Parallel processing with crossbeam
    let num_workers = 20;
    let (tx, rx): (Sender<Vec<(String, String, u64, u64, String, u32, u32, u32)>>, 
                   Receiver<Vec<(String, String, u64, u64, String, u32, u32, u32)>>) = bounded(100);
    
    // Spawn workers
    let binaries_arc = Arc::new(binaries);
    let mut handles = vec![];
    
    for worker_id in 0..num_workers {
        let binaries = binaries_arc.clone();
        let tx = tx.clone();
        
        let handle = thread::spawn(move || {
            let chunk_size = binaries.len().div_ceil(num_workers);
            let start = worker_id * chunk_size;
            let end = (start + chunk_size).min(binaries.len());
            
            for (i, binary_path) in binaries[start..end].iter().enumerate() {
                if let Ok(functions) = extract_functions(binary_path) {
                    if i % 10 == 0 {
                        println!("  Worker {} - {} - {} functions", 
                            worker_id, 
                            std::path::Path::new(binary_path).file_name().unwrap().to_str().unwrap(),
                            functions.len());
                    }
                    
                    if !functions.is_empty() {
                        let _ = tx.send(functions);
                    }
                }
            }
        });
        
        handles.push(handle);
    }
    
    drop(tx); // Close sender
    
    // Writer thread
    let writer_handle = {
        let schema = schema.clone();
        let writer = writer.clone();
        
        thread::spawn(move || -> Result<usize> {
            let mut total_functions = 0;
            let batch_size = 10000;
            
            let mut binary_batch = Vec::new();
            let mut name_batch = Vec::new();
            let mut addr_batch = Vec::new();
            let mut size_batch = Vec::new();
            let mut sig_batch = Vec::new();
            let mut conductor_batch = Vec::new();
            let mut complexity_batch = Vec::new();
            let mut orbit_batch = Vec::new();
            
            while let Ok(functions) = rx.recv() {
                for (binary, name, addr, size, sig, conductor, complexity, orbit) in functions {
                    binary_batch.push(binary);
                    name_batch.push(name);
                    addr_batch.push(addr);
                    size_batch.push(size);
                    sig_batch.push(sig);
                    conductor_batch.push(conductor);
                    complexity_batch.push(complexity);
                    orbit_batch.push(orbit);
                    
                    total_functions += 1;
                    
                    if binary_batch.len() >= batch_size {
                        write_batch(
                            &mut writer.lock().unwrap(),
                            &schema,
                            &binary_batch,
                            &name_batch,
                            &addr_batch,
                            &size_batch,
                            &sig_batch,
                            &conductor_batch,
                            &complexity_batch,
                            &orbit_batch,
                        )?;
                        
                        println!("📊 Wrote batch - total: {} functions", total_functions);
                        
                        binary_batch.clear();
                        name_batch.clear();
                        addr_batch.clear();
                        size_batch.clear();
                        sig_batch.clear();
                        conductor_batch.clear();
                        complexity_batch.clear();
                        orbit_batch.clear();
                    }
                }
            }
            
            // Write remaining
            if !binary_batch.is_empty() {
                write_batch(
                    &mut writer.lock().unwrap(),
                    &schema,
                    &binary_batch,
                    &name_batch,
                    &addr_batch,
                    &size_batch,
                    &sig_batch,
                    &conductor_batch,
                    &complexity_batch,
                    &orbit_batch,
                )?;
            }
            
            Ok(total_functions)
        })
    };
    
    // Wait for workers
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Wait for writer
    let total_functions = writer_handle.join().unwrap()?;
    
    // Close writer
    let writer = Arc::try_unwrap(writer).unwrap().into_inner().unwrap();
    writer.close()?;
    
    println!("\n✅ Wrote {} functions to Parquet", total_functions);
    println!("💾 Saved to: {}", output_path);
    
    // Show file size
    let metadata = fs::metadata(&output_path)?;
    println!("📊 File size: {} bytes ({:.2} MB)", 
        metadata.len(), 
        metadata.len() as f64 / 1024.0 / 1024.0);
    
    Ok(())
}

fn find_all_elf_binaries() -> Result<Vec<String>> {
    let output = std::process::Command::new("find")
        .args(["/nix/store", "-maxdepth", "3", "-type", "f", "-executable"])
        .output()?;
    
    let mut paths: Vec<String> = Vec::new();
    
    for line in String::from_utf8_lossy(&output.stdout).lines() {
        // Quick check if it's an ELF file
        if let Ok(bytes) = fs::read(line).map(|b| b.get(0..4).unwrap_or(&[]).to_vec()) {
            if bytes.starts_with(&[0x7f, 0x45, 0x4c, 0x46]) { // ELF magic
                paths.push(line.to_string());
            }
        }
    }
    
    Ok(paths)
}

fn write_batch(
    writer: &mut ArrowWriter<File>,
    schema: &Arc<arrow::datatypes::Schema>,
    library: &[String],
    name: &[String],
    addr: &[u64],
    size: &[u64],
    sig: &[String],
    conductor: &[u32],
    complexity: &[u32],
    orbit: &[u32],
) -> Result<()> {
    let batch = RecordBatch::try_new(
        schema.clone(),
        vec![
            Arc::new(StringArray::from(library.to_vec())) as ArrayRef,
            Arc::new(StringArray::from(name.to_vec())) as ArrayRef,
            Arc::new(UInt64Array::from(addr.to_vec())) as ArrayRef,
            Arc::new(UInt64Array::from(size.to_vec())) as ArrayRef,
            Arc::new(StringArray::from(sig.to_vec())) as ArrayRef,
            Arc::new(UInt32Array::from(conductor.to_vec())) as ArrayRef,
            Arc::new(UInt32Array::from(complexity.to_vec())) as ArrayRef,
            Arc::new(UInt32Array::from(orbit.to_vec())) as ArrayRef,
        ],
    )?;
    
    writer.write(&batch)?;
    Ok(())
}

fn extract_functions(path: &str) -> Result<Vec<(String, String, u64, u64, String, u32, u32, u32)>> {
    let binary_name = std::path::Path::new(path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    
    let buffer = fs::read(path)?;
    let elf = Elf::parse(&buffer)?;
    
    let mut functions = Vec::new();
    
    for sym in elf.dynsyms.iter() {
        if let Some(name) = elf.dynstrtab.get_at(sym.st_name) {
            if !name.is_empty() && sym.st_value != 0 && sym.st_size > 0 {
                let offset = sym.st_value as usize;
                let size = sym.st_size as usize;
                
                if offset < buffer.len() && offset + size <= buffer.len() {
                    let func_bytes = &buffer[offset..offset + size.min(128)];
                    let (sig, conductor, complexity, orbit) = analyze_function(func_bytes);
                    
                    functions.push((
                        binary_name.clone(),
                        name.to_string(),
                        sym.st_value,
                        sym.st_size,
                        sig,
                        conductor,
                        complexity,
                        orbit,
                    ));
                }
            }
        }
    }
    
    Ok(functions)
}

fn analyze_function(bytes: &[u8]) -> (String, u32, u32, u32) {
    let mut signature_parts = Vec::new();
    let mut conductor_sum = 0u32;
    let mut complexity = 0u32;
    let mut orbit_hash = 0u32;
    
    for start in (0..bytes.len().min(64)).step_by(4) {
        let end = (start + 4).min(bytes.len());
        if end > start {
            let pattern = &bytes[start..end];
            let (form, orbit, weight, conductor) = classify_instruction(pattern);
            
            signature_parts.push(form.chars().next().unwrap_or('?'));
            conductor_sum += conductor;
            complexity += weight;
            orbit_hash = orbit_hash.wrapping_add(orbit);
        }
    }
    
    let signature: String = signature_parts.iter().take(8).collect();
    
    (signature, conductor_sum, complexity, orbit_hash)
}

fn classify_instruction(pattern: &[u8]) -> (String, u32, u32, u32) {
    let orbit = pattern.iter().fold(0u32, |acc, &b| acc.wrapping_mul(31).wrapping_add(b as u32)) % 1000;
    let weight = pattern.iter().filter(|&&b| b != 0).count() as u32;
    let conductor = 3000 + pattern.len() as u32 * 10 + weight * 100;
    
    let form = if pattern.starts_with(&[0xf3, 0x0f]) {
        "endbr64"
    } else if pattern.starts_with(&[0xc3]) {
        "ret"
    } else if pattern.starts_with(&[0x48, 0x89]) {
        "mov_r64"
    } else if pattern.starts_with(&[0x48, 0x8b]) {
        "mov_load"
    } else if pattern.starts_with(&[0x41, 0x57]) || pattern.starts_with(&[0x41, 0x55]) {
        "prologue"
    } else if pattern.starts_with(&[0x0f, 0x1f]) {
        "nop_pad"
    } else if pattern.iter().all(|&b| b == 0) {
        "zero_pad"
    } else if weight == 0 {
        "zero_pad"
    } else if weight == pattern.len() as u32 {
        "dense"
    } else {
        "mixed"
    };
    
    (form.to_string(), orbit, weight, conductor)
}
