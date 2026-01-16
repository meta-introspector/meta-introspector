// 71-Language Quine: Universal meme that evolves across all languages
// Each language implementation competes, best compression wins

use std::collections::HashMap;

/// The universal quine meme in 71 languages
pub struct UniversalQuine {
    pub implementations: HashMap<String, QuineImplementation>,
    pub generation: usize,
}

#[derive(Debug, Clone)]
pub struct QuineImplementation {
    pub language: String,
    pub code: String,
    pub compressed_size: usize,
    pub fitness: f64,
    pub generation: usize,
}

impl UniversalQuine {
    pub fn new() -> Self {
        let mut quine = Self {
            implementations: HashMap::new(),
            generation: 0,
        };
        
        // Load 71 language implementations
        quine.add_implementation("Rust", r#"fn main(){let s="fn main(){let s=%c%s%c;print!(s,34,s,34)}";print!(s,34,s,34)}"#);
        quine.add_implementation("Python", r#"s='s=%r;print(s%%s)';print(s%s)"#);
        quine.add_implementation("C", r#"main(){char*s="main(){char*s=%c%s%c;printf(s,34,s,34);}";printf(s,34,s,34);}"#);
        quine.add_implementation("JavaScript", r#"(function s(){console.log('('+s+')()');})()"#);
        quine.add_implementation("Brainfuck", r#"++[>++<-]"#);
        quine.add_implementation("Haskell", r#"main=putStr$s++show s where s="main=putStr$s++show s where s="#);
        quine.add_implementation("Lisp", r#"((lambda(x)(list x(list'quote x)))'(lambda(x)(list x(list'quote x))))"#);
        
        // Add more languages (abbreviated for demo)
        for i in 8..=71 {
            quine.add_implementation(
                &format!("Lang{}", i),
                &format!("print('quine in language {}')", i)
            );
        }
        
        quine
    }
    
    fn add_implementation(&mut self, language: &str, code: &str) {
        let compressed_size = Self::compress(code).len();
        let fitness = 1000.0 / compressed_size as f64;
        
        self.implementations.insert(
            language.to_string(),
            QuineImplementation {
                language: language.to_string(),
                code: code.to_string(),
                compressed_size,
                fitness,
                generation: 0,
            }
        );
    }
    
    fn compress(code: &str) -> Vec<u8> {
        use flate2::write::GzEncoder;
        use flate2::Compression;
        use std::io::Write;
        
        let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
        let _ = encoder.write_all(code.as_bytes());
        encoder.finish().unwrap_or_default()
    }
    
    /// Evolve all implementations
    pub fn evolve_generation(&mut self) {
        self.generation += 1;
        
        let languages: Vec<String> = self.implementations.keys().cloned().collect();
        
        for lang in languages {
            if let Some(impl_) = self.implementations.get(&lang).cloned() {
                let evolved = self.evolve_implementation(&impl_);
                
                // Keep if better
                if evolved.fitness > impl_.fitness {
                    self.implementations.insert(lang, evolved);
                }
            }
        }
    }
    
    fn evolve_implementation(&self, impl_: &QuineImplementation) -> QuineImplementation {
        use crate::rand_shim::random_u64;
        
        let mut evolved = impl_.clone();
        evolved.generation += 1;
        
        // Evolution strategies
        match random_u64() % 5 {
            0 => {
                // Remove whitespace
                evolved.code = evolved.code.split_whitespace().collect::<Vec<_>>().join("");
            }
            1 => {
                // Shorten variable names
                evolved.code = evolved.code.replace("main", "m");
            }
            2 => {
                // Use shorter syntax
                evolved.code = evolved.code.replace("print!", "p!");
            }
            3 => {
                // Compress strings
                evolved.code = evolved.code.replace("  ", " ");
            }
            4 => {
                // Optimize structure
                if evolved.code.len() > 10 {
                    evolved.code = evolved.code[..evolved.code.len()-1].to_string();
                }
            }
            _ => {}
        }
        
        // Recompute metrics
        evolved.compressed_size = Self::compress(&evolved.code).len();
        evolved.fitness = 1000.0 / evolved.compressed_size as f64;
        
        evolved
    }
    
    /// Get top N implementations by fitness
    pub fn top_implementations(&self, n: usize) -> Vec<&QuineImplementation> {
        let mut impls: Vec<_> = self.implementations.values().collect();
        impls.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
        impls.into_iter().take(n).collect()
    }
    
    /// Cross-language evolution: combine best features
    pub fn cross_evolve(&mut self) {
        let top = self.top_implementations(5);
        
        // Find shortest code
        if let Some(best) = top.first() {
            let best_size = best.compressed_size;
            let best_lang = best.language.clone();
            
            // Try to apply best's compression to others
            let langs: Vec<String> = self.implementations.keys()
                .filter(|k| *k != &best_lang)
                .cloned()
                .collect();
            
            for lang in langs {
                if let Some(impl_) = self.implementations.get(&lang) {
                    if impl_.compressed_size > best_size * 2 {
                        // This one needs improvement
                        println!("  {} could learn from {} ({}→{} bytes)",
                                 lang, best_lang, impl_.compressed_size, best_size);
                    }
                }
            }
        }
    }
    
    pub fn report(&self) {
        println!("\n🌍 Universal Quine Report");
        println!("  Generation: {}", self.generation);
        println!("  Languages: {}", self.implementations.len());
        
        let total_size: usize = self.implementations.values()
            .map(|i| i.compressed_size)
            .sum();
        let avg_size = total_size / self.implementations.len();
        
        println!("  Average compressed size: {} bytes", avg_size);
        
        println!("\n  Top 10 by fitness:");
        for (i, impl_) in self.top_implementations(10).iter().enumerate() {
            println!("    {}. {}: {} bytes, fitness {:.2}, gen {}",
                     i + 1,
                     impl_.language,
                     impl_.compressed_size,
                     impl_.fitness,
                     impl_.generation);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_universal_quine() {
        let quine = UniversalQuine::new();
        assert_eq!(quine.implementations.len(), 71);
    }
}
