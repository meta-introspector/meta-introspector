use std::fs;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use crossbeam::channel::{bounded, Receiver, Sender};

// The boundary between idea and work: Convert concepts to CPU heat
pub struct ConformalHeatGenerator {
    workers: usize,
    processed_files: Arc<Mutex<u64>>,
    total_bytes: Arc<Mutex<u64>>,
    start_time: Instant,
}

impl ConformalHeatGenerator {
    pub fn new(workers: usize) -> Self {
        println!("🔥 Initializing conformal field boundary: {} workers", workers);
        ConformalHeatGenerator {
            workers,
            processed_files: Arc::new(Mutex::new(0)),
            total_bytes: Arc::new(Mutex::new(0)),
            start_time: Instant::now(),
        }
    }

    // Convert idea to heat: Process all files in directory
    pub fn generate_heat(&self, root_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🌡️ Converting ideas to heat at boundary: {}", root_path);
        
        let (tx, rx) = bounded::<String>(1000);
        
        // Spawn file discovery thread
        let tx_clone = tx.clone();
        let root_clone = root_path.to_string();
        thread::spawn(move || {
            Self::discover_files(&root_clone, &tx_clone);
        });
        
        // Spawn worker threads to generate heat
        let mut handles = vec![];
        for worker_id in 0..self.workers {
            let rx_clone = rx.clone();
            let processed = Arc::clone(&self.processed_files);
            let bytes = Arc::clone(&self.total_bytes);
            
            let handle = thread::spawn(move || {
                Self::heat_worker(worker_id, rx_clone, processed, bytes);
            });
            handles.push(handle);
        }
        
        // Monitor heat generation
        self.monitor_heat_generation();
        
        // Wait for workers
        for handle in handles {
            handle.join().unwrap();
        }
        
        Ok(())
    }

