# Find/Grep Usage Audit - Complete Codebase

## Executive Summary

Comprehensive audit of `find` and `grep` usage across Rust, Nix, and Shell code.

### Statistics
- **Shell scripts**: 173 files, 1074 `find` + 353 `grep` operations
- **Rust files**: 112 files with Command::new usage, 23 with fs::read_dir/walkdir
- **Nix files**: 1 file with explicit grep dependency
- **Total**: 286+ files need review/migration

## 1. Shell Script Usage (DONE ✅)

### Status
- ✅ Centralized in `lib/search_utils.sh`
- ✅ 30+ utility functions created
- ✅ Migration guide complete
- ✅ Example migration done (`quick-find.sh`)

### Remaining Work
- [ ] Migrate 172 remaining shell scripts
- [ ] See `docs/SEARCH_UTILS_MIGRATION.md`

## 2. Rust Code Usage (NEEDS WORK ⚠️)

### 2.1 Command::new("find") Usage

**Total**: 15 files directly spawn `find` command

#### High Priority Files

1. **`real_compile_proof.rs`** (2 uses)
   ```rust
   Command::new("find")
       .arg(format!("{}/rust-build", temp_dir))
       .arg("-name").arg("*.rs")
       .arg("-type").arg("f")
   ```
   **Issue**: Spawns external process, no error handling
   **Fix**: Use `walkdir` or `glob` crate

2. **`eigenvector_word_model.rs`** (2 uses)
   ```rust
   Command::new("grep")
       .args(["-rn", &self.word, code_dir, "--include=*.rs"])
   Command::new("find")
       .args([code_dir, "-name", "*.rs"])
   ```
   **Issue**: External grep for code search
   **Fix**: Use `regex` + `walkdir`

3. **`minimal_build_server.rs`** (1 use)
   ```rust
   Command::new("grep")
       .args(["-r", pattern, path])
   ```
   **Issue**: Web server spawning grep
   **Fix**: Use `grep-rs` or `ripgrep` library

4. **`dataset-indexer.rs`** (2 uses)
   ```rust
   Command::new("find")
       .arg(dataset_dir)
       .arg("-name").arg("*.parquet")
   ```
   **Fix**: Use `glob::glob("**/*.parquet")`

5. **`ordered_decl_compressor.rs`** (1 use)
   ```rust
   Command::new("find")
       .arg(dir)
       .arg("-name").arg("*.rs")
   ```
   **Fix**: Use `walkdir`

#### Medium Priority (10 more files)

- `git-activity-collector/src/main.rs` - 2 find commands
- `demos/archived/demo_swarm_hunt.rs` - 2 find commands
- `dynamic-library-interceptor/src/lib.rs` - 1 find
- `demos/archived/demo_git_pack_market.rs` - 1 find
- `demos/archived/demo_scan_git_packs.rs` - 1 find
- `cascading-repo-analyzer.rs` - 1 find
- `save_compressed_data.rs` - 1 find
- `size_histogram.rs` - 1 find
- `https_commit_fetcher.rs` - 1 find
- `compression_report.rs` - 1 find

### 2.2 fs::read_dir Usage

**Total**: 23 files use `std::fs::read_dir`

#### Good Examples ✅

1. **`canonical_directory_walker.rs`** - PERFECT!
   ```rust
   pub struct DirectoryWalker {
       max_depth: Option<usize>,
       follow_symlinks: bool,
       filter: Option<Box<dyn Fn(&Path) -> bool>>,
   }
   ```
   **Status**: This is the RIGHT way - centralized walker
   **Action**: Use this everywhere!

2. **`nixso2wrap/src/nix_scanner.rs`** - Good use of walkdir
   ```rust
   for entry in WalkDir::new(store_path)
       .follow_links(false)
       .into_iter()
       .filter_map(|e| e.ok())
   ```
   **Status**: Proper use of walkdir crate
   **Action**: Keep as-is

#### Needs Refactoring ⚠️

Files using raw `fs::read_dir` that should use `DirectoryWalker`:

1. `compression_study.rs` - 2 uses
2. `compression_loader.rs` - 2 uses
3. `unified_nix_service.rs` - 2 uses
4. `conformal_structure_analyzer.rs` - 1 use + glob
5. `src/holistic_mapper.rs` - 2 uses
6. `conformal_structure_analyzer_fixed.rs` - 2 uses
7. `rustc_perf_scanner.rs` - 1 use
8. `nix_as_a_service.rs` - 1 use
9. `flake-71-perf-collector/src/main.rs` - 1 use
10. `syn_spectrum.rs` - 1 use
11. `src/lib.rs` - 1 use
12. `recursive_rustc_wrapper.rs` - 1 use

### 2.3 glob::glob Usage

**Total**: 5 files use `glob` crate

