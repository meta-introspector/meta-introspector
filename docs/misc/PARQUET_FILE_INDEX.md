# Parquet-Based File Indexing System

## Overview

You have **3 parquet-based file indexers** already implemented:

1. **file_index_service.rs** - Main centralized file index
2. **git_file_mapper.rs** - Git provenance mapping
3. **canonical_data_store.rs** - Canonical data storage

## 1. File Index Service (Primary)

**Location**: `/mnt/data1/meta-introspector/file_index_service.rs`

### Features
- ✅ **Centralized**: Single source of truth
- ✅ **Parquet cache**: Persistent storage
- ✅ **In-memory index**: Fast queries
- ✅ **Predictive**: Learns access patterns
- ✅ **Priority-based**: Smart caching

### Architecture
```
Shell Scripts → HTTP/CLI → File Index Service → Parquet Cache
                                ↓
                         In-Memory Index
                         (sorted by priority)
```

### Data Structure
```rust
pub struct FileEntry {
    pub path: PathBuf,
    pub size: u64,
    pub modified: u64,
    pub extension: Option<String>,
    pub is_dir: bool,
    
    // Access tracking
    pub access_count: u64,
    pub last_accessed: u64,
    pub priority_score: f64,
}
```

### Key Functions
- `initialize()` - Load from cache or scan filesystem
- `query_by_pattern()` - Search files by pattern
- `query_by_extension()` - Filter by extension
- `predict_next_queries()` - Predict likely queries
- `save_to_cache()` - Persist to Parquet
- `load_from_cache()` - Load from Parquet

## 2. Git File Mapper

**Location**: `/mnt/data1/meta-introspector/git_file_mapper.rs`

### Purpose
Maps files to git provenance (repo, commit, branch)

### Output
```
FILE_GIT_MAPPING.csv (814 MB)
```

### Schema
```rust
struct GitFileMapping {
    file_path: String,
    git_repo: String,
    commit: String,
    branch: String,
    remote: String,
    url: String,
    is_fork: bool,
}
```

## 3. Canonical Data Store

**Location**: `/mnt/data1/meta-introspector/canonical_data_store.rs`

### Purpose
Unified parquet storage for all data types

## HuggingFace Datasets

You have **4 HF dataset repositories**:

### 1. hf-build-telemetry
**Purpose**: Build and compilation telemetry  
**Format**: Parquet  
**Location**: `/mnt/data1/meta-introspector/hf-build-telemetry/`

### 2. hf-markov-analysis
**Purpose**: Markov chain analysis of code  
**Format**: Parquet  
**Location**: `/mnt/data1/meta-introspector/hf-markov-analysis/`

### 3. hf-git-activity
**Purpose**: Git activity tracking  
**Format**: Parquet  
**Location**: `/mnt/data1/meta-introspector/hf-git-activity/`

### 4. meta-introspector (main dataset)
**HuggingFace**: https://huggingface.co/datasets/introspector/meta-introspector

## Integration Plan

### Step 1: Use Existing File Index Service

```rust
use file_index_service::FileIndexService;

fn main() {
    let mut index = FileIndexService::new(
        PathBuf::from("/mnt/data1/meta-introspector/indexes"),
        vec![
            PathBuf::from("/mnt/data1/meta-introspector"),
            PathBuf::from("/mnt/data1/nix"),
            PathBuf::from("/mnt/data1/time2"),
        ]
    );
    
    // Initialize (loads from cache or scans)
    index.initialize().unwrap();
    
    // Query
    let rust_files = index.query_by_extension("rs");
    println!("Found {} Rust files", rust_files.len());
    
    // Save to parquet
    index.save_to_cache().unwrap();
}
```

### Step 2: Import Existing Find Results

```rust
impl FileIndexService {
    pub fn import_from_find_results(&mut self, path: &Path) -> Result<usize> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        let mut count = 0;
        for line in reader.lines() {
            let line = line?;
            let path = self.convert_to_absolute(&line);
            
            if let Ok(metadata) = std::fs::metadata(&path) {
                let entry = FileEntry {
                    path: path.clone(),
                    size: metadata.len(),
                    modified: metadata.modified()?.duration_since(UNIX_EPOCH)?.as_secs(),
                    extension: path.extension().map(|s| s.to_string_lossy().to_string()),
                    is_dir: metadata.is_dir(),
                    access_count: 0,
                    last_accessed: now(),
                    priority_score: 0.0,
                };
                
                self.index.write().unwrap().insert(path, entry);
                count += 1;
            }
        }
        
        Ok(count)
    }
}
```

