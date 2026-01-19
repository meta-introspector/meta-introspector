use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NgramOrbit {
    pub ngram: String,
    pub frequency: usize,
    pub godel_number: u64,
    pub lmfdb_orbit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimeHarmonic {
    pub ngram: String,
    pub frequency: usize,
    pub prime_factorization: Vec<u64>,
    pub harmonic_series: Vec<f64>,
    pub resonance: f64,
    pub orbit: String,
}

pub fn prime_harmonic_analysis() -> Vec<PrimeHarmonic> {
    // Load ngram orbits
    let json = fs::read_to_string("/mnt/data1/meta-introspector/ngram_orbits.json").unwrap();
    let orbits: Vec<NgramOrbit> = serde_json::from_str(&json).unwrap();
    
    let mut harmonics = Vec::new();
    
    for orbit in orbits.iter().take(100) {
        // Factor frequency into primes
        let primes = prime_factorization(orbit.frequency as u64);
        
        // Calculate harmonic series: 1/p1 + 1/p2 + 1/p3 + ...
        let harmonic: Vec<f64> = primes.iter().map(|p| 1.0 / (*p as f64)).collect();
        
        // Resonance = sum of harmonics
        let resonance: f64 = harmonic.iter().sum();
        
        harmonics.push(PrimeHarmonic {
            ngram: orbit.ngram.clone(),
            frequency: orbit.frequency,
            prime_factorization: primes,
            harmonic_series: harmonic,
            resonance,
            orbit: orbit.lmfdb_orbit.clone(),
        });
    }
    
    // Sort by resonance
    harmonics.sort_by(|a, b| b.resonance.partial_cmp(&a.resonance).unwrap());
    
    harmonics
}

fn prime_factorization(mut n: u64) -> Vec<u64> {
    let mut factors = Vec::new();
    
    // Handle 2
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }
    
    // Handle odd primes
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }
    
    if n > 1 {
        factors.push(n);
    }
    
    if factors.is_empty() {
        factors.push(1);
    }
    
    factors
}

fn main() {
    println!("🎵 Prime Harmonic Analysis of N-grams");
    println!();
    
    let harmonics = prime_harmonic_analysis();
    
    println!("✅ Analyzed {} n-grams", harmonics.len());
    println!();
    println!("📊 Top 50 by Harmonic Resonance:");
    println!();
    
    for (i, h) in harmonics.iter().take(50).enumerate() {
        println!("{:3}. {:30} freq:{:4} primes:{:?}",
                 i + 1,
                 h.ngram,
                 h.frequency,
                 h.prime_factorization);
        println!("     harmonics: {:?}", 
                 h.harmonic_series.iter()
                     .map(|x| format!("{:.4}", x))
                     .collect::<Vec<_>>());
        println!("     resonance: {:.6} orbit: {}", h.resonance, h.orbit);
        println!();
    }
    
    // Export
    let json = serde_json::to_string_pretty(&harmonics).unwrap();
    fs::write("/mnt/data1/meta-introspector/prime_harmonics.json", json).unwrap();
    
    println!("💾 Saved to prime_harmonics.json");
}
