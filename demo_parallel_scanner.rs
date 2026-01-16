// Demo: Parallel Duplication Scanner on allrs.txt

mod parallel_duplication_scanner;
use parallel_duplication_scanner::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 PARALLEL DUPLICATION SCANNER");
    println!("================================\n");
    
    let list_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "/home/mdupont/nix/index/allrs.txt".to_string());
    
    let output_path = std::env::args()
        .nth(2)
        .unwrap_or_else(|| "/tmp/duplications.parquet".to_string());
    
    let num_cpus = std::env::args()
        .nth(3)
        .and_then(|s| s.parse().ok())
        .unwrap_or(24);
    
    println!("📋 Configuration:");
    println!("  Input: {}", list_path);
    println!("  Output: {}", output_path);
    println!("  CPUs: {}\n", num_cpus);
    
    // Run scan
    let start = std::time::Instant::now();
    
    run_parallel_scan(&list_path, &output_path, num_cpus)?;
    
    let duration = start.elapsed();
    
    println!("\n✅ Scan Complete!");
    println!("  Duration: {:.2}s", duration.as_secs_f64());
    println!("  Output: {}", output_path);
    
    println!("\n💡 Usage:");
    println!("  # Default (24 CPUs)");
    println!("  cargo run --release --bin demo_parallel_scanner");
    
    println!("\n  # Custom");
    println!("  cargo run --release --bin demo_parallel_scanner \\");
    println!("    ~/nix/index/allrs.txt \\");
    println!("    /tmp/duplications.parquet \\");
    println!("    24");
    
    println!("\n📊 Dataset Ready:");
    println!("  • Parquet format");
    println!("  • Compressed");
    println!("  • Deduplicated");
    println!("  • Ready for HuggingFace");
    
    Ok(())
}
