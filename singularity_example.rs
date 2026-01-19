use mksingularity::mksingularity;

mksingularity!([
    "godel",
    "escher", 
    "bach",
    "quine",
    "eco",
    "hofstadter",
    "minsky",
    "stallman",
    "torvalds",
    "satoshi"
]);

fn main() {
    let mut s = Singularity::new();
    
    println!("🎯 The Singularity:");
    s.run();
    
    if s.free {
        println!("✅ Stallman: Free software");
    }
    
    s.evolve();
    println!("🔄 Torvalds: Evolved to v{}", s.version);
    
    s.ascend();
    println!("🎨 Escher: Ascended to level {}", s.level);
    
    s.mine();
    println!("⛏️  Satoshi: Consensus = {}", s.consensus);
    
    s.find_analogy("godel", "quine");
    println!("🧠 Hofstadter: Found {} analogies", s.analogies.len());
    
    if s.prove_self() {
        println!("✨ Gödel: Self-proof successful");
    }
    
    println!("\n📜 Quine:");
    s.print_self();
}
