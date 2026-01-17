//! # Canonical Directory Walker
//! 
//! THE ONLY PLACE for directory traversal.
//! Centralizes all walkdir/read_dir patterns.

use std::path::{Path, PathBuf};
use std::fs;

pub struct DirectoryWalker {
    max_depth: Option<usize>,
    follow_symlinks: bool,
    filter: Option<Box<dyn Fn(&Path) -> bool>>,
}

impl DirectoryWalker {
    pub fn new() -> Self {
        Self {
            max_depth: None,
            follow_symlinks: false,
            filter: None,
        }
    }
    
    pub fn max_depth(mut self, depth: usize) -> Self {
        self.max_depth = Some(depth);
        self
    }
    
    pub fn follow_symlinks(mut self, follow: bool) -> Self {
        self.follow_symlinks = follow;
        self
    }
    
    pub fn filter<F>(mut self, f: F) -> Self 
    where F: Fn(&Path) -> bool + 'static {
        self.filter = Some(Box::new(f));
        self
    }
    
    /// Walk directory and collect all matching paths
    pub fn walk(&self, root: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
        let mut results = Vec::new();
        self.walk_recursive(root, 0, &mut results)?;
        Ok(results)
    }
    
    fn walk_recursive(&self, dir: &Path, depth: usize, results: &mut Vec<PathBuf>) -> Result<(), std::io::Error> {
        if let Some(max) = self.max_depth {
            if depth > max {
                return Ok(());
            }
        }
        
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            // Apply filter
            if let Some(ref filter) = self.filter {
                if !filter(&path) {
                    continue;
                }
            }
            
            results.push(path.clone());
            
            if path.is_dir() {
                if self.follow_symlinks || !path.is_symlink() {
                    self.walk_recursive(&path, depth + 1, results)?;
                }
            }
        }
        
        Ok(())
    }
}

impl Default for DirectoryWalker {
    fn default() -> Self {
        Self::new()
    }
}

/// Find all .so files in /nix/store
pub fn find_all_so_files() -> Result<Vec<PathBuf>, std::io::Error> {
    println!("🔍 Scanning /nix/store for .so files...");
    
    let walker = DirectoryWalker::new()
        .max_depth(3) // Don't go too deep
        .filter(|p| {
            p.extension()
                .and_then(|e| e.to_str())
                .map(|e| e == "so")
                .unwrap_or(false)
        });
    
    walker.walk(Path::new("/nix/store"))
}

/// Find all files matching pattern
pub fn find_files_matching<F>(root: &Path, predicate: F) -> Result<Vec<PathBuf>, std::io::Error>
where F: Fn(&Path) -> bool + 'static {
    DirectoryWalker::new()
        .filter(predicate)
        .walk(root)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_walker() {
        let walker = DirectoryWalker::new().max_depth(1);
        let results = walker.walk(Path::new(".")).unwrap();
        assert!(!results.is_empty());
    }
}

fn main() {
    println!("canonical_directory_walker - add usage here");
}
