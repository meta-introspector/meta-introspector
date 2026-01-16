// Compression library loader and profiler
// Load compression .so files from nix store, profile them, find similar code

use std::collections::HashMap;
use libloading::{Library, Symbol};

#[derive(Debug, Clone)]
pub struct CompressionLibrary {
    pub name: String,
    pub nix_path: String,
    pub symbols: Vec<String>,
    pub profile: CompressionProfile,
}

#[derive(Debug, Clone)]
pub struct CompressionProfile {
    pub key_functions: Vec<String>,
    pub instruction_patterns: Vec<Vec<u8>>,
    pub complexity_signature: Vec<usize>,
}

/// Find compression libraries in nix store
pub fn find_compression_libs() -> Vec<String> {
    let mut libs = Vec::new();
    
    // Key library names
    let key_names = vec![
        "libz.so",      // zlib
        "liblzma.so",   // xz/lzma
        "libbz2.so",    // bzip2
        "libzstd.so",   // zstandard
        "liblz4.so",    // lz4
        "libbrotli",    // brotli (multiple .so files)
    ];
    
    // Search nix store
    let nix_store = "/nix/store";
    
    for entry in std::fs::read_dir(nix_store).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        
        // Look in lib/ subdirectory
        let lib_dir = path.join("lib");
        if lib_dir.exists() {
            for lib_entry in std::fs::read_dir(&lib_dir).unwrap() {
                let lib_entry = lib_entry.unwrap();
                let lib_path = lib_entry.path();
                let lib_name = lib_path.file_name().unwrap().to_str().unwrap();
                
                // Check if it matches key names
                for key in &key_names {
                    if lib_name.contains(key) {
                        libs.push(lib_path.to_str().unwrap().to_string());
                    }
                }
            }
        }
    }
    
    libs
}

/// Load compression library and extract symbols
pub fn load_compression_lib(path: &str) -> Result<CompressionLibrary, String> {
    unsafe {
        let lib = Library::new(path)
            .map_err(|e| format!("Failed to load {}: {}", path, e))?;
        
        // Extract symbols using nm
        let symbols = extract_symbols(path)?;
        
        // Build profile
        let profile = build_profile(path, &symbols)?;
        
        Ok(CompressionLibrary {
            name: extract_lib_name(path),
            nix_path: path.to_string(),
            symbols,
            profile,
        })
    }
}

fn extract_symbols(path: &str) -> Result<Vec<String>, String> {
    let output = std::process::Command::new("nm")
        .arg("-D")  // Dynamic symbols
        .arg(path)
        .output()
        .map_err(|e| format!("nm failed: {}", e))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    let symbols: Vec<String> = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 3 {
                Some(parts[2].to_string())
            } else {
                None
            }
        })
        .collect();
    
    Ok(symbols)
}

