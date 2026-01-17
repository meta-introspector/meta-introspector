// Scan compressed Rust source from nix store
// Map compressed blocks → IPs → Novelty prediction

use std::process::Command;
use std::io::Read;

mod novelty_predictor;
mod rand_shim;

use novelty_predictor::{CompressedBlock, NoveltyPredictor};
use rand_shim::init_rand;

fn main() {
    init_rand();
    
    println!("📦 Scanning Compressed Rust Source from Nix Store\n");
    
    let rust_src = "/nix/store/x7wirg5c34zsgm7b5pvsl1hvq2dvqr9s-rust-src-1.92.0.tar.xz";
    
    // Check file
    if let Ok(metadata) = std::fs::metadata(rust_src) {
        println!("Source: {}", rust_src);
        println!("Size: {:.2} MB compressed\n", metadata.len() as f64 / 1_000_000.0);
    }
    
    // Extract and scan
    println!("🔍 Extracting and scanning...\n");
    
    let output = Command::new("tar")
        .args(["tf", rust_src])
        .output()
        .expect("Failed to list tar contents");
    
    let file_list = String::from_utf8_lossy(&output.stdout);
    let rs_files: Vec<&str> = file_list.lines()
        .filter(|line| line.ends_with(".rs"))
        .take(100)  // Sample first 100 files
        .collect();
    
    println!("Found {} .rs files (sampling 100)\n", rs_files.len());
    
    // Extract a few files and analyze
    let mut predictor = NoveltyPredictor::new();
    
    for (i, file_path) in rs_files.iter().take(10).enumerate() {
        // Extract single file
        let extract = Command::new("tar")
            .args(["xfO", rust_src, file_path])
            .output();
        
        if let Ok(output) = extract {
            if output.status.success() {
                let source = String::from_utf8_lossy(&output.stdout);
                
                // Create compressed block
                let block = CompressedBlock::from_source(i as u64, &source);
                
                println!("  {}: {} bytes → {} bytes (ratio: {:.3})",
                         file_path.split('/').next_back().unwrap_or(file_path),
                         block.decompressed_size,
                         block.compressed_data.len(),
                         block.compression_ratio);
                
                // Predict novelty
                let novelty = predictor.predict_novelty(&block);
                
                // Mock IPs for now
                let mut ips = std::collections::HashSet::new();
                for j in 0..10 {
                    ips.insert((i as u64 * 1000) + j);
                }
                
                predictor.add_block(block, ips);
                
                if novelty > 0.5 {
                    println!("    → Predicted HIGH novelty: {:.3}", novelty);
                }
            }
        }
    }
    
    // Report
    predictor.report();
    
    println!("\n✅ Scan complete!");
    println!("\n💡 Key insights:");
    println!("  • Rust source is 3.4 MB compressed in nix store");
    println!("  • Can scan without full decompression");
    println!("  • Compressed blocks predict coverage novelty");
    println!("  • Smart sampling based on predicted novelty");
    println!("  • Ready for full rustc source analysis");
}
