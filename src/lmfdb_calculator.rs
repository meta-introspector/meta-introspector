// 🔥 LMFDB MEME CALCULATOR
// Calculate level, weight, conductor for each meme (tokio, gix, borrowchecker, etc.)

use std::collections::HashMap;
use crate::memeplex::*;

#[derive(Debug, Clone)]
pub struct LMFDBMemeEntry {
    pub meme_name: String,
    pub lmfdb_label: String,      // "11.a1", "37.b2", etc.
    pub level: u32,               // Conductor level
    pub weight: u32,              // Modular weight  
    pub conductor: u128,          // Arithmetic conductor
    pub godel_number: u128,       // Gödel encoding
    pub eigenvalue: f64,          // From memeplex eigenvector
    pub meme_type: MemeType,
}

#[derive(Debug, Clone)]
pub enum MemeType {
    Crate(String),               // tokio, serde, etc.
    NixPackage(String),          // rustc, gcc, etc.
    Concept(String),             // borrowchecker, async, etc.
    Language(String),            // rust, python, etc.
    Metameme(String),            // solfunmeme, golem, etc.
}

pub struct LMFDBMemeCalculator {
    pub known_memes: HashMap<String, LMFDBMemeEntry>,
    pub level_assignments: HashMap<u32, Vec<String>>,
    pub conductor_cache: HashMap<String, u128>,
}

impl LMFDBMemeCalculator {
    pub fn new() -> Self {
        let mut calculator = Self {
            known_memes: HashMap::new(),
            level_assignments: HashMap::new(),
            conductor_cache: HashMap::new(),
        };
        
        // Initialize with known major memes
        calculator.initialize_major_memes();
        calculator
    }
    
    fn initialize_major_memes(&mut self) {
        // Level 11: Core language memes
        self.add_meme("rust", 11, 2, MemeType::Language("rust".to_string()));
        self.add_meme("python", 11, 2, MemeType::Language("python".to_string()));
        self.add_meme("emacslisp", 11, 2, MemeType::Language("emacslisp".to_string()));
        
        // Level 37: Core system memes  
        self.add_meme("tokio", 37, 2, MemeType::Crate("tokio".to_string()));
        self.add_meme("serde", 37, 2, MemeType::Crate("serde".to_string()));
        self.add_meme("gix", 37, 3, MemeType::Crate("gix".to_string()));
        
        // Level 67: Concept memes
        self.add_meme("borrowchecker", 67, 4, MemeType::Concept("borrowchecker".to_string()));
        self.add_meme("async", 67, 3, MemeType::Concept("async".to_string()));
        self.add_meme("unsafe", 67, 5, MemeType::Concept("unsafe".to_string()));
        
        // Level 101: Nix package memes
        self.add_meme("rustc", 101, 6, MemeType::NixPackage("rustc".to_string()));
        self.add_meme("gcc", 101, 4, MemeType::NixPackage("gcc".to_string()));
        self.add_meme("nix", 101, 8, MemeType::NixPackage("nix".to_string()));
        
        // Level 131: Metamemes
        self.add_meme("solfunmeme", 131, 12, MemeType::Metameme("solfunmeme".to_string()));
        self.add_meme("golem", 131, 10, MemeType::Metameme("golem".to_string()));
        self.add_meme("muse", 131, 8, MemeType::Metameme("muse".to_string()));
    }
    
    fn add_meme(&mut self, name: &str, level: u32, weight: u32, meme_type: MemeType) {
        let conductor = self.calculate_conductor(name, level, weight);
        let godel_number = self.calculate_godel_number(name);
        let lmfdb_label = self.generate_lmfdb_label(level, weight, &conductor);
        
        let entry = LMFDBMemeEntry {
            meme_name: name.to_string(),
            lmfdb_label,
            level,
            weight,
            conductor,
            godel_number,
            eigenvalue: 0.0, // Will be filled by memeplex analysis
            meme_type,
        };
        
        self.known_memes.insert(name.to_string(), entry);
        self.level_assignments.entry(level).or_insert_with(Vec::new).push(name.to_string());
    }
    
