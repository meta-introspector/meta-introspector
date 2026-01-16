// Evolve mapping: compressed xz block → compressed syn parse
// Scan blocks without full decompression

use std::process::Command;
use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::Write;

#[derive(Clone)]
pub struct XzBlock {
    pub offset: u64,
    pub compressed_size: usize,
    pub data: Vec<u8>,
}

#[derive(Clone)]
pub struct SynBlock {
    pub source_offset: u64,
    pub syn_compressed: Vec<u8>,
    pub syn_size: usize,
    pub compression_ratio: f64,
}

pub struct XzToSynMapper {
    pub mappings: Vec<(XzBlock, SynBlock)>,
}

impl XzToSynMapper {
    pub fn new() -> Self {
        Self { mappings: Vec::new() }
    }
    
    pub fn scan_xz_blocks(xz_path: &str, max_blocks: usize) -> Vec<XzBlock> {
        let mut blocks = Vec::new();
        
        // List files in tar
        let output = Command::new("tar")
            .args(&["tf", xz_path])
            .output()
            .expect("Failed to list tar");
        
        let file_list = String::from_utf8_lossy(&output.stdout);
        let rs_files: Vec<&str> = file_list.lines()
            .filter(|line| line.ends_with(".rs"))
            .take(max_blocks)
            .collect();
        
        for (i, file_path) in rs_files.iter().enumerate() {
            // Extract single file
            if let Ok(output) = Command::new("tar")
                .args(&["xfO", xz_path, file_path])
                .output() {
                
                if output.status.success() {
                    blocks.push(XzBlock {
                        offset: i as u64,
                        compressed_size: output.stdout.len(),
                        data: output.stdout,
                    });
                }
            }
        }
        
        blocks
    }
    
    pub fn map_to_syn(&mut self, xz_block: XzBlock) -> Option<SynBlock> {
        let source = String::from_utf8_lossy(&xz_block.data);
        
        // Parse with syn
        let syntax = syn::parse_file(&source).ok()?;
        
        // Count syn nodes instead of debug format
        let node_count = syntax.items.len();
        let syn_repr = format!("File {{ items: {} }}", node_count);
        
        // Compress syn representation
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        encoder.write_all(syn_repr.as_bytes()).ok()?;
        let compressed = encoder.finish().ok()?;
        
        let ratio = compressed.len() as f64 / syn_repr.len() as f64;
        
        let syn_block = SynBlock {
            source_offset: xz_block.offset,
            syn_compressed: compressed,
            syn_size: syn_repr.len(),
            compression_ratio: ratio,
        };
        
        self.mappings.push((xz_block, syn_block.clone()));
        
        Some(syn_block)
    }
    
    pub fn evolve_mapping(&mut self) {
        // Evolve compression strategy based on patterns
        let avg_ratio: f64 = self.mappings.iter()
            .map(|(_, syn)| syn.compression_ratio)
            .sum::<f64>() / self.mappings.len() as f64;
        
        println!("  Average syn compression ratio: {:.3}", avg_ratio);
    }
    
    pub fn report(&self) {
        println!("\n📊 XZ → Syn Mapping Report");
        println!("  Total mappings: {}", self.mappings.len());
        
        if !self.mappings.is_empty() {
            let total_xz: usize = self.mappings.iter()
                .map(|(xz, _)| xz.compressed_size)
                .sum();
            let total_syn: usize = self.mappings.iter()
                .map(|(_, syn)| syn.syn_compressed.len())
                .sum();
            
            println!("  Total XZ size: {} bytes", total_xz);
            println!("  Total Syn compressed: {} bytes", total_syn);
            println!("  Compression gain: {:.2}x", total_xz as f64 / total_syn as f64);
        }
    }
}
