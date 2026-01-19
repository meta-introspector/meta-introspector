//! Centralized search utilities for Rust code
//! 
//! Replaces all Command::new("find") and Command::new("grep") with native Rust.
//! 
//! # Usage
//! 
//! ```rust
//! use meta_introspector::search_utils::*;
//! 
//! // Find Rust files
//! let files = find_rust_files(".")?;
//! 
//! // Find by extension
//! let toml_files = find_by_extension(".", "toml")?;
//! 
//! // Find by pattern
//! let configs = find_by_pattern("**/*.{toml,yaml}")?;
//! 
//! // Grep in files
//! let matches = grep_in_files("fn main", &files)?;
//! ```

use std::path::{Path, PathBuf};
use std::io;
use walkdir::WalkDir;

/// Find all files with given extension
pub fn find_by_extension<P: AsRef<Path>>(dir: P, ext: &str) -> io::Result<Vec<PathBuf>> {
    Ok(WalkDir::new(dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension()
            .and_then(|s| s.to_str()) == Some(ext))
        .map(|e| e.path().to_path_buf())
        .collect())
}

/// Find all Rust files (.rs)
pub fn find_rust_files<P: AsRef<Path>>(dir: P) -> io::Result<Vec<PathBuf>> {
    find_by_extension(dir, "rs")
}

/// Find all Nix files (.nix)
pub fn find_nix_files<P: AsRef<Path>>(dir: P) -> io::Result<Vec<PathBuf>> {
    find_by_extension(dir, "nix")
}

/// Find all TOML files (.toml)
pub fn find_toml_files<P: AsRef<Path>>(dir: P) -> io::Result<Vec<PathBuf>> {
    find_by_extension(dir, "toml")
}

/// Find files matching glob pattern
pub fn find_by_pattern(pattern: &str) -> Result<Vec<PathBuf>, glob::PatternError> {
    Ok(glob::glob(pattern)?
        .filter_map(Result::ok)
        .collect())
}

/// Find files by name (exact match)
pub fn find_by_name<P: AsRef<Path>>(dir: P, name: &str) -> io::Result<Vec<PathBuf>> {
    Ok(WalkDir::new(dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.file_name().to_str() == Some(name))
        .map(|e| e.path().to_path_buf())
        .collect())
}

/// Find Cargo.toml files
pub fn find_cargo_tomls<P: AsRef<Path>>(dir: P) -> io::Result<Vec<PathBuf>> {
    find_by_name(dir, "Cargo.toml")
}

/// Find flake.nix files
pub fn find_flakes<P: AsRef<Path>>(dir: P) -> io::Result<Vec<PathBuf>> {
    find_by_name(dir, "flake.nix")
}

/// Find files by multiple extensions
pub fn find_by_extensions<P: AsRef<Path>>(dir: P, exts: &[&str]) -> io::Result<Vec<PathBuf>> {
    Ok(WalkDir::new(dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path().extension()
                .and_then(|s| s.to_str())
                .map(|ext| exts.contains(&ext))
                .unwrap_or(false)
        })
        .map(|e| e.path().to_path_buf())
        .collect())
}

/// Match result from grep
#[derive(Debug, Clone)]
pub struct Match {
    pub file: PathBuf,
    pub line: usize,
    pub content: String,
}

/// Grep for pattern in files using regex
pub fn grep_in_files(pattern: &str, files: &[PathBuf]) -> Result<Vec<Match>, regex::Error> {
    use regex::Regex;
    let re = Regex::new(pattern)?;
    let mut matches = Vec::new();
    
    for file in files {
        if let Ok(content) = std::fs::read_to_string(file) {
            for (line_num, line) in content.lines().enumerate() {
                if re.is_match(line) {
                    matches.push(Match {
                        file: file.clone(),
                        line: line_num + 1,
                        content: line.to_string(),
                    });
                }
            }
        }
    }
    
    Ok(matches)
}

