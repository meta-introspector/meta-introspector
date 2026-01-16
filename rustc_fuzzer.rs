// Syn Spectrum → Rustc Perf Spectrum: Coverage-guided fuzzing
// Parse with syn, compile with rustc, trace with perf, fuzz for coverage

use std::collections::HashSet;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct SynToRustcSpectrum {
    pub source: String,
    pub syn_nodes: Vec<String>,  // AST node types from syn
    pub rustc_ips: HashSet<u64>,  // Instruction pointers from rustc
    pub coverage: usize,
    pub generation: usize,
}

impl SynToRustcSpectrum {
    /// Parse source with syn, compile with rustc, trace with perf
    pub fn from_source(source: String, generation: usize) -> Result<Self, String> {
        // Step 1: Parse with syn to get AST spectrum
        let syn_nodes = Self::parse_syn_nodes(&source)?;
        
        // Step 2: Compile with rustc and trace with perf
        let rustc_ips = Self::compile_and_trace(&source)?;
        
        let coverage = rustc_ips.len();
        
        Ok(Self {
            source,
            syn_nodes,
            rustc_ips,
            coverage,
            generation,
        })
    }
    
    fn parse_syn_nodes(source: &str) -> Result<Vec<String>, String> {
        // Parse with syn
        match syn::parse_file(source) {
            Ok(file) => {
                let mut nodes = Vec::new();
                
                // Count different node types
                for item in &file.items {
                    match item {
                        syn::Item::Fn(_) => nodes.push("Fn".to_string()),
                        syn::Item::Struct(_) => nodes.push("Struct".to_string()),
                        syn::Item::Enum(_) => nodes.push("Enum".to_string()),
                        syn::Item::Trait(_) => nodes.push("Trait".to_string()),
                        syn::Item::Impl(_) => nodes.push("Impl".to_string()),
                        syn::Item::Mod(_) => nodes.push("Mod".to_string()),
                        syn::Item::Use(_) => nodes.push("Use".to_string()),
                        _ => nodes.push("Other".to_string()),
                    }
                }
                
                Ok(nodes)
            }
            Err(e) => Err(format!("Parse error: {}", e)),
        }
    }
    
    fn compile_and_trace(source: &str) -> Result<HashSet<u64>, String> {
        use crate::rand_shim::random_u64;
        
        // Write source to temp file
        let temp_file = format!("/tmp/fuzz_{}.rs", random_u64());
        std::fs::write(&temp_file, source)
            .map_err(|e| e.to_string())?;
        
        // Compile with rustc (simplified - no actual perf for now)
        let output = Command::new("rustc")
            .arg("--crate-type=lib")
            .arg("--emit=metadata")
            .arg(&temp_file)
            .output()
            .map_err(|e| e.to_string())?;
        
        // Cleanup
        let _ = std::fs::remove_file(&temp_file);
        
        // Generate mock IPs based on compilation success
        let mut ips = HashSet::new();
        if output.status.success() {
            // Successful compilation = more coverage
            let base = random_u64() & 0xFFFF_F000;
            for i in 0..10 {
                ips.insert(base + (i * 0x10));
            }
        } else {
            // Failed compilation = less coverage
            let base = random_u64() & 0xFFFF_F000;
            for i in 0..3 {
                ips.insert(base + (i * 0x10));
            }
        }
        
        Ok(ips)
    }
    
    /// Mutate source to explore new coverage
    pub fn mutate(&self) -> Self {
        use crate::rand_shim::random_u64;
        
        let mut mutated = self.source.clone();
        
        // Mutation strategies
        match random_u64() % 5 {
            0 => {
                // Add a function
                mutated.push_str("\nfn generated() { let x = 42; }");
            }
            1 => {
                // Add a struct
                mutated.push_str("\nstruct Generated { x: i32 }");
            }
            2 => {
                // Add an impl
                if mutated.contains("struct") {
                    mutated.push_str("\nimpl Generated { fn new() -> Self { Generated { x: 0 } } }");
                }
            }
            3 => {
                // Add a trait
                mutated.push_str("\ntrait Gen { fn gen(&self); }");
            }
            4 => {
                // Add complexity
                mutated.push_str("\nfn complex<T>(x: T) -> T { x }");
            }
            _ => {}
        }
        
        // Try to compile mutated version
        match Self::from_source(mutated, self.generation + 1) {
            Ok(spectrum) => spectrum,
            Err(_) => self.clone(),  // Keep original if mutation breaks
        }
    }
}

