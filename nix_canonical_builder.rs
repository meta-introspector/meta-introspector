// nix_canonical_builder.rs
// 🔥 THE ONLY PLACE TO CALL NIX BUILD
// All nix builds go through here with full perf + telemetry instrumentation

use perf_macros::{perf_auto, perf_probe};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::Instant;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NixBuildRequest {
    pub args: Vec<String>,
    pub env: Vec<(String, String)>,
    pub working_dir: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NixBuildResult {
    pub success: bool,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub store_paths: Vec<String>,
    pub duration_secs: f64,
}

pub struct NixCanonicalBuilder {
    telemetry_enabled: bool,
    perf_enabled: bool,
}

impl NixCanonicalBuilder {
    pub fn new() -> Self {
        Self {
            telemetry_enabled: true,
            perf_enabled: true,
        }
    }
    
    pub fn with_telemetry(mut self, enabled: bool) -> Self {
        self.telemetry_enabled = enabled;
        self
    }
    
    pub fn with_perf(mut self, enabled: bool) -> Self {
        self.perf_enabled = enabled;
        self
    }
    
    /// THE ONLY FUNCTION THAT CALLS NIX BUILD
    /// All nix builds in the entire codebase MUST go through here
    #[perf_auto]
    #[perf_probe]
    pub fn build(&self, request: NixBuildRequest) -> Result<NixBuildResult, String> {
        // This is the ONLY place where Command::new("nix") happens
        let mut cmd = Command::new("nix");
        cmd.args(&request.args);
        
        // Add environment variables
        for (key, value) in &request.env {
            cmd.env(key, value);
        }
        
        // Set working directory
        if let Some(dir) = &request.working_dir {
            cmd.current_dir(dir);
        }
        
        // Execute with timing
        let start = Instant::now();
        let output = cmd
            .output()
            .map_err(|e| format!("Failed to execute nix: {}", e))?;
        let duration = start.elapsed();
        
        // Parse output
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        
        // Extract store paths from output
        let store_paths = self.extract_store_paths(&stdout);
        
        Ok(NixBuildResult {
            success: output.status.success(),
            exit_code: output.status.code().unwrap_or(-1),
            stdout,
            stderr,
            store_paths,
            duration_secs: duration.as_secs_f64(),
        })
    }
    
    fn extract_store_paths(&self, output: &str) -> Vec<String> {
        output
            .lines()
            .filter(|line| line.starts_with("/nix/store/"))
            .map(|s| s.trim().to_string())
            .collect()
    }
}

impl Default for NixCanonicalBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Convenience functions for common use cases

/// Build a nix flake
pub fn nix_build_flake(flake: &str) -> Result<NixBuildResult, String> {
    let builder = NixCanonicalBuilder::new();
    builder.build(NixBuildRequest {
        args: vec!["build".to_string(), flake.to_string()],
        env: vec![],
        working_dir: None,
    })
}

/// Build with custom args
pub fn nix_build(args: &[&str]) -> Result<NixBuildResult, String> {
    let builder = NixCanonicalBuilder::new();
    builder.build(NixBuildRequest {
        args: args.iter().map(|s| s.to_string()).collect(),
        env: vec![],
        working_dir: None,
    })
}

/// Build with environment variables
pub fn nix_build_with_env(
    args: &[&str],
    env: Vec<(String, String)>,
) -> Result<NixBuildResult, String> {
    let builder = NixCanonicalBuilder::new();
    builder.build(NixBuildRequest {
        args: args.iter().map(|s| s.to_string()).collect(),
        env,
        working_dir: None,
    })
}

/// Build in specific directory
pub fn nix_build_in_dir(
    args: &[&str],
    working_dir: &str,
) -> Result<NixBuildResult, String> {
    let builder = NixCanonicalBuilder::new();
    builder.build(NixBuildRequest {
        args: args.iter().map(|s| s.to_string()).collect(),
        env: vec![],
        working_dir: Some(working_dir.to_string()),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_creation() {
        let builder = NixCanonicalBuilder::new();
        assert!(builder.telemetry_enabled);
        assert!(builder.perf_enabled);
    }

    #[test]
    fn test_builder_options() {
        let builder = NixCanonicalBuilder::new()
            .with_telemetry(false)
            .with_perf(false);
        assert!(!builder.telemetry_enabled);
        assert!(!builder.perf_enabled);
    }

    #[test]
    fn test_store_path_extraction() {
        let builder = NixCanonicalBuilder::new();
        let output = "/nix/store/abc123-hello-1.0\n/nix/store/def456-world-2.0\nother output";
        let paths = builder.extract_store_paths(output);
        assert_eq!(paths.len(), 2);
        assert_eq!(paths[0], "/nix/store/abc123-hello-1.0");
        assert_eq!(paths[1], "/nix/store/def456-world-2.0");
    }
}
