//! Canonical Rust perf recording library
//! All Rust code should use this instead of raw perf record commands

use std::process::Command;

/// Record perf data for a command
/// 
/// # Example
/// ```no_run
/// use crate::perf;
/// perf::record("my-build", &["cargo", "build", "--release"]);
/// ```
pub fn record(name: &str, command: &[&str]) -> std::io::Result<()> {
    let output_file = format!("{}.perf.data", name);
    
    Command::new("perf")
        .arg("record")
        .arg("-o")
        .arg(&output_file)
        .arg("-F")
        .arg("99")
        .arg("-g")
        .arg("--call-graph")
        .arg("dwarf")
        .arg("--")
        .args(command)
        .status()?;
    
    eprintln!("Perf data: {}", output_file);
    Ok(())
}

/// Use nix perf-lib for reproducible recording
pub fn record_with_nix(target: &str) -> std::io::Result<()> {
    Command::new("nix")
        .arg("run")
        .arg("github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix#perf-build")
        .arg("--")
        .arg(target)
        .status()?;
    
    Ok(())
}
