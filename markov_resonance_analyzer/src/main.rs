use goblin::elf::Elf;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use crossbeam::channel::{bounded, Receiver, Sender};
use std::thread;
use serde::{Serialize, Deserialize};
use std::env;

// Shared memory budget tracker (20GB = 20 * 1024^3 bytes)
const MEMORY_BUDGET_BYTES: usize = 20 * 1024 * 1024 * 1024;
const MAX_CELLS_PER_FILE: usize = 100;
const CELL_SIZE_ESTIMATE: usize = 64; // 32 bytes window + overhead

#[derive(Clone, Serialize, Deserialize)]
struct SymbolScore {
    name: String,
    file: String,
    cell: usize,
    cell_offset: usize,
    score: f64,
}

#[derive(Clone, Serialize, Deserialize)]
struct CellData {
    cell_id: usize,
    offset: usize,
    window_size: usize,
    resonance_score: f64,
    byte_pattern: Vec<u8>,
}

#[derive(Clone, Serialize, Deserialize)]
struct FileDistribution {
    file: String,
    window_size: usize,
    num_windows: usize,
    resonance_vector: Vec<f64>,
    cells: Vec<CellData>,
}

#[derive(Serialize, Deserialize)]
struct GlobalMatrix {
    files: Vec<FileDistribution>,
    similarity_matrix: Vec<Vec<f64>>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    let file_list = if args.len() > 1 {
        args[1].clone()
    } else {
        "elf_files_list.txt".to_string()
    };
    
    println!("📋 Using file list: {}", file_list);
    println!("💾 Pre-allocating 20GB shared memory pool...");
    
    // Shared memory budget tracker - all workers share this
    let memory_used = Arc::new(AtomicUsize::new(0));
    
    // Use 2x CPU cores for I/O bound workload
    let num_workers = num_cpus::get() * 2;
    
    // Pre-allocate collections with capacity
    let (sender, receiver) = bounded::<String>(1000);
    let results = Arc::new(Mutex::new(Vec::with_capacity(5_000_000)));
    let distributions = Arc::new(Mutex::new(Vec::with_capacity(100_000)));
    let files_queued = Arc::new(Mutex::new(0usize));
    let files_processed = Arc::new(Mutex::new(0usize));
    let files_skipped = Arc::new(AtomicUsize::new(0));
    
    println!("✅ Memory pool ready (20GB budget shared across {} workers)", num_workers);
    println!("🔧 Spawning {} workers (2x {} CPU cores)", num_workers, num_cpus::get());
    
    // Spawn workers
    for worker_id in 0..num_workers {
        let rx = receiver.clone();
        let results_clone = Arc::clone(&results);
        let distributions_clone = Arc::clone(&distributions);
        let processed_clone = Arc::clone(&files_processed);
        let memory_clone = Arc::clone(&memory_used);
        let skipped_clone = Arc::clone(&files_skipped);
        
        thread::spawn(move || {
            worker(worker_id, rx, results_clone, distributions_clone, 
                   processed_clone, memory_clone, skipped_clone);
        });
    }
    
    let sender_clone = sender.clone();
    let queued_clone = Arc::clone(&files_queued);
    
    // Spawn finder thread that streams files to workers
    let finder_handle = thread::spawn(move || {
        let cache_file = file_list;
        
        if !Path::new(&cache_file).exists() {
            eprintln!("ERROR: File list not found: {}", cache_file);
            eprintln!("Please run: find /nix/store -type f \\( -name \"*.so\" -o -executable \\) > {}", cache_file);
            std::process::exit(1);
        }
        
        println!("📂 Loading file list from: {}", cache_file);
        let cached = fs::read_to_string(&cache_file).expect("Failed to read file list");
        let lines: Vec<&str> = cached.lines().collect();
        let count = lines.len();
        
        println!("📊 Found {} files to analyze", count);
        println!("🔍 Verifying files exist and queueing...");
        
        let mut missing = 0;
        for (i, line) in lines.iter().enumerate() {
            if !Path::new(line).exists() {
                missing += 1;
                if missing <= 5 {
                    eprintln!("  ⚠️  File missing: {}", line);
                }
            }
            if sender_clone.send(line.to_string()).is_err() {
                break;
            }
            if (i + 1) % 5000 == 0 {
                println!("  ⏳ Queued {}/{} files...", i + 1, count);
            }
        }
        
        if missing > 0 {
            eprintln!("⚠️  {} files are missing (may have been deleted)", missing);
        }
        
        println!("✅ File discovery complete: {} files queued for processing", count);
        *queued_clone.lock().unwrap() = count;
        count
    });
    
