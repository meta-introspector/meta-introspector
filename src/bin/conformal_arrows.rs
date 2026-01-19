use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConformalArrow {
    pub source: String,
    pub target: String,
    pub arrow_type: ArrowType,
    pub orbit_path: Vec<String>,
    pub conformal_invariant: f64,
    pub preservation_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArrowType {
    WordToWord(String, String),      // "meta" → "meme"
    EmojiToEmoji(String, String),    // 🔮 → 🌌
    WordToEmoji(String, String),     // "recursive" → 🌀
    EmojiToWord(String, String),     // 🔑 → "key"
    Mixed(Vec<String>),              // "meta" → 🔮 → "meme"
}

pub fn extract_conformal_arrows() -> Vec<ConformalArrow> {
    let mut arrows = Vec::new();
    
    // Load models
    let word_json = fs::read_to_string("/mnt/data1/meta-introspector/ngram_orbits.json").unwrap();
    let words: Vec<serde_json::Value> = serde_json::from_str(&word_json).unwrap();
    
    let emoji_json = fs::read_to_string("/mnt/data1/meta-introspector/emoji_frequencies.json").unwrap();
    let emojis: Vec<serde_json::Value> = serde_json::from_str(&emoji_json).unwrap();
    
    // Extract word n-grams (already have bigrams/trigrams)
    arrows.extend(extract_word_arrows(&words));
    
    // Extract emoji sequences
    arrows.extend(extract_emoji_arrows(&emojis));
    
    // Extract mixed sequences (word-emoji-word)
    arrows.extend(extract_mixed_arrows(&words, &emojis));
    
    // Calculate conformal invariants
    for arrow in &mut arrows {
        arrow.conformal_invariant = calculate_conformal_invariant(arrow);
        arrow.preservation_score = calculate_preservation_score(arrow);
    }
    
    arrows.sort_by(|a, b| b.preservation_score.partial_cmp(&a.preservation_score).unwrap());
    arrows
}

fn extract_word_arrows(words: &[serde_json::Value]) -> Vec<ConformalArrow> {
    let mut arrows = Vec::new();
    
    // Find bigrams and trigrams
    for word in words {
        let ngram = word["ngram"].as_str().unwrap();
        let orbit = word["lmfdb_orbit"].as_str().unwrap();
        
        if ngram.contains(' ') {
            let parts: Vec<&str> = ngram.split_whitespace().collect();
            if parts.len() == 2 {
                arrows.push(ConformalArrow {
                    source: parts[0].to_string(),
                    target: parts[1].to_string(),
                    arrow_type: ArrowType::WordToWord(parts[0].to_string(), parts[1].to_string()),
                    orbit_path: vec![orbit.to_string()],
                    conformal_invariant: 0.0,
                    preservation_score: 0.0,
                });
            } else if parts.len() == 3 {
                // Trigram: A → B → C
                arrows.push(ConformalArrow {
                    source: parts[0].to_string(),
                    target: parts[2].to_string(),
                    arrow_type: ArrowType::Mixed(parts.iter().map(|s| s.to_string()).collect()),
                    orbit_path: vec![orbit.to_string()],
                    conformal_invariant: 0.0,
                    preservation_score: 0.0,
                });
            }
        }
    }
    
    arrows
}

fn extract_emoji_arrows(emojis: &[serde_json::Value]) -> Vec<ConformalArrow> {
    let mut arrows = Vec::new();
    
    // Scan source files for emoji sequences
    let sources = vec![
        "/mnt/data1/2023/08/19/meta-meme.wiki",
        "/home/mdupont/nix/ai-ml-zk-ops",
    ];
    
    for source in sources {
        if let Ok(entries) = fs::read_dir(source) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("md") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        arrows.extend(find_emoji_sequences(&content, emojis));
                    }
                }
            }
        }
    }
    
    arrows
}

fn find_emoji_sequences(content: &str, emojis: &[serde_json::Value]) -> Vec<ConformalArrow> {
    let mut arrows = Vec::new();
    let emoji_chars: Vec<char> = content.chars()
        .filter(|c| is_emoji(*c))
        .collect();
    
    // Find consecutive emoji pairs
    for window in emoji_chars.windows(2) {
        let source = window[0].to_string();
        let target = window[1].to_string();
        
        // Find orbits
        let source_orbit = find_emoji_orbit(&source, emojis);
        let target_orbit = find_emoji_orbit(&target, emojis);
        
        arrows.push(ConformalArrow {
            source: source.clone(),
            target: target.clone(),
            arrow_type: ArrowType::EmojiToEmoji(source, target),
            orbit_path: vec![source_orbit, target_orbit],
            conformal_invariant: 0.0,
            preservation_score: 0.0,
        });
    }
    
    arrows
}

