// Demo: Fuzz rustc with syn-guided mutations

mod rustc_fuzzer;
mod rand_shim;

use rustc_fuzzer::RustcFuzzer;
use rand_shim::init_rand;

fn main() {
    init_rand();
    
    println!("🔍 Rustc Coverage Fuzzer\n");
    println!("Syn spectrum → Rustc perf spectrum → Coverage-guided fuzzing\n");
    
    let mut fuzzer = RustcFuzzer::new();
    
    // Seed corpus with interesting inputs
    println!("📝 Seeding corpus...\n");
    
    let seeds = vec![
        // Simple function
        "fn main() { }",
        
        // Struct
        "struct Point { x: i32, y: i32 }",
        
        // Impl
        r#"
struct Point { x: i32 }
impl Point {
    fn new(x: i32) -> Self { Point { x } }
}
"#,
        
        // Trait
        r#"
trait Shape {
    fn area(&self) -> f64;
}
"#,
        
        // Enum
        r#"
enum Color {
    Red,
    Green,
    Blue,
}
"#,
        
        // Generic
        r#"
fn identity<T>(x: T) -> T { x }
"#,
        
        // Complex
        r#"
struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        Container { value }
    }
    
    fn get(&self) -> &T {
        &self.value
    }
}

trait Processor<T> {
    fn process(&self, input: T) -> T;
}
"#,
    ];
    
    for seed in seeds {
        if let Err(e) = fuzzer.add_seed(seed.to_string()) {
            println!("  Warning: {}", e);
        }
    }
    
    println!("\n  Initial corpus: {} inputs", fuzzer.corpus.len());
    println!("  Initial coverage: {} IPs\n", fuzzer.total_coverage.len());
    
    // Fuzz for coverage
    fuzzer.fuzz(50);
    
    // Report
    fuzzer.report();
    
    println!("\n✅ Fuzzing complete!");
    println!("\n💡 Key insights:");
    println!("  • Syn AST guides mutation strategy");
    println!("  • Rustc compilation traces coverage");
    println!("  • New coverage = interesting input");
    println!("  • Corpus grows with unique behaviors");
    println!("  • Spectrum mapping: Syn nodes → Rustc IPs");
}
