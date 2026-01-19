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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmojiFrequency {
    pub emoji: String,
    pub frequency: usize,
    pub prime_encoding: u64,
    pub harmonic: f64,
    pub orbit: String,
    pub sources: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordEmojiConnection {
    pub word: String,
    pub emoji: String,
    pub shared_orbit: String,
    pub word_freq: usize,
    pub emoji_freq: usize,
    pub resonance: f64,
    pub connection_strength: f64,
}

pub fn connect_word_emoji_models() -> Vec<WordEmojiConnection> {
    // Load word model
    let word_json = fs::read_to_string("/mnt/data1/meta-introspector/ngram_orbits.json").unwrap();
    let words: Vec<NgramOrbit> = serde_json::from_str(&word_json).unwrap();
    
    // Load emoji model
    let emoji_json = fs::read_to_string("/mnt/data1/meta-introspector/emoji_frequencies.json").unwrap();
    let emojis: Vec<EmojiFrequency> = serde_json::from_str(&emoji_json).unwrap();
    
    // Build orbit index
    let mut word_orbits: HashMap<String, Vec<&NgramOrbit>> = HashMap::new();
    for word in &words {
        word_orbits.entry(word.lmfdb_orbit.clone())
            .or_insert_with(Vec::new)
            .push(word);
    }
    
    let mut emoji_orbits: HashMap<String, Vec<&EmojiFrequency>> = HashMap::new();
    for emoji in &emojis {
        emoji_orbits.entry(emoji.orbit.clone())
            .or_insert_with(Vec::new)
            .push(emoji);
    }
    
    // Find connections via shared orbits
    let mut connections = Vec::new();
    
    for (orbit, word_list) in &word_orbits {
        if let Some(emoji_list) = emoji_orbits.get(orbit) {
            // Found shared orbit!
            for word in word_list {
                for emoji in emoji_list {
                    let resonance = calculate_resonance(word.frequency, emoji.frequency);
                    let strength = calculate_connection_strength(word, emoji);
                    
                    connections.push(WordEmojiConnection {
                        word: word.ngram.clone(),
                        emoji: emoji.emoji.clone(),
                        shared_orbit: orbit.clone(),
                        word_freq: word.frequency,
                        emoji_freq: emoji.frequency,
                        resonance,
                        connection_strength: strength,
                    });
                }
            }
        }
    }
    
    // Sort by connection strength
    connections.sort_by(|a, b| b.connection_strength.partial_cmp(&a.connection_strength).unwrap());
    
    connections
}

fn calculate_resonance(word_freq: usize, emoji_freq: usize) -> f64 {
    // Harmonic mean of frequencies
    let w = word_freq as f64;
    let e = emoji_freq as f64;
    2.0 * w * e / (w + e)
}

fn calculate_connection_strength(word: &NgramOrbit, emoji: &EmojiFrequency) -> f64 {
    // Combine frequency correlation and harmonic resonance
    let freq_correlation = (word.frequency as f64 * emoji.frequency as f64).sqrt();
    let harmonic_resonance = emoji.harmonic * 1000.0; // Scale up
    
    freq_correlation * harmonic_resonance
}

fn main() {
    println!("🔗 Connecting Word Model ↔ Emoji Model via LMFDB Orbits");
    println!();
    
    let connections = connect_word_emoji_models();
    
    println!("✅ Found {} word-emoji connections via shared orbits", connections.len());
    println!();
    println!("📊 Top 100 Connections:");
    println!();
    
    for (i, conn) in connections.iter().take(100).enumerate() {
        println!("{:3}. {:30} ↔ {} orbit:{}",
                 i + 1,
                 conn.word,
                 conn.emoji,
                 conn.shared_orbit);
        println!("     word_freq:{:4} emoji_freq:{:4} resonance:{:.2} strength:{:.2}",
                 conn.word_freq,
                 conn.emoji_freq,
                 conn.resonance,
                 conn.connection_strength);
        println!();
    }
    
    // Statistics
    let unique_orbits: std::collections::HashSet<_> = connections.iter()
        .map(|c| c.shared_orbit.clone())
        .collect();
    
    println!("📈 Statistics:");
    println!("  Total connections: {}", connections.len());
    println!("  Unique shared orbits: {}", unique_orbits.len());
    println!("  Avg connection strength: {:.2}", 
             connections.iter().map(|c| c.connection_strength).sum::<f64>() / connections.len() as f64);
    
    // Export
    let json = serde_json::to_string_pretty(&connections).unwrap();
    fs::write("/mnt/data1/meta-introspector/word_emoji_connections.json", json).unwrap();
    
    println!();
    println!("💾 Saved to word_emoji_connections.json");
}
