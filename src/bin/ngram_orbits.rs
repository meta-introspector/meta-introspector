use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NgramOrbit {
    pub ngram: String,
    pub frequency: usize,
    pub godel_number: u64,
    pub lmfdb_orbit: String,
}

pub fn extract_ngrams_from_tickets() -> Vec<NgramOrbit> {
    let dir = "/mnt/data1/nix/vendor/rust/cargo2nix/ai-ml-zk-ops/documentation/art/art/memes/extracted_tickets";
    
    let mut unigrams: HashMap<String, usize> = HashMap::new();
    let mut bigrams: HashMap<String, usize> = HashMap::new();
    let mut trigrams: HashMap<String, usize> = HashMap::new();
    
    // Read all tickets
    for entry in fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            let content = fs::read_to_string(&path).unwrap_or_default();
            let words: Vec<&str> = content.split_whitespace().collect();
            
            // Unigrams
            for word in &words {
                let clean = word.to_lowercase().trim_matches(|c: char| !c.is_alphanumeric()).to_string();
                if clean.len() > 2 {
                    *unigrams.entry(clean).or_insert(0) += 1;
                }
            }
            
            // Bigrams
            for window in words.windows(2) {
                let bigram = format!("{} {}", window[0], window[1]).to_lowercase();
                *bigrams.entry(bigram).or_insert(0) += 1;
            }
            
            // Trigrams
            for window in words.windows(3) {
                let trigram = format!("{} {} {}", window[0], window[1], window[2]).to_lowercase();
                *trigrams.entry(trigram).or_insert(0) += 1;
            }
        }
    }
    
    let mut orbits = Vec::new();
    
    // Map unigrams to orbits
    for (ngram, freq) in unigrams.iter() {
        if *freq > 2 {
            orbits.push(create_orbit(ngram, *freq));
        }
    }
    
    // Map bigrams to orbits
    for (ngram, freq) in bigrams.iter() {
        if *freq > 1 {
            orbits.push(create_orbit(ngram, *freq));
        }
    }
    
    // Map trigrams to orbits
    for (ngram, freq) in trigrams.iter() {
        if *freq > 1 {
            orbits.push(create_orbit(ngram, *freq));
        }
    }
    
    orbits.sort_by(|a, b| b.frequency.cmp(&a.frequency));
    orbits
}

fn create_orbit(ngram: &str, frequency: usize) -> NgramOrbit {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    ngram.hash(&mut hasher);
    let godel = hasher.finish();
    
    let conductor = (godel % 1000) as u32;
    let level = (frequency as f64).log10() as u32 + 1;
    let orbit = format!("{}.a{}", conductor, level);
    
    NgramOrbit {
        ngram: ngram.to_string(),
        frequency,
        godel_number: godel,
        lmfdb_orbit: orbit,
    }
}

fn main() {
    println!("🔍 Extracting n-grams from 194 tickets...");
    
    let orbits = extract_ngrams_from_tickets();
    
    println!("✅ Found {} unique n-grams", orbits.len());
    println!();
    
    // Top 50 by frequency
    println!("📊 Top 50 n-grams by frequency:");
    println!();
    
    for (i, orbit) in orbits.iter().take(50).enumerate() {
        println!("{:3}. {:40} freq:{:4} orbit:{} godel:{}",
                 i + 1,
                 orbit.ngram,
                 orbit.frequency,
                 orbit.lmfdb_orbit,
                 orbit.godel_number);
    }
    
    // Export to JSON
    let json = serde_json::to_string_pretty(&orbits).unwrap();
    fs::write("/mnt/data1/meta-introspector/ngram_orbits.json", json).unwrap();
    
    println!();
    println!("💾 Saved {} n-gram orbits to ngram_orbits.json", orbits.len());
}
