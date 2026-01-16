// Languages as Market Makers: Each language buys a different spectrum
// Rust buys complex AST spectrum, Brainfuck buys minimal spectrum

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LanguageSpectrum {
    pub name: String,
    pub min_complexity: usize,
    pub max_complexity: usize,
    pub node_types: Vec<String>,  // AST nodes this language supports
    pub balance: u64,
    pub bought_spectrums: Vec<u64>,  // Spectrum IDs
}

impl LanguageSpectrum {
    /// Rust: Buys complex, rich AST spectrums
    pub fn rust() -> Self {
        Self {
            name: "Rust".to_string(),
            min_complexity: 10,
            max_complexity: 1000,
            node_types: vec![
                "Fn".to_string(),
                "Struct".to_string(),
                "Impl".to_string(),
                "Trait".to_string(),
                "Enum".to_string(),
                "Mod".to_string(),
                "Macro".to_string(),
                "Generic".to_string(),
                "Lifetime".to_string(),
            ],
            balance: 1000000,
            bought_spectrums: Vec::new(),
        }
    }
    
    /// Brainfuck: Buys minimal, simple spectrums
    pub fn brainfuck() -> Self {
        Self {
            name: "Brainfuck".to_string(),
            min_complexity: 1,
            max_complexity: 10,
            node_types: vec![
                "Loop".to_string(),
                "Inc".to_string(),
                "Dec".to_string(),
                "Move".to_string(),
                "Input".to_string(),
                "Output".to_string(),
            ],
            balance: 10000,
            bought_spectrums: Vec::new(),
        }
    }
    
    /// C: Buys procedural spectrums
    pub fn c() -> Self {
        Self {
            name: "C".to_string(),
            min_complexity: 5,
            max_complexity: 100,
            node_types: vec![
                "Function".to_string(),
                "Struct".to_string(),
                "Pointer".to_string(),
                "Array".to_string(),
                "Loop".to_string(),
            ],
            balance: 500000,
            bought_spectrums: Vec::new(),
        }
    }
    
    /// Python: Buys dynamic, high-level spectrums
    pub fn python() -> Self {
        Self {
            name: "Python".to_string(),
            min_complexity: 3,
            max_complexity: 200,
            node_types: vec![
                "Def".to_string(),
                "Class".to_string(),
                "Lambda".to_string(),
                "Comprehension".to_string(),
                "Decorator".to_string(),
            ],
            balance: 750000,
            bought_spectrums: Vec::new(),
        }
    }
    
    /// Haskell: Buys functional, algebraic spectrums
    pub fn haskell() -> Self {
        Self {
            name: "Haskell".to_string(),
            min_complexity: 8,
            max_complexity: 500,
            node_types: vec![
                "TypeClass".to_string(),
                "ADT".to_string(),
                "Pattern".to_string(),
                "Monad".to_string(),
                "Functor".to_string(),
            ],
            balance: 800000,
            bought_spectrums: Vec::new(),
        }
    }
    
    /// Check if this language wants to buy this spectrum
    pub fn wants_spectrum(&self, spectrum: &CodeSpectrum) -> bool {
        // Check complexity range
        if spectrum.complexity < self.min_complexity || spectrum.complexity > self.max_complexity {
            return false;
        }
        
        // Check if spectrum has nodes we support
        for node in &spectrum.node_counts {
            if self.node_types.contains(&node.0) && *node.1 > 0 {
                return true;
            }
        }
        
        false
    }
    
    /// Calculate bid price for spectrum
    pub fn bid_price(&self, spectrum: &CodeSpectrum) -> u64 {
        if !self.wants_spectrum(spectrum) {
            return 0;
        }
        
        // Price based on how well spectrum matches our preferences
        let mut price = spectrum.complexity as u64 * 100;
        
        // Bonus for matching node types
        let matching_nodes = spectrum.node_counts.iter()
            .filter(|(node, count)| self.node_types.contains(node) && **count > 0)
            .count();
        
        price += matching_nodes as u64 * 500;
        
        // Can we afford it?
        price.min(self.balance)
    }
    
    /// Buy spectrum
    pub fn buy(&mut self, spectrum_id: u64, price: u64) -> bool {
        if self.balance >= price {
            self.balance -= price;
            self.bought_spectrums.push(spectrum_id);
            true
        } else {
            false
        }
    }
}

/// Code spectrum for sale
#[derive(Debug, Clone)]
pub struct CodeSpectrum {
    pub id: u64,
    pub source: String,
    pub complexity: usize,
    pub node_counts: HashMap<String, usize>,
    pub compressed_size: usize,
    pub owner: String,
    pub price: u64,
}

