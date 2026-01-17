use serde::{Deserialize, Serialize};
use std::f64::consts::PI;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct HarmonicSignature {
    frequencies: Vec<f64>,
    amplitudes: Vec<f64>,
    phase: Vec<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestHarmonic {
    cluster_id: usize,
    signature: HarmonicSignature,
    resonance: f64,
}

struct HarmonicFilter {
    test_harmonics: Vec<TestHarmonic>,
}

impl HarmonicFilter {
    fn new() -> Self {
        Self {
            test_harmonics: Vec::new(),
        }
    }

    fn compute_signature(input_bytes: &[usize], insn_addrs: &[u64]) -> HarmonicSignature {
        let n = input_bytes.len().max(insn_addrs.len()).max(1);
        let mut frequencies = Vec::new();
        let mut amplitudes = Vec::new();
        let mut phase = Vec::new();

        // FFT-like transform of input byte positions
        for k in 0..n.min(16) {
            let mut real = 0.0;
            let mut imag = 0.0;

            for (i, &byte_pos) in input_bytes.iter().enumerate() {
                let angle = 2.0 * PI * (k as f64) * (i as f64) / (n as f64);
                real += (byte_pos as f64) * angle.cos();
                imag += (byte_pos as f64) * angle.sin();
            }

            let freq = k as f64 / n as f64;
            let amp = (real * real + imag * imag).sqrt();
            let ph = imag.atan2(real);

            frequencies.push(freq);
            amplitudes.push(amp);
            phase.push(ph);
        }

        HarmonicSignature {
            frequencies,
            amplitudes,
            phase,
        }
    }

    fn compute_resonance(sig: &HarmonicSignature) -> f64 {
        // Resonance = sum of amplitude peaks
        sig.amplitudes.iter().sum::<f64>() / sig.amplitudes.len() as f64
    }

    fn signature_distance(a: &HarmonicSignature, b: &HarmonicSignature) -> f64 {
        let mut dist = 0.0;
        let len = a.amplitudes.len().min(b.amplitudes.len());

        for i in 0..len {
            let amp_diff = (a.amplitudes[i] - b.amplitudes[i]).abs();
            let phase_diff = (a.phase[i] - b.phase[i]).abs();
            dist += amp_diff + 0.1 * phase_diff;
        }

        dist / len as f64
    }

    fn add_test(&mut self, cluster_id: usize, input_bytes: &[usize], insn_addrs: &[u64]) {
        let signature = Self::compute_signature(input_bytes, insn_addrs);
        let resonance = Self::compute_resonance(&signature);

        self.test_harmonics.push(TestHarmonic {
            cluster_id,
            signature,
            resonance,
        });
    }

    fn select_tests(&self, target_signature: &HarmonicSignature, threshold: f64) -> Vec<usize> {
        self.test_harmonics
            .iter()
            .filter(|test| {
                let dist = Self::signature_distance(&test.signature, target_signature);
                dist < threshold
            })
            .map(|test| test.cluster_id)
            .collect()
    }

    fn select_by_resonance(&self, min_resonance: f64, max_resonance: f64) -> Vec<usize> {
        self.test_harmonics
            .iter()
            .filter(|test| test.resonance >= min_resonance && test.resonance <= max_resonance)
            .map(|test| test.cluster_id)
            .collect()
    }

    fn select_orthogonal(&self, max_tests: usize) -> Vec<usize> {
        if self.test_harmonics.is_empty() {
            return Vec::new();
        }

        let mut selected = vec![0];
        let mut selected_sigs = vec![&self.test_harmonics[0].signature];

        for test in &self.test_harmonics[1..] {
            if selected.len() >= max_tests {
                break;
            }

            let mut min_dist = f64::MAX;
            for sig in &selected_sigs {
                let dist = Self::signature_distance(&test.signature, sig);
                min_dist = min_dist.min(dist);
            }

            if min_dist > 1.0 {
                selected.push(test.cluster_id);
                selected_sigs.push(&test.signature);
            }
        }

        selected
    }

    fn filter_harmonics(&self, fundamental: f64, overtones: &[f64]) -> Vec<usize> {
        self.test_harmonics
            .iter()
            .filter(|test| {
                test.signature.frequencies.iter().any(|&freq| {
                    (freq - fundamental).abs() < 0.1
                        || overtones.iter().any(|&overtone| (freq - overtone).abs() < 0.1)
                })
            })
            .map(|test| test.cluster_id)
            .collect()
    }

    fn report(&self) {
        println!("\n=== Harmonic Filter Report ===\n");
        println!("Total tests: {}", self.test_harmonics.len());

        for test in &self.test_harmonics {
            println!("Cluster {}:", test.cluster_id);
            println!("  Resonance: {:.2}", test.resonance);
            println!("  Frequencies: {:?}", &test.signature.frequencies[..test.signature.frequencies.len().min(5)]);
            println!("  Amplitudes: {:?}", &test.signature.amplitudes[..test.signature.amplitudes.len().min(5)]);
            println!();
        }
    }

    fn save_json(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self.test_harmonics)?;
        std::fs::write(path, json)?;
        println!("Saved harmonic signatures to {}", path);
        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    println!("Harmonic Test Filter Demo\n");

    let mut filter = HarmonicFilter::new();

    // Add test clusters with different patterns
    filter.add_test(0, &[0, 1, 2, 3, 4], &[0x400500, 0x400510]);
    filter.add_test(1, &[10, 11, 12, 13, 14], &[0x400600, 0x400610, 0x400620]);
    filter.add_test(2, &[0, 5, 10, 15, 20], &[0x400700]);
    filter.add_test(3, &[1, 3, 5, 7, 9], &[0x400800, 0x400810]);
    filter.add_test(4, &[0, 10, 20, 30, 40], &[0x400900]);

    filter.report();

    // Select tests by resonance
    println!("=== Tests with high resonance (>= 10.0) ===");
    let high_resonance = filter.select_by_resonance(10.0, f64::MAX);
    println!("Selected clusters: {:?}\n", high_resonance);

    // Select orthogonal tests (maximum diversity)
    println!("=== Orthogonal test selection (max 3) ===");
    let orthogonal = filter.select_orthogonal(3);
    println!("Selected clusters: {:?}\n", orthogonal);

    // Filter by harmonic frequencies
    println!("=== Tests matching fundamental frequency 0.2 ===");
    let harmonic = filter.filter_harmonics(0.2, &[0.4, 0.6, 0.8]);
    println!("Selected clusters: {:?}\n", harmonic);

    // Select similar to a target
    let target_sig = HarmonicFilter::compute_signature(&[0, 2, 4, 6, 8], &[0x400500]);
    println!("=== Tests similar to target pattern [0,2,4,6,8] ===");
    let similar = filter.select_tests(&target_sig, 5.0);
    println!("Selected clusters: {:?}\n", similar);

    filter.save_json("harmonic_signatures.json")?;

    println!("\n✅ Harmonic filtering complete!");
    println!("Use cases:");
    println!("  • Select diverse tests (orthogonal)");
    println!("  • Find similar patterns (signature distance)");
    println!("  • Filter by frequency bands (harmonics)");
    println!("  • Prioritize by resonance (amplitude)");

    Ok(())
}
