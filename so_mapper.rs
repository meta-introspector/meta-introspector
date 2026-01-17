// Map all LLVM and GCC shared objects, call GCC like Rust via .so

use std::process::Command;
use std::collections::{HashMap, HashSet};

#[derive(Clone)]
pub struct SharedObject {
    pub path: String,
    pub symbols: Vec<String>,
    pub size: u64,
}

pub struct SoMapper {
    pub llvm_sos: Vec<SharedObject>,
    pub gcc_sos: Vec<SharedObject>,
    pub symbol_map: HashMap<String, Vec<String>>,
}

impl SoMapper {
    pub fn new() -> Self {
        Self {
            llvm_sos: Vec::new(),
            gcc_sos: Vec::new(),
            symbol_map: HashMap::new(),
        }
    }
    
    pub fn map_llvm_sos(&mut self) {
        // Find rustc binary
        if let Ok(output) = Command::new("which").arg("rustc").output() {
            let rustc_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            
            // Run ldd on rustc to find LLVM shared objects
            if let Ok(ldd_output) = Command::new("ldd").arg(&rustc_path).output() {
                let ldd_result = String::from_utf8_lossy(&ldd_output.stdout);
                
                for line in ldd_result.lines() {
                    if line.contains("LLVM") || line.contains("llvm") {
                        if let Some(path) = line.split_whitespace()
                            .find(|s| s.starts_with('/') && s.ends_with(".so")) {
                            
                            self.add_llvm_so(path);
                        }
                    }
                }
            }
        }
    }
    
    pub fn map_gcc_sos(&mut self) {
        // Find gcc binary
        if let Ok(output) = Command::new("which").arg("gcc").output() {
            let gcc_path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            
            // Run ldd on gcc
            if let Ok(ldd_output) = Command::new("ldd").arg(&gcc_path).output() {
                let ldd_result = String::from_utf8_lossy(&ldd_output.stdout);
                
                for line in ldd_result.lines() {
                    if let Some(path) = line.split_whitespace()
                        .find(|s| s.starts_with('/') && s.ends_with(".so")) {
                        
                        self.add_gcc_so(path);
                    }
                }
            }
        }
    }
    
    fn add_llvm_so(&mut self, path: &str) {
        if let Some(so) = self.load_so(path) {
            self.llvm_sos.push(so);
        }
    }
    
    fn add_gcc_so(&mut self, path: &str) {
        if let Some(so) = self.load_so(path) {
            self.gcc_sos.push(so);
        }
    }
    
    fn load_so(&mut self, path: &str) -> Option<SharedObject> {
        // Get size
        let size = std::fs::metadata(path).ok()?.len();
        
        // Extract symbols with nm
        let symbols = if let Ok(nm_output) = Command::new("nm")
            .args(["-D", path])
            .output() {
            
            let nm_result = String::from_utf8_lossy(&nm_output.stdout);
            nm_result.lines()
                .filter_map(|line| {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 3 {
                        Some(parts[2].to_string())
                    } else {
                        None
                    }
                })
                .take(100)  // Limit to 100 symbols per .so
                .collect()
        } else {
            Vec::new()
        };
        
        // Map symbols
        for symbol in &symbols {
            self.symbol_map.entry(symbol.clone())
                .or_default()
                .push(path.to_string());
        }
        
        Some(SharedObject {
            path: path.to_string(),
            symbols,
            size,
        })
    }
    
    pub fn find_common_symbols(&self) -> Vec<String> {
        let llvm_symbols: HashSet<String> = self.llvm_sos.iter()
            .flat_map(|so| so.symbols.iter())
            .cloned()
            .collect();
        
        let gcc_symbols: HashSet<String> = self.gcc_sos.iter()
            .flat_map(|so| so.symbols.iter())
            .cloned()
            .collect();
        
        llvm_symbols.intersection(&gcc_symbols)
            .cloned()
            .collect()
    }
    
    pub fn report(&self) {
        println!("\n📊 Shared Object Mapping Report");
        println!("  LLVM .so files: {}", self.llvm_sos.len());
        println!("  GCC .so files: {}", self.gcc_sos.len());
        
        let llvm_size: u64 = self.llvm_sos.iter().map(|so| so.size).sum();
        let gcc_size: u64 = self.gcc_sos.iter().map(|so| so.size).sum();
        
        println!("  LLVM total size: {:.2} MB", llvm_size as f64 / 1_000_000.0);
        println!("  GCC total size: {:.2} MB", gcc_size as f64 / 1_000_000.0);
        
        let common = self.find_common_symbols();
        println!("  Common symbols: {}", common.len());
        
        if !self.llvm_sos.is_empty() {
            println!("\n  LLVM shared objects:");
            for so in self.llvm_sos.iter().take(5) {
                println!("    {} ({} symbols, {:.2} MB)", 
                         so.path.split('/').next_back().unwrap_or(&so.path),
                         so.symbols.len(),
                         so.size as f64 / 1_000_000.0);
            }
        }
        
        if !self.gcc_sos.is_empty() {
            println!("\n  GCC shared objects:");
            for so in self.gcc_sos.iter().take(5) {
                println!("    {} ({} symbols, {:.2} MB)", 
                         so.path.split('/').next_back().unwrap_or(&so.path),
                         so.symbols.len(),
                         so.size as f64 / 1_000_000.0);
            }
        }
    }
}