#### Files
1. `conformal_structure_analyzer.rs` - Good use
2. `archive_declarations.rs` - Good use for pattern matching
3. `duplicate_finder.rs` - `glob::glob("**/*.rs")`
4. `lmfdb-self-analyzer/src/bin/quick_char_extract.rs`
5. `lmfdb-self-analyzer/src/bin/extract_actual_chars.rs`

**Status**: Glob is fine for pattern matching, keep these

### 2.4 walkdir Crate Usage

**Total**: 2 files use `walkdir`

1. `nixso2wrap/src/main.rs` - ✅ Good
2. `nixso2wrap/src/nix_scanner.rs` - ✅ Good

**Status**: Proper usage, keep as-is

## 3. Nix Code Usage

### Files Found
1. `incomplete_experiments/keyword-searcher/flake.nix`
   ```nix
   nativeBuildInputs = [ pkgs.bash pkgs.gnugrep ];
   ```
   **Status**: Legitimate - needs grep for shell scripts
   **Action**: Keep as-is

### Analysis
- Minimal Nix usage of find/grep
- Most Nix code uses builtins (filterSource, etc.)
- No issues found

## 4. Recommended Actions

### Immediate (High Priority)

#### 1. Create Rust Search Utilities Library

```rust
// lib/search_utils.rs

use walkdir::WalkDir;
use glob::glob;
use std::path::{Path, PathBuf};

/// Find files by extension
pub fn find_by_extension(dir: &Path, ext: &str) -> Vec<PathBuf> {
    WalkDir::new(dir)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.path().extension()
            .and_then(|s| s.to_str()) == Some(ext))
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// Find Rust files
pub fn find_rust_files(dir: &Path) -> Vec<PathBuf> {
    find_by_extension(dir, "rs")
}

/// Find files matching pattern
pub fn find_by_pattern(pattern: &str) -> Result<Vec<PathBuf>, glob::PatternError> {
    Ok(glob(pattern)?
        .filter_map(Result::ok)
        .collect())
}

/// Grep in files (using regex)
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

pub struct Match {
    pub file: PathBuf,
    pub line: usize,
    pub content: String,
}
```

#### 2. Migrate High-Priority Rust Files

Priority order:
1. `real_compile_proof.rs` - Replace 2 find commands
2. `eigenvector_word_model.rs` - Replace find + grep
3. `minimal_build_server.rs` - Replace grep (security issue!)
4. `dataset-indexer.rs` - Replace 2 find commands
5. `ordered_decl_compressor.rs` - Replace find

#### 3. Standardize on DirectoryWalker

All files using raw `fs::read_dir` should use `canonical_directory_walker.rs`:

```rust
use crate::canonical_directory_walker::DirectoryWalker;

// Instead of:
for entry in std::fs::read_dir(dir)? { ... }

// Use:
let walker = DirectoryWalker::new()
    .max_depth(3)
    .filter(|p| p.extension() == Some("rs"));
let files = walker.walk(dir)?;
```

### Short-term

#### 4. Create Cargo Feature for Search

```toml
[dependencies]
walkdir = "2"
glob = "0.3"
regex = "1"
grep-searcher = { version = "0.1", optional = true }

[features]
fast-search = ["grep-searcher"]  # Use ripgrep library
```

#### 5. Add Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_find_rust_files() {
        let files = find_rust_files(Path::new("."));
        assert!(!files.is_empty());
        assert!(files.iter().all(|f| f.extension() == Some("rs")));
    }
    
    #[test]
    fn test_grep_in_files() {
        let files = vec![PathBuf::from("test.rs")];
        let matches = grep_in_files("fn main", &files).unwrap();
        assert!(!matches.is_empty());
    }
}
```

### Long-term

#### 6. Performance Optimization

- Use `rayon` for parallel directory walking
- Use `grep-searcher` (ripgrep library) for fast text search
- Add caching layer for repeated searches
- Memory-map large files

#### 7. Security Hardening

**CRITICAL**: `minimal_build_server.rs` spawns grep with user input!

```rust
// VULNERABLE:
Command::new("grep")
    .args(["-r", pattern, path])  // pattern from user!

// FIX:
use grep_searcher::Searcher;
use grep_regex::RegexMatcher;

