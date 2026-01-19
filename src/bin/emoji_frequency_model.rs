use std::collections::HashMap;
use std::fs;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmojiFrequency {
    pub emoji: String,
    pub frequency: usize,
    pub prime_encoding: u64,
    pub harmonic: f64,
    pub orbit: String,
    pub sources: Vec<String>,
}

pub fn emoji_frequency_analysis() -> Vec<EmojiFrequency> {
    let mut emoji_counts: HashMap<String, (usize, Vec<String>)> = HashMap::new();
    
    // Sources to scan
    let sources = vec![
        "/home/mdupont/nix/ai-ml-zk-ops",
        "/mnt/data1/2023/08/19/meta-meme.wiki",
        "/mnt/data1/nix/vendor/rust/cargo2nix/ai-ml-zk-ops/documentation/art/art/memes",
    ];
    
    for source in &sources {
        scan_directory(source, &mut emoji_counts);
    }
    
    // Convert to EmojiFrequency
    let mut frequencies: Vec<EmojiFrequency> = emoji_counts
        .into_iter()
        .map(|(emoji, (freq, sources))| {
            let prime = emoji_to_prime(&emoji);
            let harmonic = 1.0 / (prime as f64);
            let orbit = format!("{}.a{}", prime % 1000, (freq as f64).log10() as u32 + 1);
            
            EmojiFrequency {
                emoji,
                frequency: freq,
                prime_encoding: prime,
                harmonic,
                orbit,
                sources,
            }
        })
        .collect();
    
    frequencies.sort_by(|a, b| b.frequency.cmp(&a.frequency));
    frequencies
}

fn scan_directory(path: &str, counts: &mut HashMap<String, (usize, Vec<String>)>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            
            if path.is_dir() {
                scan_directory(path.to_str().unwrap(), counts);
            } else if let Some(ext) = path.extension() {
                if ext == "md" || ext == "txt" || ext == "org" {
                    if let Ok(content) = fs::read_to_string(&path) {
                        extract_emojis(&content, &path, counts);
                    }
                }
            }
        }
    }
}

fn extract_emojis(content: &str, path: &Path, counts: &mut HashMap<String, (usize, Vec<String>)>) {
    for ch in content.chars() {
        if is_emoji(ch) {
            let emoji = ch.to_string();
            let source = path.to_string_lossy().to_string();
            
            counts.entry(emoji)
                .and_modify(|(count, sources)| {
                    *count += 1;
                    if !sources.contains(&source) {
                        sources.push(source.clone());
                    }
                })
                .or_insert((1, vec![source]));
        }
    }
}

fn is_emoji(ch: char) -> bool {
    matches!(ch as u32,
        0x1F300..=0x1F9FF | // Misc Symbols and Pictographs
        0x1F600..=0x1F64F | // Emoticons
        0x1F680..=0x1F6FF | // Transport and Map
        0x2600..=0x26FF |   // Misc symbols
        0x2700..=0x27BF |   // Dingbats
        0x1F900..=0x1F9FF | // Supplemental Symbols
        0x1FA70..=0x1FAFF   // Symbols and Pictographs Extended-A
    )
}

fn emoji_to_prime(emoji: &str) -> u64 {
    // Map emoji to prime number (like in 42.md)
    let primes = vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53,
        59, 61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113
    ];
    
    let hash = emoji.chars().next().unwrap() as u64;
    primes[(hash % primes.len() as u64) as usize]
}

fn main() {
    println!("🔍 Emoji Frequency Analysis");
    println!();
    println!("Scanning:");
    println!("  - /home/mdupont/nix/ai-ml-zk-ops");
    println!("  - /mnt/data1/2023/08/19/meta-meme.wiki");
    println!("  - /mnt/data1/nix/vendor/rust/cargo2nix/ai-ml-zk-ops/documentation/art/art/memes");
    println!();
    
    let frequencies = emoji_frequency_analysis();
    
    println!("✅ Found {} unique emojis", frequencies.len());
    println!();
    println!("📊 Top 100 Emojis by Frequency:");
    println!();
    
    for (i, ef) in frequencies.iter().take(100).enumerate() {
        println!("{:3}. {} freq:{:5} prime:{:3} harmonic:{:.4} orbit:{} sources:{}",
                 i + 1,
                 ef.emoji,
                 ef.frequency,
                 ef.prime_encoding,
                 ef.harmonic,
                 ef.orbit,
                 ef.sources.len());
    }
    
    // Export
    let json = serde_json::to_string_pretty(&frequencies).unwrap();
    fs::write("/mnt/data1/meta-introspector/emoji_frequencies.json", json).unwrap();
    
    println!();
    println!("💾 Saved {} emoji frequencies to emoji_frequencies.json", frequencies.len());
}