/// Grep for pattern in directory (recursive)
pub fn grep_in_dir<P: AsRef<Path>>(pattern: &str, dir: P, ext: Option<&str>) -> Result<Vec<Match>, regex::Error> {
    let files = if let Some(extension) = ext {
        find_by_extension(dir, extension).unwrap_or_default()
    } else {
        WalkDir::new(dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().to_path_buf())
            .collect()
    };
    
    grep_in_files(pattern, &files)
}

/// Count matches of pattern in file
pub fn grep_count<P: AsRef<Path>>(pattern: &str, file: P) -> Result<usize, regex::Error> {
    use regex::Regex;
    let re = Regex::new(pattern)?;
    
    if let Ok(content) = std::fs::read_to_string(file) {
        Ok(content.lines().filter(|line| re.is_match(line)).count())
    } else {
        Ok(0)
    }
}

/// Find files containing pattern
pub fn find_files_with_pattern<P: AsRef<Path>>(pattern: &str, dir: P, ext: Option<&str>) -> Result<Vec<PathBuf>, regex::Error> {
    use regex::Regex;
    let re = Regex::new(pattern)?;
    
    let files = if let Some(extension) = ext {
        find_by_extension(dir, extension).unwrap_or_default()
    } else {
        WalkDir::new(dir)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .map(|e| e.path().to_path_buf())
            .collect()
    };
    
    Ok(files.into_iter()
        .filter(|file| {
            if let Ok(content) = std::fs::read_to_string(file) {
                re.is_match(&content)
            } else {
                false
            }
        })
        .collect())
}

/// Find with max depth
pub fn find_with_depth<P: AsRef<Path>>(dir: P, ext: &str, max_depth: usize) -> io::Result<Vec<PathBuf>> {
    Ok(WalkDir::new(dir)
        .max_depth(max_depth)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension()
            .and_then(|s| s.to_str()) == Some(ext))
        .map(|e| e.path().to_path_buf())
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_find_rust_files() {
        let files = find_rust_files(".").unwrap();
        assert!(!files.is_empty());
        assert!(files.iter().all(|f| f.extension().unwrap() == "rs"));
    }

    #[test]
    fn test_find_by_extension() {
        let temp = TempDir::new().unwrap();
        fs::write(temp.path().join("test.rs"), "fn main() {}").unwrap();
        fs::write(temp.path().join("test.toml"), "[package]").unwrap();
        
        let rs_files = find_by_extension(temp.path(), "rs").unwrap();
        assert_eq!(rs_files.len(), 1);
        
        let toml_files = find_by_extension(temp.path(), "toml").unwrap();
        assert_eq!(toml_files.len(), 1);
    }

    #[test]
    fn test_grep_in_files() {
        let temp = TempDir::new().unwrap();
        let test_file = temp.path().join("test.rs");
        fs::write(&test_file, "fn main() {\n    println!(\"hello\");\n}").unwrap();
        
        let matches = grep_in_files("fn main", &[test_file]).unwrap();
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].line, 1);
    }

    #[test]
    fn test_grep_count() {
        let temp = TempDir::new().unwrap();
        let test_file = temp.path().join("test.rs");
        fs::write(&test_file, "fn main() {}\nfn test() {}\nfn other() {}").unwrap();
        
        let count = grep_count("fn ", &test_file).unwrap();
        assert_eq!(count, 3);
    }

    #[test]
    fn test_find_files_with_pattern() {
        let temp = TempDir::new().unwrap();
        fs::write(temp.path().join("has_main.rs"), "fn main() {}").unwrap();
        fs::write(temp.path().join("no_main.rs"), "fn test() {}").unwrap();
        
        let files = find_files_with_pattern("fn main", temp.path(), Some("rs")).unwrap();
        assert_eq!(files.len(), 1);
        assert!(files[0].ends_with("has_main.rs"));
    }
}
