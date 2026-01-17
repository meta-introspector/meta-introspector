// Code Duplication Scanner: Mathematical fingerprints for Rust code
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct CodeFingerprint {
    pub ast_hash: String,           // Syntax tree hash
    pub token_hash: String,         // Token sequence hash
    pub structure_hash: String,     // Structure hash (ignoring names)
    pub semantic_hash: String,      // Semantic hash (types, control flow)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateCode {
    pub fingerprint: CodeFingerprint,
    pub locations: Vec<CodeLocation>,
    pub similarity: f64,
    pub lines: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeLocation {
    pub repo: String,
    pub file: String,
    pub start_line: usize,
    pub end_line: usize,
    pub function: Option<String>,
}

pub struct DuplicationScanner {
    pub fingerprints: HashMap<CodeFingerprint, Vec<CodeLocation>>,
    pub duplicates: Vec<DuplicateCode>,
}

impl DuplicationScanner {
    pub fn new() -> Self {
        Self {
            fingerprints: HashMap::new(),
            duplicates: Vec::new(),
        }
    }
    
    pub fn scan_repo(&mut self, repo_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        println!("🔍 Scanning repo: {}", repo_path);
        
        // Find all Rust files
        let rust_files = self.find_rust_files(repo_path)?;
        
        for file in rust_files {
            self.scan_file(&file, repo_path)?;
        }
        
        Ok(())
    }
    
    fn scan_file(&mut self, file_path: &str, repo: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(file_path)?;
        
        // Parse with syn
        let syntax = syn::parse_file(&content)?;
        
        // Extract functions
        for item in &syntax.items {
            if let syn::Item::Fn(func) = item {
                let fingerprint = self.compute_fingerprint(func)?;
                let location = CodeLocation {
                    repo: repo.to_string(),
                    file: file_path.to_string(),
                    start_line: 0, // TODO: Get from span
                    end_line: 0,
                    function: Some(func.sig.ident.to_string()),
                };
                
                self.fingerprints.entry(fingerprint)
                    .or_insert_with(Vec::new)
                    .push(location);
            }
        }
        
        Ok(())
    }
    
    fn compute_fingerprint(&self, func: &syn::ItemFn) -> Result<CodeFingerprint, Box<dyn std::error::Error>> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        // AST hash - use quote to convert to tokens
        let mut ast_hasher = DefaultHasher::new();
        quote::quote!(#func).to_string().hash(&mut ast_hasher);
        let ast_hash = format!("{:x}", ast_hasher.finish());
        
        // Token hash - token sequence
        let tokens = self.extract_tokens(func);
        let mut token_hasher = DefaultHasher::new();
        tokens.hash(&mut token_hasher);
        let token_hash = format!("{:x}", token_hasher.finish());
        
        // Structure hash - ignoring identifiers
        let structure = self.extract_structure(func);
        let mut struct_hasher = DefaultHasher::new();
        structure.hash(&mut struct_hasher);
        let structure_hash = format!("{:x}", struct_hasher.finish());
        
        // Semantic hash - types and control flow
        let semantic = self.extract_semantics(func);
        let mut sem_hasher = DefaultHasher::new();
        semantic.hash(&mut sem_hasher);
        let semantic_hash = format!("{:x}", sem_hasher.finish());
        
        Ok(CodeFingerprint {
            ast_hash,
            token_hash,
            structure_hash,
            semantic_hash,
        })
    }
    
    fn extract_tokens(&self, func: &syn::ItemFn) -> Vec<String> {
        // Extract token sequence
        let mut tokens = Vec::new();
        
        // Function signature tokens
        tokens.push("fn".to_string());
        tokens.push(func.sig.ident.to_string());
        
        // Parameter tokens
        for input in &func.sig.inputs {
            match input {
                syn::FnArg::Typed(pat_type) => {
                    tokens.push(quote::quote!(#pat_type).to_string());
                }
                _ => {}
            }
        }
        
        // Return type
        if let syn::ReturnType::Type(_, ty) = &func.sig.output {
            tokens.push(quote::quote!(#ty).to_string());
        }
        
        tokens
    }
    
    fn extract_structure(&self, func: &syn::ItemFn) -> Vec<String> {
        // Extract structure ignoring names
        let mut structure = Vec::new();
        
        // Control flow patterns
        for stmt in &func.block.stmts {
            match stmt {
                syn::Stmt::Expr(expr, _) => {
                    structure.push(self.classify_expr(expr));
                }
                syn::Stmt::Local(_) => {
                    structure.push("let".to_string());
                }
                _ => {}
            }
        }
        
        structure
    }
    
    fn classify_expr(&self, expr: &syn::Expr) -> String {
        match expr {
            syn::Expr::If(_) => "if".to_string(),
            syn::Expr::Match(_) => "match".to_string(),
            syn::Expr::ForLoop(_) => "for".to_string(),
            syn::Expr::While(_) => "while".to_string(),
            syn::Expr::Loop(_) => "loop".to_string(),
            syn::Expr::Call(_) => "call".to_string(),
            syn::Expr::MethodCall(_) => "method".to_string(),
            syn::Expr::Binary(_) => "binary".to_string(),
            _ => "expr".to_string(),
        }
    }
    
    fn extract_semantics(&self, func: &syn::ItemFn) -> Vec<String> {
        // Extract semantic patterns
        let mut semantics = Vec::new();
        
        // Type patterns
        for input in &func.sig.inputs {
            if let syn::FnArg::Typed(pat_type) = input {
                semantics.push(format!("param:{}", quote::quote!(#pat_type)));
            }
        }
        
        // Return type pattern
        if let syn::ReturnType::Type(_, ty) = &func.sig.output {
            semantics.push(format!("return:{}", quote::quote!(#ty)));
        }
        
        semantics
    }
    
    pub fn find_duplicates(&mut self) -> Vec<DuplicateCode> {
        println!("🔎 Finding duplicates...");
        
        let mut duplicates = Vec::new();
        
        for (fingerprint, locations) in &self.fingerprints {
            if locations.len() > 1 {
                // Found duplicate!
                duplicates.push(DuplicateCode {
                    fingerprint: fingerprint.clone(),
                    locations: locations.clone(),
                    similarity: 1.0, // Exact match
                    lines: 0, // TODO: Calculate
                });
            }
        }
        
        // Find near-duplicates (similar structure)
        self.find_near_duplicates(&mut duplicates);
        
        self.duplicates = duplicates.clone();
        duplicates
    }
    
    fn find_near_duplicates(&self, duplicates: &mut Vec<DuplicateCode>) {
        // Group by structure hash (ignoring names)
        let mut by_structure: HashMap<String, Vec<(CodeFingerprint, Vec<CodeLocation>)>> = HashMap::new();
        
        for (fp, locs) in &self.fingerprints {
            by_structure.entry(fp.structure_hash.clone())
                .or_insert_with(Vec::new)
                .push((fp.clone(), locs.clone()));
        }
        
        // Find groups with multiple different implementations
        for (structure_hash, group) in by_structure {
            if group.len() > 1 {
                // Near-duplicate: same structure, different names
                let all_locations: Vec<CodeLocation> = group.iter()
                    .flat_map(|(_, locs)| locs.clone())
                    .collect();
                
                if all_locations.len() > 1 {
                    duplicates.push(DuplicateCode {
                        fingerprint: group[0].0.clone(),
                        locations: all_locations,
                        similarity: 0.8, // Structural similarity
                        lines: 0,
                    });
                }
            }
        }
    }
    
    fn find_rust_files(&self, path: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        let output = std::process::Command::new("find")
            .arg(path)
            .arg("-name")
            .arg("*.rs")
            .arg("-type")
            .arg("f")
            .output()?;
        
        let files: Vec<String> = String::from_utf8_lossy(&output.stdout)
            .lines()
            .map(|s| s.to_string())
            .collect();
        
        Ok(files)
    }
    
    pub fn export_report(&self, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let report = serde_json::json!({
            "total_fingerprints": self.fingerprints.len(),
            "total_duplicates": self.duplicates.len(),
            "duplicates": self.duplicates,
        });
        
        std::fs::write(output_path, serde_json::to_string_pretty(&report)?)?;
        println!("✅ Exported report to: {}", output_path);
        
        Ok(())
    }
    
    pub fn export_to_nix_store(&self) -> Result<String, Box<dyn std::error::Error>> {
        // Store in content-addressable nix store
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        format!("{:?}", self.duplicates).hash(&mut hasher);
        let hash = format!("{:x}", hasher.finish());
        
        let nix_path = format!("/tmp/duplication-scan/{}/{}", &hash[..2], hash);
        std::fs::create_dir_all(&nix_path)?;
        
        self.export_report(&format!("{}/report.json", nix_path))?;
        
        Ok(nix_path)
    }
}

// Scan multiple repos
pub fn scan_repos(repos: &[&str]) -> Result<DuplicationScanner, Box<dyn std::error::Error>> {
    let mut scanner = DuplicationScanner::new();
    
    for repo in repos {
        scanner.scan_repo(repo)?;
    }
    
    scanner.find_duplicates();
    
    Ok(scanner)
}

fn main() {
    println!("code_duplication_scanner - add usage here");
}
