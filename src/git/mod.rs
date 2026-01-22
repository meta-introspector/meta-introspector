//! Canonical git command wrappers
use std::process::Command;

pub fn add(files: &[&str]) -> std::io::Result<()> {
    Command::new("scripts/git/add.sh").args(files).status()?;
    Ok(())
}

pub fn commit(message: &str) -> std::io::Result<()> {
    Command::new("scripts/git/commit.sh").arg("-m").arg(message).status()?;
    Ok(())
}
