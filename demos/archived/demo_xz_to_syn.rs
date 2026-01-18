// Demo: Scan xz blocks → compressed syn parses

#[path = "../../xz_to_syn_mapper.rs"] mod xz_to_syn_mapper;
#[path = "../../rand_shim.rs"] mod rand_shim;

use xz_to_syn_mapper::XzToSynMapper;
use rand_shim::init_rand;

fn main() {
    init_rand();
    
    println!("🔄 Evolving XZ → Syn Mapping\n");
    
    let rust_src = "/nix/store/x7wirg5c34zsgm7b5pvsl1hvq2dvqr9s-rust-src-1.92.0.tar.xz";
    
    println!("📦 Scanning XZ blocks...\n");
    let xz_blocks = XzToSynMapper::scan_xz_blocks(rust_src, 20);
    
    println!("Found {} XZ blocks\n", xz_blocks.len());
    
    let mut mapper = XzToSynMapper::new();
    
    println!("🧬 Mapping to Syn...\n");
    for (i, xz_block) in xz_blocks.into_iter().enumerate() {
        let xz_size = xz_block.compressed_size;
        
        if let Some(syn_block) = mapper.map_to_syn(xz_block) {
            println!("  Block {}: {} bytes → {} bytes syn (ratio: {:.3})",
                     i,
                     xz_size,
                     syn_block.syn_compressed.len(),
                     syn_block.compression_ratio);
        }
    }
    
    println!("\n🧬 Evolving mapping...");
    mapper.evolve_mapping();
    
    mapper.report();
    
    println!("\n✅ Mapping evolved!");
    println!("\n💡 Key insights:");
    println!("  • Scan XZ blocks without full decompression");
    println!("  • Parse each block with syn");
    println!("  • Compress syn AST representation");
    println!("  • Compare compression ratios");
    println!("  • Evolve mapping strategy based on patterns");
}
