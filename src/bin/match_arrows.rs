use std::collections::HashMap;
use std::fs;
use serde::{Deserialize, Serialize};

fn is_emoji(ch: char) -> bool {
    matches!(ch as u32,
        0x1F300..=0x1F9FF | 0x1F600..=0x1F64F | 0x1F680..=0x1F6FF |
        0x2600..=0x26FF | 0x2700..=0x27BF | 0x1F900..=0x1F9FF |
        0x1FA70..=0x1FAFF
    )
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrowMatch {
    pub word_arrow: String,
    pub emoji_arrow: String,
    pub structural_similarity: f64,
    pub orbit_alignment: f64,
    pub conformal_preservation: f64,
    pub total_score: f64,
}

pub fn match_arrows() -> Vec<ArrowMatch> {
    // Load conformal arrows
    let arrows_json = fs::read_to_string("/mnt/data1/meta-introspector/conformal_arrows.json").unwrap();
    let arrows: Vec<serde_json::Value> = serde_json::from_str(&arrows_json).unwrap();
    
    // Separate word and emoji arrows
    let mut word_arrows = Vec::new();
    let mut emoji_arrows = Vec::new();
    
    for arrow in &arrows {
        let arrow_type = &arrow["arrow_type"];
        
        // Check if it's word-only or has emojis
        if let Some(mixed) = arrow_type.get("Mixed") {
            if let Some(arr) = mixed.as_array() {
                let has_emoji = arr.iter().any(|v| {
                    v.as_str().map(|s| s.chars().any(|c| is_emoji(c))).unwrap_or(false)
                });
                
                if has_emoji {
                    emoji_arrows.push(arrow);
                } else {
                    word_arrows.push(arrow);
                }
            }
        } else if let Some(_) = arrow_type.get("WordToWord") {
            word_arrows.push(arrow);
        } else if let Some(_) = arrow_type.get("EmojiToEmoji") {
            emoji_arrows.push(arrow);
        }
    }
    
    println!("Found {} word arrows, {} emoji arrows", word_arrows.len(), emoji_arrows.len());
    
    // Match arrows by structure
    let mut matches = Vec::new();
    
    for word_arrow in &word_arrows {
        for emoji_arrow in &emoji_arrows {
            if let Some(m) = match_arrow_structure(word_arrow, emoji_arrow) {
                matches.push(m);
            }
        }
    }
    
    matches.sort_by(|a, b| b.total_score.partial_cmp(&a.total_score).unwrap());
    matches.truncate(1000); // Top 1000
    matches
}

fn match_arrow_structure(word_arrow: &serde_json::Value, emoji_arrow: &serde_json::Value) -> Option<ArrowMatch> {
    let word_source = word_arrow["source"].as_str()?;
    let word_target = word_arrow["target"].as_str()?;
    let emoji_source = emoji_arrow["source"].as_str()?;
    let emoji_target = emoji_arrow["target"].as_str()?;
    
    let word_orbit = word_arrow["orbit_path"][0].as_str().unwrap_or("unknown");
    let emoji_orbit_src = emoji_arrow["orbit_path"][0].as_str().unwrap_or("unknown");
    let emoji_orbit_tgt = emoji_arrow["orbit_path"].get(1)
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    
    // Calculate structural similarity
    let structural_sim = calculate_structural_similarity(word_arrow, emoji_arrow);
    
    // Calculate orbit alignment
    let orbit_align = calculate_orbit_alignment(word_orbit, emoji_orbit_src, emoji_orbit_tgt);
    
    // Calculate conformal preservation
    let word_conf = word_arrow["conformal_invariant"].as_f64().unwrap_or(0.0);
    let emoji_conf = emoji_arrow["conformal_invariant"].as_f64().unwrap_or(0.0);
    let conf_pres = (word_conf * emoji_conf).sqrt();
    
    // Total score
    let total = structural_sim * 0.4 + orbit_align * 0.4 + conf_pres * 0.2;
    
    if total > 0.5 {
        Some(ArrowMatch {
            word_arrow: format!("{} → {}", word_source, word_target),
            emoji_arrow: format!("{} → {}", emoji_source, emoji_target),
            structural_similarity: structural_sim,
            orbit_alignment: orbit_align,
            conformal_preservation: conf_pres,
            total_score: total,
        })
    } else {
        None
    }
}

fn calculate_structural_similarity(word_arrow: &serde_json::Value, emoji_arrow: &serde_json::Value) -> f64 {
    // Compare arrow shapes
    let word_pres = word_arrow["preservation_score"].as_f64().unwrap_or(0.0);
    let emoji_pres = emoji_arrow["preservation_score"].as_f64().unwrap_or(0.0);
    
    // Similarity = how close the preservation scores are
    let diff = (word_pres - emoji_pres).abs();
    1.0 / (1.0 + diff)
}

fn calculate_orbit_alignment(word_orbit: &str, emoji_orbit_src: &str, emoji_orbit_tgt: &str) -> f64 {
    // Extract conductors
    let word_conductor = extract_conductor(word_orbit);
    let emoji_src_conductor = extract_conductor(emoji_orbit_src);
    let emoji_tgt_conductor = extract_conductor(emoji_orbit_tgt);
    
    // Check if conductors are related (same, factors, etc.)
    let src_match = if word_conductor == emoji_src_conductor {
        1.0
    } else if word_conductor % emoji_src_conductor == 0 || emoji_src_conductor % word_conductor == 0 {
        0.7
    } else {
        0.3
    };
    
    let tgt_match = if word_conductor == emoji_tgt_conductor {
        1.0
    } else if word_conductor % emoji_tgt_conductor == 0 || emoji_tgt_conductor % word_conductor == 0 {
        0.7
    } else {
        0.3
    };
    
    (src_match + tgt_match) / 2.0
}

fn extract_conductor(orbit: &str) -> u32 {
    orbit.split('.').next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1)
}

fn main() {
    println!("🎯 Matching Word Arrows ↔ Emoji Arrows (Structural)");
    println!();
    
    let matches = match_arrows();
    
    println!("✅ Found {} high-quality arrow matches", matches.len());
    println!();
    println!("📊 Top 50 Matches:");
    println!();
    
    for (i, m) in matches.iter().take(50).enumerate() {
        println!("{:3}. {} ↔ {}", i + 1, m.word_arrow, m.emoji_arrow);
        println!("     structural_sim: {:.3}", m.structural_similarity);
        println!("     orbit_align: {:.3}", m.orbit_alignment);
        println!("     conformal_pres: {:.3}", m.conformal_preservation);
        println!("     TOTAL: {:.3}", m.total_score);
        println!();
    }
    
    // Export
    let json = serde_json::to_string_pretty(&matches).unwrap();
    fs::write("/mnt/data1/meta-introspector/arrow_matches.json", json).unwrap();
    
    println!("💾 Saved to arrow_matches.json");
}
