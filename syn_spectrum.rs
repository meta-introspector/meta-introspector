// Syn Spectrum: AST structure defines code spectrum
// Parse with syn, serialize with serde, compress, analyze

use syn::{parse_file, File};
use serde_json;

#[derive(Debug, Clone)]
pub struct SynSpectrum {
    pub source_code: String,
    pub ast_json: String,
    pub compressed_ast: Vec<u8>,
    pub compressed_source: Vec<u8>,
    pub spectrum_signature: Vec<u8>,
}

impl SynSpectrum {
    /// Parse source code into AST spectrum
    pub fn from_source(source: String) -> Result<Self, String> {
        // Parse with syn
        let ast = parse_file(&source)
            .map_err(|e| format!("Parse error: {}", e))?;
        
        // Use quote to convert syn::File to string (syn-serde doesn't work directly)
        let ast_json = quote::quote!(#ast).to_string();
        
        // Compress both
        let compressed_ast = Self::compress(&ast_json);
        let compressed_source = Self::compress(&source);
        
        // Spectrum signature = hash of AST structure
        let spectrum_signature = Self::compute_signature(&ast_json);
        
        Ok(Self {
            source_code: source,
            ast_json,
            compressed_ast,
            compressed_source,
            spectrum_signature,
        })
    }
    
    fn compress(data: &str) -> Vec<u8> {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;
        
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        let _ = encoder.write_all(data.as_bytes());
        encoder.finish().unwrap_or_default()
    }
    
    fn compute_signature(ast_json: &str) -> Vec<u8> {
        // Extract structure: count of each AST node type
        let mut signature = Vec::new();
        
        for node_type in &["Fn", "Struct", "Impl", "Trait", "Enum", "Mod", "Use"] {
            let count = ast_json.matches(node_type).count() as u8;
            signature.push(count);
        }
        
        signature
    }
    
    /// Decompress and reconstruct
    pub fn decompress_ast(&self) -> Result<String, String> {
        use flate2::read::GzDecoder;
        use std::io::Read;
        
        let mut decoder = GzDecoder::new(&self.compressed_ast[..]);
        let mut decompressed = String::new();
        decoder.read_to_string(&mut decompressed)
            .map_err(|e| format!("Decompress error: {}", e))?;
        
        Ok(decompressed)
    }
    
    /// Compare two spectrums
    pub fn similarity(&self, other: &SynSpectrum) -> f64 {
        // Compare spectrum signatures
        let mut matches = 0;
        let total = self.spectrum_signature.len();
        
        for i in 0..total.min(other.spectrum_signature.len()) {
            let diff = (self.spectrum_signature[i] as i32 - other.spectrum_signature[i] as i32).abs();
            if diff <= 1 {
                matches += 1;
            }
        }
        
        matches as f64 / total as f64
    }
    
    /// Compression ratio: AST vs source
    pub fn ast_compression_ratio(&self) -> f64 {
        self.compressed_ast.len() as f64 / self.compressed_source.len() as f64
    }
    
    pub fn report(&self) {
        println!("📊 Syn Spectrum Analysis:");
        println!("  Source: {} bytes", self.source_code.len());
        println!("  AST JSON: {} bytes", self.ast_json.len());
        println!("  Compressed source: {} bytes", self.compressed_source.len());
        println!("  Compressed AST: {} bytes", self.compressed_ast.len());
        println!("  AST/Source ratio: {:.2}", self.ast_compression_ratio());
        println!("  Spectrum signature: {:?}", self.spectrum_signature);
    }
}

/// Spectrum analyzer: Find code patterns via AST structure
pub struct SpectrumAnalyzer {
    spectrums: Vec<SynSpectrum>,
}

impl SpectrumAnalyzer {
    pub fn new() -> Self {
        Self {
            spectrums: Vec::new(),
        }
    }
    
    /// Load source files and analyze
    pub fn load_directory(&mut self, path: &str) -> Result<(), String> {
        let entries = std::fs::read_dir(path)
            .map_err(|e| e.to_string())?;
        
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("rs") {
                if let Ok(source) = std::fs::read_to_string(&path) {
                    if let Ok(spectrum) = SynSpectrum::from_source(source) {
                        self.spectrums.push(spectrum);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Find similar code by spectrum
    pub fn find_similar(&self, target: &SynSpectrum, threshold: f64) -> Vec<(usize, f64)> {
        let mut similar = Vec::new();
        
        for (i, spectrum) in self.spectrums.iter().enumerate() {
            let similarity = target.similarity(spectrum);
            if similarity >= threshold {
                similar.push((i, similarity));
            }
        }
        
        similar.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        similar
    }
    
    /// Cluster by spectrum signature
    pub fn cluster_by_spectrum(&self) -> Vec<Vec<usize>> {
        let mut clusters: Vec<Vec<usize>> = Vec::new();
        let mut assigned = vec![false; self.spectrums.len()];
        
        for i in 0..self.spectrums.len() {
            if assigned[i] {
                continue;
            }
            
            let mut cluster = vec![i];
            assigned[i] = true;
            
            for j in i+1..self.spectrums.len() {
                if !assigned[j] && self.spectrums[i].similarity(&self.spectrums[j]) > 0.8 {
                    cluster.push(j);
                    assigned[j] = true;
                }
            }
            
            clusters.push(cluster);
        }
        
        clusters
    }
    
    pub fn report(&self) {
        println!("\n📊 Spectrum Analyzer Report");
        println!("  Total files: {}", self.spectrums.len());
        
        // Average compression ratios
        let avg_ratio: f64 = self.spectrums.iter()
            .map(|s| s.ast_compression_ratio())
            .sum::<f64>() / self.spectrums.len() as f64;
        
        println!("  Average AST/Source ratio: {:.2}", avg_ratio);
        
        // Cluster analysis
        let clusters = self.cluster_by_spectrum();
        println!("  Spectrum clusters: {}", clusters.len());
        
        for (i, cluster) in clusters.iter().take(5).enumerate() {
            println!("    Cluster {}: {} files", i + 1, cluster.len());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_syn_spectrum() {
        let source = r#"
fn main() {
    println!("Hello");
}

struct Point {
    x: i32,
    y: i32,
}
"#;
        
        let spectrum = SynSpectrum::from_source(source.to_string()).unwrap();
        assert!(spectrum.compressed_ast.len() > 0);
        assert!(spectrum.compressed_source.len() > 0);
        
        spectrum.report();
    }
}
