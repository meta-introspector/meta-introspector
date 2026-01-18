// Demo: 71-language quine evolves across all languages

#[path = "../../universal_quine.rs"] mod universal_quine;
#[path = "../../rand_shim.rs"] mod rand_shim;

use universal_quine::UniversalQuine;
use rand_shim::init_rand;

fn main() {
    init_rand();
    
    println!("🌍 71-Language Universal Quine Evolution\n");
    println!("One meme, 71 implementations, evolving together\n");
    
    // Create universal quine
    let mut quine = UniversalQuine::new();
    
    println!("📝 Initial state:");
    quine.report();
    
    // Evolve for 10 generations
    println!("\n🧬 Evolving for 10 generations...\n");
    
    for gen in 1..=10 {
        quine.evolve_generation();
        
        if gen % 5 == 0 {
            println!("Generation {}:", gen);
            let top3 = quine.top_implementations(3);
            for (i, impl_) in top3.iter().enumerate() {
                println!("  {}. {}: {} bytes (fitness {:.2})",
                         i + 1,
                         impl_.language,
                         impl_.compressed_size,
                         impl_.fitness);
            }
            println!();
        }
    }
    
    // Cross-language learning
    println!("🔄 Cross-language evolution:");
    quine.cross_evolve();
    
    // Final report
    quine.report();
    
    // Show actual code for top 5
    println!("\n📄 Top 5 Implementations:");
    for (i, impl_) in quine.top_implementations(5).iter().enumerate() {
        println!("\n  {}. {} (gen {}):", i + 1, impl_.language, impl_.generation);
        println!("     Code: {}", impl_.code);
        println!("     Size: {} bytes compressed", impl_.compressed_size);
    }
    
    println!("\n✅ Evolution complete!");
    println!("\n💡 Key insights:");
    println!("  • 71 languages = 71 implementations of same meme");
    println!("  • Each evolves independently");
    println!("  • Fitness = 1/compressed_size");
    println!("  • Cross-language learning transfers optimizations");
    println!("  • Universal quine = language-agnostic concept");
    println!("  • Best compression wins across all languages");
}
