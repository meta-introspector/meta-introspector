// 🔥 SOURCE-TO-BINARY MARKOV MAPPER
// Maps source code to binary via char/word Markov models and AST compilation tracking

use std::collections::HashMap;
use syn::{parse_file, visit::Visit};

pub struct SourceBinaryMapper {
    pub source_models: HashMap<String, SourceMarkovModel>,
    pub binary_models: HashMap<String, BinaryMarkovModel>,
    pub compilation_map: HashMap<String, CompilationLink>,
}

pub struct SourceMarkovModel {
    pub file_path: String,
    pub char_transitions: HashMap<char, HashMap<char, u32>>,     // Char-level
    pub word_transitions: HashMap<String, HashMap<String, u32>>, // Word-level
    pub token_patterns: HashMap<String, u32>,                   // AST tokens
    pub max_word_length: usize,
}

pub struct CompilationLink {
    pub source_file: String,
    pub binary_file: String,
    pub ast_hash: u64,
    pub compile_timestamp: u64,
    pub symbols_generated: Vec<String>,
}

impl SourceMarkovModel {
    pub fn from_rust_source(path: &str, content: &str) -> Self {
        let mut model = Self {
            file_path: path.to_string(),
            char_transitions: HashMap::new(),
            word_transitions: HashMap::new(),
            token_patterns: HashMap::new(),
            max_word_length: 128,
        };
        
        model.analyze_char_level(content);
        model.analyze_word_level(content);
        model.analyze_ast_tokens(content);
        
        model
    }
    
    fn analyze_char_level(&mut self, content: &str) {
        let chars: Vec<char> = content.chars().collect();
        
        // Char-to-char transitions
        for window in chars.windows(2) {
            let from = window[0];
            let to = window[1];
            
            *self.char_transitions
                .entry(from)
                .or_insert_with(HashMap::new)
                .entry(to)
                .or_insert(0) += 1;
        }
    }
    
    fn analyze_word_level(&mut self, content: &str) {
        // Extract words up to 128 chars
        let words: Vec<String> = content
            .split_whitespace()
            .filter_map(|word| {
                let clean = word.chars()
                    .filter(|c| c.is_alphanumeric() || *c == '_')
                    .collect::<String>();
                if clean.len() <= self.max_word_length && clean.len() > 0 {
                    Some(clean)
                } else {
                    None
                }
            })
            .collect();
        
        // Word-to-word transitions
        for window in words.windows(2) {
            let from = &window[0];
            let to = &window[1];
            
            *self.word_transitions
                .entry(from.clone())
                .or_insert_with(HashMap::new)
                .entry(to.clone())
                .or_insert(0) += 1;
        }
    }
    
    fn analyze_ast_tokens(&mut self, content: &str) {
        if let Ok(ast) = parse_file(content) {
            let mut visitor = TokenVisitor::new();
            visitor.visit_file(&ast);
            self.token_patterns = visitor.tokens;
        }
    }
    
    pub fn similarity_to(&self, other: &SourceMarkovModel) -> SourceSimilarity {
        SourceSimilarity {
            char_similarity: cosine_similarity_char(&self.char_transitions, &other.char_transitions),
            word_similarity: cosine_similarity_word(&self.word_transitions, &other.word_transitions),
            token_similarity: cosine_similarity_tokens(&self.token_patterns, &other.token_patterns),
        }
    }
}

pub struct SourceSimilarity {
    pub char_similarity: f64,
    pub word_similarity: f64,
    pub token_similarity: f64,
}

impl SourceSimilarity {
    pub fn overall_similarity(&self) -> f64 {
        // Weighted: tokens most important, then words, then chars
        self.token_similarity * 0.5 + 
        self.word_similarity * 0.3 + 
        self.char_similarity * 0.2
    }
}

struct TokenVisitor {
    tokens: HashMap<String, u32>,
}

impl TokenVisitor {
    fn new() -> Self {
        Self { tokens: HashMap::new() }
    }
}

impl<'ast> Visit<'ast> for TokenVisitor {
    fn visit_ident(&mut self, ident: &'ast syn::Ident) {
        *self.tokens.entry(ident.to_string()).or_insert(0) += 1;
    }
    
    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        *self.tokens.entry("fn".to_string()).or_insert(0) += 1;
        syn::visit::visit_item_fn(self, item);
    }
    
    fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
        *self.tokens.entry("struct".to_string()).or_insert(0) += 1;
        syn::visit::visit_item_struct(self, item);
    }
}

impl SourceBinaryMapper {
    pub fn new() -> Self {
        Self {
            source_models: HashMap::new(),
            binary_models: HashMap::new(),
            compilation_map: HashMap::new(),
        }
    }
    
    pub fn add_source(&mut self, path: &str, content: &str) {
        let model = SourceMarkovModel::from_rust_source(path, content);
        self.source_models.insert(path.to_string(), model);
    }
    