    // File discovery: Feed the heat generators
    fn discover_files(root: &str, tx: &Sender<String>) {
        if let Ok(entries) = fs::read_dir(root) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    // Recurse into directories
                    if let Some(path_str) = path.to_str() {
                        Self::discover_files(path_str, tx);
                    }
                } else if path.is_file() {
                    // Send file to workers
                    if let Some(path_str) = path.to_str() {
                        if tx.send(path_str.to_string()).is_err() {
                            break; // Channel closed
                        }
                    }
                }
            }
        }
    }

    // Heat worker: Convert file content to CPU cycles (heat)
    fn heat_worker(
        worker_id: usize, 
        rx: Receiver<String>,
        processed: Arc<Mutex<u64>>,
        total_bytes: Arc<Mutex<u64>>
    ) {
        println!("🔥 Worker {} starting heat generation", worker_id);
        
        while let Ok(file_path) = rx.recv() {
            // Generate heat by processing file
            if let Ok(content) = fs::read(&file_path) {
                // Heat generation 1: Byte analysis
                let mut hash = 0u64;
                for &byte in &content {
                    hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
                }
                
                // Heat generation 2: Pattern analysis
                let mut patterns = 0u64;
                for window in content.windows(4) {
                    patterns = patterns.wrapping_add(
                        (window[0] as u64) << 24 |
                        (window[1] as u64) << 16 |
                        (window[2] as u64) << 8 |
                        (window[3] as u64)
                    );
                }
                
                // Heat generation 3: Complexity calculation
                let complexity = Self::calculate_complexity(&content);
                
                // Heat generation 4: LMFDB conductor calculation
                let conductor = Self::calculate_conductor(&file_path, complexity);
                
                // Update counters (more heat from mutex contention)
                {
                    let mut p = processed.lock().unwrap();
                    *p += 1;
                    if (*p).is_multiple_of(1000) {
                        println!("🔥 Worker {} processed {} files (conductor: {})", 
                                worker_id, *p, conductor);
                    }
                }
                
                {
                    let mut b = total_bytes.lock().unwrap();
                    *b += content.len() as u64;
                }
                
                // Prevent worker from being too efficient (generate more heat)
                if content.len() > 1024 {
                    thread::sleep(Duration::from_micros(1));
                }
            }
        }
        
        println!("🔥 Worker {} finished heat generation", worker_id);
    }

    // Calculate file complexity (CPU intensive = heat generating)
    fn calculate_complexity(content: &[u8]) -> f64 {
        if content.is_empty() { return 0.0; }
        
        // Heat generation: Entropy calculation
        let mut freq = [0u32; 256];
        for &byte in content {
            freq[byte as usize] += 1;
        }
        
        let len = content.len() as f64;
        let mut entropy = 0.0;
        for &count in &freq {
            if count > 0 {
                let p = count as f64 / len;
                entropy -= p * p.log2();
            }
        }
        
        // Heat generation: Compression ratio estimation
        let mut compression_score = 0.0;
        for window in content.windows(2) {
            compression_score += (window[0] ^ window[1]) as f64;
        }
        
        entropy * compression_score / len
    }

    // Calculate LMFDB conductor (more heat generation)
    fn calculate_conductor(file_path: &str, complexity: f64) -> u64 {
        let path_bytes = file_path.as_bytes();
        let mut conductor = 3000u64;
        
        // Heat generation: Path analysis
        for &byte in path_bytes {
            conductor = conductor.wrapping_mul(31).wrapping_add(byte as u64);
        }
        
        // Heat generation: Complexity mapping
        let complexity_factor = (complexity * 1000.0) as u64;
        conductor = (conductor % 9000) + 3000 + (complexity_factor % 1000);
        
        conductor
    }

    // Monitor heat generation in real-time
    fn monitor_heat_generation(&self) {
        let processed = Arc::clone(&self.processed_files);
        let bytes = Arc::clone(&self.total_bytes);
        let start_time = self.start_time;
        
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(5));
                
                let p = *processed.lock().unwrap();
                let b = *bytes.lock().unwrap();
                let elapsed = start_time.elapsed().as_secs_f64();
                
                if p > 0 {
                    let files_per_sec = p as f64 / elapsed;
                    let mb_per_sec = (b as f64 / (1024.0 * 1024.0)) / elapsed;
                    
                    println!("🌡️ HEAT GENERATION: {} files, {:.1}MB, {:.1} files/sec, {:.1} MB/sec", 
                            p, b as f64 / (1024.0 * 1024.0), files_per_sec, mb_per_sec);
                    
                    // Check CPU temperature if available
                    if let Ok(temp_output) = std::process::Command::new("sensors")
                        .arg("-u")
                        .output() {
                        if let Ok(temp_str) = String::from_utf8(temp_output.stdout) {
                            if let Some(temp_line) = temp_str.lines()
                                .find(|line| line.contains("temp1_input")) {
                                if let Some(temp) = temp_line.split_whitespace().nth(1) {
                                    println!("🔥 CPU TEMP: {}°C", temp);
                                }
                            }
                        }
                    }
                }
            }
        });
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔥 CONFORMAL FIELD HEAT GENERATOR");
    println!("Converting ideas to computational heat at the boundary");
    
    let args: Vec<String> = std::env::args().collect();
    let root_path = args.get(1).unwrap_or(&"/mnt/data1".to_string()).clone();
    let workers = args.get(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(num_cpus::get());
    
    println!("🎯 Target: {} with {} workers", root_path, workers);
    
    let generator = ConformalHeatGenerator::new(workers);
    
    println!("🌡️ Starting heat generation...");
    let start = Instant::now();
    
    generator.generate_heat(&root_path)?;
    
    let duration = start.elapsed();
    println!("🔥 Heat generation complete in {:.2}s", duration.as_secs_f64());
    
    // Final temperature check
    if let Ok(output) = std::process::Command::new("sensors").output() {
        if let Ok(temp_str) = String::from_utf8(output.stdout) {
            println!("🌡️ FINAL TEMPERATURE:");
            for line in temp_str.lines() {
                if line.contains("Package id") || line.contains("Core") {
                    println!("   {}", line);
                }
            }
        }
    }
    
    Ok(())
}