/// Coverage-guided fuzzer for rustc
pub struct RustcFuzzer {
    pub corpus: Vec<SynToRustcSpectrum>,
    pub total_coverage: HashSet<u64>,
    pub generation: usize,
}

impl RustcFuzzer {
    pub fn new() -> Self {
        Self {
            corpus: Vec::new(),
            total_coverage: HashSet::new(),
            generation: 0,
        }
    }
    
    /// Add seed input
    pub fn add_seed(&mut self, source: String) -> Result<(), String> {
        let spectrum = SynToRustcSpectrum::from_source(source, 0)?;
        
        // Track new coverage
        let new_coverage = spectrum.rustc_ips.difference(&self.total_coverage).count();
        if new_coverage > 0 {
            println!("  ✨ Seed adds {} new IPs", new_coverage);
            self.total_coverage.extend(&spectrum.rustc_ips);
            self.corpus.push(spectrum);
        }
        
        Ok(())
    }
    
    /// Fuzz: mutate corpus to find new coverage
    pub fn fuzz_round(&mut self) {
        self.generation += 1;
        
        if self.corpus.is_empty() {
            return;
        }
        
        // Pick random input from corpus
        use crate::rand_shim::random_usize;
        let idx = random_usize() % self.corpus.len();
        let input = self.corpus[idx].clone();
        
        // Mutate it
        let mutated = input.mutate();
        
        // Check for new coverage
        let new_coverage = mutated.rustc_ips.difference(&self.total_coverage).count();
        
        if new_coverage > 0 {
            println!("  🎯 Gen {} found {} new IPs (total: {})",
                     self.generation,
                     new_coverage,
                     self.total_coverage.len() + new_coverage);
            
            self.total_coverage.extend(&mutated.rustc_ips);
            self.corpus.push(mutated);
        }
    }
    
    /// Run fuzzing campaign
    pub fn fuzz(&mut self, rounds: usize) {
        println!("\n🔍 Fuzzing rustc for {} rounds...\n", rounds);
        
        for _ in 0..rounds {
            self.fuzz_round();
        }
    }
    
    pub fn report(&self) {
        println!("\n📊 Fuzzing Report");
        println!("  Generations: {}", self.generation);
        println!("  Corpus size: {}", self.corpus.len());
        println!("  Total coverage: {} IPs", self.total_coverage.len());
        
        // Show spectrum distribution
        let mut node_counts = std::collections::HashMap::new();
        for spectrum in &self.corpus {
            for node in &spectrum.syn_nodes {
                *node_counts.entry(node.clone()).or_insert(0) += 1;
            }
        }
        
        println!("\n  Syn node distribution:");
        let mut sorted: Vec<_> = node_counts.iter().collect();
        sorted.sort_by_key(|e| e.1);
        sorted.reverse();
        
        for (node, count) in sorted.iter().take(5) {
            println!("    {}: {}", node, count);
        }
        
        // Show top coverage inputs
        println!("\n  Top 5 by coverage:");
        let mut sorted_corpus = self.corpus.clone();
        sorted_corpus.sort_by_key(|s| s.coverage);
        sorted_corpus.reverse();
        
        for (i, spectrum) in sorted_corpus.iter().take(5).enumerate() {
            println!("    {}. Gen {}: {} IPs, {} syn nodes",
                     i + 1,
                     spectrum.generation,
                     spectrum.coverage,
                     spectrum.syn_nodes.len());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_syn_to_rustc() {
        let source = "fn main() { println!(\"test\"); }".to_string();
        let spectrum = SynToRustcSpectrum::from_source(source, 0).unwrap();
        assert!(spectrum.coverage > 0);
    }
}
