// Use nix build to record all .so and instructions for LLVM and GCC

use std::process::Command;
use std::collections::HashSet;

pub struct NixBuildRecorder {
    pub build_log: String,
    pub shared_objects: HashSet<String>,
    pub instructions: Vec<String>,
}

impl NixBuildRecorder {
    pub fn new() -> Self {
        Self {
            build_log: String::new(),
            shared_objects: HashSet::new(),
            instructions: Vec::new(),
        }
    }
    
    pub fn record_build(&mut self, source: &str, backend: &str) -> Option<()> {
        // Write source to temp file
        let temp_path = format!("/tmp/nix_build_{}.rs", std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs());
        
        std::fs::write(&temp_path, source).ok()?;
        
        // Build with nix-shell and record everything
        let build_cmd = format!(
            "nix-shell -p rustc {} --run 'rustc --emit=asm,llvm-ir -C opt-level=2 --crate-type lib {} 2>&1'",
            if backend == "gcc" { "gcc" } else { "" },
            temp_path
        );
        
        let output = Command::new("sh")
            .arg("-c")
            .arg(&build_cmd)
            .output()
            .ok()?;
        
        self.build_log = String::from_utf8_lossy(&output.stdout).to_string();
        self.build_log.push_str(&String::from_utf8_lossy(&output.stderr));
        
        // Extract .so references from build log
        for line in self.build_log.lines() {
            if line.contains(".so") {
                if let Some(so_path) = line.split_whitespace()
                    .find(|s| s.contains(".so")) {
                    self.shared_objects.insert(so_path.to_string());
                }
            }
        }
        
        // Record instructions from generated assembly
        let asm_path = temp_path.replace(".rs", ".s");
        if let Ok(asm) = std::fs::read_to_string(&asm_path) {
            for line in asm.lines() {
                let trimmed = line.trim();
                if !trimmed.is_empty() && !trimmed.starts_with('.') && !trimmed.starts_with('#') {
                    self.instructions.push(trimmed.to_string());
                }
            }
        }
        
        std::fs::remove_file(&temp_path).ok();
        
        Some(())
    }
    
    pub fn report(&self) {
        println!("\n📊 Nix Build Recording Report");
        println!("  Build log: {} lines", self.build_log.lines().count());
        println!("  Shared objects: {}", self.shared_objects.len());
        println!("  Instructions: {}", self.instructions.len());
        
        if !self.shared_objects.is_empty() {
            println!("\n  Shared objects used:");
            for so in self.shared_objects.iter().take(10) {
                println!("    {}", so);
            }
        }
        
        if !self.instructions.is_empty() {
            println!("\n  Sample instructions:");
            for instr in self.instructions.iter().take(10) {
                println!("    {}", instr);
            }
        }
    }
}
