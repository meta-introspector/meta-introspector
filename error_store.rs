use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildError {
    pub bin: String,
    pub error_type: String,
    pub message: String,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub suggestion: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorReport {
    pub timestamp: String,
    pub total_errors: usize,
    pub by_type: HashMap<String, Vec<BuildError>>,
    pub by_bin: HashMap<String, Vec<BuildError>>,
}

static ERROR_STORE: Mutex<Option<ErrorReport>> = Mutex::new(None);

pub fn add_error(error: BuildError) {
    let mut store = ERROR_STORE.lock().unwrap();
    if store.is_none() {
        *store = Some(ErrorReport {
            timestamp: chrono::Utc::now().to_rfc3339(),
            total_errors: 0,
            by_type: HashMap::new(),
            by_bin: HashMap::new(),
        });
    }
    
    let report = store.as_mut().unwrap();
    report.total_errors += 1;
    
    report.by_type
        .entry(error.error_type.clone())
        .or_default()
        .push(error.clone());
    
    report.by_bin
        .entry(error.bin.clone())
        .or_default()
        .push(error);
}

pub fn get_report() -> Option<ErrorReport> {
    ERROR_STORE.lock().unwrap().clone()
}

pub fn suggest_fix(error: &BuildError) -> Option<String> {
    let error_code = error.error_type.trim_start_matches("E");
    
    match error_code {
        "E0433" | "0433" if error.message.contains("gix") => {
            Some("Move gix usage to libgit.so - use git trait instead".to_string())
        }
        "E0433" | "0433" if error.message.contains("reqwest") => {
            Some("Move reqwest usage to libhttp.so - use http trait instead".to_string())
        }
        "E0433" | "0433" if error.message.contains("sha256") => {
            Some("Add sha2 crate: use sha2::{Sha256, Digest}; let hash = Sha256::digest(data);".to_string())
        }
        "E0601" | "0601" if error.message.contains("main") => {
            Some("Add fn main() {} or remove [[bin]] entry from Cargo.toml".to_string())
        }
        "E0599" | "0599" if error.message.contains("no method named `clone`") => {
            Some("Add #[derive(Clone)] to the struct/enum definition".to_string())
        }
        "E0277" | "0277" if error.message.contains("Handler") => {
            Some("Function signature mismatch - check axum handler requirements".to_string())
        }
        _ => None
    }
}
