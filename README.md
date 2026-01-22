# meta-introspector: Unified Code Analysis Dataset

[![Meta Meme](https://img.shields.io/badge/meta--meme-pythonista-blue?style=for-the-badge&logo=python)](https://huggingface.co/datasets/introspector/meta-meme)
[![Bootstrap Dataset](https://img.shields.io/badge/bootstrap-3556%20repos-green?style=for-the-badge&logo=git)](https://huggingface.co/datasets/introspector/meta-meme)

**Organization**: [introspector](https://huggingface.co/introspector)  
**Dataset**: [meta-introspector](https://huggingface.co/datasets/introspector/meta-introspector)  
**License**: AGPL-3.0

> 🎭 **What's your meta meme?** Are you a js d00d or pythonista? [Find out!](META_MEME_GUIDE.md)

## 🚀 mkbootstrap! - The Ultimate Macro System

**mkbootstrap!** is a declarative workflow system that composes the entire 71^71 multiverse as Lisp-like macros.

### Quick Start

```bash
# Build the system
cargo build --release --bin mkbootstrap

# Run on 5 languages (bash, python, ruby, rust, nix_flake)
./target/release/mkbootstrap

# Build complexity lattice
cargo build --release --bin lattice_builder
./target/release/lattice_builder

# Visualize
dot -Tpng data/complexity_lattice.dot -o complexity_lattice.png
```

### The Vision: 71^71

Everything is a composable macro:

```rust
// 71^1 = 71 proofs
mkbootstrap!(level: 1, "languages")

// 71^2 = 5,041 proofs  
mkbootstrap!(level: 2, "languages", "databases")

// 71^3 = 357,911 proofs
mkbootstrap!(level: 3, "languages", "databases", "solvers")

// 71^71 = 10^133 proofs (computational singularity)
mkbootstrap!(level: 71)
```

### Complexity Lattice

The system proves computational hierarchy through Galois field analysis:

```
Level 5: agda           (GF(2^14) = 16,384 states)
Level 4: genetic, jax   (GF(2^13) =  8,192 states)
Level 3: rust, coq, ...  (GF(2^12) =  4,096 states)
Level 2: isabelle, mzn  (GF(2^11) =  2,048 states)
Level 1: bash, python   (GF(2^10) =  1,024 states)
```

**22 languages, 5 levels, 153 relationships** - A mathematical proof of complexity hierarchy!

See [CANONICAL_PATH.md](CANONICAL_PATH.md) for the complete vision.

## 🎯 What is this?

The **meta-introspector** dataset contains unified indexes and analysis results from the meta-introspector project:

- **3M+ file index** with git provenance
- **Repository metadata** for all analyzed repos
- **Markov symbol analysis** with similarity scores
- **Eigenvector analysis** of code patterns
- **Telemetry data** from Rust compilation
- **Moonshine analysis** of ELF binaries

## 🚀 Quick Start: Bootstrap

### Single Command

```bash
./bootstrap.sh
```

Run repeatedly to iterate. Each run:
2. Generates self-metadata
3. Commits changes
4. Stores perf data in `/nix/store/`

See [BOOTSTRAP.md](BOOTSTRAP.md) for details.

## 🚀 Quick Start: Build with Telemetry

### 1. Queue Projects for Building

```bash
# Queue main project
cd /mnt/data1/meta-introspector
./nix_builder.sh queue /mnt/data1/meta-introspector

# Queue sub-projects (e.g., zos-server)
./nix_builder.sh queue ~/zos-server

# Check queue
cat ~/.local/share/nix-builder/queue.txt
```

### 2. Start Build Queue with Telemetry

```bash
# Start builder in background
nohup ./nix_builder.sh watch > nix_builder.log 2>&1 &

# Monitor progress
tail -f nix_builder.log

# Check running builds
ps aux | grep nix_builder
```

### 3. Inspect Telemetry Data

```bash
# View parquet files
ls -lh *.parquet

# Query build logs (requires query-parquet binary)
cargo run --release --bin query-parquet -- \
  nix_build_logs.parquet \
  "SELECT * FROM nix_build_logs LIMIT 5"

# Check build logs
ls -lh ~/.local/share/nix-builder/logs/
```

### 4. Git Mirror System

```bash
# Check discovered URLs
wc -l data/master_url_list.txt  # 13,757 unique URLs

# Check clone progress
tail -f slow_clone.log
du -sh /mnt/data1/git  # Current mirror size

# Check queue status
wc -l data/queue_all.txt  # Remaining to clone
```

## 🔬 Galois Field Analysis & Complexity Lattice

The system analyzes perf data to discover Galois field coverage patterns and builds a mathematical proof of computational complexity hierarchy.

### Quick Start

```bash
# Run mkbootstrap! workflow (build + perf + analysis)
cargo build --release --bin mkbootstrap
./target/release/mkbootstrap

# Build complexity lattice from results
cargo build --release --bin lattice_builder
./target/release/lattice_builder

# Visualize the lattice
dot -Tpng data/complexity_lattice.dot -o complexity_lattice.png

# Or use Makefile
make help              # Show all commands
make test-all          # Test all 71 languages output "71"
```

### Current Results

**22 languages analyzed across 5 complexity levels:**

- **Level 5**: agda (GF(2^14) = 16,384 states)
- **Level 4**: genetic, jax_gpu (GF(2^13) = 8,192 states)
- **Level 3**: rust, coq, haskell, llvm, datalog, etc. (GF(2^12) = 4,096 states)
- **Level 2**: isabelle, minizinc (GF(2^11) = 2,048 states)
- **Level 1**: bash, python, ruby, nix (GF(2^10) = 1,024 states)

**153 partial order relationships** prove the complexity hierarchy!

### The 71 Languages

All tests in `const_71_test/` output "71":
- **67 programming languages**: Rust, Python, Haskell, Agda, Coq, etc.
- **5 build systems**: Nix Flakes, Make, CMake, Bazel, Terraform
- **1 bootstrap baseline**: GNU Mes (GF(2^19) = 524,288 states)

### Manual Analysis

```bash
# Analyze single language
cargo build --release --bin harmonic_analyzer
./target/release/harmonic_analyzer data/71_flakes_perf/rust_build.perf.data
```

### Output Format

```
🔍 Fast Galois Break Point Predictor
✅ 524288 samples

  GF(2^18): 100.000000% ✅ FULL - adding GF(2^20)
  GF(2^19): 100.000000% ✅ FULL - adding GF(2^21)

📊 FINAL COVERAGE:
  GF(2^20): 372841/1048576 (70.992470%)
  GF(2^21): 372841/2097152 (35.496235%)
```

The analyzer uses an **adaptive algorithm**:
- Starts at bits 18/19 (known break point from Mes bootstrap)
- Single pass through perf data
- Removes fields when they hit 100% coverage
- Adds next higher bit size automatically
- Minimal memory usage (only active fields tracked)

See [OGG_PRIME_19_HARMONIC_BREAK.md](OGG_PRIME_19_HARMONIC_BREAK.md) for mathematical significance.

## 📊 Telemetry Outputs

All builds create parquet files in the project root:

- `nix_build_logs.parquet` - Build success/failure logs
- `nix_store_grammars.parquet` - Grammar extraction (49,655 rows)
- `markov_symbol_scores.parquet` - Symbol analysis (106MB)
- `string_usage.parquet` - String usage patterns

Build metadata stored in:
- `~/.local/share/nix-builder/cache/` - Build cache
- `~/.local/share/nix-builder/logs/` - Detailed logs
- `/nix/store/*-reproducible/metadata.json` - Reproducibility data

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
