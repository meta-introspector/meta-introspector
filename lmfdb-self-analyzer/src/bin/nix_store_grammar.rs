// Extract grammars from ALL nix store .so files using canonical infrastructure

use std::path::PathBuf;
use std::fs;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use lmfdb_rust_mapping::grammar_extraction::*;
use goblin::elf::Elf;
use arrow::array::{StringArray, UInt64Array};
use arrow::record_batch::RecordBatch;
use arrow::datatypes::{DataType, Field, Schema};
use parquet::arrow::ArrowWriter;

// Import canonical modules (inline for now)
mod canonical_walker {
    use std::path::{Path, PathBuf};
    use std::fs;
    
    pub fn find_all_so_files() -> Result<Vec<PathBuf>, std::io::Error> {
        let mut results = Vec::new();
        walk_dir(Path::new("/nix/store"), 0, 3, &mut results)?;
        Ok(results)
    }
    
    fn walk_dir(dir: &Path, depth: usize, max_depth: usize, results: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
        if depth > max_depth {
            return Ok(());
        }
        
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|e| e.to_str()).map(|e| e == "so").unwrap_or(false) {
                results.push(path.clone());
            }
            
            if path.is_dir() && !path.is_symlink() {
                walk_dir(&path, depth + 1, max_depth, results)?;
            }
        }
        
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔬 Grammar Extraction from ALL Nix Store .so files");
    println!("Using canonical infrastructure (crossbeam + parquet)\n");
    
    // Find all .so files
    println!("🔍 Scanning /nix/store...");
    let so_files = canonical_walker::find_all_so_files()?;
    println!("✅ Found {} .so files\n", so_files.len());
    
    // Use crossbeam workers (2x CPU cores)
    let num_workers = num_cpus::get() * 2;
    println!("🔧 Spawning {} workers\n", num_workers);
    
    let (sender, receiver) = crossbeam::channel::bounded::<PathBuf>(1000);
    let processed = Arc::new(AtomicUsize::new(0));
    let grammars_found = Arc::new(AtomicUsize::new(0));
    let all_grammars = Arc::new(Mutex::new(Vec::new()));
    
    // Spawn workers
    let mut handles = vec![];
    for worker_id in 0..num_workers {
        let rx = receiver.clone();
        let proc = Arc::clone(&processed);
        let found = Arc::clone(&grammars_found);
        let grammars = Arc::clone(&all_grammars);
        
        let handle = thread::spawn(move || {
            let mut extractor = GrammarExtractor::new();
            
            while let Ok(path) = rx.recv() {
                if let Ok(binary_data) = fs::read(&path) {
                    if let Ok(elf) = Elf::parse(&binary_data) {
                        // Find .text section
                        if let Some(text_section) = elf.section_headers.iter()
                            .find(|sh| elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("") == ".text") {
                            
                            let text_start = text_section.sh_offset as usize;
                            let text_size = (text_section.sh_size as usize).min(1024 * 1024); // Max 1MB per file
                            
                            if text_start + text_size <= binary_data.len() {
                                let text_bytes = &binary_data[text_start..text_start + text_size];
                                
                                // Extract grammars from all symbols
                                for sym in elf.syms.iter().take(100) { // Sample 100 symbols per file
                                    if sym.st_size > 0 && sym.st_value >= text_section.sh_addr {
                                        let func_start = (sym.st_value - text_section.sh_addr) as usize;
                                        let func_size = (sym.st_size as usize).min(512);
                                        
                                        if func_start + func_size <= text_bytes.len() {
                                            let func_bytes = &text_bytes[func_start..func_start + func_size];
                                            let states = extractor.extract_dfa(func_bytes);
                                            
                                            if !states.is_empty() {
                                                let pattern = extractor.extract_grammar(states);
                                                let name = elf.strtab.get_at(sym.st_name).unwrap_or("").to_string();
                                                
                                                grammars.lock().unwrap().push((
                                                    name,
                                                    pattern.lmfdb_label.to_string(),
                                                    pattern.modular_signature,
                                                    pattern.states.len(),
                                                    path.display().to_string(),
                                                ));
                                                
                                                found.fetch_add(1, Ordering::Relaxed);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                
                proc.fetch_add(1, Ordering::Relaxed);
                
                if proc.load(Ordering::Relaxed) % 100 == 0 {
                    println!("  Worker {}: {} files, {} grammars", 
                        worker_id, 
                        proc.load(Ordering::Relaxed),
                        found.load(Ordering::Relaxed)
                    );
                }
            }
        });
        handles.push(handle);
    }
    
    // Send files to workers
    for so_file in so_files {
        sender.send(so_file)?;
    }
    drop(sender);
    
    // Wait for completion
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("\n✅ Complete!");
    println!("   Processed: {} files", processed.load(Ordering::Relaxed));
    println!("   Grammars: {} found", grammars_found.load(Ordering::Relaxed));
    
    // Save to parquet
    println!("\n💾 Saving to parquet...");
    let final_grammars = all_grammars.lock().unwrap();
    save_to_parquet(&final_grammars, "nix_store_grammars.parquet")?;
    println!("✅ Saved {} grammars to nix_store_grammars.parquet", final_grammars.len());
    
    Ok(())
}

fn save_to_parquet(
    grammars: &[(String, String, u64, usize, String)], // (name, lmfdb, sig, states, path)
    output_path: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let schema = Arc::new(Schema::new(vec![
        Field::new("function_name", DataType::Utf8, false),
        Field::new("lmfdb_label", DataType::Utf8, false),
        Field::new("signature", DataType::UInt64, false),
        Field::new("states", DataType::UInt64, false),
        Field::new("binary_path", DataType::Utf8, false),
    ]));
    
    let file = fs::File::create(output_path)?;
    let mut writer = ArrowWriter::try_new(file, schema.clone(), None)?;
    
    // Write in 100k batches
    for chunk in grammars.chunks(100_000) {
        let names: Vec<&str> = chunk.iter().map(|(n, _, _, _, _)| n.as_str()).collect();
        let labels: Vec<&str> = chunk.iter().map(|(_, l, _, _, _)| l.as_str()).collect();
        let sigs: Vec<u64> = chunk.iter().map(|(_, _, s, _, _)| *s).collect();
        let states: Vec<u64> = chunk.iter().map(|(_, _, _, st, _)| *st as u64).collect();
        let paths: Vec<&str> = chunk.iter().map(|(_, _, _, _, p)| p.as_str()).collect();
        
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(StringArray::from(names)),
                Arc::new(StringArray::from(labels)),
                Arc::new(UInt64Array::from(sigs)),
                Arc::new(UInt64Array::from(states)),
                Arc::new(StringArray::from(paths)),
            ],
        )?;
        
        writer.write(&batch)?;
    }
    
    writer.close()?;
    Ok(())
}
