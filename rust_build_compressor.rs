use std::fs;
use std::path::Path;
use std::time::Instant;
use crossbeam::channel;
use std::thread;

fn main() {
    let start = Instant::now();
    let rust_build_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust-build";
    
    println!("🚀 Starting rust-build compression...");
    
    // Find all .rs files
    let output = std::process::Command::new("find")
        .arg(rust_build_path)
        .arg("-name")
        .arg("*.rs")
        .arg("-type")
        .arg("f")
        .output()
        .expect("Failed to find files");
    
    let files: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect();
    
    println!("📁 Found {} Rust files", files.len());
    
    let (sender, receiver) = channel::bounded(1000);
    let mut handles = vec![];
    
    // Spawn 20 worker threads
    for _ in 0..20 {
        let rx = receiver.clone();
        let handle = thread::spawn(move || {
            let mut total_original = 0u64;
            let mut total_compressed = 0u64;
            let mut files_processed = 0;
            
            while let Ok(file_path) = rx.recv() {
                if let Ok(content) = fs::read_to_string(&file_path) {
                    let original_size = content.len() as u64;
                    let compressed = compress_rust_content(&content);
                    let compressed_size = compressed.len() as u64;
                    
                    total_original += original_size;
                    total_compressed += compressed_size;
                    files_processed += 1;
                    
                    if files_processed % 100 == 0 {
                        println!("  📄 Processed {} files", files_processed);
                    }
                }
            }
            (files_processed, total_original, total_compressed)
        });
        handles.push(handle);
    }
    
    // Send files to workers
    for file in files {
        sender.send(file).unwrap();
    }
    drop(sender);
    
    // Collect results
    let mut total_files = 0;
    let mut total_original = 0u64;
    let mut total_compressed = 0u64;
    
    for handle in handles {
        let (files, orig, comp) = handle.join().unwrap();
        total_files += files;
        total_original += orig;
        total_compressed += comp;
    }
    
    let compression_ratio = (total_compressed as f64 / total_original as f64) * 100.0;
    let savings = 100.0 - compression_ratio;
    
    println!("\n🎯 rust-build Compression Complete!");
    println!("📊 Files processed: {}", total_files);
    println!("📦 Original size: {:.2}MB", total_original as f64 / 1_000_000.0);
    println!("🗜️  Compressed size: {:.2}MB", total_compressed as f64 / 1_000_000.0);
    println!("💾 Space savings: {:.1}%", savings);
    println!("⏱️  Time taken: {:.2}s", start.elapsed().as_secs_f64());
}

fn compress_rust_content(content: &str) -> Vec<u8> {
    // Simple grammar-based compression
    let mut compressed = Vec::new();
    let tokens: Vec<&str> = content.split_whitespace().collect();
    
    for token in tokens {
        match token {
            "fn" => compressed.push(1),
            "struct" => compressed.push(2),
            "impl" => compressed.push(3),
            "use" => compressed.push(4),
            "pub" => compressed.push(5),
            "let" => compressed.push(6),
            "mut" => compressed.push(7),
            "if" => compressed.push(8),
            "else" => compressed.push(9),
            "match" => compressed.push(10),
            _ => {
                compressed.push(0);
                compressed.extend_from_slice(token.as_bytes());
                compressed.push(255);
            }
        }
    }
    compressed
}