    // Drop sender so workers know when queue is done
    drop(sender);
    
    // Wait for finder to complete
    let total_files = finder_handle.join().unwrap();
    
    // Monitor progress and save partial results
    println!("\n📊 Monitoring progress...");
    let mut last_saved = 0;
    let mut last_processed = 0;
    let mut stalled_count = 0;
    
    loop {
        thread::sleep(std::time::Duration::from_secs(5));
        
        let symbols_count = results.lock().unwrap().len();
        let processed = *files_processed.lock().unwrap();
        let skipped = files_skipped.load(Ordering::Relaxed);
        let mem_used = memory_used.load(Ordering::Relaxed);
        let mem_gb = mem_used as f64 / (1024.0 * 1024.0 * 1024.0);
        let percent = (processed as f64 / total_files as f64 * 100.0) as u32;
        
        println!("📈 Progress: {}/{} files ({}%), {} symbols, {:.2}GB used, {} skipped", 
                 processed, total_files, percent, symbols_count, mem_gb, skipped);
        
        // Check if stalled
        if processed == last_processed {
            stalled_count += 1;
            if stalled_count >= 6 {
                println!("⚠️  No progress for 30 seconds, assuming workers finished");
                break;
            }
        } else {
            stalled_count = 0;
        }
        last_processed = processed;
        
        // Save partial results every 1000 new symbols
        if symbols_count - last_saved >= 1000 {
            let partial = results.lock().unwrap().clone();
            let json = serde_json::to_string_pretty(&partial)?;
            fs::write("markov_symbol_scores_partial.json", json)?;
            println!("💾 Saved partial results ({} symbols)", symbols_count);
            last_saved = symbols_count;
        }
        
        // Check if done
        if processed >= total_files {
            println!("✅ All files processed!");
            break;
        }
    }
    
    let all_symbols = results.lock().unwrap().clone();
    let all_distributions = distributions.lock().unwrap().clone();
    let final_mem = memory_used.load(Ordering::Relaxed);
    let final_skipped = files_skipped.load(Ordering::Relaxed);
    
    println!("\n✅ Analysis complete!");
    println!("   Symbols: {}", all_symbols.len());
    println!("   Distributions: {}", all_distributions.len());
    println!("   Memory: {:.2}GB / 20GB", final_mem as f64 / (1024.0 * 1024.0 * 1024.0));
    println!("   Skipped: {}\n", final_skipped);
    
    // Merge failed files from all workers
    let mut all_failed = Vec::new();
    for worker_id in 0..num_workers {
        let exclude_file = format!("failed_files_worker_{}.txt", worker_id);
        if let Ok(content) = fs::read_to_string(&exclude_file) {
            all_failed.extend(content.lines().map(String::from));
            let _ = fs::remove_file(&exclude_file);
        }
    }
    if !all_failed.is_empty() {
        fs::write("failed_files_exclude.txt", all_failed.join("\n"))?;
        println!("📝 Wrote {} failed files to failed_files_exclude.txt", all_failed.len());
    }
    
    // Save final JSON
    let json = serde_json::to_string_pretty(&all_symbols)?;
    fs::write("markov_symbol_scores.json", json)?;
    println!("✓ Saved final results to markov_symbol_scores.json");
    
    // Compute global similarity matrix in parallel
    println!("\nComputing global distribution similarity matrix with {} workers...", num_workers);
    let n = all_distributions.len();
    let mut similarity_matrix = vec![vec![0.0; n]; n];
    
    let distributions_arc = Arc::new(all_distributions.clone());
    let matrix_arc = Arc::new(Mutex::new(&mut similarity_matrix));
    let rows_processed = Arc::new(AtomicUsize::new(0));
    
    // Create work queue for rows
    let (row_sender, row_receiver) = bounded::<usize>(n);
    for i in 0..n {
        row_sender.send(i).unwrap();
    }
    drop(row_sender);
    
    // Spawn workers for similarity computation
    let mut handles = vec![];
    for _ in 0..num_workers {
        let rx = row_receiver.clone();
        let dists = Arc::clone(&distributions_arc);
        let processed = Arc::clone(&rows_processed);
        
        let handle = thread::spawn(move || {
            let mut local_results = vec![];
            while let Ok(i) = rx.recv() {
                for j in i..n {
                    let sim = cosine_similarity(&dists[i].resonance_vector, 
                                               &dists[j].resonance_vector);
                    local_results.push((i, j, sim));
                }
                processed.fetch_add(1, Ordering::Relaxed);
            }
            local_results
        });
        handles.push(handle);
    }
    
