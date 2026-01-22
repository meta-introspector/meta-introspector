//! Canonical build command wrappers
use std::process::Command;

pub fn nix_build(args: &[&str]) -> std::io::Result<()> {
    Command::new("scripts/build/nix.sh").args(args).status()?;
    Ok(())
}

pub fn cargo_build(args: &[&str]) -> std::io::Result<()> {
    Command::new("scripts/build/cargo.sh").args(args).status()?;
    Ok(())
}
