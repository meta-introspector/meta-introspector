// Demo: Predict novelty from compressed code patterns

mod novelty_predictor;
mod rustc_fuzzer;
mod rand_shim;

use novelty_predictor::{CompressedBlock, NoveltyPredictor};
use rustc_fuzzer::RustcFuzzer;
use rand_shim::init_rand;

fn main() {
    init_rand();
    
    println!("🔮 Novelty Predictor: Compressed Code → New Coverage\n");
    println!("Learn which compressed patterns reach new IPs\n");
    
    let mut predictor = NoveltyPredictor::new();
    let mut fuzzer = RustcFuzzer::new();
    
    // Phase 1: Learn from corpus
    println!("📚 Phase 1: Learning from corpus...\n");
    
    let seeds = ["fn main() {}",
        "struct Point { x: i32 }",
        "trait Shape { fn area(&self) -> f64; }",
        "enum Color { Red, Green, Blue }",
        "impl Point { fn new() -> Self { Point { x: 0 } } }"];
    
    for (id, source) in seeds.iter().enumerate() {
        // Add to fuzzer
        if fuzzer.add_seed(source.to_string()).is_ok() {
            // Get coverage
            if let Some(spectrum) = fuzzer.corpus.last() {
                // Create compressed block
                let block = CompressedBlock::from_source(id as u64, source);
                
                println!("  Block {}: {} bytes → {} bytes (ratio: {:.3})",
                         id,
                         block.decompressed_size,
                         block.compressed_data.len(),
                         block.compression_ratio);
                
                // Add to predictor
                predictor.add_block(block, spectrum.rustc_ips.clone());
            }
        }
    }
    
    // Phase 2: Fuzz and learn patterns
    println!("\n🔬 Phase 2: Fuzzing to discover patterns...\n");
    
    for gen in 0..20 {
        fuzzer.fuzz_round();
        
        // Every 5 generations, learn from new discoveries
        if gen % 5 == 4 {
            if let Some(spectrum) = fuzzer.corpus.last() {
                let block = CompressedBlock::from_source(
                    (seeds.len() + gen) as u64,
                    &spectrum.source
                );
                
                let novelty = predictor.predict_novelty(&block);
                
                println!("  Gen {}: Predicted novelty {:.3}, actual coverage {} IPs",
                         gen + 1,
                         novelty,
                         spectrum.coverage);
                
                predictor.add_block(block, spectrum.rustc_ips.clone());
            }
        }
    }
    
    // Phase 3: Test predictions
    println!("\n🎯 Phase 3: Testing predictions...\n");
    
    let test_cases = ["fn complex<T>(x: T) -> T { x }",
        "struct Container<T> { value: T }",
        "trait Processor { fn process(&self); }"];
    
    for (i, source) in test_cases.iter().enumerate() {
        let block = CompressedBlock::from_source(1000 + i as u64, source);
        let predicted_novelty = predictor.predict_novelty(&block);
        
        println!("  Test {}: Predicted novelty {:.3}",
                 i + 1,
                 predicted_novelty);
        
        if predicted_novelty > 0.5 {
            println!("    → HIGH NOVELTY: Likely to reach new coverage!");
        } else {
            println!("    → Low novelty: Similar to known patterns");
        }
    }
    
    // Report
    predictor.report();
    
    // Find most novel blocks
    println!("\n🌟 Top 5 Novel Blocks:");
    let novel = predictor.find_novel_blocks(0.3);
    for (i, (block_id, novelty)) in novel.iter().take(5).enumerate() {
        println!("  {}. Block {}: novelty {:.3}", i + 1, block_id, novelty);
    }
    
    println!("\n✅ Prediction complete!");
    println!("\n💡 Key insights:");
    println!("  • Compressed byte signatures predict novelty");
    println!("  • Similar compression patterns → similar coverage");
    println!("  • Can prioritize fuzzing high-novelty inputs");
    println!("  • Compression ratio correlates with code complexity");
    println!("  • Novelty = fraction of new IPs reached");
}
