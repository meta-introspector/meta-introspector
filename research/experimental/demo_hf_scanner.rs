// Demo: Scan HuggingFace for small Rust coding models

mod hf_model_scanner;
#[path = "../../rand_shim.rs"] mod rand_shim;

use hf_model_scanner::{HfModelScanner, get_gpu_memory};
use rand_shim::init_rand;

fn main() {
    init_rand();
    
    println!("🤗 HUGGINGFACE MODEL SCANNER: Find small Rust coding models\n");
    println!("{}", "=".repeat(80));
    
    // Detect GPU memory
    let gpu_mem = get_gpu_memory();
    println!("\n🎮 GPU Memory: {} MB\n", gpu_mem);
    
    // Create scanner with GPU constraint
    let mut scanner = HfModelScanner::new(gpu_mem);
    
    println!("🔍 Scanning HuggingFace for Rust coding models...\n");
    
    // Try API search
    if scanner.search_api().is_some() {
        println!("  ✓ API search completed");
    } else {
        println!("  ⚠ API search failed, using mock data");
        
        // Add some known small Rust models
        scanner.models.push(hf_model_scanner::HfModel {
            name: "bigcode/tiny-starcoder".to_string(),
            size_mb: 164,
            tags: vec!["code".to_string(), "rust".to_string()],
            downloads: 10000,
            fits_in_gpu: true,
        });
        
        scanner.models.push(hf_model_scanner::HfModel {
            name: "Salesforce/codegen-350M-mono".to_string(),
            size_mb: 350,
            tags: vec!["code".to_string()],
            downloads: 50000,
            fits_in_gpu: gpu_mem >= 350,
        });
        
        scanner.models.push(hf_model_scanner::HfModel {
            name: "replit/replit-code-v1-3b".to_string(),
            size_mb: 3000,
            tags: vec!["code".to_string()],
            downloads: 100000,
            fits_in_gpu: gpu_mem >= 3000,
        });
    }
    
    println!("\n{}", "=".repeat(80));
    
    scanner.report();
    
    println!("\n{}", "=".repeat(80));
    println!("\n📦 Sampling small models...\n");
    
    let samples = scanner.sample_models(5);
    
    println!("Selected {} models for analysis:", samples.len());
    for (i, model) in samples.iter().enumerate() {
        println!("  {}. {} ({} MB)", i + 1, model.name, model.size_mb);
    }
    
    println!("\n{}", "=".repeat(80));
    println!("\n🧠 What we'll do with these models:\n");
    
    println!("For each model:");
    println!("  1. Download to GPU");
    println!("  2. Extract symbol table");
    println!("  3. Record weight activations for syn types");
    println!("  4. Map syn → weights");
    println!("  5. Distill to Rust submodules");
    println!("  6. Store in blockchain");
    
    println!("\nEconomic model:");
    println!("  • Nodes earn coins for processing models");
    println!("  • Pay for GPU time");
    println!("  • Earn more for finding new mappings");
    println!("  • Share findings via P2P network");
    
    println!("\nIntegration:");
    println!("  • HuggingFace models → Symbol extraction");
    println!("  • Symbols → Syn type mappings");
    println!("  • Mappings → Distilled submodules");
    println!("  • Submodules → Content store");
    println!("  • Provenance → Blockchain");
    
    println!("\n{}", "=".repeat(80));
    println!("\n✅ READY TO PROCESS:\n");
    
    println!("Small models that fit in {} MB GPU:", gpu_mem);
    for model in scanner.filter_small_models().iter().take(5) {
        println!("  ✓ {} ({} MB)", model.name, model.size_mb);
    }
    
    println!("\nNext steps:");
    println!("  1. Download selected models");
    println!("  2. Run ollama introspector on each");
    println!("  3. Extract syn → weight mappings");
    println!("  4. Compare across models");
    println!("  5. Find universal patterns");
    
    println!("\n{}", "=".repeat(80));
}
