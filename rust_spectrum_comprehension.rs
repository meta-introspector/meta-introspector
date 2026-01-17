// Use processed Rust stdlib spectrum to comprehend other code
// Map unknown code → known stdlib patterns

use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct SpectrumPattern {
    pub ips: HashSet<u64>,
    pub source_file: String,
    pub compression_ratio: f64,
}

pub struct RustSpectrum {
    pub patterns: Vec<SpectrumPattern>,
    pub ip_to_patterns: HashMap<u64, Vec<usize>>,
    pub total_ips: HashSet<u64>,
}

impl RustSpectrum {
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            ip_to_patterns: HashMap::new(),
            total_ips: HashSet::new(),
        }
    }
    
    pub fn add_pattern(&mut self, source_file: String, ips: HashSet<u64>, compression_ratio: f64) {
        let pattern_idx = self.patterns.len();
        
        for ip in &ips {
            self.ip_to_patterns.entry(*ip)
                .or_default()
                .push(pattern_idx);
            self.total_ips.insert(*ip);
        }
        
        self.patterns.push(SpectrumPattern {
            ips,
            source_file,
            compression_ratio,
        });
    }
    
    pub fn comprehend(&self, unknown_ips: &HashSet<u64>) -> Comprehension {
        let mut matched_patterns: HashMap<usize, usize> = HashMap::new();
        let mut known_ips = HashSet::new();
        
        for ip in unknown_ips {
            if let Some(pattern_indices) = self.ip_to_patterns.get(ip) {
                known_ips.insert(*ip);
                for &idx in pattern_indices {
                    *matched_patterns.entry(idx).or_insert(0) += 1;
                }
            }
        }
        
        let unknown_ips_set: HashSet<u64> = unknown_ips.difference(&known_ips).copied().collect();
        
        // Find best matching patterns
        let mut matches: Vec<(usize, usize, f64)> = matched_patterns.iter()
            .map(|(&idx, &count)| {
                let pattern = &self.patterns[idx];
                let similarity = count as f64 / pattern.ips.len() as f64;
                (idx, count, similarity)
            })
            .collect();
        
        matches.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        
        Comprehension {
            total_ips: unknown_ips.len(),
            known_ips: known_ips.len(),
            unknown_ips: unknown_ips_set.len(),
            coverage: known_ips.len() as f64 / unknown_ips.len() as f64,
            best_matches: matches.into_iter().take(5).collect(),
        }
    }
    
    pub fn report(&self) {
        println!("\n📊 Rust Spectrum Report");
        println!("  Total patterns: {}", self.patterns.len());
        println!("  Total unique IPs: {}", self.total_ips.len());
        println!("  Average IPs per pattern: {:.1}", 
                 self.total_ips.len() as f64 / self.patterns.len() as f64);
    }
}

pub struct Comprehension {
    pub total_ips: usize,
    pub known_ips: usize,
    pub unknown_ips: usize,
    pub coverage: f64,
    pub best_matches: Vec<(usize, usize, f64)>,
}

impl Comprehension {
    pub fn report(&self, spectrum: &RustSpectrum) {
        println!("\n🔍 Comprehension Report");
        println!("  Total IPs: {}", self.total_ips);
        println!("  Known IPs: {} ({:.1}%)", self.known_ips, self.coverage * 100.0);
        println!("  Unknown IPs: {} ({:.1}%)", self.unknown_ips, (1.0 - self.coverage) * 100.0);
        
        if !self.best_matches.is_empty() {
            println!("\n  📚 Best matching stdlib patterns:");
            for (idx, count, similarity) in &self.best_matches {
                let pattern = &spectrum.patterns[*idx];
                println!("    {} - {} IPs matched ({:.1}% similarity)",
                         pattern.source_file, count, similarity * 100.0);
            }
        }
    }
}
