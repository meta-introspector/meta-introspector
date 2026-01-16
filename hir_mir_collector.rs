// Collect rustc HIR/MIR dumps and map to syn objects

use std::process::Command;
use std::collections::HashMap;

#[derive(Clone)]
pub struct HirMirDump {
    pub source_hash: String,
    pub hir: String,
    pub mir: String,
    pub syn_type: String,
}

pub struct HirMirCollector {
    pub dumps: Vec<HirMirDump>,
    pub syn_to_hir: HashMap<String, Vec<String>>,
    pub syn_to_mir: HashMap<String, Vec<String>>,
}

impl HirMirCollector {
    pub fn new() -> Self {
        Self {
            dumps: Vec::new(),
            syn_to_hir: HashMap::new(),
            syn_to_mir: HashMap::new(),
        }
    }
    
    pub fn collect(&mut self, source: &str, syn_type: &str) -> Option<HirMirDump> {
        // Write source to temp file
        let temp_path = format!("/tmp/rustc_dump_{}.rs", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        std::fs::write(&temp_path, source).ok()?;
        
        // Compile with HIR dump
        let hir_output = Command::new("rustc")
            .args(&[
                "-Z", "unpretty=hir",
                "--crate-type", "lib",
                &temp_path
            ])
            .output()
            .ok()?;
        
        let hir = String::from_utf8_lossy(&hir_output.stdout).to_string();
        
        // Compile with MIR dump
        let mir_output = Command::new("rustc")
            .args(&[
                "-Z", "dump-mir=all",
                "--crate-type", "lib",
                &temp_path
            ])
            .output()
            .ok()?;
        
        let mir = String::from_utf8_lossy(&mir_output.stdout).to_string();
        
        // Cleanup
        let _ = std::fs::remove_file(&temp_path);
        
        let hash = format!("{:x}", source.len());
        
        let dump = HirMirDump {
            source_hash: hash.clone(),
            hir: hir.clone(),
            mir: mir.clone(),
            syn_type: syn_type.to_string(),
        };
        
        // Map syn type to HIR/MIR
        self.syn_to_hir.entry(syn_type.to_string())
            .or_insert_with(Vec::new)
            .push(hash.clone());
        
        self.syn_to_mir.entry(syn_type.to_string())
            .or_insert_with(Vec::new)
            .push(hash.clone());
        
        self.dumps.push(dump.clone());
        
        Some(dump)
    }
    
    pub fn report(&self) {
        println!("\n📊 HIR/MIR Collection Report");
        println!("  Total dumps: {}", self.dumps.len());
        println!("  Syn types with HIR: {}", self.syn_to_hir.len());
        println!("  Syn types with MIR: {}", self.syn_to_mir.len());
        
        println!("\n  Syn → HIR/MIR mapping:");
        for (syn_type, hashes) in &self.syn_to_hir {
            println!("    {}: {} samples", syn_type, hashes.len());
        }
    }
}

