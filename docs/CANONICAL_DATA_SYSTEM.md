# Canonical Data System

**Location**: `canonical_data_store.rs` + `data_registry.rs`  
**Purpose**: Centralized data input/output with Parquet format and crossbeam workers

## Overview

We have TWO canonical systems that should be used by ALL data-producing programs:

1. **canonical_data_store.rs** - Low-level crossbeam + Parquet writer
2. **data_registry.rs** - High-level dataset and pipeline registry

## Architecture

```
Program → CanonicalDataStore → Parquet File → DataRegistry
         (crossbeam workers)   (100K batches)  (metadata)
```

## 1. Canonical Data Store (canonical_data_store.rs)

### Purpose
- THE ONLY PLACE for crossbeam + parquet data storage
- Reuses proven patterns from markov_resonance_analyzer
- Handles parallel data collection and batch writing

### Usage Pattern

```rust
use canonical_data_store::{CanonicalDataStore, save_grammar_parquet};

// Create store with crossbeam channels
let store = CanonicalDataStore::<MyDataType>::new(100);
let sender = store.sender();
let receiver = store.receiver();

// Spawn workers
for _ in 0..store.num_workers() {
    let rx = receiver.clone();
    thread::spawn(move || {
        while let Ok(data) = rx.recv() {
            // Process data
        }
    });
}

// Collect results and save to Parquet
let results: Vec<(String, String, u64, usize, f64)> = collect_results();
save_grammar_parquet(&results, "output.parquet")?;
```

### Key Features
- **Crossbeam channels**: Bounded channels for backpressure
- **Worker pool**: 2x CPU cores (optimized for I/O)
- **Batch writing**: 100K rows per batch (proven optimal)
- **Parquet format**: Columnar storage with compression

### Parquet Schema Template

```rust
let schema = Arc::new(Schema::new(vec![
    Field::new("column1", DataType::Utf8, false),
    Field::new("column2", DataType::UInt64, false),
    Field::new("column3", DataType::Float64, false),
]));
```

## 2. Data Registry (data_registry.rs)

### Purpose
- Central registry of all datasets and pipelines
- Tracks metadata, paths, formats, sizes
- Manages data lineage and dependencies

### Data Paths

```rust
use data_registry::DataPaths;

DataPaths::REGISTRY      // "data/registry.json"
DataPaths::RAW           // "data/raw"
DataPaths::PROCESSED     // "data/processed"
DataPaths::CACHE         // "data/cache"
DataPaths::ANALYSIS      // "data/analysis"
DataPaths::COMPRESSED    // "data/compressed"
DataPaths::BUILD_ORDER   // "data/build_order"
```

### Usage Pattern

```rust
use data_registry::{DataRegistry, DataFormat, DataPaths};

// Initialize
DataPaths::ensure_dirs()?;
let mut registry = DataRegistry::load()?;

// Register dataset
registry.register_dataset(
    "my_dataset",
    PathBuf::from("data/analysis/my_data.parquet"),
    DataFormat::Parquet,
    "Description of dataset"
)?;

// Register pipeline
registry.register_pipeline(
    "my_pipeline",
    "my_binary",
    vec!["input_dataset".to_string()],
    vec!["output_dataset".to_string()]
)?;

// Query
if let Some(path) = registry.get_dataset_path("my_dataset") {
    // Load data from path
}
```

### DataFormat Enum

```rust
pub enum DataFormat {
    Json,
    Csv,
    Parquet,  // ADD THIS (currently missing)
    Binary,
    Archive,
}
```

## Migration Guide: JSON → Parquet

### Step 1: Update Program to Use Canonical Store

**Before:**
```rust
// Old way - direct JSON write
let data = analyze_something();
let json = serde_json::to_string_pretty(&data)?;
fs::write("output.json", json)?;
```

**After:**
```rust
// New way - canonical store
use canonical_data_store::{CanonicalDataStore, save_grammar_parquet};

let store = CanonicalDataStore::new(100);
let sender = store.sender();

// Collect data via crossbeam
let data = analyze_something_parallel(sender);

// Save to Parquet
save_grammar_parquet(&data, "data/analysis/output.parquet")?;

// Register in registry
let mut registry = DataRegistry::load()?;
registry.register_dataset(
    "output",
    PathBuf::from("data/analysis/output.parquet"),
    DataFormat::Parquet,
    "Analysis output"
)?;
```

### Step 2: Add Parquet Schema

Each program needs a schema function:

```rust
pub fn create_schema() -> Arc<Schema> {
    Arc::new(Schema::new(vec![
        Field::new("field1", DataType::Utf8, false),
        Field::new("field2", DataType::UInt64, false),
        // ... add all fields
    ]))
}
```

### Step 3: Batch Writing

```rust
pub fn save_to_parquet(
    data: &[MyStruct],
    output_path: &str
) -> Result<(), Box<dyn std::error::Error>> {
    let schema = create_schema();
    let file = fs::File::create(output_path)?;
    let mut writer = ArrowWriter::try_new(file, schema.clone(), None)?;
    
    // Write in 100K batches
    for chunk in data.chunks(100_000) {
        let batch = create_record_batch(chunk, schema.clone())?;
        writer.write(&batch)?;
    }
    
    writer.close()?;
    Ok(())
}
```

## Programs to Migrate

### High Priority (Large JSON files)

1. **markov_resonance_analyzer** ✅ Already uses canonical pattern
   - `markov_symbol_scores.json` → `markov_symbol_scores.parquet`
   
2. **git_file_mapper** (1.2GB CSV)
   - `FILE_GIT_MAPPING.csv` → `file_git_mapping.parquet`
   - Schema: file_path, git_repo, commit, branch, remote, url
   
