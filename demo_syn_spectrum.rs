// Demo: Analyze code via syn AST spectrum

mod syn_spectrum;
mod rand_shim;

use syn_spectrum::{SynSpectrum, SpectrumAnalyzer};
use rand_shim::init_rand;

fn main() {
    init_rand();
    
    println!("🌈 Syn Spectrum Analysis\n");
    println!("Parse code → AST → JSON → Compress → Spectrum signature\n");
    
    // Example 1: Simple function
    let simple_code = r#"
fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;
    
    println!("📝 Example 1: Simple function");
    if let Ok(spectrum1) = SynSpectrum::from_source(simple_code.to_string()) {
        spectrum1.report();
    }
    
    // Example 2: Struct with impl
    let struct_code = r#"
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
    
    fn distance(&self) -> f64 {
        ((self.x * self.x + self.y * self.y) as f64).sqrt()
    }
}
"#;
    
    println!("\n📝 Example 2: Struct with impl");
    if let Ok(spectrum2) = SynSpectrum::from_source(struct_code.to_string()) {
        spectrum2.report();
    }
    
    // Example 3: Trait and enum
    let trait_code = r#"
trait Shape {
    fn area(&self) -> f64;
}

enum Color {
    Red,
    Green,
    Blue,
}

struct Circle {
    radius: f64,
    color: Color,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14159 * self.radius * self.radius
    }
}
"#;
    
    println!("\n📝 Example 3: Trait and enum");
    if let Ok(spectrum3) = SynSpectrum::from_source(trait_code.to_string()) {
        spectrum3.report();
    }
    
    // Compare spectrums
    println!("\n🔍 Spectrum Comparison:");
    if let (Ok(s1), Ok(s2), Ok(s3)) = (
        SynSpectrum::from_source(simple_code.to_string()),
        SynSpectrum::from_source(struct_code.to_string()),
        SynSpectrum::from_source(trait_code.to_string()),
    ) {
        println!("  Simple vs Struct: {:.2}% similar", s1.similarity(&s2) * 100.0);
        println!("  Simple vs Trait: {:.2}% similar", s1.similarity(&s3) * 100.0);
        println!("  Struct vs Trait: {:.2}% similar", s2.similarity(&s3) * 100.0);
    }
    
    // Analyze current directory
    println!("\n📂 Analyzing current directory...");
    let mut analyzer = SpectrumAnalyzer::new();
    if let Err(e) = analyzer.load_directory(".") {
        println!("  Warning: {}", e);
    } else {
        analyzer.report();
    }
    
    println!("\n✅ Analysis complete!");
    println!("\n💡 Key insights:");
    println!("  • AST structure defines code spectrum");
    println!("  • Spectrum signature = node type counts");
    println!("  • Similar spectrums = similar code structure");
    println!("  • Compressed AST reveals code complexity");
    println!("  • AST/Source ratio shows structural density");
}
