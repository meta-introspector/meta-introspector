# Unstaged Rust Code Changes Documentation

**Date**: 2026-01-15  
**Status**: Ready for review and staging

## Summary

5 Rust files with changes:
- 1 modified file (crossbeam_value_lattice.rs)
- 4 new files (dataset-indexer.rs, generate-monthly-reports.rs, metis-partition-markov.rs, repo_analysis_planner.rs)

---

## 1. Modified: crossbeam_value_lattice.rs

### Purpose
Enhanced the 20-core parallel value lattice analyzer with CLI argument support and configurable paths.

### Key Changes

#### Added Dependencies
```rust
use clap::Parser;
use std::path::{Path, PathBuf};
```

#### New CLI Arguments Structure
```rust
#[derive(Parser, Debug)]
struct Args {
    input_path: PathBuf,      // Source directory to analyze
    output_dir: PathBuf,      // Results destination
    plan_mode: bool,          // Dry-run mode
    list_files_only: bool,    // List Rust files and exit
}
```

#### Removed Hardcoded Paths
**Before**: Hardcoded `/mnt/data1/meta-introspector/analysis/`  
**After**: Uses `args.output_dir` parameter

**Affected Functions**:
- `load_progress()` - Now takes `output_dir: &Path` parameter
- `save_progress()` - Now takes `output_dir: &Path` parameter
- `main()` - Uses `args.input_path` instead of `current_dir()`

#### New Features

1. **List Files Only Mode** (`--list-files-only`)
   - Prints all Rust files that would be processed
   - Exits without analysis
   - Used by repo_analysis_planner.rs for file discovery

2. **Plan Mode** (`--plan-mode`)
   - Shows what would be analyzed without execution
   - Prints file count and paths
   - Useful for validation before long-running jobs

3. **Configurable I/O**
   - Input path: `--input-path <DIR>`
   - Output path: `--output-dir <DIR>`
   - Progress tracking per output directory

#### Bug Fixes
- Fixed file path tracking in progress (was hardcoded "processed")
- Now stores actual file paths in `ValueUsage` structs
- Proper cloning for thread-safe operations

### Migration Impact
**Breaking Change**: Command-line arguments now required

**Old Usage**:
```bash
cargo run --bin crossbeam-value-lattice
```

**New Usage**:
```bash
cargo run --bin crossbeam-value-lattice -- \
  --input-path /path/to/rust/project \
  --output-dir /path/to/results
```

---

## 2. New File: dataset-indexer.rs

### Purpose
Discovers and catalogs all datasets across HuggingFace, local storage, and untracked directories.

### Features

#### Three Dataset Categories

1. **HuggingFace Datasets**
   - Tracks published datasets on HF (introspector org)
   - Discovers h4 org datasets via GitHub API
   - Records: name, org, URL, purpose, size, status

2. **Local Datasets**
   - Scans known data directories
   - Calculates size (MB) and file count
   - Flags HuggingFace upload candidates
   - Tracked directories:
     - `data/activity` - Git activity by platform/user/year/month
     - `data/perf_sessions` - Perf capture sessions
     - `data/71_flakes_perf` - 71 flakes performance analysis
     - `data/build_analysis` - Real build strace analysis
     - `data/telemetry` - Build telemetry data

3. **Untracked Datasets**
   - Finds `data-*` directories
   - Identifies large undocumented datasets
   - Recommends actions based on size:
     - >100 MB: Create HF dataset
     - >10 MB: Add to git-activity dataset
     - <10 MB: Keep local or add to git

#### Output
Generates `data/dataset-index.json` with complete inventory.

### Dependencies
```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Usage
```bash
cargo run --bin dataset-indexer
```

### Example Output
```
🔍 Discovering datasets...

📊 Dataset Index Summary:
   HuggingFace: 6 datasets
   Local:       5 datasets
   Untracked:   3 datasets

✅ Index saved to: data/dataset-index.json
```

---

## 3. New File: generate-monthly-reports.rs

### Purpose
Aggregates Git activity data across multiple identities and generates per-user monthly reports.

### Features

#### Identity Merging
Merges activity from multiple user identities:
- jmikedupont2
- mike.dupont
- mike
- Mike DuPont

Creates unified `mike-merged.json` reports.

#### Multi-Platform Aggregation
Scans all platforms and users in `data/activity/`:
```
data/activity/{platform}/{user}/{year}/{month}/activity.json
```

#### Report Structure
```rust
struct MonthlyReport {
    year: u32,
    month: String,
    user: String,
    commits: usize,
    files: usize,
    insertions: usize,
    deletions: usize,
    repos: HashMap<String, usize>,  // Repo -> commit count
}
```

#### Output Organization
```
reports/{year}/{month}/
  ├── mike-merged.json
  ├── user1.json
  ├── user2.json
  └── ...
