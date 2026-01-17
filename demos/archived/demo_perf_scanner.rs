// Demo: Fast perf scan of our Rust code to build comprehension

mod rustc_perf_scanner;
mod rust_spectrum_comprehension;
mod rand_shim;

use rustc_perf_scanner::RustcPerfScanner;
use rust_spectrum_comprehension::RustSpectrum;
use rand_shim::init_rand;

fn main() {
    init_rand();
    
    println!("⚡ Fast Rustc Perf Scanner\n");
    println!("Scanning our code to build comprehension...\n");
    
    let mut scanner = RustcPerfScanner::new();
    
    // Scan our key files
    let our_files = vec![
        "/mnt/data1/meta-introspector/distributed_trading.rs",
        "/mnt/data1/meta-introspector/compression_memes.rs",
        "/mnt/data1/meta-introspector/meme_evolver.rs",
    ];
    
    println!("📦 Scanning files...\n");
    
    for file in our_files {
        if let Some(fp) = scanner.scan_file(file) {
            println!("  ✓ {} - {} IPs, ratio {:.3}", 
                     file.split('/').next_back().unwrap_or(file),
                     fp.ips.len(),
                     fp.compression_ratio);
        }
    }
    
    scanner.report();
    
    // Build spectrum from fingerprints
    println!("\n🔬 Building spectrum from fingerprints...\n");
    
    let mut spectrum = RustSpectrum::new();
    
    for fp in &scanner.fingerprints {
        spectrum.add_pattern(
            fp.test_name.clone(),
            fp.ips.clone(),
            fp.compression_ratio
        );
    }
    
    spectrum.report();
    
    println!("\n✅ Perf scan complete!");
    println!("\n💡 Key insights:");
    println!("  • Fast perf scan extracts rustc IPs");
    println!("  • Compress traces to fingerprints");
    println!("  • Build spectrum from our own code");
    println!("  • Each file = unique IP signature");
    println!("  • Comprehension through perf traces");
}