    pub fn add_binary(&mut self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let model = crate::binary_markov::BinaryMarkovModel::from_binary(path)?;
        self.binary_models.insert(path.to_string(), model);
        Ok(())
    }
    
    pub fn link_compilation(&mut self, source: &str, binary: &str, symbols: Vec<String>) {
        let ast_hash = self.calculate_ast_hash(source);
        
        let link = CompilationLink {
            source_file: source.to_string(),
            binary_file: binary.to_string(),
            ast_hash,
            compile_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap().as_secs(),
            symbols_generated: symbols,
        };
        
        self.compilation_map.insert(source.to_string(), link);
    }
    
    fn calculate_ast_hash(&self, source_path: &str) -> u64 {
        // Simple hash of AST structure
        if let Some(model) = self.source_models.get(source_path) {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            use std::hash::{Hash, Hasher};
            
            for (token, count) in &model.token_patterns {
                token.hash(&mut hasher);
                count.hash(&mut hasher);
            }
            
            hasher.finish()
        } else {
            0
        }
    }
    
    pub fn find_source_binary_correlations(&self) -> Vec<SourceBinaryCorrelation> {
        let mut correlations = Vec::new();
        
        for (source_path, source_model) in &self.source_models {
            if let Some(link) = self.compilation_map.get(source_path) {
                if let Some(binary_model) = self.binary_models.get(&link.binary_file) {
                    let correlation = SourceBinaryCorrelation {
                        source_path: source_path.clone(),
                        binary_path: link.binary_file.clone(),
                        ast_hash: link.ast_hash,
                        symbol_overlap: self.calculate_symbol_overlap(&link.symbols_generated, binary_model),
                        compile_timestamp: link.compile_timestamp,
                    };
                    
                    correlations.push(correlation);
                }
            }
        }
        
        correlations
    }
    
    fn calculate_symbol_overlap(&self, source_symbols: &[String], binary_model: &crate::binary_markov::BinaryMarkovModel) -> f64 {
        // Placeholder - would analyze symbol presence in binary
        source_symbols.len() as f64 / 100.0 // Normalize
    }
}

#[derive(Debug)]
pub struct SourceBinaryCorrelation {
    pub source_path: String,
    pub binary_path: String,
    pub ast_hash: u64,
    pub symbol_overlap: f64,
    pub compile_timestamp: u64,
}

fn cosine_similarity_char(map1: &HashMap<char, HashMap<char, u32>>, map2: &HashMap<char, HashMap<char, u32>>) -> f64 {
    // Flatten char transitions for cosine similarity
    let mut vec1 = HashMap::new();
    let mut vec2 = HashMap::new();
    
    for (c1, inner) in map1 {
        for (c2, count) in inner {
            let key = format!("{}{}", c1, c2);
            vec1.insert(key, *count);
        }
    }
    
    for (c1, inner) in map2 {
        for (c2, count) in inner {
            let key = format!("{}{}", c1, c2);
            vec2.insert(key, *count);
        }
    }
    
    cosine_similarity_generic(&vec1, &vec2)
}

fn cosine_similarity_word(map1: &HashMap<String, HashMap<String, u32>>, map2: &HashMap<String, HashMap<String, u32>>) -> f64 {
    let mut vec1 = HashMap::new();
    let mut vec2 = HashMap::new();
    
    for (w1, inner) in map1 {
        for (w2, count) in inner {
            let key = format!("{}→{}", w1, w2);
            vec1.insert(key, *count);
        }
    }
    
    for (w1, inner) in map2 {
        for (w2, count) in inner {
            let key = format!("{}→{}", w1, w2);
            vec2.insert(key, *count);
        }
    }
    
    cosine_similarity_generic(&vec1, &vec2)
}

fn cosine_similarity_tokens(map1: &HashMap<String, u32>, map2: &HashMap<String, u32>) -> f64 {
    cosine_similarity_generic(map1, map2)
}

fn cosine_similarity_generic<K: std::hash::Hash + Eq>(map1: &HashMap<K, u32>, map2: &HashMap<K, u32>) -> f64 {
    let mut dot_product = 0.0;
    let mut norm1 = 0.0;
    let mut norm2 = 0.0;
    
    let all_keys: std::collections::HashSet<_> = map1.keys().chain(map2.keys()).collect();
    
    for key in all_keys {
        let v1 = *map1.get(key).unwrap_or(&0) as f64;
        let v2 = *map2.get(key).unwrap_or(&0) as f64;
        
        dot_product += v1 * v2;
        norm1 += v1 * v1;
        norm2 += v2 * v2;
    }
    
    if norm1 == 0.0 || norm2 == 0.0 { 0.0 }
    else { dot_product / (norm1.sqrt() * norm2.sqrt()) }
}