let matcher = RegexMatcher::new(&sanitize_pattern(pattern))?;
let mut searcher = Searcher::new();
// ... safe search
```

## 5. Migration Checklist

### Rust Files - Command::new("find")
- [ ] `real_compile_proof.rs`
- [ ] `eigenvector_word_model.rs`
- [ ] `dataset-indexer.rs`
- [ ] `ordered_decl_compressor.rs`
- [ ] `git-activity-collector/src/main.rs`
- [ ] `demos/archived/demo_swarm_hunt.rs`
- [ ] `dynamic-library-interceptor/src/lib.rs`
- [ ] `demos/archived/demo_git_pack_market.rs`
- [ ] `demos/archived/demo_scan_git_packs.rs`
- [ ] `cascading-repo-analyzer.rs`
- [ ] `save_compressed_data.rs`
- [ ] `size_histogram.rs`
- [ ] `https_commit_fetcher.rs`
- [ ] `compression_report.rs`
- [ ] `untracked_by_dir.rs`

### Rust Files - Command::new("grep")
- [ ] `eigenvector_word_model.rs` (HIGH PRIORITY - security)
- [ ] `minimal_build_server.rs` (CRITICAL - user input!)
- [ ] `lmfdb-self-analyzer/src/bin/find_unique_instructions.rs`

### Rust Files - fs::read_dir → DirectoryWalker
- [ ] `compression_study.rs`
- [ ] `compression_loader.rs`
- [ ] `unified_nix_service.rs`
- [ ] `conformal_structure_analyzer.rs`
- [ ] `src/holistic_mapper.rs`
- [ ] `conformal_structure_analyzer_fixed.rs`
- [ ] `rustc_perf_scanner.rs`
- [ ] `nix_as_a_service.rs`
- [ ] `flake-71-perf-collector/src/main.rs`
- [ ] `syn_spectrum.rs`
- [ ] `src/lib.rs`
- [ ] `recursive_rustc_wrapper.rs`

### Shell Scripts (173 files)
- [x] `quick-find.sh` (DONE)
- [ ] 172 remaining (see `docs/SEARCH_UTILS_MIGRATION.md`)

## 6. Benefits of Migration

### Performance
- **Before**: Spawn process, fork, exec, parse output
- **After**: Native Rust, no process overhead
- **Speedup**: 10-100x for small searches

### Security
- **Before**: Shell injection risks with user input
- **After**: Type-safe, no shell involved
- **Risk**: Eliminated

### Reliability
- **Before**: Depends on external tools being installed
- **After**: Pure Rust, always available
- **Portability**: Windows, Linux, macOS

### Maintainability
- **Before**: String parsing, brittle
- **After**: Structured data, type-safe
- **Errors**: Proper error handling

## 7. Code Examples

### Before (Bad)
```rust
let output = Command::new("find")
    .arg(dir)
    .arg("-name")
    .arg("*.rs")
    .output()?;
let files: Vec<_> = String::from_utf8_lossy(&output.stdout)
    .lines()
    .map(PathBuf::from)
    .collect();
```

### After (Good)
```rust
use crate::search_utils::find_rust_files;

let files = find_rust_files(Path::new(dir));
```

### Before (Dangerous)
```rust
// User input directly to shell!
Command::new("grep")
    .args(["-r", user_pattern, path])
```

### After (Safe)
```rust
use grep_searcher::{Searcher, Sink};
use grep_regex::RegexMatcher;

let matcher = RegexMatcher::new_line_matcher(&user_pattern)?;
let mut searcher = Searcher::new();
searcher.search_path(&matcher, path, sink)?;
```

## 8. Dependencies to Add

```toml
[dependencies]
walkdir = "2.4"           # Directory traversal
glob = "0.3"              # Pattern matching
regex = "1.10"            # Text search
grep-searcher = "0.1"     # Fast grep (optional)
grep-regex = "0.1"        # Regex for grep
rayon = "1.8"             # Parallel processing
```

## 9. Testing Strategy

### Unit Tests
- Test each utility function
- Test edge cases (empty dirs, symlinks, permissions)
- Test pattern matching

### Integration Tests
- Compare output with actual find/grep
- Benchmark performance
- Test on real codebase

### Security Tests
- Test with malicious patterns
- Test path traversal attempts
- Test resource exhaustion

## 10. Timeline

### Week 1
- [x] Audit complete
- [ ] Create `lib/search_utils.rs`
- [ ] Migrate 5 high-priority Rust files
- [ ] Add tests

### Week 2
- [ ] Migrate remaining 10 Rust Command::new files
- [ ] Migrate 12 fs::read_dir files to DirectoryWalker
- [ ] Security audit of minimal_build_server.rs

### Week 3-4
- [ ] Migrate 172 shell scripts
- [ ] Performance benchmarks
- [ ] Documentation

## 11. Success Metrics

- ✅ Zero Command::new("find") in Rust code
- ✅ Zero Command::new("grep") in Rust code
- ✅ All fs::read_dir use DirectoryWalker
- ✅ All shell scripts use lib/search_utils.sh
- ✅ 10x+ performance improvement
- ✅ Zero security vulnerabilities
- ✅ 100% test coverage

## 12. Related Work

- `lib/search_utils.sh` - Shell utilities (DONE)
- `canonical_directory_walker.rs` - Rust walker (EXISTS)
- `docs/SEARCH_UTILS_MIGRATION.md` - Shell migration guide
- `docs/FIND_GREP_CENTRALIZATION.md` - Shell summary

---

**Status**: Audit Complete ✅  
**Next**: Create `lib/search_utils.rs` and migrate high-priority files  
**Priority**: HIGH (security issue in minimal_build_server.rs)
