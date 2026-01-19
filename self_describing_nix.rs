//! # Self-Describing Nix System
//! 
//! Nix derivations describe themselves through access patterns.
//! Build traces become semantic metadata.

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

/// Nix derivation with learned semantics
#[derive(Debug, Serialize, Deserialize)]
pub struct SelfDescribingDerivation {
    pub drv_path: PathBuf,
    pub name: String,
    pub inputs: Vec<PathBuf>,
    pub outputs: Vec<PathBuf>,
    
    // Learned semantics
    pub semantic_type: String,      // Auto-labeled: "rust_binary", "library", etc.
    pub access_patterns: Vec<String>, // How it's used
    pub data_flow: DataFlow,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DataFlow {
    pub reads: Vec<PathBuf>,
    pub writes: Vec<PathBuf>,
    pub transforms: Vec<Transform>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Transform {
    pub from: PathBuf,
    pub to: PathBuf,
    pub operation: String,  // "compile", "link", "copy", etc.
}

impl SelfDescribingDerivation {
    /// Auto-label derivation type from patterns
    pub fn infer_type(&mut self) {
        self.semantic_type = if self.outputs.iter().any(|p| p.to_str().unwrap().ends_with(".so")) {
            "shared_library"
        } else if self.outputs.iter().any(|p| p.to_str().unwrap().contains("/bin/")) {
            "executable"
        } else if self.name.contains("rust") {
            "rust_package"
        } else if self.name.contains("python") {
            "python_package"
        } else {
            "generic_derivation"
        }.to_string();
    }
    
    /// Learn from strace
    pub fn learn_from_strace(&mut self, strace_log: &str) {
        // Parse strace to understand data flow
        for line in strace_log.lines() {
            if line.contains("openat") && line.contains(".rs") {
                // Reading Rust source
                self.data_flow.reads.push(PathBuf::from("*.rs"));
            }
            if line.contains("write") && line.contains(".o") {
                // Writing object file
                self.data_flow.writes.push(PathBuf::from("*.o"));
                self.data_flow.transforms.push(Transform {
                    from: PathBuf::from("*.rs"),
                    to: PathBuf::from("*.o"),
                    operation: "compile".to_string(),
                });
            }
        }
    }
}

/// System-wide semantic database
#[derive(Debug, Serialize, Deserialize)]
pub struct SemanticDatabase {
    pub derivations: HashMap<PathBuf, SelfDescribingDerivation>,
    pub pattern_language: HashMap<String, Vec<PathBuf>>,
    pub data_lineage: HashMap<PathBuf, Vec<PathBuf>>,
}

impl SemanticDatabase {
    pub fn new() -> Self {
        Self {
            derivations: HashMap::new(),
            pattern_language: HashMap::new(),
            data_lineage: HashMap::new(),
        }
    }
    
    /// Query by semantic label
    pub fn query_semantic(&self, label: &str) -> Vec<&SelfDescribingDerivation> {
        self.derivations.values()
            .filter(|d| d.semantic_type == label)
            .collect()
    }
    
    /// Trace data lineage
    pub fn trace_lineage(&self, file: &PathBuf) -> Vec<PathBuf> {
        self.data_lineage.get(file).cloned().unwrap_or_default()
    }
}
