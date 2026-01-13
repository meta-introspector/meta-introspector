pub mod abi_extractor;
pub mod mcp_generator;
pub mod nix_scanner;
pub mod wrapper_generator;

pub use crate::{
    LibraryInfo, SymbolInfo, SymbolType, FunctionSignature, Parameter, NixStoreAnalysis
};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LibraryInfo {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub symbols: Vec<SymbolInfo>,
    pub dependencies: Vec<String>,
    pub architecture: String,
    pub abi_hash: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SymbolInfo {
    pub name: String,
    pub demangled_name: Option<String>,
    pub symbol_type: SymbolType,
    pub address: u64,
    pub size: Option<u64>,
    pub signature: Option<FunctionSignature>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum SymbolType {
    Function,
    Object,
    Section,
    File,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionSignature {
    pub return_type: String,
    pub parameters: Vec<Parameter>,
    pub calling_convention: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Parameter {
    pub name: Option<String>,
    pub param_type: String,
    pub is_pointer: bool,
    pub is_const: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NixStoreAnalysis {
    pub total_libraries: usize,
    pub total_symbols: usize,
    pub libraries: Vec<LibraryInfo>,
    pub symbol_index: HashMap<String, Vec<String>>, // symbol -> libraries
    pub dependency_graph: HashMap<String, Vec<String>>,
    pub analysis_timestamp: chrono::DateTime<chrono::Utc>,
}

// Re-export main functionality for use as a library
pub use abi_extractor::extract_abi;
pub use mcp_generator::{generate_mcp_service, start_mcp_server};
pub use nix_scanner::scan_nix_store;
pub use wrapper_generator::generate_wrapper;
