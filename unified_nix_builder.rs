// 🔥 UNIFIED NIX BUILDER - Single centralized nix build system
use std::process::Command;

use crate::telemetry_lib;
include!("latest_dev.rs");

pub struct NixBuilder {
    telemetry_enabled: bool,
}

impl NixBuilder {
    pub fn new() -> Self {
        Self {
            telemetry_enabled: true,
        }
    }

    pub fn build(&self, args: &[&str]) -> Result<NixBuildResult, String> {
        if self.telemetry_enabled {
            init_all_call_wrappers!();
        }

        let mut cmd = Command::new("nix");
        cmd.args(args);

        // Add LD_PRELOAD if available
        if telemetry_lib::telemetry_lib::preload_lib_exists() {
            cmd.env("LD_PRELOAD", telemetry_lib::telemetry_lib::get_preload_lib());
        } else if self.telemetry_enabled {
            panic!("LD_PRELOAD library not found: {:?}", telemetry_lib::telemetry_lib::get_preload_lib());
        }

        match cmd.output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                
                Ok(NixBuildResult {
                    success: output.status.success(),
                    exit_code: output.status.code().unwrap_or(-1),
                    stdout: stdout.to_string(),
                    stderr: stderr.to_string(),
                })
            }
            Err(e) => Err(format!("Failed to run nix: {}", e)),
        }
    }

    pub fn build_with_full_preload(&self, args: &[&str]) -> Result<NixBuildResult, String> {
        if self.telemetry_enabled {
            init_all_call_wrappers!();
        }

        // Get all shared libraries that nix uses
        let nix_libs = self.get_nix_dependencies()?;
        println!("🔍 Found {} shared libraries for nix", nix_libs.len());
        
        let mut cmd = Command::new("strace");
        cmd.args(&["-f", "-e", "trace=execve,openat", "-o", "/tmp/nix_strace.log"]);
        cmd.arg("nix");
        cmd.args(args);

        // Build LD_PRELOAD with all libraries
        let mut preload_libs = Vec::new();
        
        // Add our telemetry interceptor
        if telemetry_lib::telemetry_lib::preload_lib_exists() {
            preload_libs.push(telemetry_lib::telemetry_lib::PRELOAD_LIB_PATH.to_string());
        } else if self.telemetry_enabled {
            panic!("LD_PRELOAD library not found: {:?}", telemetry_lib::telemetry_lib::get_preload_lib());
        }
        
        // Add all nix shared libraries
        preload_libs.extend(nix_libs);
        
        let preload_str = preload_libs.join(":");
        cmd.env("LD_PRELOAD", &preload_str);
        
        println!("🔧 LD_PRELOAD with {} libraries", preload_libs.len());
        println!("📊 Running with strace + full LD_PRELOAD...");

        match cmd.output() {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                
                // Analyze strace output
                self.analyze_strace_capture()?;
                
                Ok(NixBuildResult {
                    success: output.status.success(),
                    exit_code: output.status.code().unwrap_or(-1),
                    stdout: stdout.to_string(),
                    stderr: stderr.to_string(),
                })
            }
            Err(e) => Err(format!("Failed to run nix with strace: {}", e)),
        }
    }

    fn get_nix_dependencies(&self) -> Result<Vec<String>, String> {
        let output = Command::new("ldd")
            .arg("/usr/bin/nix")
            .output()
            .map_err(|e| format!("Failed to run ldd: {}", e))?;
            
        let ldd_output = String::from_utf8_lossy(&output.stdout);
        let mut libs = Vec::new();
        
        for line in ldd_output.lines() {
            if let Some(start) = line.find(" => ") {
                if let Some(end) = line[start + 4..].find(" (") {
                    let lib_path = line[start + 4..start + 4 + end].trim();
                    if lib_path.starts_with("/") && lib_path.ends_with(".so") {
                        libs.push(lib_path.to_string());
                    }
                }
            }
        }
        
        println!("📚 ldd found {} libraries:", libs.len());
        for lib in &libs {
            println!("  {}", lib);
        }
        
        Ok(libs)
    }

    fn analyze_strace_capture(&self) -> Result<(), String> {
        let strace_content = std::fs::read_to_string("/tmp/nix_strace.log")
            .map_err(|e| format!("Failed to read strace log: {}", e))?;
            
        let execve_count = strace_content.matches("execve(").count();
        let openat_count = strace_content.matches("openat(").count();
        
        println!("✅ STRACE CAPTURE ANALYSIS:");
        println!("  📋 execve calls: {}", execve_count);
        println!("  📂 openat calls: {}", openat_count);
        println!("  📄 Full log: /tmp/nix_strace.log");
        
        Ok(())
    }

    pub fn build_rust_nightly(&self) -> Result<String, String> {
        let result = self.build_with_full_preload(&["build", "./rustc-only-build", "--print-out-paths"])?;
        
        if result.success {
            Ok(result.stdout.trim().to_string())
        } else {
            Err(format!("Nix build failed: {}", result.stderr))
        }
    }
}

#[derive(Debug)]
pub struct NixBuildResult {
    pub success: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

// Service wrapper for async usage
pub struct NixService {
    builder: NixBuilder,
}

impl NixService {
    pub fn new() -> Self {
        Self {
            builder: NixBuilder::new(),
        }
    }

    pub async fn build_async(&self, args: Vec<String>) -> Result<NixBuildResult, String> {
        let builder = NixBuilder::new();
        tokio::task::spawn_blocking(move || {
            let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            builder.build(&args_refs)
        }).await.map_err(|e| format!("Async error: {}", e))?
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nix_builder() {
        let builder = NixBuilder::new();
        let result = builder.build(&["--version"]);
        assert!(result.is_ok());
    }
}
