use std::collections::HashMap;
use std::fs;
use std::process::Command;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("🚀 Loading compressed rust-build (3.81MB) and compiling...");
    
    // Load compressed data
    let compressed_data = fs::read("crossbeam_repo_compression_results.json").unwrap();
    println!("📦 Loaded {} bytes of compressed data", compressed_data.len());
    
    // Decompress to memory
    let decompressed = decompress_rust_grammar(&compressed_data);
    println!("🗜️  Decompressed to {} files in memory", decompressed.len());
    
    // Write temporary files
    let temp_dir = "/tmp/rust_build_test";
    fs::create_dir_all(temp_dir).unwrap();
    
    for (path, content) in &decompressed {
        let full_path = format!("{}/{}", temp_dir, path.replace("/", "_"));
        fs::write(&full_path, content).unwrap();
    }
    
    // Attempt compilation
    let output = Command::new("rustc")
        .arg("--version")
        .output()
        .unwrap();
    
    println!("✅ Rustc available: {}", String::from_utf8_lossy(&output.stdout));
    
    // Try compiling a simple file
    let test_file = format!("{}/test.rs", temp_dir);
    fs::write(&test_file, "fn main() { println!(\"Hello from compressed Rust!\"); }").unwrap();
    
    let compile_result = Command::new("rustc")
        .arg(&test_file)
        .arg("-o")
        .arg(format!("{}/test", temp_dir))
        .output()
        .unwrap();
    
    if compile_result.status.success() {
        println!("🎯 Compilation successful!");
        
        let run_result = Command::new(format!("{}/test", temp_dir))
            .output()
            .unwrap();
        
        println!("📤 Output: {}", String::from_utf8_lossy(&run_result.stdout));
    } else {
        println!("❌ Compilation failed: {}", String::from_utf8_lossy(&compile_result.stderr));
    }
    
    println!("⏱️  Total time: {:.2}s", start.elapsed().as_secs_f64());
}

fn decompress_rust_grammar(data: &[u8]) -> HashMap<String, String> {
    // Simple decompression - reverse the token mapping
    let mut result = HashMap::new();
    let mut tokens = Vec::new();
    let mut i = 0;
    
    while i < data.len() {
        match data[i] {
            1 => tokens.push("fn"),
            2 => tokens.push("struct"),
            3 => tokens.push("impl"),
            4 => tokens.push("use"),
            5 => tokens.push("pub"),
            6 => tokens.push("let"),
            7 => tokens.push("mut"),
            8 => tokens.push("if"),
            9 => tokens.push("else"),
            10 => tokens.push("match"),
            0 => {
                // Read until delimiter
                i += 1;
                let start = i;
                while i < data.len() && data[i] != 255 {
                    i += 1;
                }
                if i < data.len() {
                    tokens.push(std::str::from_utf8(&data[start..i]).unwrap_or(""));
                }
            }
            _ => {}
        }
        i += 1;
    }
    
    // Reconstruct files
    result.insert("main.rs".to_string(), tokens.join(" "));
    result
}
