// perf-complexity: Auto-label instruction data via GNU Mes bootstrap layers
use std::collections::HashMap;
use std::path::Path;
use serde::{Serialize, Deserialize};

/// Complexity label learned from basis system A labeling target system B
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplexityLabel {
    /// Layer in basis system A
    pub layer: usize,
    
    /// Entropy of instruction pattern
    pub entropy: f64,
    
    /// Frequency in target system B
    pub frequency: usize,
    
    /// Galois field size (2^n) for this layer
    pub gf_size: usize,
    
    /// Orthogonal projection strength (how well A labels B)
    pub projection: f64,
}

/// Instruction pattern with auto-learned label
#[derive(Debug, Serialize, Deserialize)]
pub struct LabeledPattern {
    pub ip: u64,
    pub frequency: usize,
    pub label: ComplexityLabel,
    pub layer: usize,
}

/// Perf complexity analyzer: System A labels System B orthogonally
pub struct PerfComplexity {
    /// System A: Basis system instruction frequencies
    basis_freq: HashMap<u64, usize>,
    
    /// System B: Target system instruction frequencies
    target_freq: HashMap<u64, usize>,
    
    /// Learned orthogonal labels (A → B projection)
    labels: HashMap<u64, ComplexityLabel>,
    
    /// Basis system layers (from nix store)
    basis_layers: Vec<BasisLayer>,
}

/// A layer in the basis system A
#[derive(Debug, Clone)]
pub struct BasisLayer {
    pub name: String,
    pub ips: Vec<u64>,
    pub gf_size: usize,  // GF(2^n)
}

impl PerfComplexity {
    pub fn new() -> Self {
        Self {
            basis_freq: HashMap::new(),
            target_freq: HashMap::new(),
            labels: HashMap::new(),
            basis_layers: Vec::new(),
        }
    }
    
    /// Load basis system A (e.g., GNU Mes, or any other system)
    pub fn load_basis_system(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Loading basis system A: {}", path.display());
        
        let file = std::fs::File::open(path)?;
        let mmap = unsafe { memmap2::Mmap::map(&file)? };
        let perf_file = linux_perf_data::PerfFileReader::parse_file(&mmap)?;
        
        for record in perf_file.records() {
            if let Ok(linux_perf_data::PerfFileRecord::Sample(sample)) = record {
                if let Some(ip) = sample.ip {
                    *self.basis_freq.entry(ip).or_insert(0) += 1;
                }
            }
        }
        
        println!("   Basis IPs: {}", self.basis_freq.len());
        Ok(())
    }
    
    /// Load target system B to be labeled
    pub fn load_target_system(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("📊 Loading target system B: {}", path.display());
        
        let file = std::fs::File::open(path)?;
        let mmap = unsafe { memmap2::Mmap::map(&file)? };
        let perf_file = linux_perf_data::PerfFileReader::parse_file(&mmap)?;
        
        for record in perf_file.records() {
            if let Ok(linux_perf_data::PerfFileRecord::Sample(sample)) = record {
                if let Some(ip) = sample.ip {
                    *self.target_freq.entry(ip).or_insert(0) += 1;
                }
            }
        }
        
        println!("   Target IPs: {}", self.target_freq.len());
        Ok(())
    }
    
    /// Learn basis layers from meta-perf convergence
    pub fn learn_basis_layers(&mut self, meta_perf_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        println!("🧙 Learning basis layers from: {}", meta_perf_path.display());
        
        let convergence = meta_perf_path.join("convergence");
        
        // Discover layers automatically
        let mut layer_num = 0;
        loop {
            let layer_file = convergence.join(format!("level{}.ips", layer_num));
            if !layer_file.exists() {
                break;
            }
            
            let ips_str = std::fs::read_to_string(&layer_file)?;
            let ips: Vec<u64> = ips_str.lines()
                .filter_map(|line| u64::from_str_radix(line.trim(), 16).ok())
                .collect();
            
            // GF size grows with layer: 2^(10+layer)
            let gf_size = 1 << (10 + layer_num);
            
            self.basis_layers.push(BasisLayer {
                name: format!("layer{}", layer_num),
                ips: ips.clone(),
                gf_size,
            });
            
            println!("   Layer {}: {} IPs, GF(2^{})", 
                layer_num, ips.len(), 10 + layer_num);
            
            layer_num += 1;
        }
        
        Ok(())
    }
    
    /// Compute orthogonal projection: how well basis A labels target B
    pub fn compute_orthogonal_labels(&mut self) {
        println!("🔬 Computing orthogonal projection A → B...");
        
        for (&target_ip, &target_freq) in &self.target_freq {
            // Find which basis layer best explains this target IP
            let mut best_layer = 0;
            let mut best_projection = 0.0;
            
            for (layer_idx, layer) in self.basis_layers.iter().enumerate() {
                // Projection strength = overlap with basis layer
                let projection = if layer.ips.contains(&target_ip) {
                    1.0
                } else {
                    // Compute distance to nearest basis IP
                    let min_dist = layer.ips.iter()
                        .map(|&basis_ip| (target_ip as i64 - basis_ip as i64).abs())
                        .min()
                        .unwrap_or(i64::MAX);
                    
                    1.0 / (1.0 + min_dist as f64 / 1000.0)
                };
                
                if projection > best_projection {
                    best_projection = projection;
                    best_layer = layer_idx;
                }
            }
            
            let entropy = self.compute_entropy(target_ip, target_freq);
            let gf_size = self.basis_layers.get(best_layer)
                .map(|l| l.gf_size)
                .unwrap_or(1024);
            
            self.labels.insert(target_ip, ComplexityLabel {
                layer: best_layer,
                entropy,
                frequency: target_freq,
                gf_size,
                projection: best_projection,
            });
        }
        
        println!("   Labeled {} target IPs using {} basis layers", 
            self.labels.len(), self.basis_layers.len());
    }
    
    /// Compute Shannon entropy for instruction pattern
    fn compute_entropy(&self, ip: u64, freq: usize) -> f64 {
        if freq == 0 {
            return 0.0;
        }
        
        let total: usize = self.target_freq.values().sum();
        let p = freq as f64 / total as f64;
        
        -p * p.log2()
    }
    
    /// Analyze complexity and output labeled patterns
    pub fn analyze(&self) -> Vec<LabeledPattern> {
        println!("🔬 Analyzing orthogonal complexity...");
        
        let mut patterns: Vec<_> = self.labels.iter()
            .map(|(&ip, label)| {
                LabeledPattern {
                    ip,
                    frequency: label.frequency,
                    label: label.clone(),
                    layer: label.layer,
                }
            })
            .collect();
        
        patterns.sort_by(|a, b| {
            b.label.projection.partial_cmp(&a.label.projection)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        
        println!("   Labeled {} patterns", patterns.len());
        println!("   Orthogonal projection quality: {:.2}%", 
            patterns.iter().map(|p| p.label.projection).sum::<f64>() / patterns.len() as f64 * 100.0);
        
        patterns
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_orthogonal_labeling() {
        let mut pc = PerfComplexity::new();
        
        // Basis system A
        pc.basis_freq.insert(0x400000, 100);
        pc.basis_layers.push(BasisLayer {
            name: "layer0".to_string(),
            ips: vec![0x400000],
            gf_size: 1024,
        });
        
        // Target system B
        pc.target_freq.insert(0x400010, 50);
        
        pc.compute_orthogonal_labels();
        
        assert!(pc.labels.contains_key(&0x400010));
        assert!(pc.labels[&0x400010].projection > 0.0);
    }
}
