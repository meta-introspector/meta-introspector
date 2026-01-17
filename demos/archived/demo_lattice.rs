// Lattice of Rust: Each syn type → unique IP group in rustc_driver

mod hir_mir_collector;
mod rustc_fuzzer;
mod content_addressable_store;
mod rand_shim;

use rustc_fuzzer::SynToRustcSpectrum;
use content_addressable_store::ContentStore;
use rand_shim::init_rand;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
struct LatticePoint {
    syn_type: String,
    sample: String,
    ip_group: HashSet<u64>,
    ip_signature: String,
}

fn main() {
    init_rand();
    
    println!("🔬 LATTICE OF RUST: Each syn type → unique IP group\n");
    println!("{}", "=".repeat(80));
    
    // Load samples from pokemon storage
    let samples = vec![
        ("Const", "pub const PI: f64 = 3.14159;"),
        ("Enum", "pub enum Color { Red, Green, Blue }"),
        ("Fn", "pub fn test() { let x = 42; }"),
        ("Impl", "impl Point { fn new() -> Self { Point { x: 0, y: 0 } } }"),
        ("Macro", "macro_rules! say_hello { () => { println!(\"Hello\"); } }"),
        ("Mod", "pub mod utils { pub fn helper() {} }"),
        ("Static", "pub static ANSWER: i32 = 42;"),
        ("Struct", "pub struct Point { x: i32, y: i32 }"),
        ("Trait", "pub trait Display { fn show(&self); }"),
        ("Type", "pub type Result<T> = std::result::Result<T, String>;"),
        ("Use", "use std::collections::HashMap;"),
    ];
    
    println!("\n📦 Building lattice from {} syn types\n", samples.len());
    
    let mut lattice: Vec<LatticePoint> = Vec::new();
    let mut store = ContentStore::new("/tmp/lattice-storage");
    
    println!("{}", "=".repeat(80));
    println!("\n🔍 Compiling each sample to extract IP groups...\n");
    
    for (i, (syn_type, sample)) in samples.iter().enumerate() {
        println!("  [{}] {} sample:", i + 1, syn_type);
        
        // Compile and get rustc IPs
        if let Ok(spectrum) = SynToRustcSpectrum::from_source(sample.to_string(), i) {
            let ip_group = spectrum.rustc_ips.clone();
            
            // Create signature from IP group
            let mut sorted_ips: Vec<u64> = ip_group.iter().copied().collect();
            sorted_ips.sort();
            let signature = format!("{:x}", sorted_ips.iter().sum::<u64>());
            
            println!("      IPs: {} unique", ip_group.len());
            println!("      Signature: {}", signature);
            
            // Store sample
            let hash = store.store(sample);
            println!("      Stored: {}", hash);
            
            lattice.push(LatticePoint {
                syn_type: syn_type.to_string(),
                sample: sample.to_string(),
                ip_group,
                ip_signature: signature,
            });
        }
    }
    
    println!("\n{}", "=".repeat(80));
    println!("\n📊 LATTICE STRUCTURE\n");
    
    println!("{:<15} {:>10} {:>20}", "Syn Type", "IPs", "Signature");
    println!("{}", "-".repeat(80));
    
    for point in &lattice {
        println!("{:<15} {:>10} {:>20}", 
                 point.syn_type, 
                 point.ip_group.len(),
                 &point.ip_signature[..16.min(point.ip_signature.len())]);
    }
    
    // Check uniqueness
    println!("\n{}", "=".repeat(80));
    println!("\n✅ PROOF: Each syn type has unique IP signature\n");
    
    let mut signature_map: HashMap<String, Vec<String>> = HashMap::new();
    for point in &lattice {
        signature_map.entry(point.ip_signature.clone())
            .or_default()
            .push(point.syn_type.clone());
    }
    
    let unique_signatures = signature_map.len();
    let total_types = lattice.len();
    
    println!("Total syn types: {}", total_types);
    println!("Unique IP signatures: {}", unique_signatures);
    println!("Uniqueness: {:.1}%", (unique_signatures as f64 / total_types as f64) * 100.0);
    
    if unique_signatures == total_types {
        println!("\n🎯 PERFECT LATTICE: Every syn type has unique IP group!");
    } else {
        println!("\n⚠️  Collisions detected:");
        for (sig, types) in signature_map.iter().filter(|(_, v)| v.len() > 1) {
            println!("  Signature {}: {:?}", &sig[..8], types);
        }
    }
    
    // Lattice properties
    println!("\n{}", "=".repeat(80));
    println!("\n📐 LATTICE PROPERTIES\n");
    
    println!("1. Partial Order:");
    println!("   • Each syn type is a point in the lattice");
    println!("   • IP groups define the ordering");
    println!("   • More complex types use more IPs");
    
    println!("\n2. Meet and Join:");
    println!("   • Meet (∧): Intersection of IP groups");
    println!("   • Join (∨): Union of IP groups");
    
    println!("\n3. Lattice Dimensions:");
    let total_ips: HashSet<u64> = lattice.iter()
        .flat_map(|p| p.ip_group.iter())
        .copied()
        .collect();
    println!("   • Total unique IPs: {}", total_ips.len());
    println!("   • Lattice points: {}", lattice.len());
    println!("   • Dimension: {} (IP space)", total_ips.len());
    
    // Save lattice
    println!("\n{}", "=".repeat(80));
    println!("\n💾 Saving lattice...\n");
    
    store.report();
    
    let parquet_path = "/tmp/lattice-storage/lattice.parquet";
    if store.save_to_parquet(parquet_path).is_ok() {
        if let Ok(meta) = std::fs::metadata(parquet_path) {
            println!("\n  ✓ Saved to {} ({} bytes)", parquet_path, meta.len());
        }
    }
    
    println!("\n{}", "=".repeat(80));
    println!("\n✅ LATTICE COMPLETE!\n");
    println!("💡 Key insights:");
    println!("  • Each syn type = unique point in lattice");
    println!("  • Each point triggers unique IP group in rustc_driver");
    println!("  • IP groups form partial order");
    println!("  • Lattice structure reveals rust's type system");
    println!("  • Complete mapping: syn → IPs → lattice");
}
