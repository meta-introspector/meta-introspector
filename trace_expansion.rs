// Trace expansion: 4MB xz → 30GB execution trace analysis

use std::collections::HashMap;

pub struct TraceExpansion {
    pub compressed_size: u64,      // 4 MB xz
    pub decompressed_size: u64,    // ~100 MB source
    pub trace_size: u64,           // 30 GB execution trace
    pub expansion_ratio: f64,
}

impl TraceExpansion {
    pub fn new(compressed: u64, decompressed: u64, trace: u64) -> Self {
        Self {
            compressed_size: compressed,
            decompressed_size: decompressed,
            trace_size: trace,
            expansion_ratio: trace as f64 / compressed as f64,
        }
    }
    
    pub fn report(&self) {
        println!("\n📊 Trace Expansion Analysis");
        println!("  Compressed (xz): {:.2} MB", self.compressed_size as f64 / 1_000_000.0);
        println!("  Decompressed (source): {:.2} MB", self.decompressed_size as f64 / 1_000_000.0);
        println!("  Execution trace: {:.2} GB", self.trace_size as f64 / 1_000_000_000.0);
        println!("  Expansion ratio: {:.0}x", self.expansion_ratio);
        println!("  Compression potential: {:.0}x", self.trace_size as f64 / self.compressed_size as f64);
    }
}

pub struct TraceCompressor {
    pub blocks: Vec<TraceBlock>,
    pub compression_map: HashMap<String, Vec<usize>>,
}

#[derive(Clone)]
pub struct TraceBlock {
    pub offset: u64,
    pub size: usize,
    pub signature: String,
    pub frequency: usize,
}

impl TraceCompressor {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            compression_map: HashMap::new(),
        }
    }
    
    pub fn add_block(&mut self, block: TraceBlock) {
        let sig = block.signature.clone();
        let idx = self.blocks.len();
        
        self.compression_map.entry(sig)
            .or_insert_with(Vec::new)
            .push(idx);
        
        self.blocks.push(block);
    }
    
    pub fn compress(&self) -> (u64, u64) {
        // Original trace size
        let original: u64 = self.blocks.iter().map(|b| b.size as u64).sum();
        
        // Compressed: unique signatures + references
        let unique_sigs: usize = self.compression_map.len();
        let avg_sig_size = 100;  // bytes per signature
        let ref_size = 8;  // bytes per reference
        
        let compressed = (unique_sigs * avg_sig_size) + (self.blocks.len() * ref_size);
        
        (original, compressed as u64)
    }
    
    pub fn report(&self) {
        let (original, compressed) = self.compress();
        
        println!("\n📦 Trace Compression Report");
        println!("  Total blocks: {}", self.blocks.len());
        println!("  Unique signatures: {}", self.compression_map.len());
        println!("  Original size: {:.2} GB", original as f64 / 1_000_000_000.0);
        println!("  Compressed size: {:.2} MB", compressed as f64 / 1_000_000.0);
        println!("  Compression ratio: {:.0}x", original as f64 / compressed as f64);
        
        // Find most frequent patterns
        let mut freq_vec: Vec<_> = self.compression_map.iter()
            .map(|(sig, indices)| (sig, indices.len()))
            .collect();
        freq_vec.sort_by(|a, b| b.1.cmp(&a.1));
        
        println!("\n  Top repeated patterns:");
        for (sig, count) in freq_vec.iter().take(5) {
            let display_sig = if sig.len() > 30 { &sig[..30] } else { sig };
            println!("    {} - {} occurrences", display_sig, count);
        }
    }
}

pub fn estimate_trace_size(source_size: u64, complexity: f64) -> u64 {
    // Heuristic: trace size grows with source size and complexity
    // Average: 1 MB source → 300 MB trace (300x)
    // Complexity multiplier: 0.5 to 2.0
    
    let base_expansion = 300.0;
    let complexity_factor = 0.5 + (complexity * 1.5);
    
    (source_size as f64 * base_expansion * complexity_factor) as u64
}