3. **file_stats_analyzer** (37M files)
   - `_mnt_data1_files.csv` → `file_stats.parquet`
   - Schema: path, size, mtime, type, extension
   
4. **ldd2wrap_all_calls**
   - `all_calls_dataset_*.json` → `all_calls.parquet`
   - Schema: timestamp, function, library, args

5. **full_rustc_compressor**
   - `rust_build_compressed.json` → `rust_build_compressed.parquet`
   - Schema: file, original_size, compressed_size, ratio

### Medium Priority

6. **conformal_structure_analyzer**
   - `automorphism_analysis.json` → `automorphism_analysis.parquet`
   - `conformal_boundary.json` → `conformal_boundary.parquet`
   - `rustjunk_eigenvector.json` → `rustjunk_eigenvector.parquet`

7. **eigenvector_word_model**
   - `word_eigenvectors.json` → Already outputs to `data/eigenvectors/`
   - Add Parquet format

8. **rustc_syscall_proof**
   - `syscall_summary_*.csv` → `syscall_summary.parquet`
   - `execve_calls_*.json` → `execve_calls.parquet`

9. **telemetry_server**
   - `telemetry_results_*.json` → `telemetry_results.parquet`

10. **crossbeam_repo_compressor**
    - `crossbeam_repo_compression_results.json` → `compression_results.parquet`

### Low Priority (Small files)

11. All `datatype_markov_*.json` files
12. Build analysis JSON files
13. Grammar extraction JSON files

## Standard Schemas

### File Provenance Schema
```rust
Schema::new(vec![
    Field::new("file_path", DataType::Utf8, false),
    Field::new("git_repo", DataType::Utf8, true),
    Field::new("commit", DataType::Utf8, true),
    Field::new("branch", DataType::Utf8, true),
    Field::new("remote", DataType::Utf8, true),
    Field::new("url", DataType::Utf8, true),
])
```

### Telemetry Schema
```rust
Schema::new(vec![
    Field::new("timestamp", DataType::UInt64, false),
    Field::new("session_id", DataType::Utf8, false),
    Field::new("event_type", DataType::Utf8, false),
    Field::new("function", DataType::Utf8, false),
    Field::new("duration_ms", DataType::UInt64, false),
    Field::new("syscalls", DataType::Utf8, true),
])
```

### Symbol Analysis Schema
```rust
Schema::new(vec![
    Field::new("symbol", DataType::Utf8, false),
    Field::new("file_id", DataType::UInt64, false),
    Field::new("score", DataType::Float64, false),
    Field::new("frequency", DataType::UInt64, false),
])
```

## Parquet Splitter Utility

Create `parquet_splitter.rs`:

```rust
use parquet::file::reader::{FileReader, SerializedFileReader};
use parquet::arrow::ArrowWriter;
use std::fs::File;
use std::sync::Arc;

pub fn split_parquet(
    input_path: &str,
    output_prefix: &str,
    rows_per_file: usize
) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let file = File::open(input_path)?;
    let reader = SerializedFileReader::new(file)?;
    let metadata = reader.metadata();
    
    let mut output_files = Vec::new();
    let mut file_index = 0;
    
    // Read and split
    for row_group in 0..metadata.num_row_groups() {
        let output_path = format!("{}_{:04}.parquet", output_prefix, file_index);
        
        // Write row group to new file
        // ... implementation
        
        output_files.push(output_path);
        file_index += 1;
    }
    
    Ok(output_files)
}
```

## Benefits

### Storage
- **Compression**: 5-10x smaller than JSON
- **Columnar**: Only read columns you need
- **Typed**: Schema enforcement

### Performance
- **Parallel reads**: Multiple threads can read different columns
- **Predicate pushdown**: Filter at storage layer
- **Memory efficient**: Stream large files

### Integration
- **Arrow**: Zero-copy between Parquet and Arrow
- **Polars**: Fast DataFrame operations
- **DuckDB**: SQL queries on Parquet files
- **HuggingFace**: Native Parquet support

## Example: Complete Migration

### Before (report_generator.rs)
```rust
let unified = fs::read_to_string("UNIFIED_MASTER_TABLE.md")?;
let json = serde_json::to_string_pretty(&data)?;
fs::write("output.json", json)?;
```

### After
```rust
use canonical_data_store::{CanonicalDataStore, save_grammar_parquet};
use data_registry::{DataRegistry, DataFormat, DataPaths};

// Setup
DataPaths::ensure_dirs()?;
let store = CanonicalDataStore::new(100);

// Process with crossbeam
let data = process_data_parallel(store.sender())?;

// Save to Parquet
let output_path = "data/analysis/report.parquet";
save_grammar_parquet(&data, output_path)?;

// Register
let mut registry = DataRegistry::load()?;
registry.register_dataset(
    "report",
    PathBuf::from(output_path),
    DataFormat::Parquet,
    "Unified analysis report"
)?;

println!("✅ Saved to {}", output_path);
```

## Next Steps

1. **Add Parquet to DataFormat enum** in data_registry.rs
2. **Create parquet_splitter.rs** utility
3. **Migrate git_file_mapper** (highest impact - 1.2GB)
4. **Migrate file_stats_analyzer** (37M files)
5. **Create schema library** with standard schemas
6. **Add Parquet readers** for each schema
7. **Update all JSON writers** to use canonical store
8. **Add data validation** on write
9. **Create data catalog** tool to browse registry
10. **Add incremental updates** to avoid full regeneration

## Related Files

- `canonical_data_store.rs` - Core implementation
- `data_registry.rs` - Registry system
- `markov_resonance_analyzer/src/main.rs` - Reference implementation
- `docs/FILE_IO_INVENTORY.md` - All file I/O operations
- `docs/DATA_FILES_INVENTORY.md` - All data files