fn extract_mixed_arrows(words: &[serde_json::Value], emojis: &[serde_json::Value]) -> Vec<ConformalArrow> {
    let mut arrows = Vec::new();
    
    // Scan for word-emoji-word patterns
    let sources = vec![
        "/mnt/data1/2023/08/19/meta-meme.wiki",
        "/home/mdupont/nix/ai-ml-zk-ops",
    ];
    
    for source in sources {
        if let Ok(entries) = fs::read_dir(source) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().and_then(|s| s.to_str()) == Some("md") {
                    if let Ok(content) = fs::read_to_string(&path) {
                        arrows.extend(find_mixed_sequences(&content, words, emojis));
                    }
                }
            }
        }
    }
    
    arrows
}

fn find_mixed_sequences(content: &str, words: &[serde_json::Value], emojis: &[serde_json::Value]) -> Vec<ConformalArrow> {
    let mut arrows = Vec::new();
    
    // Simple pattern: word emoji word
    let tokens: Vec<String> = content.split_whitespace()
        .take(1000) // Limit for performance
        .map(|s| s.to_string())
        .collect();
    
    for window in tokens.windows(3) {
        let has_emoji = window.iter().any(|t| t.chars().any(|c| is_emoji(c)));
        if has_emoji {
            arrows.push(ConformalArrow {
                source: window[0].clone(),
                target: window[2].clone(),
                arrow_type: ArrowType::Mixed(window.to_vec()),
                orbit_path: vec!["mixed".to_string()],
                conformal_invariant: 0.0,
                preservation_score: 0.0,
            });
        }
    }
    
    arrows
}

fn calculate_conformal_invariant(arrow: &ConformalArrow) -> f64 {
    // Conformal invariant = preserved under transformations
    // Use cross-ratio-like measure
    match &arrow.arrow_type {
        ArrowType::WordToWord(_, _) => 1.0,
        ArrowType::EmojiToEmoji(_, _) => 2.0,
        ArrowType::WordToEmoji(_, _) => 1.5,
        ArrowType::EmojiToWord(_, _) => 1.5,
        ArrowType::Mixed(_) => 3.0,
    }
}

fn calculate_preservation_score(arrow: &ConformalArrow) -> f64 {
    // How well the arrow preserves structure
    let orbit_consistency = if arrow.orbit_path.len() > 1 {
        // Check if orbits are related
        1.0 / arrow.orbit_path.len() as f64
    } else {
        1.0
    };
    
    arrow.conformal_invariant * orbit_consistency
}

fn is_emoji(ch: char) -> bool {
    matches!(ch as u32,
        0x1F300..=0x1F9FF | 0x1F600..=0x1F64F | 0x1F680..=0x1F6FF |
        0x2600..=0x26FF | 0x2700..=0x27BF | 0x1F900..=0x1F9FF |
        0x1FA70..=0x1FAFF
    )
}

fn find_emoji_orbit(emoji: &str, emojis: &[serde_json::Value]) -> String {
    for e in emojis {
        if e["emoji"].as_str() == Some(emoji) {
            return e["orbit"].as_str().unwrap_or("unknown").to_string();
        }
    }
    "unknown".to_string()
}

fn main() {
    println!("➡️  Extracting Conformal Arrows (N-grams as Morphisms)");
    println!();
    
    let arrows = extract_conformal_arrows();
    
    println!("✅ Found {} conformal arrows", arrows.len());
    println!();
    println!("📊 Top 50 Arrows (by preservation score):");
    println!();
    
    for (i, arrow) in arrows.iter().take(50).enumerate() {
        println!("{:3}. {} → {}", i + 1, arrow.source, arrow.target);
        println!("     type: {:?}", arrow.arrow_type);
        println!("     orbit_path: {:?}", arrow.orbit_path);
        println!("     conformal_invariant: {:.2}", arrow.conformal_invariant);
        println!("     preservation_score: {:.2}", arrow.preservation_score);
        println!();
    }
    
    // Export
    let json = serde_json::to_string_pretty(&arrows).unwrap();
    fs::write("/mnt/data1/meta-introspector/conformal_arrows.json", json).unwrap();
    
    println!("💾 Saved to conformal_arrows.json");
}
