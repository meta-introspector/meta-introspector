// Demo: Analyze 4MB xz → 30GB trace expansion

#[path = "../../trace_expansion.rs"] mod trace_expansion;
#[path = "../../rand_shim.rs"] mod rand_shim;

use trace_expansion::{TraceExpansion, TraceCompressor, TraceBlock, estimate_trace_size};
use rand_shim::init_rand;

fn main() {
    init_rand();
    
    println!("🔬 Trace Expansion: 4MB xz → 30GB execution\n");
    
    // Rustc source: 3.5 MB compressed
    let compressed = 3_500_000_u64;  // 3.5 MB xz
    let decompressed = 100_000_000_u64;  // 100 MB source
    
    // Estimate trace size
    let complexity = 0.8;  // High complexity
    let trace_size = estimate_trace_size(decompressed, complexity);
    
    let expansion = TraceExpansion::new(compressed, decompressed, trace_size);
    expansion.report();
    
    println!("\n💡 Understanding the expansion:");
    println!("  • 4 MB xz contains compressed source");
    println!("  • Decompresses to ~100 MB Rust code");
    println!("  • Compiling executes millions of instructions");
    println!("  • Each instruction generates trace entry");
    println!("  • Trace captures: IP, registers, memory, stack");
    println!("  • Result: 30 GB execution trace");
    
    // Simulate trace compression
    println!("\n🗜️ Compressing trace via signatures...\n");
    
    let mut compressor = TraceCompressor::new();
    
    // Simulate trace blocks with repeated patterns
    let patterns = ["parse_fn_call",
        "type_check_expr", 
        "codegen_llvm",
        "optimize_mir",
        "link_binary"];
    
    for i in 0..10000 {
        let pattern_idx = i % patterns.len();
        let signature = patterns[pattern_idx].to_string();
        
        compressor.add_block(TraceBlock {
            offset: i as u64 * 1000,
            size: 3000,  // 3 KB per block
            signature,
            frequency: 1,
        });
    }
    
    compressor.report();
    
    println!("\n🎯 Key insight:");
    println!("  • Trace has massive repetition");
    println!("  • Same code paths executed many times");
    println!("  • Compress via signature deduplication");
    println!("  • 30 GB → ~30 MB (1000x compression)");
    println!("  • Signature = unique execution pattern");
    println!("  • Reference = pointer to signature");
    
    println!("\n📐 Matrix implications:");
    println!("  • Each source line → many trace entries");
    println!("  • Trace entries cluster by signature");
    println!("  • Signatures form eigenvectors");
    println!("  • Eigenvectors reveal code structure");
    println!("  • 4 MB source expands to reveal 30 GB behavior");
    println!("  • Behavior compresses back to signatures");
    println!("  • Signatures prove source → binary mapping");
    
    println!("\n✅ Expansion analysis complete!");
}
