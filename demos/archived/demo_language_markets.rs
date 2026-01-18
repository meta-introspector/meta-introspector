// Demo: Languages as market makers buying different spectrums

mod language_market_makers;
#[path = "../../rand_shim.rs"] mod rand_shim;

use language_market_makers::SpectrumMarketplace;
use rand_shim::init_rand;

fn main() {
    init_rand();
    
    println!("🌈 Language Market Makers\n");
    println!("Each language is a meme that buys a different spectrum\n");
    
    let mut marketplace = SpectrumMarketplace::new();
    
    // List various code spectrums
    println!("📝 Listing code spectrums for sale...\n");
    
    // Rust-style code (complex)
    marketplace.list_spectrum(r#"
fn calculate(x: i32, y: i32) -> i32 {
    x * y + 42
}

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn distance(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

trait Shape {
    fn area(&self) -> f64;
}
"#.to_string());
    
    // Simple procedural (C-style)
    marketplace.list_spectrum(r#"
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
"#.to_string());
    
    // Very simple (Brainfuck-style)
    marketplace.list_spectrum(r#"
fn increment(x: i32) -> i32 {
    x + 1
}
"#.to_string());
    
    // Functional style (Haskell-like)
    marketplace.list_spectrum(r#"
fn map<T, U>(vec: Vec<T>, f: fn(T) -> U) -> Vec<U> {
    vec.into_iter().map(f).collect()
}

fn filter<T>(vec: Vec<T>, pred: fn(&T) -> bool) -> Vec<T> {
    vec.into_iter().filter(pred).collect()
}

trait Functor<T> {
    fn fmap<U>(self, f: fn(T) -> U) -> Self;
}
"#.to_string());
    
    // Object-oriented (Python-style)
    marketplace.list_spectrum(r#"
struct Animal {
    name: String,
    age: i32,
}

impl Animal {
    fn new(name: String, age: i32) -> Self {
        Animal { name, age }
    }
    
    fn speak(&self) {
        println!("{} says hello", self.name);
    }
}
"#.to_string());
    
    println!("  Listed {} code spectrums\n", marketplace.spectrums.len());
    
    // Show spectrum details
    for spectrum in &marketplace.spectrums {
        println!("  Spectrum {}: complexity={}, nodes={:?}",
                 spectrum.id,
                 spectrum.complexity,
                 spectrum.node_counts.iter()
                     .filter(|(_, &count)| count > 0)
                     .map(|(name, count)| format!("{}:{}", name, count))
                     .collect::<Vec<_>>()
                     .join(", "));
    }
    
    // Run auctions
    println!("\n💰 Running auctions...");
    for round in 0..3 {
        marketplace.run_auction(round);
    }
    
    // Final report
    marketplace.report();
    
    println!("\n✅ Market complete!");
    println!("\n💡 Key insights:");
    println!("  • Rust buys complex AST spectrums (structs, traits, impls)");
    println!("  • Brainfuck buys minimal spectrums (simple functions)");
    println!("  • C buys procedural spectrums (functions, loops)");
    println!("  • Python buys OOP spectrums (classes, methods)");
    println!("  • Haskell buys functional spectrums (higher-order, traits)");
    println!("  • Each language is a market maker for its spectrum");
    println!("  • Code complexity determines which language buys it");
}