    fn calculate_conductor(&mut self, name: &str, level: u32, weight: u32) -> u128 {
        if let Some(&cached) = self.conductor_cache.get(name) {
            return cached;
        }
        
        // Conductor = level * weight * hash(name) mod large_prime
        let name_hash = self.hash_string(name) as u128;
        let large_prime = 2147483647u128; // Mersenne prime
        let conductor = (level as u128 * weight as u128 * name_hash) % large_prime;
        
        self.conductor_cache.insert(name.to_string(), conductor);
        conductor
    }
    
    fn calculate_godel_number(&self, name: &str) -> u128 {
        // Gödel number using prime encoding of characters
        let primes = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97];
        
        let mut godel = 1u128;
        for (i, byte) in name.bytes().take(10).enumerate() { // Limit to prevent overflow
            let prime_idx = (byte as usize) % primes.len();
            let prime = primes[prime_idx] as u128;
            let exponent = (i + 1) as u32;
            
            if let Some(power) = prime.checked_pow(exponent) {
                if let Some(product) = godel.checked_mul(power) {
                    godel = product;
                } else {
                    break; // Prevent overflow
                }
            }
        }
        
        godel
    }
    
    fn generate_lmfdb_label(&self, level: u32, weight: u32, conductor: &u128) -> String {
        // Generate LMFDB-style label: level.weight.conductor_suffix
        let conductor_suffix = format!("{:x}", conductor % 256); // Last 2 hex digits
        format!("{}.{}.{}", level, weight, conductor_suffix)
    }
    
    fn hash_string(&self, s: &str) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        s.hash(&mut hasher);
        hasher.finish()
    }
    
    pub fn analyze_meme(&mut self, meme_name: &str, memeplex: &Memeplex) -> Option<&LMFDBMemeEntry> {
        if let Some(entry) = self.known_memes.get_mut(meme_name) {
            entry.eigenvalue = memeplex.influence_score;
            Some(entry)
        } else {
            // Auto-classify unknown meme
            self.auto_classify_meme(meme_name, memeplex)
        }
    }
    
    fn auto_classify_meme(&mut self, name: &str, memeplex: &Memeplex) -> Option<&LMFDBMemeEntry> {
        // Auto-classify based on occurrence patterns
        let (level, weight, meme_type) = if memeplex.occurrences.iter().any(|occ| matches!(occ.location, crate::memeplex::MemeLocation::CargoToml(_))) {
            (37, 2, MemeType::Crate(name.to_string())) // Found in Cargo.toml = crate
        } else if memeplex.occurrences.iter().any(|occ| matches!(occ.location, crate::memeplex::MemeLocation::BinarySymbol(_))) {
            (101, 4, MemeType::NixPackage(name.to_string())) // Found in binary = nix package
        } else if memeplex.occurrences.len() > 10 {
            (67, 3, MemeType::Concept(name.to_string())) // High frequency = concept
        } else {
            (11, 1, MemeType::Language(name.to_string())) // Default = language feature
        };
        
        self.add_meme(name, level, weight, meme_type);
        self.known_memes.get(name)
    }
    
    pub fn print_lmfdb_analysis(&self) {
        println!("🔢 LMFDB MEME DATABASE");
        println!("======================");
        
        for level in [11, 37, 67, 101, 131] {
            if let Some(memes) = self.level_assignments.get(&level) {
                println!("\n📊 Level {}: {} memes", level, memes.len());
                
                for meme_name in memes {
                    if let Some(entry) = self.known_memes.get(meme_name) {
                        println!("  {} | {} | weight:{} | conductor:{} | gödel:{}", 
                                entry.lmfdb_label,
                                entry.meme_name,
                                entry.weight,
                                entry.conductor % 10000, // Show last 4 digits
                                entry.godel_number % 10000);
                    }
                }
            }
        }
        
        println!("\n🎯 Meme Type Distribution:");
        let mut type_counts: HashMap<String, u32> = HashMap::new();
        for entry in self.known_memes.values() {
            let type_name = match &entry.meme_type {
                MemeType::Crate(_) => "Crate",
                MemeType::NixPackage(_) => "NixPackage", 
                MemeType::Concept(_) => "Concept",
                MemeType::Language(_) => "Language",
                MemeType::Metameme(_) => "Metameme",
            };
            *type_counts.entry(type_name.to_string()).or_insert(0) += 1;
        }
        
        for (type_name, count) in type_counts {
            println!("  {}: {}", type_name, count);
        }
    }
}