fn extract_lib_name(path: &str) -> String {
    std::path::Path::new(path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

/// Build compression profile from library
fn build_profile(path: &str, symbols: &[String]) -> Result<CompressionProfile, String> {
    // Identify key compression functions
    let key_functions = identify_key_functions(symbols);
    
    // Extract instruction patterns using objdump
    let instruction_patterns = extract_instruction_patterns(path, &key_functions)?;
    
    // Compute complexity signature
    let complexity_signature = compute_complexity_signature(&instruction_patterns);
    
    Ok(CompressionProfile {
        key_functions,
        instruction_patterns,
        complexity_signature,
    })
}

fn identify_key_functions(symbols: &[String]) -> Vec<String> {
    let mut key_funcs = Vec::new();
    
    // Common compression function patterns
    let patterns = vec![
        "compress",
        "decompress",
        "encode",
        "decode",
        "deflate",
        "inflate",
        "lz77",
        "huffman",
        "range",
        "entropy",
    ];
    
    for symbol in symbols {
        let lower = symbol.to_lowercase();
        for pattern in &patterns {
            if lower.contains(pattern) {
                key_funcs.push(symbol.clone());
                break;
            }
        }
    }
    
    key_funcs
}

fn extract_instruction_patterns(path: &str, functions: &[String]) -> Result<Vec<Vec<u8>>, String> {
    let mut patterns = Vec::new();
    
    // Use objdump to disassemble
    let output = std::process::Command::new("objdump")
        .arg("-d")
        .arg(path)
        .output()
        .map_err(|e| format!("objdump failed: {}", e))?;
    
    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // Extract instruction bytes for key functions
    for func in functions {
        if let Some(pattern) = extract_function_pattern(&stdout, func) {
            patterns.push(pattern);
        }
    }
    
    Ok(patterns)
}

fn extract_function_pattern(disasm: &str, func_name: &str) -> Option<Vec<u8>> {
    let mut in_function = false;
    let mut bytes = Vec::new();
    
    for line in disasm.lines() {
        if line.contains(func_name) {
            in_function = true;
            continue;
        }
        
        if in_function {
            // Parse instruction bytes
            if line.contains(':') {
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() >= 2 {
                    let hex_part = parts[1].split_whitespace().next()?;
                    if let Ok(byte) = u8::from_str_radix(hex_part, 16) {
                        bytes.push(byte);
                    }
                }
            }
            
            // Stop at next function
            if line.contains("Disassembly of section") {
                break;
            }
        }
        
        // Limit pattern size
        if bytes.len() >= 256 {
            break;
        }
    }
    
    if bytes.is_empty() {
        None
    } else {
        Some(bytes)
    }
}

fn compute_complexity_signature(patterns: &[Vec<u8>]) -> Vec<usize> {
    let mut signature = Vec::new();
    
    for pattern in patterns {
        // Count unique bytes
        let mut unique = std::collections::HashSet::new();
        for &byte in pattern {
            unique.insert(byte);
        }
        signature.push(unique.len());
        
        // Count repeated sequences
        let mut repeats = 0;
        for window in pattern.windows(4) {
            if pattern.windows(4).filter(|w| w == &window).count() > 1 {
                repeats += 1;
            }
        }
        signature.push(repeats);
    }
    
    signature
}

/// Find similar compression libraries using profiles
pub fn find_similar_libs(
    target_profile: &CompressionProfile,
    all_libs: &[CompressionLibrary]
) -> Vec<(String, f64)> {
    let mut similarities = Vec::new();
    
    for lib in all_libs {
        let similarity = compute_profile_similarity(target_profile, &lib.profile);
        if similarity > 0.5 {
            similarities.push((lib.name.clone(), similarity));
        }
    }
    
    // Sort by similarity
    similarities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    similarities
}

fn compute_profile_similarity(p1: &CompressionProfile, p2: &CompressionProfile) -> f64 {
    // Compare key functions
    let func_overlap = p1.key_functions.iter()
        .filter(|f| p2.key_functions.contains(f))
        .count() as f64;
    let func_total = (p1.key_functions.len() + p2.key_functions.len()) as f64;
    let func_sim = if func_total > 0.0 { func_overlap / func_total } else { 0.0 };
    
    // Compare instruction patterns
    let mut pattern_sim = 0.0;
    for pat1 in &p1.instruction_patterns {
        for pat2 in &p2.instruction_patterns {
            pattern_sim += pattern_similarity(pat1, pat2);
        }
    }
    if !p1.instruction_patterns.is_empty() && !p2.instruction_patterns.is_empty() {
        pattern_sim /= (p1.instruction_patterns.len() * p2.instruction_patterns.len()) as f64;
    }
    
    // Compare complexity signatures
    let sig_sim = signature_similarity(&p1.complexity_signature, &p2.complexity_signature);
    
    // Weighted average
    (func_sim * 0.3 + pattern_sim * 0.4 + sig_sim * 0.3)
}

fn pattern_similarity(p1: &[u8], p2: &[u8]) -> f64 {
    let min_len = p1.len().min(p2.len());
    if min_len == 0 { return 0.0; }
    
    let matches = p1.iter().zip(p2.iter())
        .take(min_len)
        .filter(|(a, b)| a == b)
        .count();
    
    matches as f64 / min_len as f64
}

fn signature_similarity(s1: &[usize], s2: &[usize]) -> f64 {
    let min_len = s1.len().min(s2.len());
    if min_len == 0 { return 0.0; }
    
    let mut diff_sum = 0.0;
    for i in 0..min_len {
        let diff = (s1[i] as f64 - s2[i] as f64).abs();
        diff_sum += diff;
    }
    
    1.0 - (diff_sum / (min_len as f64 * 256.0))
}

/// Scan nix store for compression libraries
pub fn scan_nix_store_for_compression() -> Vec<CompressionLibrary> {
    let mut libs = Vec::new();
    
    println!("🔍 Scanning nix store for compression libraries...");
    
    let lib_paths = find_compression_libs();
    println!("   Found {} potential libraries", lib_paths.len());
    
    for path in lib_paths {
        match load_compression_lib(&path) {
            Ok(lib) => {
                println!("   ✅ Loaded: {} ({} symbols)", lib.name, lib.symbols.len());
                libs.push(lib);
            }
            Err(e) => {
                println!("   ❌ Failed: {}", e);
            }
        }
    }
    
    libs
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_find_libs() {
        let libs = find_compression_libs();
        assert!(libs.len() > 0);
    }
    
    #[test]
    fn test_extract_lib_name() {
        let name = extract_lib_name("/nix/store/abc-zlib-1.2.11/lib/libz.so.1");
        assert_eq!(name, "libz.so.1");
    }
}
