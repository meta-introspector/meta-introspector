// 🔥 AST SIMILARITY MATRIX
// Compare every module's AST against every other module

use syn::{parse_file, Item, ItemFn, ItemStruct, ItemEnum};
use std::collections::HashMap;

pub struct AstMatrix {
    pub modules: Vec<ModuleAst>,
    pub similarity_matrix: Vec<Vec<f64>>,
}

pub struct ModuleAst {
    pub name: String,
    pub functions: Vec<String>,
    pub structs: Vec<String>,
    pub enums: Vec<String>,
    pub patterns: HashMap<String, u32>,
}

impl AstMatrix {
    pub fn new() -> Self {
        Self {
            modules: Vec::new(),
            similarity_matrix: Vec::new(),
        }
    }
    
    pub fn add_module(&mut self, name: &str, rust_code: &str) {
        if let Ok(ast) = parse_file(rust_code) {
            let module_ast = ModuleAst::from_syn_file(name, &ast);
            self.modules.push(module_ast);
        }
    }
    
    pub fn compute_matrix(&mut self) {
        let n = self.modules.len();
        self.similarity_matrix = vec![vec![0.0; n]; n];
        
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    self.similarity_matrix[i][j] = 1.0;
                } else {
                    self.similarity_matrix[i][j] = self.modules[i].similarity_to(&self.modules[j]);
                }
            }
        }
    }
    
    pub fn print_matrix(&self) {
        println!("📊 AST SIMILARITY MATRIX");
        println!("========================");
        
        // Header
        print!("         ");
        for module in &self.modules {
            print!("{:8.8} ", module.name);
        }
        println!();
        
        // Matrix rows
        for (i, row) in self.similarity_matrix.iter().enumerate() {
            print!("{:8.8} ", self.modules[i].name);
            for &similarity in row {
                print!("{:8.3} ", similarity);
            }
            println!();
        }
    }
    
    pub fn find_duplicates(&self, threshold: f64) -> Vec<(String, String, f64)> {
        let mut duplicates = Vec::new();
        
        for i in 0..self.modules.len() {
            for j in i+1..self.modules.len() {
                let similarity = self.similarity_matrix[i][j];
                if similarity > threshold {
                    duplicates.push((
                        self.modules[i].name.clone(),
                        self.modules[j].name.clone(),
                        similarity
                    ));
                }
            }
        }
        
        duplicates.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());
        duplicates
    }
}

impl ModuleAst {
    pub fn from_syn_file(name: &str, ast: &syn::File) -> Self {
        let mut module = Self {
            name: name.to_string(),
            functions: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            patterns: HashMap::new(),
        };
        
        for item in &ast.items {
            match item {
                Item::Fn(ItemFn { sig, .. }) => {
                    module.functions.push(sig.ident.to_string());
                    *module.patterns.entry("function".to_string()).or_insert(0) += 1;
                }
                Item::Struct(ItemStruct { ident, .. }) => {
                    module.structs.push(ident.to_string());
                    *module.patterns.entry("struct".to_string()).or_insert(0) += 1;
                }
                Item::Enum(ItemEnum { ident, .. }) => {
                    module.enums.push(ident.to_string());
                    *module.patterns.entry("enum".to_string()).or_insert(0) += 1;
                }
                _ => {
                    *module.patterns.entry("other".to_string()).or_insert(0) += 1;
                }
            }
        }
        
        module
    }
    
    pub fn similarity_to(&self, other: &ModuleAst) -> f64 {
        // Jaccard similarity on function names
        let func_similarity = jaccard_similarity(&self.functions, &other.functions);
        
        // Jaccard similarity on struct names  
        let struct_similarity = jaccard_similarity(&self.structs, &other.structs);
        
        // Pattern distribution similarity
        let pattern_similarity = pattern_similarity(&self.patterns, &other.patterns);
        
        // Weighted average
        (func_similarity * 0.5 + struct_similarity * 0.3 + pattern_similarity * 0.2)
    }
}

fn jaccard_similarity(set1: &[String], set2: &[String]) -> f64 {
    let s1: std::collections::HashSet<_> = set1.iter().collect();
    let s2: std::collections::HashSet<_> = set2.iter().collect();
    
    let intersection = s1.intersection(&s2).count();
    let union = s1.union(&s2).count();
    
    if union == 0 { 0.0 } else { intersection as f64 / union as f64 }
}

fn pattern_similarity(p1: &HashMap<String, u32>, p2: &HashMap<String, u32>) -> f64 {
    let mut dot_product = 0.0;
    let mut norm1 = 0.0;
    let mut norm2 = 0.0;
    
    let all_keys: std::collections::HashSet<_> = p1.keys().chain(p2.keys()).collect();
    
    for key in all_keys {
        let v1 = *p1.get(key).unwrap_or(&0) as f64;
        let v2 = *p2.get(key).unwrap_or(&0) as f64;
        
        dot_product += v1 * v2;
        norm1 += v1 * v1;
        norm2 += v2 * v2;
    }
    
    if norm1 == 0.0 || norm2 == 0.0 { 0.0 }
    else { dot_product / (norm1.sqrt() * norm2.sqrt()) }
}