### Step 3: Export to HuggingFace Dataset

```rust
impl FileIndexService {
    pub fn export_to_hf_dataset(&self, output_dir: &Path) -> Result<()> {
        use arrow::array::*;
        use arrow::datatypes::*;
        use parquet::arrow::ArrowWriter;
        
        // Create schema
        let schema = Schema::new(vec![
            Field::new("path", DataType::Utf8, false),
            Field::new("size", DataType::UInt64, false),
            Field::new("modified", DataType::UInt64, false),
            Field::new("extension", DataType::Utf8, true),
            Field::new("access_count", DataType::UInt64, false),
            Field::new("priority_score", DataType::Float64, false),
        ]);
        
        // Collect data
        let index = self.index.read().unwrap();
        let mut paths = Vec::new();
        let mut sizes = Vec::new();
        let mut modified = Vec::new();
        let mut extensions = Vec::new();
        let mut access_counts = Vec::new();
        let mut priority_scores = Vec::new();
        
        for entry in index.values() {
            paths.push(entry.path.to_string_lossy().to_string());
            sizes.push(entry.size);
            modified.push(entry.modified);
            extensions.push(entry.extension.clone());
            access_counts.push(entry.access_count);
            priority_scores.push(entry.priority_score);
        }
        
        // Create record batch
        let batch = RecordBatch::try_new(
            Arc::new(schema.clone()),
            vec![
                Arc::new(StringArray::from(paths)),
                Arc::new(UInt64Array::from(sizes)),
                Arc::new(UInt64Array::from(modified)),
                Arc::new(StringArray::from(extensions)),
                Arc::new(UInt64Array::from(access_counts)),
                Arc::new(Float64Array::from(priority_scores)),
            ],
        )?;
        
        // Write to parquet
        let file = File::create(output_dir.join("files.parquet"))?;
        let mut writer = ArrowWriter::try_new(file, Arc::new(schema), None)?;
        writer.write(&batch)?;
        writer.close()?;
        
        Ok(())
    }
}
```

## Usage Examples

### Import from files.txt
```bash
cargo run --bin file_index_cli -- import /mnt/data1/files.txt
```

### Query files
```bash
cargo run --bin file_index_cli -- query "*.rs"
cargo run --bin file_index_cli -- query --ext rs
```

### Export to HF dataset
```bash
cargo run --bin file_index_cli -- export /mnt/data1/meta-introspector/hf-file-index/
```

### Push to HuggingFace
```bash
cd /mnt/data1/meta-introspector/hf-file-index
git add files.parquet
git commit -m "Update file index"
git push
```

## Parquet Schema for File Index

```
message schema {
  required binary path (UTF8);
  required int64 size;
  required int64 modified;
  optional binary extension (UTF8);
  required boolean is_dir;
  required int64 access_count;
  required int64 last_accessed;
  required double priority_score;
  optional binary git_repo (UTF8);
  optional binary commit (UTF8);
  optional binary branch (UTF8);
}
```

## Benefits of Parquet

1. **Columnar**: Fast queries on specific columns
2. **Compressed**: ~10x smaller than CSV
3. **Typed**: Schema enforcement
4. **Fast**: Direct memory mapping
5. **Standard**: Works with Pandas, DuckDB, Arrow

## Next Steps

1. ✅ Use existing `file_index_service.rs`
2. ⬜ Import `/mnt/data1/files.txt` and `/mnt/data1/newfiles.txt`
3. ⬜ Add git provenance from `git_file_mapper.rs`
4. ⬜ Export to parquet in `indexes/files.parquet`
5. ⬜ Push to HuggingFace dataset
6. ⬜ Update all scripts to use file index service

## File Locations

```
/mnt/data1/meta-introspector/
├── file_index_service.rs          # Main service
├── file_index_cli.rs              # CLI interface
├── file_index_server.rs           # HTTP server
├── git_file_mapper.rs             # Git provenance
├── canonical_data_store.rs        # Unified storage
├── indexes/
│   ├── files.parquet              # Main index
│   ├── git_mapping.parquet        # Git provenance
│   └── cache/                     # Cache files
└── hf-file-index/                 # HuggingFace dataset
    ├── files.parquet
    ├── README.md
    └── .git/
```

## Summary

**You already have everything you need!**

- ✅ Parquet-based file indexer
- ✅ Git provenance mapper
- ✅ HuggingFace dataset infrastructure
- ✅ CLI and server interfaces

**Just need to**:
1. Import your existing find results
2. Export to parquet
3. Push to HuggingFace

**No need to build from scratch - it's already there!**
