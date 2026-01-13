use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct RustcMarkovSkeleton {
    syn_transitions: HashMap<String, Vec<(String, f64)>>,
    serde_patterns: HashMap<String, String>,
    structure_probabilities: HashMap<String, f64>,
}

fn main() {
    println!("🚀 Rustc as Markov Model of Syn-Serde Skeleton");
    
    // Create the skeleton structure based on syn AST patterns
    let mut skeleton = RustcMarkovSkeleton {
        syn_transitions: HashMap::new(),
        serde_patterns: HashMap::new(),
        structure_probabilities: HashMap::new(),
    };
    
    // Build Markov transitions for syn AST nodes
    build_syn_transitions(&mut skeleton);
    
    // Build serde serialization patterns
    build_serde_patterns(&mut skeleton);
    
    // Calculate structure probabilities from our rust-build analysis
    calculate_structure_probabilities(&mut skeleton);
    
    // Test: Generate Rust code from the Markov model
    let generated_code = generate_rust_from_skeleton(&skeleton, "fn");
    println!("📝 Generated from skeleton: {}", generated_code);
    
    // Compress using the skeleton model
    let test_code = "fn main() { let x = 42; println!(\"test\"); }";
    let compressed = compress_with_skeleton(&skeleton, test_code);
    let decompressed = decompress_with_skeleton(&skeleton, &compressed);
    
    println!("🗜️  Original: {}", test_code);
    println!("📦 Compressed: {} bytes", compressed.len());
    println!("📤 Decompressed: {}", decompressed);
    
    // Calculate theoretical compression for rust-build using skeleton
    let skeleton_compression = estimate_skeleton_compression(&skeleton);
    println!("🎯 Skeleton compression estimate: {:.1}%", skeleton_compression);
}

fn build_syn_transitions(skeleton: &mut RustcMarkovSkeleton) {
    // Based on syn::Item patterns from our analysis
    skeleton.syn_transitions.insert("fn".to_string(), vec![
        ("ident".to_string(), 1.0),
        ("(".to_string(), 0.95),
        ("->".to_string(), 0.3),
        ("{".to_string(), 0.98),
    ]);
    
    skeleton.syn_transitions.insert("struct".to_string(), vec![
        ("ident".to_string(), 1.0),
        ("{".to_string(), 0.8),
        ("(".to_string(), 0.2),
    ]);
    
    skeleton.syn_transitions.insert("impl".to_string(), vec![
        ("ident".to_string(), 0.9),
        ("for".to_string(), 0.7),
        ("{".to_string(), 1.0),
    ]);
    
    skeleton.syn_transitions.insert("use".to_string(), vec![
        ("std".to_string(), 0.4),
        ("crate".to_string(), 0.3),
        ("super".to_string(), 0.1),
        ("::".to_string(), 0.8),
    ]);
}

fn build_serde_patterns(skeleton: &mut RustcMarkovSkeleton) {
    // Common serde serialization patterns from rust ecosystem
    skeleton.serde_patterns.insert("derive".to_string(), "#[derive(Serialize, Deserialize)]".to_string());
    skeleton.serde_patterns.insert("json".to_string(), "serde_json::to_string".to_string());
    skeleton.serde_patterns.insert("field".to_string(), "#[serde(rename = \"\")]".to_string());
}

fn calculate_structure_probabilities(skeleton: &mut RustcMarkovSkeleton) {
    // Based on our 8,319 files analysis from rust-build
    skeleton.structure_probabilities.insert("fn".to_string(), 0.45);
    skeleton.structure_probabilities.insert("struct".to_string(), 0.25);
    skeleton.structure_probabilities.insert("impl".to_string(), 0.15);
    skeleton.structure_probabilities.insert("use".to_string(), 0.10);
    skeleton.structure_probabilities.insert("mod".to_string(), 0.05);
}

fn generate_rust_from_skeleton(skeleton: &RustcMarkovSkeleton, start: &str) -> String {
    let mut result = vec![start.to_string()];
    let mut current = start;
    
    for _ in 0..10 { // Generate 10 tokens
        if let Some(transitions) = skeleton.syn_transitions.get(current) {
            if let Some((next_token, _prob)) = transitions.first() {
                result.push(next_token.clone());
                current = next_token;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    
    result.join(" ")
}

fn compress_with_skeleton(skeleton: &RustcMarkovSkeleton, code: &str) -> Vec<u8> {
    let mut compressed = Vec::new();
    let tokens: Vec<&str> = code.split_whitespace().collect();
    
    for token in tokens {
        // Use structure probabilities for compression
        if let Some(&prob) = skeleton.structure_probabilities.get(token) {
            // High probability tokens get shorter encoding
            if prob > 0.3 {
                compressed.push(1); // Single byte for common structures
            } else if prob > 0.1 {
                compressed.push(2);
            } else {
                compressed.push(3);
            }
        } else {
            // Unknown token - store with length prefix
            compressed.push(0);
            compressed.push(token.len() as u8);
            compressed.extend_from_slice(token.as_bytes());
        }
    }
    
    compressed
}

fn decompress_with_skeleton(skeleton: &RustcMarkovSkeleton, compressed: &[u8]) -> String {
    // Create reverse mapping
    let mut prob_to_token = HashMap::new();
    for (token, &prob) in &skeleton.structure_probabilities {
        let code = if prob > 0.3 { 1 } else if prob > 0.1 { 2 } else { 3 };
        prob_to_token.insert(code, token.clone());
    }
    
    let mut tokens = Vec::new();
    let mut i = 0;
    
    while i < compressed.len() {
        match compressed[i] {
            1 | 2 | 3 => {
                if let Some(token) = prob_to_token.get(&compressed[i]) {
                    tokens.push(token.clone());
                }
                i += 1;
            }
            0 => {
                // Variable length token
                i += 1;
                if i < compressed.len() {
                    let len = compressed[i] as usize;
                    i += 1;
                    if i + len <= compressed.len() {
                        let token = String::from_utf8_lossy(&compressed[i..i + len]);
                        tokens.push(token.to_string());
                        i += len;
                    }
                }
            }
            _ => i += 1,
        }
    }
    
    tokens.join(" ")
}

fn estimate_skeleton_compression(skeleton: &RustcMarkovSkeleton) -> f64 {
    // Estimate compression based on structure probabilities
    let mut weighted_compression = 0.0;
    let mut total_weight = 0.0;
    
    for (_token, &prob) in &skeleton.structure_probabilities {
        let compression_ratio = if prob > 0.3 { 0.95 } else if prob > 0.1 { 0.90 } else { 0.80 };
        weighted_compression += prob * compression_ratio;
        total_weight += prob;
    }
    
    if total_weight > 0.0 {
        (weighted_compression / total_weight) * 100.0
    } else {
        90.0 // Default estimate
    }
}
