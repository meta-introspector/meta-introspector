// Prove equivalence: syn → rustc (LLVM backend) vs syn → gcc (GCC backend)

use std::process::Command;
use std::collections::HashSet;

#[derive(Clone)]
pub struct BackendComparison {
    pub syn_type: String,
    pub sample: String,
    pub llvm_ips: HashSet<u64>,
    pub gcc_ips: HashSet<u64>,
    pub llvm_asm: String,
    pub gcc_asm: String,
}

impl BackendComparison {
    pub fn equivalence_score(&self) -> f64 {
        let intersection: HashSet<_> = self.llvm_ips.intersection(&self.gcc_ips).collect();
        let union: HashSet<_> = self.llvm_ips.union(&self.gcc_ips).collect();
        
        if union.is_empty() {
            0.0
        } else {
            intersection.len() as f64 / union.len() as f64
        }
    }
}

pub fn compile_with_llvm(source: &str) -> Option<(HashSet<u64>, String)> {
    let temp_path = format!("/tmp/llvm_test_{}.rs", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs());
    
    std::fs::write(&temp_path, source).ok()?;
    
    // Compile with LLVM backend (default)
    let output = Command::new("rustc")
        .args(&[
            "--emit", "asm",
            "-C", "opt-level=2",
            "--crate-type", "lib",
            "-o", "/tmp/llvm_out.s",
            &temp_path
        ])
        .output()
        .ok()?;
    
    let asm = std::fs::read_to_string("/tmp/llvm_out.s").unwrap_or_default();
    
    // Extract IPs from assembly (simplified)
    let mut ips = HashSet::new();
    for (i, line) in asm.lines().enumerate() {
        if line.contains("call") || line.contains("jmp") {
            ips.insert(i as u64);
        }
    }
    
    std::fs::remove_file(&temp_path).ok();
    
    Some((ips, asm))
}

pub fn compile_with_gcc(source: &str) -> Option<(HashSet<u64>, String)> {
    let temp_path = format!("/tmp/gcc_test_{}.rs", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs());
    
    std::fs::write(&temp_path, source).ok()?;
    
    // Compile with GCC backend
    let output = Command::new("rustc")
        .args(&[
            "--emit", "asm",
            "-C", "opt-level=2",
            "-C", "codegen-backend=gcc",
            "--crate-type", "lib",
            "-o", "/tmp/gcc_out.s",
            &temp_path
        ])
        .output()
        .ok()?;
    
    let asm = std::fs::read_to_string("/tmp/gcc_out.s").unwrap_or_default();
    
    // Extract IPs from assembly
    let mut ips = HashSet::new();
    for (i, line) in asm.lines().enumerate() {
        if line.contains("call") || line.contains("jmp") {
            ips.insert(i as u64);
        }
    }
    
    std::fs::remove_file(&temp_path).ok();
    
    Some((ips, asm))
}

pub fn compare_backends(syn_type: &str, sample: &str) -> Option<BackendComparison> {
    let (llvm_ips, llvm_asm) = compile_with_llvm(sample)?;
    let (gcc_ips, gcc_asm) = compile_with_gcc(sample)?;
    
    Some(BackendComparison {
        syn_type: syn_type.to_string(),
        sample: sample.to_string(),
        llvm_ips,
        gcc_ips,
        llvm_asm,
        gcc_asm,
    })
}
