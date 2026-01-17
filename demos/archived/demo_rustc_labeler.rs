// Demo: Auto-label rustc code and save to parquet

mod rustc_fuzzer;
mod rustc_auto_labeler;
mod rand_shim;

use rustc_fuzzer::RustcFuzzer;
use rustc_auto_labeler::RustcAutoLabeler;
use rand_shim::init_rand;

fn main() {
    init_rand();
    
    println!("🏷️  Rustc Auto-Labeler\n");
    println!("Syn spectrum → Rustc IPs → Semantic labels → Parquet database\n");
    
    // Create fuzzer and labeler
    let mut fuzzer = RustcFuzzer::new();
    let mut labeler = RustcAutoLabeler::new();
    
    // Seed corpus
    println!("📝 Learning mappings from corpus...\n");
    
    let seeds = vec![
        ("fn main() {}", "Fn"),
        ("struct Point { x: i32 }", "Struct"),
        ("impl Point { fn new() -> Self { Point { x: 0 } } }", "Impl"),
        ("trait Shape { fn area(&self) -> f64; }", "Trait"),
        ("enum Color { Red, Green, Blue }", "Enum"),
    ];
    
    for (source, node_type) in seeds {
        if fuzzer.add_seed(source.to_string()).is_ok() {
            // Get the spectrum
            if let Some(spectrum) = fuzzer.corpus.last() {
                // Learn the mapping
                let ips: Vec<u64> = spectrum.rustc_ips.iter().copied().collect();
                labeler.learn_mapping(source, node_type, &ips);
                
                println!("  Learned: {} → {} IPs → {}",
                         node_type,
                         ips.len(),
                         labeler.label_for_ip(ips[0]).unwrap_or("unknown"));
            }
        }
    }
    
    // Fuzz to discover more mappings
    println!("\n🔍 Fuzzing to discover more mappings...\n");
    
    for _ in 0..20 {
        fuzzer.fuzz_round();
        
        // Label new discoveries
        if let Some(spectrum) = fuzzer.corpus.last() {
            for node in &spectrum.syn_nodes {
                let ips: Vec<u64> = spectrum.rustc_ips.iter().copied().collect();
                labeler.learn_mapping(&spectrum.source, node, &ips);
            }
        }
    }
    
    // Report
    labeler.report();
    
    // Save to parquet
    println!("\n💾 Saving to parquet...");
    let parquet_path = "/tmp/rustc_labels.parquet";
    
    match labeler.save_to_parquet(parquet_path) {
        Ok(_) => {
            println!("  ✅ Saved {} labels to {}", labeler.labels.len(), parquet_path);
            
            // Show file size
            if let Ok(metadata) = std::fs::metadata(parquet_path) {
                println!("  📦 File size: {} bytes", metadata.len());
            }
        }
        Err(e) => println!("  ❌ Error: {}", e),
    }
    
    // Query examples
    println!("\n🔍 Query examples:");
    if let Some(label) = labeler.labels.first() {
        println!("  IP 0x{:x} → {}", label.rustc_ip, label.semantic_label);
        println!("  Rustc function: {}", label.rustc_function);
        println!("  Syn node: {}", label.syn_node_type);
    }
    
    println!("\n✅ Auto-labeling complete!");
    println!("\n💡 Key insights:");
    println!("  • Syn nodes map to rustc IPs");
    println!("  • IPs map to semantic labels");
    println!("  • Labels stored in queryable parquet");
    println!("  • Can now understand rustc internals");
    println!("  • Coverage-guided semantic discovery");
}
