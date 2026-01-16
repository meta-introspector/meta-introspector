# Refactoring Plan: Remove Hardcoded Data and Improve Code Quality

## Priority 1: Remove Hardcoded Paths

### Files to Fix:
1. `rust_type_markov_generator.rs` - Line 108: `/home/mdupont/zombie_driver2`
2. `struct_instance_markov.rs` - Line 185: `/home/mdupont/zombie_driver2`

### Solution: Environment-Based Configuration

```rust
use std::env;
use std::path::PathBuf;

fn get_source_directory() -> Result<PathBuf, Box<dyn std::error::Error>> {
    if let Ok(dir) = env::var("RUST_ANALYSIS_DIR") {
        Ok(PathBuf::from(dir))
    } else if let Ok(current_dir) = env::current_dir() {
        Ok(current_dir.join("data/repos"))
    } else {
        Err("Cannot determine source directory".into())
    }
}
```

### Implementation Steps:
1. **Create common config module** (`src/config.rs`)
2. **Replace hardcoded paths** in both files
3. **Add command-line argument parsing** using `clap`
4. **Update documentation** with environment variable usage

## Priority 2: Improve Error Handling

### Current Issues:
- Silent failures on file read errors
- No validation of directory existence
- Parse errors ignored without logging

### Solution: Comprehensive Error Handling

```rust
use std::fs;
use std::path::Path;
use log::{warn, error, info};

fn process_rust_files<P: AsRef<Path>>(dir: P) -> Result<ProcessingStats, Box<dyn std::error::Error>> {
    let dir = dir.as_ref();
    if !dir.exists() {
        return Err(format!("Directory does not exist: {}", dir.display()).into());
    }

    let mut stats = ProcessingStats::new();
    
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().map_or(false, |ext| ext == "rs") {
            match process_single_file(&path) {
                Ok(_) => {
                    stats.successful_files += 1;
                    info!("Processed: {}", path.display());
                }
                Err(e) => {
                    stats.failed_files += 1;
                    warn!("Failed to process {}: {}", path.display(), e);
                }
            }
        }
    }
    
    Ok(stats)
}
```

## Priority 3: Remove Code Duplication

### Duplicate Code Identified:
1. **AST Visitor Logic** - Similar patterns in both markov generators
2. **File Processing** - Identical directory traversal code
3. **JSON Serialization** - Repeated model saving logic

### Solution: Extract Common Traits

```rust
// src/analysis/mod.rs
pub trait RustAnalyzer {
    type Output: Serialize;
    
    fn analyze_file(&mut self, file_path: &Path, content: &str) -> Result<(), AnalysisError>;
    fn get_results(&self) -> &Self::Output;
    fn save_results(&self, output_path: &Path) -> Result<(), std::io::Error>;
}

// src/analysis/file_processor.rs
pub struct FileProcessor<A: RustAnalyzer> {
    analyzer: A,
    stats: ProcessingStats,
}

impl<A: RustAnalyzer> FileProcessor<A> {
    pub fn process_directory<P: AsRef<Path>>(&mut self, dir: P) -> Result<(), ProcessingError> {
        // Common directory processing logic
    }
}
```

## Priority 4: Configuration System

### Create `analysis.toml`:
```toml
[paths]
source_directory = "./data/repos"
output_directory = "./analysis/results"
temp_directory = "/tmp/rust_analysis"

[processing]
max_files = 10000
batch_size = 100
parallel_workers = 20

[logging]
level = "info"
file = "./logs/analysis.log"

[markov]
max_transitions = 1000
min_instances = 5
character_level = true

[compression]
algorithm = "sequitur"
compression_level = 9
enable_querying = true
```

### Configuration Loading:
```rust
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct AnalysisConfig {
    pub paths: PathConfig,
    pub processing: ProcessingConfig,
    pub logging: LoggingConfig,
    pub markov: MarkovConfig,
    pub compression: CompressionConfig,
}

impl AnalysisConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let config_path = std::env::var("ANALYSIS_CONFIG")
            .unwrap_or_else(|_| "analysis.toml".to_string());
        
        let content = fs::read_to_string(&config_path)
            .map_err(|e| ConfigError::FileRead(config_path, e))?;
        
        toml::from_str(&content)
            .map_err(ConfigError::Parse)
    }
}
```

