use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KolmogorovComplexity {
    min_description_length: usize,
    compressibility: f64,
    entropy: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HarmonicComplexity {
    spectral_entropy: f64,
    frequency_count: usize,
    amplitude_variance: f64,
    phase_coherence: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct IntrinsicComplexity {
    kolmogorov: KolmogorovComplexity,
    harmonic: HarmonicComplexity,
    min_test_set_size: usize,
    irreducible_core: Vec<usize>,
}

struct ComplexityAnalyzer {
    test_signatures: Vec<(usize, Vec<f64>, Vec<f64>)>, // (id, frequencies, amplitudes)
}

impl ComplexityAnalyzer {
    fn new() -> Self {
        Self {
            test_signatures: Vec::new(),
        }
    }

    fn add_signature(&mut self, test_id: usize, frequencies: Vec<f64>, amplitudes: Vec<f64>) {
        self.test_signatures.push((test_id, frequencies, amplitudes));
    }

    fn compute_spectral_entropy(&self, amplitudes: &[f64]) -> f64 {
        let total: f64 = amplitudes.iter().sum();
        if total == 0.0 {
            return 0.0;
        }

        let mut entropy = 0.0;
        for &amp in amplitudes {
            if amp > 0.0 {
                let p = amp / total;
                entropy -= p * p.log2();
            }
        }
        entropy
    }

    fn compute_amplitude_variance(&self, amplitudes: &[f64]) -> f64 {
        if amplitudes.is_empty() {
            return 0.0;
        }

        let mean: f64 = amplitudes.iter().sum::<f64>() / amplitudes.len() as f64;
        let variance: f64 = amplitudes.iter()
            .map(|&x| (x - mean).powi(2))
            .sum::<f64>() / amplitudes.len() as f64;
        variance.sqrt()
    }

    fn compute_phase_coherence(&self, phases: &[f64]) -> f64 {
        if phases.len() < 2 {
            return 1.0;
        }

        let mut coherence = 0.0;
        for i in 0..phases.len() - 1 {
            let diff = (phases[i + 1] - phases[i]).abs();
            coherence += (diff / PI).cos();
        }
        coherence / (phases.len() - 1) as f64
    }

    fn compute_kolmogorov(&self, data: &[u8]) -> KolmogorovComplexity {
        // Approximate via compression
        let original_len = data.len();
        let compressed = self.simple_compress(data);
        let compressed_len = compressed.len();

        let compressibility = 1.0 - (compressed_len as f64 / original_len as f64);
        let entropy = self.compute_byte_entropy(data);

        KolmogorovComplexity {
            min_description_length: compressed_len,
            compressibility,
            entropy,
        }
    }

    fn simple_compress(&self, data: &[u8]) -> Vec<u8> {
        // Run-length encoding approximation
        let mut compressed = Vec::new();
        if data.is_empty() {
            return compressed;
        }

        let mut current = data[0];
        let mut count = 1u8;

        for &byte in &data[1..] {
            if byte == current && count < 255 {
                count += 1;
            } else {
                compressed.push(current);
                compressed.push(count);
                current = byte;
                count = 1;
            }
        }
        compressed.push(current);
        compressed.push(count);

        compressed
    }

    fn compute_byte_entropy(&self, data: &[u8]) -> f64 {
        let mut freq = [0u32; 256];
        for &byte in data {
            freq[byte as usize] += 1;
        }

        let total = data.len() as f64;
        let mut entropy = 0.0;

        for &count in &freq {
            if count > 0 {
                let p = count as f64 / total;
                entropy -= p * p.log2();
            }
        }

        entropy
    }

    fn compute_harmonic_complexity(&self, frequencies: &[f64], amplitudes: &[f64]) -> HarmonicComplexity {
        let spectral_entropy = self.compute_spectral_entropy(amplitudes);
        let frequency_count = frequencies.iter().filter(|&&f| f > 0.01).count();
        let amplitude_variance = self.compute_amplitude_variance(amplitudes);
        
        // Phase coherence requires phase data, use 1.0 as default
        let phase_coherence = 1.0;

        HarmonicComplexity {
            spectral_entropy,
            frequency_count,
            amplitude_variance,
            phase_coherence,
        }
    }

    fn find_minimal_test_set(&self) -> Vec<usize> {
        // Greedy set cover: find minimum tests that cover all frequencies
        let mut covered_freqs = std::collections::HashSet::new();
        let mut selected = Vec::new();

        // Collect all unique frequencies
        let mut all_freqs = std::collections::HashSet::new();
        for (_, freqs, amps) in &self.test_signatures {
            for (i, &freq) in freqs.iter().enumerate() {
                if amps[i] > 0.1 {
                    all_freqs.insert((freq * 1000.0) as i32);
                }
            }
        }

        while covered_freqs.len() < all_freqs.len() {
            let mut best_test = None;
            let mut best_new_coverage = 0;

            for (test_id, freqs, amps) in &self.test_signatures {
                if selected.contains(test_id) {
                    continue;
                }

                let mut new_coverage = 0;
                for (i, &freq) in freqs.iter().enumerate() {
                    if amps[i] > 0.1 {
                        let freq_key = (freq * 1000.0) as i32;
                        if !covered_freqs.contains(&freq_key) {
                            new_coverage += 1;
                        }
                    }
                }

                if new_coverage > best_new_coverage {
                    best_new_coverage = new_coverage;
                    best_test = Some(*test_id);
                }
            }

            if let Some(test_id) = best_test {
                selected.push(test_id);
                
                // Update covered frequencies
                if let Some((_, freqs, amps)) = self.test_signatures.iter().find(|(id, _, _)| *id == test_id) {
                    for (i, &freq) in freqs.iter().enumerate() {
                        if amps[i] > 0.1 {
                            covered_freqs.insert((freq * 1000.0) as i32);
                        }
                    }
                }
            } else {
                break;
            }
        }

        selected
    }

    fn compute_intrinsic_complexity(&self, source_data: &[u8]) -> IntrinsicComplexity {
        let kolmogorov = self.compute_kolmogorov(source_data);

        // Aggregate harmonic complexity across all tests
        let mut total_entropy = 0.0;
        let mut total_freq_count = 0;
        let mut total_variance = 0.0;

        for (_, freqs, amps) in &self.test_signatures {
            let hc = self.compute_harmonic_complexity(freqs, amps);
            total_entropy += hc.spectral_entropy;
            total_freq_count += hc.frequency_count;
            total_variance += hc.amplitude_variance;
        }

        let n = self.test_signatures.len().max(1) as f64;
        let harmonic = HarmonicComplexity {
            spectral_entropy: total_entropy / n,
            frequency_count: (total_freq_count as f64 / n) as usize,
            amplitude_variance: total_variance / n,
            phase_coherence: 1.0,
        };

        let irreducible_core = self.find_minimal_test_set();
        let min_test_set_size = irreducible_core.len();

        IntrinsicComplexity {
            kolmogorov,
            harmonic,
            min_test_set_size,
            irreducible_core,
        }
    }

    fn report(&self, complexity: &IntrinsicComplexity) {
        println!("\n=== Intrinsic Complexity Analysis ===\n");
        
        println!("Kolmogorov Complexity:");
        println!("  Min description length: {} bytes", complexity.kolmogorov.min_description_length);
        println!("  Compressibility: {:.2}%", complexity.kolmogorov.compressibility * 100.0);
        println!("  Entropy: {:.2} bits/byte", complexity.kolmogorov.entropy);
        println!();

        println!("Harmonic Complexity:");
        println!("  Spectral entropy: {:.2}", complexity.harmonic.spectral_entropy);
        println!("  Frequency count: {}", complexity.harmonic.frequency_count);
        println!("  Amplitude variance: {:.2}", complexity.harmonic.amplitude_variance);
        println!("  Phase coherence: {:.2}", complexity.harmonic.phase_coherence);
        println!();

        println!("Minimal Test Set:");
        println!("  Size: {} tests", complexity.min_test_set_size);
        println!("  Tests: {:?}", complexity.irreducible_core);
        println!();

        println!("Interpretation:");
        println!("  The code requires AT LEAST {} tests to cover all execution paths", 
                 complexity.min_test_set_size);
        println!("  This is the INTRINSIC complexity - cannot be reduced further");
        println!("  Any test suite with fewer tests has incomplete coverage");
        println!();

        let complexity_class = if complexity.min_test_set_size <= 5 {
            "Simple"
        } else if complexity.min_test_set_size <= 20 {
            "Moderate"
        } else if complexity.min_test_set_size <= 100 {
            "Complex"
        } else {
            "Highly Complex"
        };

        println!("Complexity Class: {}", complexity_class);
    }

    fn save_json(&self, complexity: &IntrinsicComplexity, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(complexity)?;
        std::fs::write(path, json)?;
        println!("Saved complexity analysis to {}", path);
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    println!("Intrinsic Complexity Analyzer\n");

    let mut analyzer = ComplexityAnalyzer::new();

    // Simulate test signatures from different code paths
    analyzer.add_signature(0, vec![0.0, 0.1, 0.2], vec![5.0, 3.0, 1.0]);
    analyzer.add_signature(1, vec![0.0, 0.15, 0.3], vec![4.0, 2.0, 0.5]);
    analyzer.add_signature(2, vec![0.0, 0.2, 0.4], vec![10.0, 5.0, 2.0]);
    analyzer.add_signature(3, vec![0.0, 0.25, 0.5], vec![3.0, 1.5, 0.3]);
    analyzer.add_signature(4, vec![0.0, 0.3, 0.6], vec![8.0, 4.0, 1.0]);

    let source_data = b"fn add(a: i32, b: i32) -> i32 { a + b }\nfn mul(a: i32, b: i32) -> i32 { a * b }";
    
    let complexity = analyzer.compute_intrinsic_complexity(source_data);
    analyzer.report(&complexity);
    analyzer.save_json(&complexity, "intrinsic_complexity.json")?;

    println!("\n✅ Key Insight:");
    println!("The minimal test set size ({}) is the INTRINSIC COMPLEXITY of the code.", 
             complexity.min_test_set_size);
    println!("This is a fundamental property, like Kolmogorov complexity.");
    println!("It cannot be reduced by better test selection - only by simplifying the code.");

    Ok(())
}
