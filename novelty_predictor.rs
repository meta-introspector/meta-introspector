// Compressed Pack Reader: Map compressed blocks → New IPs → Novelty prediction
// Predict which compressed code will reach new coverage

use flate2::read::GzDecoder;
use std::io::Read;
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
pub struct CompressedBlock {
    pub id: u64,
    pub compressed_data: Vec<u8>,
    pub decompressed_size: usize,
    pub compression_ratio: f64,
    pub byte_signature: Vec<u8>,  // First 16 bytes
}

impl CompressedBlock {
    pub fn from_source(id: u64, source: &str) -> Self {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;
        
        let source_bytes = source.as_bytes();
        let decompressed_size = source_bytes.len();
        
        // Compress
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        let _ = encoder.write_all(source_bytes);
        let compressed_data = encoder.finish().unwrap_or_default();
        
        let compression_ratio = compressed_data.len() as f64 / decompressed_size as f64;
        
        // Extract signature (first 16 bytes of compressed data)
        let byte_signature = compressed_data.iter()
            .take(16)
            .copied()
            .collect();
        
        Self {
            id,
            compressed_data,
            decompressed_size,
            compression_ratio,
            byte_signature,
        }
    }
    
    pub fn decompress(&self) -> Result<String, String> {
        let mut decoder = GzDecoder::new(&self.compressed_data[..]);
        let mut decompressed = String::new();
        decoder.read_to_string(&mut decompressed)
            .map_err(|e| e.to_string())?;
        Ok(decompressed)
    }
}

#[derive(Debug)]
pub struct NoveltyPredictor {
    pub blocks: Vec<CompressedBlock>,
    pub block_to_ips: HashMap<u64, HashSet<u64>>,  // block_id → IPs reached
    pub signature_to_novelty: HashMap<Vec<u8>, f64>,  // signature → novelty score
    pub known_ips: HashSet<u64>,
}

impl NoveltyPredictor {
    pub fn new() -> Self {
        Self {
            blocks: Vec::new(),
            block_to_ips: HashMap::new(),
            signature_to_novelty: HashMap::new(),
            known_ips: HashSet::new(),
        }
    }
    
    /// Add compressed block and its coverage
    pub fn add_block(&mut self, block: CompressedBlock, ips: HashSet<u64>) {
        // Calculate novelty: how many new IPs?
        let new_ips: HashSet<_> = ips.difference(&self.known_ips).copied().collect();
        let novelty = new_ips.len() as f64 / (ips.len() as f64 + 1.0);
        
        // Store signature → novelty mapping
        self.signature_to_novelty.insert(block.byte_signature.clone(), novelty);
        
        // Update known IPs
        self.known_ips.extend(&ips);
        
        // Store block → IPs mapping
        self.block_to_ips.insert(block.id, ips);
        self.blocks.push(block);
    }
    
    /// Predict novelty of new compressed block based on signature similarity
    pub fn predict_novelty(&self, block: &CompressedBlock) -> f64 {
        // Find similar signatures
        let mut best_similarity = 0.0;
        let mut predicted_novelty = 0.5;  // Default
        
        for (known_sig, novelty) in &self.signature_to_novelty {
            let similarity = Self::signature_similarity(&block.byte_signature, known_sig);
            
            if similarity > best_similarity {
                best_similarity = similarity;
                predicted_novelty = *novelty;
            }
        }
        
        // Adjust based on compression ratio
        let ratio_factor = (block.compression_ratio - 0.5).abs();
        predicted_novelty * (1.0 + ratio_factor)
    }
    
    fn signature_similarity(sig1: &[u8], sig2: &[u8]) -> f64 {
        let len = sig1.len().min(sig2.len());
        let mut matches = 0;
        
        for i in 0..len {
            if sig1[i] == sig2[i] {
                matches += 1;
            }
        }
        
        matches as f64 / len as f64
    }
    
    /// Find blocks that predict high novelty
    pub fn find_novel_blocks(&self, threshold: f64) -> Vec<(u64, f64)> {
        let mut novel = Vec::new();
        
        for block in &self.blocks {
            let novelty = self.predict_novelty(block);
            if novelty >= threshold {
                novel.push((block.id, novelty));
            }
        }
        
        novel.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        novel
    }
    
    pub fn report(&self) {
        println!("\n📊 Novelty Predictor Report");
        println!("  Total blocks: {}", self.blocks.len());
        println!("  Known IPs: {}", self.known_ips.len());
        println!("  Signatures learned: {}", self.signature_to_novelty.len());
        
        // Show compression ratio distribution
        let avg_ratio: f64 = self.blocks.iter()
            .map(|b| b.compression_ratio)
            .sum::<f64>() / self.blocks.len() as f64;
        
        println!("  Average compression ratio: {:.3}", avg_ratio);
        
        // Show novelty distribution
        let novelties: Vec<f64> = self.signature_to_novelty.values().copied().collect();
        if !novelties.is_empty() {
            let avg_novelty = novelties.iter().sum::<f64>() / novelties.len() as f64;
            let max_novelty = novelties.iter().fold(0.0f64, |a, &b| a.max(b));
            
            println!("  Average novelty: {:.3}", avg_novelty);
            println!("  Max novelty: {:.3}", max_novelty);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compressed_block() {
        let block = CompressedBlock::from_source(1, "fn main() {}");
        assert!(block.compressed_data.len() > 0);
        assert!(block.compression_ratio < 1.0);
    }
    
    #[test]
    fn test_novelty_predictor() {
        let mut predictor = NoveltyPredictor::new();
        let block = CompressedBlock::from_source(1, "fn test() {}");
        let mut ips = HashSet::new();
        ips.insert(0x1000);
        
        predictor.add_block(block, ips);
        assert_eq!(predictor.blocks.len(), 1);
    }
}
