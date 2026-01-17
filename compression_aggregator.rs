use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 AGGREGATING COMPRESSION RESULTS");
    
    // Parse the compression log
    let log_content = fs::read_to_string("/mnt/data1/meta-introspector/full_rustc_compression.log")?;
    
    let mut total_files = 0;
    let mut total_original_mb = 0.0;
    let mut total_compressed_mb = 0.0;
    let mut compression_ratios = Vec::new();
    let mut compressed_files = Vec::new();
    
    for line in log_content.lines() {
        if line.contains("🗜️  Compressed:") {
            // Extract file info: 🗜️  Compressed: path (X bytes -> Y tokens)
            if let Some(parts) = line.split(" (").nth(1) {
                if let Some(size_part) = parts.split(" bytes -> ").next() {
                    if let Ok(bytes) = size_part.parse::<u64>() {
                        let mb = bytes as f64 / 1_000_000.0;
                        total_original_mb += mb;
                        
                        // Extract file path
                        let path = line.split("🗜️  Compressed: ").nth(1)
                            .and_then(|s| s.split(" (").next())
                            .unwrap_or("unknown");
                        
                        compressed_files.push(path.to_string());
                    }
                }
            }
            total_files += 1;
        } else if line.contains("Space saved: ") {
            // Extract compression ratio
            if let Some(percent_str) = line.split("Space saved: ").nth(1) {
                if let Some(percent) = percent_str.split("%").next() {
                    if let Ok(ratio) = percent.parse::<f64>() {
                        compression_ratios.push(ratio);
                        total_compressed_mb += total_original_mb * (1.0 - ratio / 100.0);
                    }
                }
            }
        }
    }
    
    // Calculate averages
    let avg_compression = compression_ratios.iter().sum::<f64>() / compression_ratios.len() as f64;
    let total_space_saved = (1.0 - (total_compressed_mb / total_original_mb)) * 100.0;
    
    println!("\n📊 AGGREGATED COMPRESSION RESULTS:");
    println!("Total files compressed: {}", total_files);
    println!("Total original size: {:.2} MB", total_original_mb);
    println!("Total compressed size: {:.2} MB", total_compressed_mb);
    println!("Average compression ratio: {:.1}%", avg_compression);
    println!("Total space saved: {:.1}%", total_space_saved);
    println!("Best compression: {:.1}%", compression_ratios.iter().fold(0.0f64, |a, &b| a.max(b)));
    println!("Worst compression: {:.1}%", compression_ratios.iter().fold(100.0f64, |a, &b| a.min(b)));
    
    // Save aggregated results
    let results = serde_json::json!({
        "total_files_compressed": total_files,
        "total_original_mb": total_original_mb,
        "total_compressed_mb": total_compressed_mb,
        "average_compression_percent": avg_compression,
        "total_space_saved_percent": total_space_saved,
        "best_compression_percent": compression_ratios.iter().fold(0.0f64, |a, &b| a.max(b)),
        "worst_compression_percent": compression_ratios.iter().fold(100.0f64, |a, &b| a.min(b)),
        "compressed_files": compressed_files,
        "all_compression_ratios": compression_ratios
    });
    
    fs::write("/mnt/data1/meta-introspector/aggregated_compression_results.json", 
              serde_json::to_string_pretty(&results)?)?;
    
    println!("\n💾 Results saved to: aggregated_compression_results.json");
    println!("🎯 Individual compressed data in: /home/mdupont/nix/vendor/rust/cargo2nix/submodules/rust-build/rustc_intercept_compression.json");
    
    Ok(())
}
