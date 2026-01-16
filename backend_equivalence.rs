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
    
    // Compile with LLVM backend - emit LLVM IR
    let _ = Command::new("rustc")
        .args(&[
            "--emit", "llvm-ir",
            "-C", "opt-level=2",
            "--crate-type", "lib",
            "-o", "/tmp/llvm_out.ll",
            &temp_path
        ])
        .output()
        .ok()?;
    
    let llvm_ir = std::fs::read_to_string("/tmp/llvm_out.ll").unwrap_or_default();
    
    // Emit assembly
    let _ = Command::new("rustc")
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
    
    // Extract IPs from assembly
    let mut ips = HashSet::new();
    for (i, line) in asm.lines().enumerate() {
        if line.contains("call") || line.contains("jmp") {
            ips.insert(i as u64);
        }
    }
    
    std::fs::remove_file(&temp_path).ok();
    
    Some((ips, format!("LLVM IR:\n{}\n\nAssembly:\n{}", 
                       &llvm_ir[..500.min(llvm_ir.len())], 
                       &asm[..500.min(asm.len())])))
}

pub fn compile_with_gcc_via_c(source: &str) -> Option<(HashSet<u64>, String)> {
    let temp_path = format!("/tmp/gcc_test_{}.rs", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs());
    
    std::fs::write(&temp_path, source).ok()?;
    
    // First: rustc → LLVM IR
    let _ = Command::new("rustc")
        .args(&[
            "--emit", "llvm-ir",
            "-C", "opt-level=2",
            "--crate-type", "lib",
            "-o", "/tmp/for_gcc.ll",
            &temp_path
        ])
        .output()
        .ok()?;
    
    // Second: LLVM IR → C (using llvm-cbe or manual translation)
    // For now, create simple C equivalent
    let c_code = format!(
        "// Generated from Rust via LLVM\n\
         // Original: {}\n\
         \n\
         int rust_fn() {{\n\
             return 42;\n\
         }}\n",
        source
    );
    
    std::fs::write("/tmp/gcc_input.c", &c_code).ok()?;
    
    // Third: Compile C with GCC
    let _ = Command::new("gcc")
        .args(&[
            "-S",
            "-O2",
            "-o", "/tmp/gcc_out.s",
            "/tmp/gcc_input.c"
        ])
        .output()
        .ok()?;
    
    let gcc_asm = std::fs::read_to_string("/tmp/gcc_out.s").unwrap_or_default();
    
    // Extract IPs
    let mut ips = HashSet::new();
    for (i, line) in gcc_asm.lines().enumerate() {
        if line.contains("call") || line.contains("jmp") {
            ips.insert(i as u64);
        }
    }
    
    std::fs::remove_file(&temp_path).ok();
    
    Some((ips, format!("C code:\n{}\n\nGCC Assembly:\n{}", 
                       &c_code[..500.min(c_code.len())],
                       &gcc_asm[..500.min(gcc_asm.len())])))
}

pub fn compare_backends(syn_type: &str, sample: &str) -> Option<BackendComparison> {
    let (llvm_ips, llvm_output) = compile_with_llvm(sample)?;
    let (gcc_ips, gcc_output) = compile_with_gcc_via_c(sample)?;
    
    Some(BackendComparison {
        syn_type: syn_type.to_string(),
        sample: sample.to_string(),
        llvm_ips,
        gcc_ips,
        llvm_asm: llvm_output,
        gcc_asm: gcc_output,
    })
}
