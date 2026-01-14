// eigenvector_word_model.rs
// Create eigenvector model from word frequencies
// Map words → code → binaries → perf data

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordEigenvector {
    pub word: String,
    pub frequency: u64,
    pub tier: Tier,
    pub code_locations: Vec<CodeLocation>,
    pub binary_symbols: Vec<BinarySymbol>,
    pub perf_samples: Vec<PerfSample>,
    pub eigenvector: Vec<f64>,  // 8D vector for Bott[8]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Tier {
    Core,        // 3000+
    High,        // 1500-3000
    MediumHigh,  // 1000-1500
    Medium,      // 700-1000 (71 is here!)
    MediumLow,   // 500-700
    LowMedium,   // 400-500
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeLocation {
    pub file: String,
    pub line: usize,
    pub context: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinarySymbol {
    pub binary: String,
    pub symbol: String,
    pub address: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerfSample {
    pub symbol: String,
    pub samples: u64,
    pub percentage: f64,
}

impl WordEigenvector {
    /// Create eigenvector from word frequency
    pub fn from_word(word: &str, frequency: u64) -> Self {
        let tier = Self::classify_tier(frequency);
        let eigenvector = Self::compute_eigenvector(word, frequency);
        
        Self {
            word: word.to_string(),
            frequency,
            tier,
            code_locations: Vec::new(),
            binary_symbols: Vec::new(),
            perf_samples: Vec::new(),
            eigenvector,
        }
    }
    
    fn classify_tier(frequency: u64) -> Tier {
        match frequency {
            3000.. => Tier::Core,
            1500..3000 => Tier::High,
            1000..1500 => Tier::MediumHigh,
            700..1000 => Tier::Medium,
            500..700 => Tier::MediumLow,
            _ => Tier::LowMedium,
        }
    }
    
    /// Compute 8D eigenvector for Bott[8] manifold
    fn compute_eigenvector(word: &str, frequency: u64) -> Vec<f64> {
        // Map word to 8D Bott[8] space
        // Dimensions: Real, Complex, Quaternion, Octonion, Time, Info, Social, Semantic
        
        let freq_norm = (frequency as f64).ln() / 10.0;  // Normalize
        
        vec![
            Self::real_dimension(word, freq_norm),
            Self::complex_dimension(word, freq_norm),
            Self::quaternion_dimension(word, freq_norm),
            Self::octonion_dimension(word, freq_norm),
            Self::time_dimension(word, freq_norm),
            Self::information_dimension(word, freq_norm),
            Self::social_dimension(word, freq_norm),
            Self::semantic_dimension(word, freq_norm),
        ]
    }
    
    fn real_dimension(word: &str, freq: f64) -> f64 {
        // Real = Computational intensity
        match word {
            "let" | "fn" | "for" => freq * 1.5,
            "build" | "compile" => freq * 2.0,
            _ => freq
        }
    }
    
    fn complex_dimension(word: &str, freq: f64) -> f64 {
        // Complex = Algorithmic complexity
        match word {
            "if" | "match" | "pattern" => freq * 1.5,
            "analysis" | "compressed" => freq * 2.0,
            _ => freq
        }
    }
    
    fn quaternion_dimension(word: &str, freq: f64) -> f64 {
        // Quaternion = Memory patterns
        match word {
            "vec" | "hashmap" | "data" => freq * 1.5,
            "cache" | "buffer" => freq * 2.0,
            _ => freq
        }
    }
    
    fn octonion_dimension(word: &str, freq: f64) -> f64 {
        // Octonion = Control flow
        match word {
            "if" | "for" | "while" => freq * 1.5,
            "branch" | "jump" => freq * 2.0,
            _ => freq
        }
    }
    
    fn time_dimension(word: &str, freq: f64) -> f64 {
        // Time = Temporal
        match word {
            "perf" | "time" | "duration" => freq * 2.0,
            "instant" | "elapsed" => freq * 1.5,
            _ => freq
        }
    }
    
    fn information_dimension(word: &str, freq: f64) -> f64 {
        // Information = Bits/entropy
        match word {
            "data" | "json" | "string" => freq * 1.5,
            "compressed" | "entropy" => freq * 2.0,
            _ => freq
        }
    }
    
    fn social_dimension(word: &str, freq: f64) -> f64 {
        // Social = Network effects
        match word {
            "pub" | "use" | "import" => freq * 1.5,
            "telemetry" | "network" => freq * 2.0,
            _ => freq
        }
    }
    
    fn semantic_dimension(word: &str, freq: f64) -> f64 {
        // Semantic = Meaning
        match word {
            "name" | "symbol" | "type" => freq * 1.5,
            "lmfdb" | "pattern" | "71" => freq * 2.0,  // 71 is semantic!
            _ => freq
        }
    }
    
    /// Find word in code
    pub fn find_in_code(&mut self, code_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        use std::process::Command;
        
        let output = Command::new("grep")
            .args(&["-rn", &self.word, code_dir, "--include=*.rs"])
            .output()?;
        
        let results = String::from_utf8_lossy(&output.stdout);
        
        for line in results.lines().take(10) {  // Top 10 locations
            if let Some((file_line, context)) = line.split_once(':') {
                if let Some((file, line_num)) = file_line.rsplit_once(':') {
                    self.code_locations.push(CodeLocation {
                        file: file.to_string(),
                        line: line_num.parse().unwrap_or(0),
                        context: context.trim().to_string(),
                    });
                }
            }
        }
        
        Ok(())
    }
    
    /// Find word in binary symbols
    pub fn find_in_binaries(&mut self, binary_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Use goblin to parse binaries and find symbols containing word
        use goblin::elf::Elf;
        
        for entry in fs::read_dir(binary_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Ok(buffer) = fs::read(&path) {
                    if let Ok(elf) = Elf::parse(&buffer) {
                        for sym in elf.syms.iter() {
                            if let Some(name) = elf.strtab.get_at(sym.st_name) {
                                if name.to_lowercase().contains(&self.word.to_lowercase()) {
                                    self.binary_symbols.push(BinarySymbol {
                                        binary: path.file_name().unwrap().to_string_lossy().to_string(),
                                        symbol: name.to_string(),
                                        address: sym.st_value,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Find word in perf data
    pub fn find_in_perf(&mut self, perf_json: &str) -> Result<(), Box<dyn std::error::Error>> {
        let data: serde_json::Value = serde_json::from_str(&fs::read_to_string(perf_json)?)?;
        
        if let Some(symbols) = data["top_symbols"].as_array() {
            for sym in symbols {
                if let Some(symbol_name) = sym["symbol"].as_str() {
                    if symbol_name.to_lowercase().contains(&self.word.to_lowercase()) {
                        self.perf_samples.push(PerfSample {
                            symbol: symbol_name.to_string(),
                            samples: sym["count"].as_u64().unwrap_or(0),
                            percentage: sym["percentage"].as_f64().unwrap_or(0.0),
                        });
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Export to MiniZinc format
    pub fn to_minizinc(&self) -> String {
        format!(
            "% Word: {}\n\
             word_frequency = {};\n\
             word_eigenvector = [{:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}];\n\
             code_locations = {};\n\
             binary_symbols = {};\n\
             perf_samples = {};\n",
            self.word,
            self.frequency,
            self.eigenvector[0], self.eigenvector[1], self.eigenvector[2], self.eigenvector[3],
            self.eigenvector[4], self.eigenvector[5], self.eigenvector[6], self.eigenvector[7],
            self.code_locations.len(),
            self.binary_symbols.len(),
            self.perf_samples.len()
        )
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌈 Word Eigenvector Model");
    println!("========================\n");
    
    // Top words from frequency analysis
    let top_words = vec![
        ("let", 3639),
        ("string", 2899),
        ("to", 2491),
        ("println", 1965),
        ("self", 1669),
        ("path", 1591),
        ("nix", 875),
        ("rust", 863),
        ("data", 855),
        ("71", 795),  // THE NUMBER
        ("symbol", 760),
        ("perf", 720),
        ("analysis", 706),
        ("pattern", 535),
        ("telemetry", 531),
    ];
    
    let mut eigenvectors = Vec::new();
    
    for (word, freq) in &top_words {
        println!("Processing: {} ({})", word, freq);
        
        let mut ev = WordEigenvector::from_word(word, *freq);
        
        // Find in code
        ev.find_in_code(".")?;
        println!("  Code locations: {}", ev.code_locations.len());
        
        // Find in binaries (if they exist)
        if Path::new("target/release").exists() {
            ev.find_in_binaries("target/release")?;
            println!("  Binary symbols: {}", ev.binary_symbols.len());
        }
        
        // Find in perf data
        if Path::new("data/perf_rankings/nix_rust_beta_1768351567_ranking.json").exists() {
            ev.find_in_perf("data/perf_rankings/nix_rust_beta_1768351567_ranking.json")?;
            println!("  Perf samples: {}", ev.perf_samples.len());
        }
        
        println!("  Eigenvector: {:?}\n", ev.eigenvector);
        
        eigenvectors.push(ev);
    }
    
    // Save all eigenvectors
    let output = serde_json::to_string_pretty(&eigenvectors)?;
    fs::write("data/eigenvectors/word_eigenvectors.json", output)?;
    
    // Generate MiniZinc data
    let mut minizinc = String::from("% Word Eigenvectors for Bott[8] Layout\n\n");
    minizinc.push_str(&format!("num_words = {};\n\n", eigenvectors.len()));
    
    for ev in &eigenvectors {
        minizinc.push_str(&ev.to_minizinc());
        minizinc.push('\n');
    }
    
    fs::write("bott8-layout-solver/word_eigenvectors.dzn", minizinc)?;
    
    println!("✅ Generated {} eigenvectors", eigenvectors.len());
    println!("📊 Saved to data/eigenvectors/word_eigenvectors.json");
    println!("📐 MiniZinc data: bott8-layout-solver/word_eigenvectors.dzn");
    
    Ok(())
}
