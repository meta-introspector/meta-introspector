//! # Compiler as Auto-Labeler
//! 
//! The compiler traces become semantic labels.
//! Build process describes what code does.

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Compilation trace with semantics
#[derive(Debug, Serialize, Deserialize)]
pub struct CompilationTrace {
    pub source: String,
    pub binary: String,
    pub symbols: Vec<String>,
    pub dependencies: Vec<String>,
    
    // Auto-generated labels
    pub semantic_labels: Vec<String>,
    pub inferred_purpose: String,
}

impl CompilationTrace {
    /// Auto-label from symbols
    pub fn auto_label(&mut self) {
        self.semantic_labels.clear();
        
        for symbol in &self.symbols {
            if symbol.contains("http") || symbol.contains("server") {
                self.semantic_labels.push("network_service".to_string());
            }
            if symbol.contains("parse") || symbol.contains("compile") {
                self.semantic_labels.push("compiler".to_string());
            }
            if symbol.contains("crypto") || symbol.contains("hash") {
                self.semantic_labels.push("cryptography".to_string());
            }
            if symbol.contains("file") || symbol.contains("read") {
                self.semantic_labels.push("file_io".to_string());
            }
        }
        
        self.semantic_labels.sort();
        self.semantic_labels.dedup();
        
        // Infer overall purpose
        self.inferred_purpose = if self.semantic_labels.contains(&"network_service".to_string()) {
            "server_application"
        } else if self.semantic_labels.contains(&"compiler".to_string()) {
            "build_tool"
        } else {
            "utility"
        }.to_string();
    }
}

/// Regex as language compiler
#[derive(Debug)]
pub struct RegexLanguageCompiler {
    pub patterns: HashMap<String, CompiledPattern>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompiledPattern {
    pub regex: String,
    pub semantic_meaning: String,
    pub typical_matches: Vec<String>,
    pub optimization: String,  // How to optimize this query
}

impl RegexLanguageCompiler {
    pub fn new() -> Self {
        Self { patterns: HashMap::new() }
    }
    
    /// Compile regex into semantic query
    pub fn compile(&mut self, regex: &str) -> CompiledPattern {
        let semantic = self.infer_semantics(regex);
        let optimization = self.suggest_optimization(regex);
        
        CompiledPattern {
            regex: regex.to_string(),
            semantic_meaning: semantic,
            typical_matches: vec![],
            optimization,
        }
    }
    
    fn infer_semantics(&self, regex: &str) -> String {
        if regex.contains("*.rs") { "rust_sources" }
        else if regex.contains("*.nix") { "nix_expressions" }
        else if regex.contains("*.so") { "shared_libraries" }
        else if regex.contains("Cargo.toml") { "rust_manifests" }
        else { "generic_files" }
        .to_string()
    }
    
    fn suggest_optimization(&self, regex: &str) -> String {
        if regex.contains("*") {
            "use_extension_index".to_string()
        } else if regex.contains("/") {
            "use_path_index".to_string()
        } else {
            "use_name_index".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_label() {
        let mut trace = CompilationTrace {
            source: "server.rs".to_string(),
            binary: "server".to_string(),
            symbols: vec!["http_server".to_string(), "parse_request".to_string()],
            dependencies: vec![],
            semantic_labels: vec![],
            inferred_purpose: String::new(),
        };
        
        trace.auto_label();
        assert!(trace.semantic_labels.contains(&"network_service".to_string()));
        assert_eq!(trace.inferred_purpose, "server_application");
    }
}
