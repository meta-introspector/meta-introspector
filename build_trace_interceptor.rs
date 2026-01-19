//! # Build Trace Interceptor
//! 
//! Intercepts all file access during Nix builds, replaces data with hashes,
//! forces code to use our indexed system. Breaks process boundaries.

use std::collections::HashMap;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

/// Intercepted file access from strace/perf
#[derive(Debug, Serialize, Deserialize)]
pub struct FileAccess {
    pub pid: u32,
    pub syscall: String,
    pub path: PathBuf,
    pub hash: String,
    pub offset: u64,
    pub size: usize,
}

/// Tainted data marker
#[derive(Debug, Serialize, Deserialize)]
pub struct TaintedData {
    pub original_path: PathBuf,
    pub hash: String,
    pub access_pattern: Vec<FileAccess>,
    pub replacement: DataReplacement,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DataReplacement {
    Hash(String),
    IndexQuery(String),
    CachedResult(Vec<u8>),
    Redirect(PathBuf),
}

/// Build trace analyzer
pub struct BuildTraceAnalyzer {
    strace_log: PathBuf,
    perf_data: PathBuf,
    accesses: HashMap<PathBuf, Vec<FileAccess>>,
    patterns: HashMap<String, usize>,
}

impl BuildTraceAnalyzer {
    pub fn new(strace: PathBuf, perf: PathBuf) -> Self {
        Self {
            strace_log: strace,
            perf_data: perf,
            accesses: HashMap::new(),
            patterns: HashMap::new(),
        }
    }
    
    pub fn parse_strace(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(&self.strace_log)?;
        
        for line in content.lines() {
            if let Some(access) = self.parse_strace_line(line) {
                self.accesses.entry(access.path.clone())
                    .or_insert_with(Vec::new)
                    .push(access);
            }
        }
        
        Ok(())
    }
    
    fn parse_strace_line(&self, line: &str) -> Option<FileAccess> {
        if line.contains("openat") && line.contains(".rs") {
            let path = line.split('"').nth(1)?;
            Some(FileAccess {
                pid: 0,
                syscall: "openat".to_string(),
                path: PathBuf::from(path),
                hash: String::new(),
                offset: 0,
                size: 0,
            })
        } else {
            None
        }
    }
    
    pub fn detect_patterns(&mut self) {
        for (path, accesses) in &self.accesses {
            let ext = path.extension()
                .and_then(|s| s.to_str())
                .unwrap_or("");
            
            *self.patterns.entry(format!("ext:{}", ext)).or_insert(0) += accesses.len();
            
            if path.to_str().unwrap().contains("/src/") {
                *self.patterns.entry("path:src".to_string()).or_insert(0) += 1;
            }
        }
    }
    
    pub fn generate_replacements(&self) -> Vec<TaintedData> {
        self.accesses.iter().map(|(path, accesses)| {
            let hash = format!("sha256:{}", path.display());
            
            TaintedData {
                original_path: path.clone(),
                hash: hash.clone(),
                access_pattern: accesses.clone(),
                replacement: if accesses.len() > 10 {
                    DataReplacement::IndexQuery(format!("path:{}", path.display()))
                } else {
                    DataReplacement::Hash(hash)
                },
            }
        }).collect()
    }
}

/// LD_PRELOAD interceptor
#[no_mangle]
pub extern "C" fn openat(dirfd: i32, pathname: *const i8, flags: i32) -> i32 {
    unsafe {
        let path = std::ffi::CStr::from_ptr(pathname).to_str().unwrap();
        
        if should_intercept(path) {
            let query = format!("path:{}", path);
            if let Some(cached) = query_file_index(&query) {
                return create_memfd(&cached);
            }
        }
        
        libc::openat(dirfd, pathname, flags)
    }
}

fn should_intercept(path: &str) -> bool {
    path.ends_with(".rs") || path.ends_with(".toml") || path.ends_with(".nix")
}

fn query_file_index(query: &str) -> Option<Vec<u8>> {
    let url = format!("http://localhost:3030/query/pattern?q={}", query);
    reqwest::blocking::get(&url).ok()?.bytes().ok().map(|b| b.to_vec())
}

fn create_memfd(data: &[u8]) -> i32 {
    unsafe {
        let fd = libc::memfd_create(b"cached\0".as_ptr() as *const i8, 0);
        libc::write(fd, data.as_ptr() as *const _, data.len());
        libc::lseek(fd, 0, libc::SEEK_SET);
        fd
    }
}