```

### Dependencies
```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

### Usage
```bash
cargo run --bin generate-monthly-reports
```

### Example Output
```
📊 Generating merged reports for my identities + all other users...

Generated: reports/2025/01/mike-merged.json (337 commits)
Generated: reports/2025/01/contributor1.json (42 commits)
Generated: reports/2025/01/contributor2.json (18 commits)

✅ Reports generated in reports/{year}/{month}/
```

---

## 4. New File: metis-partition-markov.rs

### Purpose
Partitions large Markov transition matrices into semantic chunks (~1MB each) using METIS graph partitioning.

### Features

#### METIS Integration
- Converts Markov matrix to METIS graph format
- Weighted edges by transition probability
- Targets ~10K states per partition (~1MB JSON)
- Falls back to semantic clustering if METIS unavailable

#### Input Format
```rust
struct MarkovMatrix {
    transitions: Vec<Vec<f64>>,  // Transition probabilities
    states: Vec<String>,          // State names
}
```

#### Output Format
```rust
struct Partition {
    partition_id: usize,
    states: Vec<String>,
    transitions: Vec<Vec<f64>>,
    semantic_summary: String,  // Top 3 state prefixes
}
```

#### Semantic Analysis
Analyzes state name patterns to generate summaries:
- Extracts common prefixes
- Counts occurrences
- Reports top 3 patterns per partition

#### Output Files
```
hf-markov-analysis-upload/
  ├── partition_0000.json
  ├── partition_0001.json
  ├── partition_0002.json
  └── ...
```

### Dependencies
```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
```

**External**: Requires `gpmetis` binary (METIS graph partitioner)

### Usage
```bash
cargo run --bin metis-partition-markov
```

### Example Output
```
🔍 Loading Markov matrix...
📊 Parsing matrix...
   States: 45000
🔧 Converting to METIS format...
   Wrote 45000 vertices, 123456 edges
📦 Partitioning into 5 semantic chunks...
✅ METIS partitioning complete
   Partition 0: 9234 states - fn(4521), struct(2341), impl(1234)
   Partition 1: 8976 states - trait(3456), enum(2987), mod(1543)
   ...
```

---

## 5. New File: repo_analysis_planner.rs

### Purpose
Generates comprehensive analysis plans for repositories with user contributions, tracking unique Rust files across branches.

### Features

#### CLI Arguments
```rust
struct Args {
    user_authors: Vec<String>,     // Author identifiers to filter
    registry: PathBuf,             // git-sources-registry.json path
    output_base_dir: PathBuf,      // Base directory for results
}
```

#### Multi-Branch Analysis
- Scans all local branches in each repository
- Filters branches with user commits
- Tracks unique Rust files by blob ID (deduplication)
- Detects Rust toolchain versions per branch

#### Rust Toolchain Detection
Checks in order:
1. `rust-toolchain.toml` - `channel` or `toolchain` field
2. `Cargo.toml` - `rust-version` field
3. Falls back to "default/unspecified"

#### Unique File Tracking
- Uses `git hash-object` to get blob IDs
- Deduplicates files across branches
- Only counts truly unique Rust files
- Integrates with `crossbeam-value-lattice --list-files-only`

#### Analysis Plan Output
```rust
struct AnalysisPlanEntry {
    repo_name: String,
    branch_name: String,
    rust_toolchain_version: String,
    rust_files_to_process: usize,
    plan_command: String,      // Dry-run command
    execute_command: String,   // Actual execution command
}
```

#### Output Structure
```
{output_base_dir}/
  └── {repo_name}/
      └── {branch_name}/
          └── (analysis results)
```

### Dependencies
```toml
clap = { version = "4.0", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
gix = "0.57"  # Git operations
```

### Usage
```bash
cargo run --bin repo_analysis_planner -- \
  --user-authors "Mike DuPont <jmikedupont2@gmail.com>,mike dupont <mike.dupont@introspector.local>" \
  --registry data/git-sources-registry.json \
  --output-base-dir /mnt/data1/analysis-results
```