## Priority 5: Memory Optimization

### Current Issues:
- All models loaded in memory simultaneously
- No streaming for large datasets
- Potential memory exhaustion on massive codebases

### Solution: Streaming Analysis

```rust
use std::io::{BufWriter, Write};

pub struct StreamingAnalyzer<W: Write> {
    writer: BufWriter<W>,
    current_batch: Vec<AnalysisResult>,
    batch_size: usize,
}

impl<W: Write> StreamingAnalyzer<W> {
    pub fn new(writer: W, batch_size: usize) -> Self {
        Self {
            writer: BufWriter::new(writer),
            current_batch: Vec::with_capacity(batch_size),
            batch_size,
        }
    }
    
    pub fn add_result(&mut self, result: AnalysisResult) -> Result<(), std::io::Error> {
        self.current_batch.push(result);
        
        if self.current_batch.len() >= self.batch_size {
            self.flush_batch()?;
        }
        
        Ok(())
    }
    
    fn flush_batch(&mut self) -> Result<(), std::io::Error> {
        for result in &self.current_batch {
            writeln!(self.writer, "{}", serde_json::to_string(result)?)?;
        }
        self.current_batch.clear();
        self.writer.flush()
    }
}
```

## Priority 6: Testing Framework

### Unit Tests:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[test]
    fn test_markov_generation() {
        let temp_dir = TempDir::new().unwrap();
        let rust_file = temp_dir.path().join("test.rs");
        
        fs::write(&rust_file, r#"
            fn main() {
                let x = 42;
                let y = "hello";
            }
        "#).unwrap();
        
        let mut analyzer = TypeCollector::new();
        let result = analyzer.process_directory(temp_dir.path());
        
        assert!(result.is_ok());
        assert!(analyzer.models.contains_key("integer"));
        assert!(analyzer.models.contains_key("String"));
    }
    
    #[test]
    fn test_compression_ratio() {
        // Test compression achieves expected ratios
    }
    
    #[test]
    fn test_error_handling() {
        // Test graceful handling of malformed files
    }
}
```

### Integration Tests:
```rust
#[test]
fn test_full_pipeline() {
    // Test complete analysis pipeline
    // Verify output format and compression ratios
    // Test with various repository structures
}
```

## Implementation Timeline

### Week 1: Core Refactoring
- [ ] Remove hardcoded paths from both markov generators
- [ ] Implement configuration system
- [ ] Add comprehensive error handling
- [ ] Create common analysis traits

### Week 2: Optimization & Testing
- [ ] Implement streaming analysis
- [ ] Add memory usage monitoring
- [ ] Create comprehensive test suite
- [ ] Performance benchmarking

### Week 3: Documentation & Polish
- [ ] Update all documentation
- [ ] Add usage examples
- [ ] Create deployment scripts
- [ ] Performance tuning

## Validation Criteria

### Functionality:
- [ ] All hardcoded paths removed
- [ ] Configuration-driven operation
- [ ] Graceful error handling
- [ ] Memory usage under 4GB for large repositories

### Performance:
- [ ] Maintain 97% compression ratios
- [ ] Processing speed ≥ 500 files/second
- [ ] Memory usage scales linearly with batch size
- [ ] Parallel processing efficiency ≥ 80%

### Code Quality:
- [ ] Zero code duplication in core logic
- [ ] 100% test coverage for critical paths
- [ ] All clippy warnings resolved
- [ ] Documentation coverage ≥ 90%

## Risk Mitigation

### Backward Compatibility:
- Maintain existing JSON output formats
- Support legacy command-line interfaces
- Gradual migration path for existing users

### Performance Regression:
- Benchmark before/after refactoring
- Profile memory usage patterns
- Load testing with large repositories

### Data Integrity:
- Validate compression ratios remain consistent
- Verify analysis accuracy with known datasets
- Test edge cases and error conditions

This refactoring plan addresses all identified issues while maintaining the breakthrough compression capabilities and adding robust production-ready features.
