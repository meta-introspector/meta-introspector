use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrbitCompression {
    pub compression_level: u8,
    pub removed_features: Vec<RustFeature>,
    pub orbit_radius: f64,
    pub name_compression: NameCompression,
    pub curry_level: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RustFeature {
    Macros,
    BorrowChecker,
    Lifetimes,
    Traits,
    Generics,
    PatternMatching,
    Closures,
    AsyncAwait,
    Modules,
    Attributes,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NameCompression {
    pub symbol_table: std::collections::HashMap<String, u16>,
    pub compression_ratio: f64,
    pub huffman_tree: Vec<u8>,
}

pub struct OrbitCompressor;

impl OrbitCompressor {
    pub fn compress_rust_orbit(
        source: &str,
        target_radius: f64
    ) -> Result<(String, OrbitCompression)> {
        let mut compressed = source.to_string();
        let mut removed = Vec::new();
        let mut compression_level = 0;
        
        // Level 1: Remove macros
        if Self::orbit_radius(&compressed) > target_radius {
            compressed = Self::remove_macros(&compressed);
            removed.push(RustFeature::Macros);
            compression_level = 1;
        }
        
        // Level 2: Remove borrow checker
        if Self::orbit_radius(&compressed) > target_radius {
            compressed = Self::remove_borrowing(&compressed);
            removed.push(RustFeature::BorrowChecker);
            removed.push(RustFeature::Lifetimes);
            compression_level = 2;
        }
        
        // Level 3: Remove traits
        if Self::orbit_radius(&compressed) > target_radius {
            compressed = Self::remove_traits(&compressed);
            removed.push(RustFeature::Traits);
            compression_level = 3;
        }
        
        // Level 4: Remove generics
        if Self::orbit_radius(&compressed) > target_radius {
            compressed = Self::remove_generics(&compressed);
            removed.push(RustFeature::Generics);
            compression_level = 4;
        }
        
        // Level 5: Compress names
        let (final_code, name_compression) = Self::compress_names(&compressed);
        
        let orbit_compression = OrbitCompression {
            compression_level,
            removed_features: removed,
            orbit_radius: Self::orbit_radius(&final_code),
            name_compression,
            curry_level: Self::compute_curry_level(&final_code),
        };
        
        Ok((final_code, orbit_compression))
    }
    
    fn remove_macros(code: &str) -> String {
        code.lines()
            .filter(|line| !line.trim_start().starts_with('#'))
            .filter(|line| !line.contains('!'))
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    fn remove_borrowing(code: &str) -> String {
        code.replace("&mut ", "")
            .replace("&", "")
            .replace("'static", "")
            .replace("'a", "")
            .replace("'_", "")
    }
    
    fn remove_traits(code: &str) -> String {
        code.lines()
            .filter(|line| !line.contains("trait "))
            .filter(|line| !line.contains("impl "))
            .filter(|line| !line.contains("dyn "))
            .collect::<Vec<_>>()
            .join("\n")
    }
    
    fn remove_generics(code: &str) -> String {
        // Remove <T>, <'a, T>, etc.
        let mut result = code.to_string();
        while let Some(start) = result.find('<') {
            if let Some(end) = result[start..].find('>') {
                result.replace_range(start..start + end + 1, "");
            } else {
                break;
            }
        }
        result
    }
    
    fn compress_names(code: &str) -> (String, NameCompression) {
        let mut symbol_table = std::collections::HashMap::new();
        let mut counter = 0u16;
        
        // Extract identifiers and compress them
        let words: Vec<&str> = code.split_whitespace().collect();
        let mut compressed_words = Vec::new();
        
        for word in words {
            if word.chars().all(|c| c.is_alphanumeric() || c == '_') && word.len() > 3 {
                let compressed = symbol_table.entry(word.to_string())
                    .or_insert_with(|| {
                        counter += 1;
                        counter
                    });
                compressed_words.push(format!("_{}", compressed));
            } else {
                compressed_words.push(word.to_string());
            }
        }
        
        let compressed_code = compressed_words.join(" ");
        let compression_ratio = compressed_code.len() as f64 / code.len() as f64;
        
        (compressed_code, NameCompression {
            symbol_table,
            compression_ratio,
            huffman_tree: vec![], // Simplified
        })
    }
    
    fn orbit_radius(code: &str) -> f64 {
        // Measure complexity as orbit radius
        let features = [
            ("fn ", 1.0),
            ("struct ", 2.0),
            ("enum ", 2.0),
            ("impl ", 3.0),
            ("trait ", 4.0),
            ("<", 2.0),        // Generics
            ("&", 1.5),        // Borrowing
            ("!", 3.0),        // Macros
            ("async ", 4.0),
            ("unsafe ", 5.0),
        ];
        
        features.iter()
            .map(|(pattern, weight)| code.matches(pattern).count() as f64 * weight)
            .sum::<f64>()
            .sqrt()
    }
    
    fn compute_curry_level(code: &str) -> u8 {
        // How much can we curry/partial-apply functions?
        let fn_count = code.matches("fn ").count();
        let param_count = code.matches('(').count();
        
        if fn_count == 0 { 0 } else { (param_count / fn_count).min(255) as u8 }
    }
}

pub struct MinimalRust;

impl MinimalRust {
    pub fn generate_minimal_syntax() -> String {
        // Ultra-compressed Rust: just functions and basic types
        r#"
fn _1(_2: _3) -> _4 {
    _5(_6)
}

fn _7(_8: _9, _10: _11) -> _12 {
    if _13 { _14 } else { _15 }
}
"#.to_string()
    }
    
    pub fn compute_compression_savings(
        original: &str,
        compressed: &OrbitCompression
    ) -> f64 {
        let original_size = original.len() as f64;
        let feature_reduction = compressed.removed_features.len() as f64 * 0.1;
        let name_compression = compressed.name_compression.compression_ratio;
        let orbit_tightening = 1.0 / (1.0 + compressed.orbit_radius);
        
        feature_reduction + name_compression + orbit_tightening
    }
}