### Example Output
```
--- Repo Analysis Planner ---
User Authors: ["Mike DuPont <jmikedupont2@gmail.com>", "mike dupont <mike.dupont@introspector.local>"]
Registry Path: data/git-sources-registry.json
Output Base Dir: /mnt/data1/analysis-results
-----------------------------

Processing repository: rust-analyzer (/path/to/rust-analyzer)
  - Found user commits in branch: main
  - Found user commits in branch: feature/new-analysis

--- Aggregated Analysis Plan ---
Total branches with user changes to analyze: 15
Estimated total Rust files to process: 3456
Unique Rust Toolchain Versions identified: {"1.75.0", "1.76.0", "default/unspecified"}

Detailed Plan:
  Repo: rust-analyzer, Branch: main
    Rust Version: 1.75.0
    Rust Files (plan): 234
    Plan Command: cargo run --bin crossbeam-value-lattice -- --input-files ... --output-dir "..." --plan-mode
    Execute Command: cargo run --bin crossbeam-value-lattice -- --input-files ... --output-dir "..."
  ...
```

---

## Integration Points

### 1. crossbeam_value_lattice.rs ← repo_analysis_planner.rs
- Planner uses `--list-files-only` to discover Rust files
- Planner generates commands with `--input-path` and `--output-dir`
- Planner uses `--plan-mode` for validation

### 2. dataset-indexer.rs → HuggingFace Upload
- Identifies datasets ready for HF upload
- Provides size/file count for upload planning
- Tracks existing HF datasets to avoid duplicates

### 3. generate-monthly-reports.rs → Investor Reports
- Feeds into investor-report-2025.rs
- Provides per-user monthly breakdowns
- Enables multi-identity tracking

### 4. metis-partition-markov.rs → HuggingFace Upload
- Prepares large Markov matrices for HF upload
- Keeps partitions under 1MB for efficient loading
- Maintains semantic coherence within partitions

---

## Testing Checklist

### crossbeam_value_lattice.rs
- [ ] Test `--list-files-only` on sample repo
- [ ] Test `--plan-mode` output
- [ ] Verify progress tracking with custom output dir
- [ ] Confirm file paths stored correctly in results

### dataset-indexer.rs
- [ ] Run on current data/ directory
- [ ] Verify HF dataset discovery
- [ ] Check size calculations
- [ ] Validate recommendations

### generate-monthly-reports.rs
- [ ] Run on data/activity/
- [ ] Verify identity merging
- [ ] Check report structure
- [ ] Validate repo aggregation

### metis-partition-markov.rs
- [ ] Test with sample Markov matrix
- [ ] Verify METIS fallback
- [ ] Check partition sizes
- [ ] Validate semantic summaries

### repo_analysis_planner.rs
- [ ] Test with git-sources-registry.json
- [ ] Verify branch filtering
- [ ] Check unique file deduplication
- [ ] Validate toolchain detection

---

## Next Steps

1. **Add to Cargo.toml**
   ```toml
   [[bin]]
   name = "dataset-indexer"
   path = "dataset-indexer.rs"
   
   [[bin]]
   name = "generate-monthly-reports"
   path = "generate-monthly-reports.rs"
   
   [[bin]]
   name = "metis-partition-markov"
   path = "metis-partition-markov.rs"
   
   [[bin]]
   name = "repo-analysis-planner"
   path = "repo_analysis_planner.rs"
   ```

2. **Add Dependencies**
   ```toml
   clap = { version = "4.0", features = ["derive"] }
   gix = "0.57"
   ```

3. **Stage Changes**
   ```bash
   git add crossbeam_value_lattice.rs
   git add dataset-indexer.rs
   git add generate-monthly-reports.rs
   git add metis-partition-markov.rs
   git add repo_analysis_planner.rs
   ```

4. **Commit**
   ```bash
   git commit -m "Add dataset management and analysis planning tools

   - Enhanced crossbeam_value_lattice with CLI args and configurable paths
   - Added dataset-indexer for HF/local/untracked dataset discovery
   - Added generate-monthly-reports for multi-identity Git activity aggregation
   - Added metis-partition-markov for large matrix partitioning
   - Added repo_analysis_planner for multi-repo/branch analysis planning"
   ```

---

## Dependencies Summary

### New Dependencies Required
```toml
clap = { version = "4.0", features = ["derive"] }
gix = "0.57"
```

### Existing Dependencies Used
```toml
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
crossbeam = "0.8"
syn = "2.0"
```

### External Tools
- `gpmetis` - METIS graph partitioner (optional, has fallback)
- `git` - Git command-line tool
- `gh` - GitHub CLI (optional, for HF dataset discovery)

---

## File Sizes

- crossbeam_value_lattice.rs: ~7.5 KB (modified)
- dataset-indexer.rs: ~5.2 KB (new)
- generate-monthly-reports.rs: ~4.8 KB (new)
- metis-partition-markov.rs: ~6.1 KB (new)
- repo_analysis_planner.rs: ~10.3 KB (new)

**Total**: ~33.9 KB of new/modified code
