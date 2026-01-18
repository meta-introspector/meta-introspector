# meta-introspector: Unified Code Analysis Dataset

**Organization**: [introspector](https://huggingface.co/introspector)  
**Dataset**: [meta-introspector](https://huggingface.co/datasets/introspector/meta-introspector)  
**License**: AGPL-3.0

## 🎯 What is this?

The **meta-introspector** dataset contains unified indexes and analysis results from the meta-introspector project:

- **3M+ file index** with git provenance
- **Repository metadata** for all analyzed repos
- **Markov symbol analysis** with similarity scores
- **Eigenvector analysis** of code patterns
- **Telemetry data** from Rust compilation
- **Moonshine analysis** of ELF binaries

## 📊 Dataset Structure

```
meta-introspector/
├── indexes/
│   ├── files.parquet           # 3M+ files with git provenance
│   ├── repos.parquet           # Repository metadata
│   ├── datasets.parquet        # HF + local datasets
│   └── projects.parquet        # Project metadata
├── markov-analysis/
│   ├── markov_symbol_scores.parquet
│   └── markov_similarity_matrix_meta.json
├── eigenvectors/
│   └── word_eigenvectors.json
├── telemetry/
│   ├── rustc_trace_schema.parquet
│   └── syscall_summary.parquet
├── moonshine/
│   └── elf_moonshine_map.txt
└── registry.json               # Central data registry
```

## 🚀 How to Use

### Python (Pandas)

```python
import pandas as pd

# Load 3M file index
files_df = pd.read_parquet('hf://datasets/introspector/meta-introspector/indexes/files.parquet')
print(f"Loaded {len(files_df)} files")

# Find all Rust files
rust_files = files_df[files_df['file_path'].str.endswith('.rs')]
print(f"Found {len(rust_files)} Rust files")

# Load Markov symbol scores
markov_df = pd.read_parquet('hf://datasets/introspector/meta-introspector/markov-analysis/markov_symbol_scores.parquet')
print(f"Loaded {len(markov_df)} symbol scores")
```

### Rust (Arrow/Parquet)

```rust
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use std::fs::File;

// Load file index
let file = File::open("indexes/files.parquet")?;
let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
let reader = builder.build()?;

for batch in reader {
    let batch = batch?;
    println!("Loaded batch with {} files", batch.num_rows());
}
```

### DuckDB (SQL Queries)

```sql
-- Query 3M file index
SELECT git_repo, COUNT(*) as file_count 
FROM 'hf://datasets/introspector/meta-introspector/indexes/files.parquet'
GROUP BY git_repo 
ORDER BY file_count DESC 
LIMIT 10;

-- Find Rust files by repo
SELECT file_path, commit, branch 
FROM 'hf://datasets/introspector/meta-introspector/indexes/files.parquet'
WHERE file_path LIKE '%.rs' 
AND git_repo = 'meta-introspector';
```

## 📈 Dataset Statistics

- **Files indexed**: 3,000,000+
- **Repositories**: 1,000+
- **Markov symbols**: 100,000+
- **Telemetry traces**: 10,000+
- **Total size**: ~2GB (compressed Parquet)

## 🔗 Related Datasets

- [solfunmeme-index](https://huggingface.co/datasets/introspector/solfunmeme-index) - 1.2M+ Rust semantic analysis records
- [git-activity](https://huggingface.co/datasets/introspector/git-activity) - Git activity tracking

## 🤝 Contributing

This dataset is automatically updated from the [meta-introspector](https://github.com/meta-introspector/meta-introspector) project.

To contribute:
1. Run analysis tools from the project
2. Generate Parquet files using canonical data store
3. Push updates using `push_to_hf.rs`

## 📄 Citation

```bibtex
@dataset{meta_introspector_2026,
  title={Meta-Introspector: Unified Code Analysis Dataset},
  author={Meta-Introspector Team},
  year={2026},
  url={https://huggingface.co/datasets/introspector/meta-introspector},
  note={3M+ files with git provenance and comprehensive analysis}
}
```

## 📊 Schema Documentation

### files.parquet
- `file_path` (string): Absolute file path
- `git_repo` (string): Repository name
- `commit` (string): Git commit hash
- `branch` (string): Git branch name
- `remote` (string): Git remote URL
- `url` (string): GitHub/GitLab URL

### repos.parquet
- `path` (string): Repository path
- `name` (string): Repository name
- `remote_url` (string): Remote URL
- `is_fork` (bool): Fork status
- `is_local` (bool): Local repository
- `branch` (string): Current branch
- `status` (string): Git status
- `last_commit` (string): Last commit hash

### markov_symbol_scores.parquet
- `symbol` (string): Symbol name
- `file_id` (uint64): File identifier
- `score` (float64): Similarity score
- `frequency` (uint64): Occurrence count

## 🎓 Use Cases

- **Code search**: Find files across 3M+ indexed files
- **Provenance tracking**: Trace files to git commits
- **Symbol analysis**: Study code patterns and similarities
- **ML training**: Train models on real-world code data
- **Research**: Analyze large-scale code repositories

---

**Generated**: 2026-01-18T00:51:42.126748768+00:00  
**Project**: https://github.com/meta-introspector/meta-introspector  
**Organization**: https://huggingface.co/introspector
