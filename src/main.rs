use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use crossbeam::channel::{bounded, Receiver, Sender};
use std::thread;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
struct SymbolScore {
    name: String,
    file: String,
    cell: usize,
    score: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (sender, receiver) = bounded::<String>(1000);
    let results = Arc::new(Mutex::new(Vec::new()));
    
    // Spawn 20 workers
    for worker_id in 0..20 {
        let rx = receiver.clone();
        let results_clone = Arc::clone(&results);
        
        thread::spawn(move || {
            worker(worker_id, rx, results_clone);
        });
    }
    
    // Find ELF files
    let paths = find_elf_files("/nix/store", 500)?;
    println!("Found {} ELF files, starting parallel analysis\n", paths.len());
    
    let total_files = paths.len();
    
    // Send to workers
    for path in paths {
        sender.send(path)?;
    }
    drop(sender);
    
    // Wait for completion with progress
    for i in 0..60 {
        thread::sleep(std::time::Duration::from_secs(1));
        let count = results.lock().unwrap().len();
        if i % 5 == 0 {
            println!("Progress: {} symbols extracted from {} files", count, total_files);
        }
    }
    
    let all_symbols = results.lock().unwrap().clone();
    println!("\nTotal symbols extracted: {}\n", all_symbols.len());
    
    // Save to JSON
    let json = serde_json::to_string_pretty(&all_symbols)?;
    fs::write("markov_symbol_scores.json", json)?;
    println!("Saved to markov_symbol_scores.json\n");
    
    // Rank globally
    let mut ranked: Vec<_> = all_symbols.iter().collect();
    ranked.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    
    println!("\n=== TOP 50 SYMBOLS BY MARKOV RESONANCE ===\n");
    for (i, sym) in ranked.iter().take(50).enumerate() {
        let short_file = Path::new(&sym.file).file_name().unwrap().to_string_lossy();
        println!("{:3}. {:40} cell={:4} score={:.4} | {}", 
                 i+1, &sym.name[..sym.name.len().min(40)], sym.cell, sym.score, short_file);
    }
    
    // Group by file
    let mut by_file: HashMap<String, Vec<&SymbolScore>> = HashMap::new();
    for sym in &all_symbols {
        by_file.entry(sym.file.clone()).or_insert_with(Vec::new).push(sym);
    }
    
    let mut file_scores: Vec<_> = by_file.iter()
        .map(|(file, syms)| {
            let avg = syms.iter().map(|s| s.score).sum::<f64>() / syms.len() as f64;
            let max = syms.iter().map(|s| s.score).fold(0.0, f64::max);
            (file, avg, max, syms.len())
        })
        .collect();
    
    file_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("\n=== TOP 30 FILES BY AVERAGE RESONANCE ===\n");
    for (i, (file, avg, max, count)) in file_scores.iter().take(30).enumerate() {
        let short_name = Path::new(file).file_name().unwrap().to_string_lossy();
        println!("{:3}. {:40} avg={:.4} max={:.4} syms={}", 
                 i+1, short_name, avg, max, count);
    }
    
    Ok(())
}

fn worker(worker_id: usize, receiver: Receiver<String>, results: Arc<Mutex<Vec<SymbolScore>>>) {
    let mut processed = 0;
    while let Ok(path) = receiver.recv() {
        if let Ok(symbols) = analyze_file(&path, 32) {
            if let Ok(mut res) = results.lock() {
                res.extend(symbols);
                processed += 1;
                if processed % 10 == 0 {
                    println!("Worker {}: {} files processed", worker_id, processed);
                }
            }
        }
    }
    println!("Worker {} finished: {} files", worker_id, processed);
}

fn find_elf_files(root: &str, limit: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = std::process::Command::new("find")
        .arg(root)
        .arg("-type").arg("f")
        .arg("(")
        .arg("-name").arg("*.so")
        .arg("-o")
        .arg("-executable")
        .arg(")")
        .output()?;
    
    let paths: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .take(limit)
        .map(String::from)
        .collect();
    
    Ok(paths)
}

fn analyze_file(path: &str, window_size: usize) -> Result<Vec<SymbolScore>, Box<dyn std::error::Error>> {
    let buffer = fs::read(path)?;
    let elf = Elf::parse(&buffer)?;
    
    let mut text_start = 0;
    let mut text_size = 0;
    
    for section in &elf.section_headers {
        if let Some(name) = elf.shdr_strtab.get_at(section.sh_name) {
            if name == ".text" {
                text_start = section.sh_offset as usize;
                text_size = section.sh_size as usize;
                break;
            }
        }
    }
    
    if text_size == 0 { return Ok(vec![]); }
    
    let text = &buffer[text_start..text_start + text_size];
    let resonance = analyze_markov_resonance(text, window_size);
    
    let mut results = Vec::new();
    
    for sym in &elf.syms {
        if sym.st_size == 0 { continue; }
        
        let sym_offset = sym.st_value as usize;
        if sym_offset < text_start { continue; }
        
        let relative_offset = sym_offset - text_start;
        let cell = relative_offset / window_size;
        
        if cell >= resonance.len() { continue; }
        
        let name = elf.strtab.get_at(sym.st_name).unwrap_or("?").to_string();
        if name.is_empty() || name == "?" { continue; }
        
        results.push(SymbolScore {
            name,
            file: path.to_string(),
            cell,
            score: resonance[cell],
        });
    }
    
    Ok(results)
}

fn analyze_markov_resonance(text: &[u8], window_size: usize) -> Vec<f64> {
    let num_windows = text.len() / window_size;
    if num_windows < 2 { return vec![]; }
    
    let mut matrix = vec![vec![0.0; num_windows]; num_windows];
    
    for i in 0..num_windows {
        let window_i = &text[i * window_size..(i + 1) * window_size];
        for j in 0..num_windows {
            let window_j = &text[j * window_size..(j + 1) * window_size];
            matrix[i][j] = hamming_similarity(window_i, window_j);
        }
    }
    
    let mut resonance = vec![0.0; num_windows];
    for i in 0..num_windows {
        let mut corr = 0.0;
        for j in 0..num_windows {
            if i != j {
                corr += matrix[i][j] * matrix[j][i];
            }
        }
        resonance[i] = corr;
    }
    
    resonance
}

fn hamming_similarity(a: &[u8], b: &[u8]) -> f64 {
    let matches = a.iter().zip(b.iter()).filter(|(x, y)| x == y).count();
    matches as f64 / a.len() as f64
}