    // Collect results from workers
    for handle in handles {
        let results = handle.join().unwrap();
        for (i, j, sim) in results {
            similarity_matrix[i][j] = sim;
            similarity_matrix[j][i] = sim;
        }
    }
    
    println!("  ✓ Computed {} x {} similarity matrix", n, n);
    
    let global_matrix = GlobalMatrix {
        files: all_distributions,
        similarity_matrix,
    };
    
    let matrix_json = serde_json::to_string_pretty(&global_matrix)?;
    fs::write("markov_global_matrix.json", matrix_json)?;
    println!("✓ Saved global matrix to markov_global_matrix.json\n");
    
    Ok(())
}

fn worker(worker_id: usize, receiver: Receiver<String>, 
          results: Arc<Mutex<Vec<SymbolScore>>>, 
          distributions: Arc<Mutex<Vec<FileDistribution>>>, 
          files_processed: Arc<Mutex<usize>>,
          memory_used: Arc<AtomicUsize>,
          files_skipped: Arc<AtomicUsize>) {
    let mut processed = 0;
    let mut failed_files = Vec::new();
    
    loop {
        match receiver.recv() {
            Ok(path) => {
                // Check memory budget before processing
                let current_mem = memory_used.load(Ordering::Relaxed);
                if current_mem >= MEMORY_BUDGET_BYTES {
                    files_skipped.fetch_add(1, Ordering::Relaxed);
                    processed += 1;
                    *files_processed.lock().unwrap() += 1;
                    continue;
                }
                
                match analyze_file_with_distribution(&path, 32) {
                    Ok((symbols, dist)) => {
                        let sym_count = symbols.len();
                        
                        // Estimate memory used by this file's data
                        let symbols_bytes = sym_count * std::mem::size_of::<SymbolScore>();
                        let cells_bytes = dist.cells.len() * (CELL_SIZE_ESTIMATE + dist.window_size);
                        let resonance_bytes = dist.resonance_vector.len() * 8;
                        let total_bytes = symbols_bytes + cells_bytes + resonance_bytes;
                        
                        // Atomically add to memory counter
                        memory_used.fetch_add(total_bytes, Ordering::Relaxed);
                        
                        results.lock().unwrap().extend(symbols);
                        distributions.lock().unwrap().push(dist);
                        
                        processed += 1;
                        *files_processed.lock().unwrap() += 1;
                    }
                    Err(e) => {
                        // Collect failed files for exclusion list
                        failed_files.push(path.clone());
                        processed += 1;
                        *files_processed.lock().unwrap() += 1;
                    }
                }
            }
            Err(_) => {
                // Write failed files to exclusion list on worker exit
                if !failed_files.is_empty() {
                    let exclude_file = format!("failed_files_worker_{}.txt", worker_id);
                    let _ = fs::write(&exclude_file, failed_files.join("\n"));
                }
                break;
            }
        }
    }
}

fn find_elf_files(root: &str, limit: usize) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let cache_file = "elf_files_cache.txt";
    
    // Try to load from cache
    if let Ok(cached) = fs::read_to_string(cache_file) {
        println!("Loading {} ELF files from cache", cached.lines().count());
        return Ok(cached.lines().take(limit).map(String::from).collect());
    }
    
    // Cache miss - do the slow find
    println!("Building ELF file cache (this is slow, only happens once)...");
    let output = std::process::Command::new("find")
        .arg(root)
        .arg("-type").arg("f")
        .arg("(")
        .arg("-name").arg("*.so")
        .arg("-o")
        .arg("-executable")
        .arg(")")
        .output()?;
    
    let all_paths = String::from_utf8_lossy(&output.stdout);
    fs::write(cache_file, &*all_paths)?;
    println!("Cached {} files to {}", all_paths.lines().count(), cache_file);
    
    Ok(all_paths.lines().take(limit).map(String::from).collect())
}

