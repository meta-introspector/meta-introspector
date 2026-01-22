// Demo: Use Rust stdlib spectrum to comprehend our own code

#[path = "../../rust_spectrum_comprehension.rs"] mod rust_spectrum_comprehension;
#[path = "../../rustc_fuzzer.rs"] mod rustc_fuzzer;
#[path = "../../rand_shim.rs"] mod rand_shim;

use rust_spectrum_comprehension::RustSpectrum;
use rustc_fuzzer::SynToRustcSpectrum;
use rand_shim::init_rand;
use std::collections::HashSet;

fn main() {
    init_rand();
    
    println!("🔬 Building Rust Spectrum from Stdlib\n");
    
    // Simulate processed stdlib blocks (from previous demo)
    let mut spectrum = RustSpectrum::new();
    
    // Add some stdlib patterns
    let stdlib_files = vec![
        ("std::collections::HashMap", 100),
        ("std::io::Read", 80),
        ("std::fmt::Display", 60),
        ("std::iter::Iterator", 120),
        ("std::sync::Mutex", 90),
    ];
    
    for (name, base_ip) in stdlib_files {
        let mut ips = HashSet::new();
        for i in 0..10 {
            ips.insert(base_ip + i);
        }
        spectrum.add_pattern(name.to_string(), ips, 0.3);
    }
    
    spectrum.report();
    
    println!("\n🔍 Comprehending our own code...\n");
    
    // Analyze our own code
    let our_files = vec![
        "demo_shared_memory.rs",
        "distributed_trading.rs",
        "compression_memes.rs",
    ];
    
    for file_path in our_files {
        let full_path = format!("/mnt/data1/meta-introspector/{}", file_path);
        
        if let Ok(source) = std::fs::read_to_string(&full_path) {
            println!("📄 Analyzing {}...", file_path);
            
            if let Ok(syn_spectrum) = SynToRustcSpectrum::from_source(source, 0) {
                let comprehension = spectrum.comprehend(&syn_spectrum.rustc_ips);
                comprehension.report(&spectrum);
                println!();
            }
        }
    }
    
    println!("✅ Comprehension complete!");
    println!("\n💡 Key insights:");
    println!("  • Stdlib spectrum maps IPs → known patterns");
    println!("  • Unknown code analyzed via rustc IPs");
    println!("  • Match IPs to stdlib patterns");
    println!("  • Measure coverage: known vs unknown");
    println!("  • Comprehend code through stdlib lens");
}
