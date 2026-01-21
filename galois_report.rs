// Galois Field Analysis Report Generator for all 71 languages
use linux_perf_data::{PerfFileReader, PerfFileRecord};
use std::collections::HashMap;
use std::fs::{File, read_dir};
use std::io::BufReader;
use std::path::Path;
use anyhow::Result;

fn find_galois_break(samples: &[u64]) -> (u32, f64) {
    use std::collections::HashSet;
    let start_bits = if samples.len() < 1000 { 8 }
        else if samples.len() < 10000 { 12 }
        else { 16 };
    
    for bits in start_bits..=24 {
        let size = 1u64 << bits;
        let mut seen = HashSet::new();
        for &s in samples {
            seen.insert(s % size);
        }
        let coverage = seen.len() as f64 / size as f64 * 100.0;
        if coverage < 99.0 {
            return (bits, coverage);
        }
    }
    (24, 100.0)
}

fn check_prime_coverage(samples: &[u64], prime: u64) -> f64 {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    for &s in samples {
        seen.insert(s % prime);
    }
    seen.len() as f64 / prime as f64 * 100.0
}

fn get_emoji_story(galois_bits: u32, prime_71_cov: f64, sample_count: usize) -> String {
    let complexity = if galois_bits <= 10 { "🟢 Trivial" }
        else if galois_bits <= 14 { "🟡 Simple" }
        else if galois_bits <= 18 { "🟠 Moderate" }
        else { "🔴 Complex" };
    
    let prime_story = if prime_71_cov >= 99.0 { "✅ Found 71!" }
        else if prime_71_cov >= 50.0 { "🔍 Partial 71" }
        else { "❌ No 71" };
    
    let size = if sample_count < 5000 { "🐭 Tiny" }
        else if sample_count < 25000 { "🐱 Small" }
        else if sample_count < 100000 { "🐕 Medium" }
        else { "🐘 Large" };
    
    format!("{} {} {}", complexity, prime_story, size)
}

fn analyze_perf_file(path: &Path) -> Result<Option<(u32, f64, f64, usize, String)>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let PerfFileReader { mut perf_file, mut record_iter } = PerfFileReader::parse_file(reader)?;
    
    let mut samples = Vec::new();
    while let Some(_) = record_iter.next_record(&mut perf_file)? {
        samples.push(samples.len() as u64);
    }
    
    if samples.len() < 10 {
        return Ok(None);
    }
    
    let (galois_bits, galois_cov) = find_galois_break(&samples);
    let prime_71_cov = check_prime_coverage(&samples, 71);
    let emoji_story = get_emoji_story(galois_bits, prime_71_cov, samples.len());
    
    Ok(Some((galois_bits, galois_cov, prime_71_cov, samples.len(), emoji_story)))
}

fn main() -> Result<()> {
    let perf_dir = Path::new("/mnt/data1/meta-introspector/data/71_flakes_perf");
    
    println!("# 71 Languages Galois Field Analysis Report\n");
    println!("| Language | Galois | Samples | Prime 71 | Story |");
    println!("|----------|--------|---------|----------|-------|");
    
    let mut results = Vec::new();
    
    for entry in read_dir(perf_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            if name.ends_with("_build.perf.data") {
                let lang = name.replace("_build.perf.data", "").rsplit('_').skip(1).collect::<Vec<_>>().join("_");
                
                match analyze_perf_file(&path) {
                    Ok(Some((bits, _cov, p71, samples, story))) => {
                        results.push((lang.clone(), bits, samples));
                        println!("| {:15} | GF(2^{:2}) | {:7} | {:5.1}% | {} |", 
                            lang, bits, samples, p71, story);
                    },
                    Ok(None) => {},
                    Err(_) => {
                        eprintln!("⚠️  Skipping {} (parse error)", lang);
                    }
                }
            }
        }
    }
    
    println!("\n## Summary");
    println!("Total analyzed: {}", results.len());
    
    if !results.is_empty() {
        let simplest = results.iter().min_by_key(|r| r.1).unwrap();
        let complex = results.iter().max_by_key(|r| r.1).unwrap();
        
        println!("\n🟢 Simplest: {} - GF(2^{})", simplest.0, simplest.1);
        println!("🔴 Most Complex: {} - GF(2^{})", complex.0, complex.1);
    }
    
    Ok(())
}