fn analyze_file_with_distribution(path: &str, window_size: usize) -> Result<(Vec<SymbolScore>, FileDistribution), Box<dyn std::error::Error>> {
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
    
    if text_size == 0 { return Ok((vec![], FileDistribution {
        file: path.to_string(),
        window_size,
        num_windows: 0,
        resonance_vector: vec![],
        cells: vec![],
    })); }
    
    let text = &buffer[text_start..text_start + text_size];
    let resonance = analyze_markov_resonance(text, window_size);
    
    // Build cell data - ONLY store TOP 100 highest resonance cells to fit 20GB limit
    let max_cells = 100;
    let num_windows = text.len() / window_size;
    
    // Create (index, score) pairs and sort by score descending
    let mut scored_cells: Vec<(usize, f64)> = resonance.iter()
        .enumerate()
        .map(|(i, &score)| (i, score))
        .collect();
    scored_cells.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    
    // Take top N cells by resonance score
    let mut cells = Vec::with_capacity(max_cells);
    for (cell_id, score) in scored_cells.iter().take(max_cells) {
        let offset = cell_id * window_size;
        let end = (offset + window_size).min(text.len());
        let byte_pattern = text[offset..end].to_vec();
        
        cells.push(CellData {
            cell_id: *cell_id,
            offset,
            window_size,
            resonance_score: *score,
            byte_pattern,
        });
    }
    
    let mut results = Vec::with_capacity(elf.syms.len());
    
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
            cell_offset: relative_offset,
            score: resonance[cell],
        });
    }
    
    let distribution = FileDistribution {
        file: path.to_string(),
        window_size,
        num_windows: resonance.len(),
        resonance_vector: resonance,
        cells,
    };
    
    Ok((results, distribution))
}

fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let min_len = a.len().min(b.len());
    if min_len == 0 { return 0.0; }
    
    let dot: f64 = a.iter().zip(b.iter()).take(min_len).map(|(x, y)| x * y).sum();
    let mag_a: f64 = a.iter().take(min_len).map(|x| x * x).sum::<f64>().sqrt();
    let mag_b: f64 = b.iter().take(min_len).map(|x| x * x).sum::<f64>().sqrt();
    
    if mag_a == 0.0 || mag_b == 0.0 { return 0.0; }
    dot / (mag_a * mag_b)
}

fn analyze_markov_resonance(text: &[u8], window_size: usize) -> Vec<f64> {
    let num_windows = text.len() / window_size;
    if num_windows < 2 { return vec![]; }
    
    // MEMORY LIMIT: Cap at 1000 windows to prevent OOM (1000x1000 = 1M f64 = 8MB)
    let max_windows = 1000;
    if num_windows > max_windows {
        // Sample windows uniformly instead of analyzing all
        return sample_resonance(text, window_size, max_windows);
    }
    
    // Pre-allocate matrix with capacity
    let mut matrix = vec![vec![0.0; num_windows]; num_windows];
    
    for i in 0..num_windows {
        let window_i = &text[i * window_size..(i + 1) * window_size];
        for j in 0..num_windows {
            let window_j = &text[j * window_size..(j + 1) * window_size];
            matrix[i][j] = hamming_similarity(window_i, window_j);
        }
    }
    
    let mut resonance = Vec::with_capacity(num_windows);
    for i in 0..num_windows {
        let mut corr = 0.0;
        for j in 0..num_windows {
            if i != j {
                corr += matrix[i][j] * matrix[j][i];
            }
        }
        resonance.push(corr);
    }
    
    resonance
}

fn sample_resonance(text: &[u8], window_size: usize, max_samples: usize) -> Vec<f64> {
    // For large binaries, sample uniformly across the text
    let total_windows = text.len() / window_size;
    let step = total_windows / max_samples;
    
    let mut sampled_windows = Vec::with_capacity(max_samples);
    for i in 0..max_samples {
        let idx = i * step;
        let start = idx * window_size;
        let end = (start + window_size).min(text.len());
        sampled_windows.push(&text[start..end]);
    }
    
    // Compute resonance only on sampled windows
    let mut matrix = vec![vec![0.0; max_samples]; max_samples];
    for i in 0..max_samples {
        for j in 0..max_samples {
            matrix[i][j] = hamming_similarity(sampled_windows[i], sampled_windows[j]);
        }
    }
    
    let mut resonance = Vec::with_capacity(max_samples);
    for i in 0..max_samples {
        let mut corr = 0.0;
        for j in 0..max_samples {
            if i != j {
                corr += matrix[i][j] * matrix[j][i];
            }
        }
        resonance.push(corr);
    }
    
    resonance
}

fn hamming_similarity(a: &[u8], b: &[u8]) -> f64 {
    let matches = a.iter().zip(b.iter()).filter(|(x, y)| x == y).count();
    matches as f64 / a.len() as f64
}
