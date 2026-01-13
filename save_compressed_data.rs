use std::fs;
use std::process::Command;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    println!("💾 SAVING COMPRESSED RUST-BUILD DATA");
    
    let rust_build_path = "/home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust-build";
    let compressed_dir = "/tmp/compressed_rust_build";
    let tar_file = "/tmp/rust_build_compressed.tar";
    
    // Create compressed directory
    fs::create_dir_all(&compressed_dir).unwrap();
    
    // Find all .rs files
    let find_output = Command::new("find")
        .arg(rust_build_path)
        .arg("-name")
        .arg("*.rs")
        .arg("-type")
        .arg("f")
        .output()
        .expect("Failed to find files");
    
    let files: Vec<&str> = std::str::from_utf8(&find_output.stdout).unwrap().lines().collect();
    
    println!("🗜️  Compressing {} files...", files.len());
    
    let mut total_original = 0u64;
    let mut total_compressed = 0u64;
    
    // Compress each file and save
    for (i, file_path) in files.iter().enumerate() {
        if let Ok(content) = fs::read_to_string(file_path) {
            let original_size = content.len() as u64;
            let compressed = compress_rust_content(&content);
            let compressed_size = compressed.len() as u64;
            
            total_original += original_size;
            total_compressed += compressed_size;
            
            // Save compressed file
            let compressed_filename = format!("{}/file_{:06}.compressed", compressed_dir, i);
            fs::write(&compressed_filename, &compressed).unwrap();
            
            if i % 1000 == 0 {
                println!("  📄 Processed {} files", i);
            }
        }
    }
    
    println!("✅ Compression complete:");
    println!("  Original: {:.2}MB", total_original as f64 / 1_000_000.0);
    println!("  Compressed: {:.2}MB", total_compressed as f64 / 1_000_000.0);
    println!("  Ratio: {:.1}%", (1.0 - total_compressed as f64 / total_original as f64) * 100.0);
    
    // Create tar file
    println!("\n📦 Creating tar archive...");
    let tar_result = Command::new("tar")
        .arg("-cf")
        .arg(&tar_file)
        .arg("-C")
        .arg("/tmp")
        .arg("compressed_rust_build")
        .output()
        .unwrap();
    
    if tar_result.status.success() {
        let tar_size = fs::metadata(&tar_file).unwrap().len();
        println!("✅ Tar created: {:.2}MB", tar_size as f64 / 1_000_000.0);
        
        // Test different compressions on the tar
        test_compressions(&tar_file, tar_size);
        
        // Compare to original rust-build size
        compare_to_original(rust_build_path, tar_size);
    }
    
    println!("⏱️  Total time: {:.2}s", start.elapsed().as_secs_f64());
}

fn compress_rust_content(content: &str) -> Vec<u8> {
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
                compressed.push(token.len() as u8);
                compressed.extend_from_slice(token.as_bytes());
            }
        }
    }
    compressed
}

fn test_compressions(tar_file: &str, original_tar_size: u64) {
    println!("\n🧪 TESTING ADDITIONAL COMPRESSIONS:");
    
    // Test gzip
    let gzip_result = Command::new("gzip")
        .arg("-c")
        .arg(tar_file)
        .output()
        .unwrap();
    
    if gzip_result.status.success() {
        let gzip_file = format!("{}.gz", tar_file);
        fs::write(&gzip_file, &gzip_result.stdout).unwrap();
        let gzip_size = gzip_result.stdout.len() as u64;
        let gzip_ratio = (1.0 - gzip_size as f64 / original_tar_size as f64) * 100.0;
        println!("  📦 Gzip: {:.2}MB ({:.1}% additional compression)", 
            gzip_size as f64 / 1_000_000.0, gzip_ratio);
    }
    
    // Test xz
    let xz_result = Command::new("xz")
        .arg("-c")
        .arg(tar_file)
        .output()
        .unwrap();
    
    if xz_result.status.success() {
        let xz_file = format!("{}.xz", tar_file);
        fs::write(&xz_file, &xz_result.stdout).unwrap();
        let xz_size = xz_result.stdout.len() as u64;
        let xz_ratio = (1.0 - xz_size as f64 / original_tar_size as f64) * 100.0;
        println!("  📦 XZ: {:.2}MB ({:.1}% additional compression)", 
            xz_size as f64 / 1_000_000.0, xz_ratio);
    }
    
    // Test bzip2
    let bzip2_result = Command::new("bzip2")
        .arg("-c")
        .arg(tar_file)
        .output()
        .unwrap();
    
    if bzip2_result.status.success() {
        let bzip2_file = format!("{}.bz2", tar_file);
        fs::write(&bzip2_file, &bzip2_result.stdout).unwrap();
        let bzip2_size = bzip2_result.stdout.len() as u64;
        let bzip2_ratio = (1.0 - bzip2_size as f64 / original_tar_size as f64) * 100.0;
        println!("  📦 Bzip2: {:.2}MB ({:.1}% additional compression)", 
            bzip2_size as f64 / 1_000_000.0, bzip2_ratio);
    }
}

fn compare_to_original(rust_build_path: &str, compressed_tar_size: u64) {
    println!("\n📊 COMPARISON TO ORIGINAL:");
    
    // Get original rust-build size
    let du_result = Command::new("du")
        .arg("-sb")
        .arg(rust_build_path)
        .output()
        .unwrap();
    
    if du_result.status.success() {
        let du_output = String::from_utf8_lossy(&du_result.stdout);
        if let Some(size_str) = du_output.split_whitespace().next() {
            if let Ok(original_size) = size_str.parse::<u64>() {
                let compression_ratio = (1.0 - compressed_tar_size as f64 / original_size as f64) * 100.0;
                
                println!("  📁 Original rust-build: {:.2}MB", original_size as f64 / 1_000_000.0);
                println!("  📦 Our compressed tar: {:.2}MB", compressed_tar_size as f64 / 1_000_000.0);
                println!("  🎯 Overall compression: {:.1}%", compression_ratio);
                println!("  📈 Compression ratio: {:.1}:1", original_size as f64 / compressed_tar_size as f64);
            }
        }
    }
    
    // Create original tarball for comparison
    println!("\n📦 Creating original tarball for comparison...");
    let original_tar = "/tmp/rust_build_original.tar";
    let original_tar_result = Command::new("tar")
        .arg("-cf")
        .arg(original_tar)
        .arg("-C")
        .arg("/home/mdupont/nix/vendor/rust/cargo2nix/submodules")
        .arg("rust-build")
        .output()
        .unwrap();
    
    if original_tar_result.status.success() {
        let original_tar_size = fs::metadata(original_tar).unwrap().len();
        let tar_compression = (1.0 - compressed_tar_size as f64 / original_tar_size as f64) * 100.0;
        
        println!("  📦 Original tar: {:.2}MB", original_tar_size as f64 / 1_000_000.0);
        println!("  📦 Compressed tar: {:.2}MB", compressed_tar_size as f64 / 1_000_000.0);
        println!("  🎯 Tar-to-tar compression: {:.1}%", tar_compression);
    }
}