impl CodeSpectrum {
    pub fn from_source(id: u64, source: String) -> Self {
        // Parse and analyze
        let complexity = source.lines().count();
        let mut node_counts = HashMap::new();
        
        // Simple heuristics
        node_counts.insert("Fn".to_string(), source.matches("fn ").count());
        node_counts.insert("Struct".to_string(), source.matches("struct ").count());
        node_counts.insert("Impl".to_string(), source.matches("impl ").count());
        node_counts.insert("Trait".to_string(), source.matches("trait ").count());
        node_counts.insert("Enum".to_string(), source.matches("enum ").count());
        node_counts.insert("Loop".to_string(), source.matches("for ").count() + source.matches("while ").count());
        
        // Compress
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;
        
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        let _ = encoder.write_all(source.as_bytes());
        let compressed_size = encoder.finish().map(|v| v.len()).unwrap_or(0);
        
        Self {
            id,
            source,
            complexity,
            node_counts,
            compressed_size,
            owner: "market".to_string(),
            price: complexity as u64 * 100,
        }
    }
}

/// Spectrum marketplace: Languages compete to buy code
pub struct SpectrumMarketplace {
    pub languages: Vec<LanguageSpectrum>,
    pub spectrums: Vec<CodeSpectrum>,
    pub trades: Vec<Trade>,
}

#[derive(Debug, Clone)]
pub struct Trade {
    pub language: String,
    pub spectrum_id: u64,
    pub price: u64,
    pub round: usize,
}

impl SpectrumMarketplace {
    pub fn new() -> Self {
        Self {
            languages: vec![
                LanguageSpectrum::rust(),
                LanguageSpectrum::brainfuck(),
                LanguageSpectrum::c(),
                LanguageSpectrum::python(),
                LanguageSpectrum::haskell(),
            ],
            spectrums: Vec::new(),
            trades: Vec::new(),
        }
    }
    
    /// Add code spectrum for sale
    pub fn list_spectrum(&mut self, source: String) -> u64 {
        let id = self.spectrums.len() as u64;
        let spectrum = CodeSpectrum::from_source(id, source);
        self.spectrums.push(spectrum);
        id
    }
    
    /// Run auction: languages bid on spectrums
    pub fn run_auction(&mut self, round: usize) {
        println!("\n💰 Auction Round {}", round);
        
        for spectrum in &self.spectrums {
            if spectrum.owner != "market" {
                continue;  // Already sold
            }
            
            // Collect bids
            let mut bids: Vec<(usize, u64)> = Vec::new();
            
            for (i, lang) in self.languages.iter().enumerate() {
                let bid = lang.bid_price(spectrum);
                if bid > 0 {
                    bids.push((i, bid));
                }
            }
            
            if bids.is_empty() {
                continue;
            }
            
            // Highest bidder wins
            bids.sort_by_key(|b| b.1);
            let (winner_idx, winning_bid) = bids.last().unwrap();
            
            if self.languages[*winner_idx].buy(spectrum.id, *winning_bid) {
                println!("  {} bought spectrum {} for {} coins (complexity: {})",
                         self.languages[*winner_idx].name,
                         spectrum.id,
                         winning_bid,
                         spectrum.complexity);
                
                self.trades.push(Trade {
                    language: self.languages[*winner_idx].name.clone(),
                    spectrum_id: spectrum.id,
                    price: *winning_bid,
                    round,
                });
            }
        }
    }
    
    pub fn report(&self) {
        println!("\n📊 Spectrum Marketplace Report");
        println!("  Total spectrums: {}", self.spectrums.len());
        println!("  Total trades: {}", self.trades.len());
        
        println!("\n  Language Portfolios:");
        for lang in &self.languages {
            println!("    {}: {} spectrums bought, {} coins left",
                     lang.name,
                     lang.bought_spectrums.len(),
                     lang.balance);
        }
        
        println!("\n  Top 5 Trades:");
        let mut sorted_trades = self.trades.clone();
        sorted_trades.sort_by_key(|t| t.price);
        sorted_trades.reverse();
        
        for (i, trade) in sorted_trades.iter().take(5).enumerate() {
            println!("    {}. {} paid {} for spectrum {}",
                     i + 1,
                     trade.language,
                     trade.price,
                     trade.spectrum_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_language_preferences() {
        let rust = LanguageSpectrum::rust();
        let bf = LanguageSpectrum::brainfuck();
        
        let complex_code = "fn main() { struct Point { x: i32 } impl Point {} }".to_string();
        let simple_code = "++[>++<-]".to_string();
        
        let complex_spectrum = CodeSpectrum::from_source(0, complex_code);
        let simple_spectrum = CodeSpectrum::from_source(1, simple_code);
        
        assert!(rust.wants_spectrum(&complex_spectrum));
        assert!(!rust.wants_spectrum(&simple_spectrum));
        
        assert!(!bf.wants_spectrum(&complex_spectrum));
        assert!(bf.wants_spectrum(&simple_spectrum));
    }
}
