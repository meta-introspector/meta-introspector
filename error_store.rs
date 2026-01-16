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
        .or_insert_with(Vec::new)
        .push(error.clone());
    
    report.by_bin
        .entry(error.bin.clone())
        .or_insert_with(Vec::new)
        .push(error);
}

pub fn get_report() -> Option<ErrorReport> {
    ERROR_STORE.lock().unwrap().clone()
}

pub fn suggest_fix(error: &BuildError) -> Option<String> {
    match error.error_type.as_str() {
        "E0433" if error.message.contains("gix") => {
            Some("Move gix usage to libgit.so - use git trait instead".to_string())
        }
        "E0433" if error.message.contains("reqwest") => {
            Some("Move reqwest usage to libhttp.so - use http trait instead".to_string())
        }
        "E0601" if error.message.contains("main") => {
            Some("Add fn main() {} or remove [[bin]] entry from Cargo.toml".to_string())
        }
        _ => None
    }
}
