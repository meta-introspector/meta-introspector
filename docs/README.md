# Meta-Introspector Repository Analysis

For a comprehensive overview of the `meta-introspector` project, its architecture, key technologies, and detailed analysis results, please refer to the [Meta-Introspector Analysis Document](docs/META_INTROSPECTOR_ANALYSIS.md).

Comprehensive analysis of 57,106 domains across 33.9M files with advanced semantic analysis and parallel processing systems.

## 🔥 Latest: LMFDB Rust Mapping Library + Self-Analysis

**New Achievement**: Created unified **LMFDB mapping library** and successfully analyzed itself!

### 📚 LMFDB Rust Mapping

**Location**: `lmfdb-rust-mapping/`

**Structure**:
- `lmfdb-types/` - Core data types (LMFDBLabel, OrbitLevel)
- `lmfdb-traits/` - Trait definitions (LMFDBClient, LMFDBMapper)
- `src/lib.rs` - Main implementation

**Self-Analysis Results** (libserde_derive.so):
- **8174 symbols** mapped to LMFDB labels
- **Conductor**: 618
- **Orbit Distribution**:
  - Genesis (11): 3715 symbols (45%)
  - Trinity (23): 3826 symbols (47%)
  - Completeness (47): 633 symbols (8%)

**Key Insight**: Most Rust symbols fall into Genesis/Trinity orbits (foundational/stable complexity).

### 🎯 71 Flakes Perf Collection

**Status**: V2 complete - 34/71 flakes successful with syscall analysis

**Features**:
- Dual perf capture (build + run)
- Inline perf analysis using linux-perf-data
- Syscall/event type extraction (FORK, MMAP2, SAMPLE, EXIT)
- No slow perf report or addr2line

**Data**: `/mnt/data1/meta-introspector/data/71_flakes_perf/`

## 🔥 Previous: Real Build Analysis with Complete Strace Capture

**Breakthrough Achievement**: Complete **real nix build analysis** with **structured data capture** - discovered actual build complexity vs telemetry assumptions.

### 🚀 Real Build Discovery

#### **Actual Build Complexity** 📊
- **32 binaries executed** (vs 14 in old telemetry - 2.3x more!)
- **91 .so files opened** during build process
- **71 ldd dependencies** from executed binaries
- **92 total unique libraries** (vs 39 in telemetry - 2.4x more!)

#### **Structured Data Capture** 💾
**Session**: `real_build_1768332029`
**Location**: `/mnt/data1/meta-introspector/data/build_analysis/`

**Data Files**:
- `real_build_1768332029_binaries.json` - All 32 executed binaries
- `real_build_1768332029_libraries.json` - All 91 opened .so files  
- `real_build_1768332029_ldd_deps.json` - All 71 ldd dependencies
- `real_build_1768332029_analysis.json` - Combined analysis summary
- `real_build_1768332029_strace.log` - Raw strace output (5.6MB)

#### **Key Discovery** 🎯
The telemetry system was **severely underestimating** build complexity:
- **Old telemetry**: 14 binaries, 39 libraries (from cached frontrun results)
- **Real build**: 32 binaries, 92 libraries (from live strace capture)
- **Gap**: 2.3x more binaries, 2.4x more libraries than expected

### 🎯 Next Phase: Telemetry System Update

**Action Required**: Update `ldd2wrap_all_calls.rs` to use real build data
**Input**: `real_build_1768332029_binaries.json` (32 real binaries)
**Expected Result**: Accurate telemetry matching strace captures

### 🚀 Structured Telemetry System

#### **Proven Results** ✅
- **456 symbols extracted** using goblin ELF parser vs 38 with nm
- **Script wrapper following** - rustc wrapper → real rustc binary (3 symbols)
- **Structured JSON logging** to `/mnt/data1/meta-introspector/data/telemetry/`
- **Project-based organization** with PROJECT_NAME environment variable

#### **Key Innovations**
1. **Goblin ELF Parser** - Replaces nm for accurate Rust binary symbol extraction
2. **Script Wrapper Following** - Follows bash script wrappers to find real binaries
3. **Structured JSON Logging** - Timestamped JSONL files with project organization
4. **Real Symbol Counts** - No more hardcoded fake numbers, actual goblin parsing
5. **Shell Script Integration** - `nix_rebuild_telemetry.sh` for complete capture

#### **Symbol Extraction Results**
- **rustc wrapper**: 0 libs, 3 symbols (script → real binary)
- **gcc wrapper**: 0 libs, 50 symbols (script → real binary)  
- **usr/bin/as**: 2 libs, 50 symbols (goblin vs 6 with nm)
- **Total**: 456 symbols vs 38 with nm (12x improvement)

### 🎯 Telemetry Architecture

#### **Macro-Based Telemetry** (`master_all_calls_allcalls_1768325605.rs`)
- **Real counts**: 14 binaries, 39 libraries, 456 symbols
- **Structured logging**: JSON with timestamp, project, binary counts
- **Build-time generation**: ldd2wrap creates wrappers with actual data
- **Zero runtime overhead**: Compile-time macro expansion

#### **LD_PRELOAD Interceptor** (`rust_preload_interceptor`)
- **Rust cdylib**: Using redhook for malloc/execve/fopen hooks
- **JSON logging**: Structured telemetry to timestamped files
- **Process tracking**: PID and timestamp for each call
- **Memory safety**: Rust implementation vs C-based interceptor

#### **Shell Script Integration** (`nix_rebuild_telemetry.sh`)
- **Project organization**: PROJECT_NAME environment variable
- **Dual telemetry**: Both LD_PRELOAD and macro-based capture
- **Log aggregation**: Combined output with structured file organization
- **Nix rebuild capture**: Real telemetry from actual build processes

### 🎯 Scaling Strategy: Multi-Tier Repository Analysis

#### **Repository Selection Complete** ✅
- **18 repositories selected** across 4 complexity tiers
- **Basic Tier (5)**: ripgrep, fd, bat, exa, starship - CLI tools and simple libraries
- **Intermediate Tier (5)**: tokio, actix-web, serde, hyper, warp - frameworks and async systems  
- **Advanced Tier (4)**: tikv, servo, swc, polkadot - compilers, databases, OS components
- **Expert Tier (4)**: rust, miri, chalk, prusti-dev - compiler internals and formal verification

#### **Comprehensive Analysis Layers**
1. **Bit-Level**: Datatype Markov models (7 primitives, 251K instances)
2. **Value Lattice**: 14,316 unique literals, 117-char convergence point
3. **Type Structure**: Enum/struct patterns, composition analysis
4. **Instance Patterns**: 173 unique types, 326 instantiations analyzed
5. **Semantic Signatures**: 289,795 instruction blocks, 97.3% unique code
6. **Grammar Compression**: 93-96% space savings with direct querying

### 🚀 Core Analysis Systems

#### **Grammar-Based Compression** (`grammar_rust_compressor.rs`)
- **Sequitur algorithm** for lossless compression with direct pattern queries
- **93.3% space savings** proven on 1000 rust-build files
- **No decompression needed** for pattern searches and frequency counting
- **Token-based representation** with pattern dictionaries

#### **Rustc Build Interceptor** (`rustc_interceptor.rs`)
- **Hijacks cargo build process** using RUSTC environment variable
- **Real-time compression** during compilation without affecting build
- **124 files processed** with consistent 94-96% compression ratios
- **Metadata passthrough** for cargo compatibility

#### **Syn-Based Declaration Compressor** (`syn_compressor.rs`)
- **AST-level parsing** using syn crate for accurate Rust analysis
- **Real function names**: `outline`, `defer`, `make_display`, `drop`, `disable`
- **97.2% compression** (3,826 bytes → 106 bytes) with semantic preservation
- **Declaration-level granularity** for fine-grained analysis

#### **Crossbeam Value Lattice Analyzer** (`crossbeam_rustc_analyzer_complete.rs`)
- **20-core parallel processing** with bounded channels (1000 capacity)
- **Depth-limited recursion** (max 10 levels) with path filtering
- **Stack overflow protection** and error recovery
- **Thermal work measurement** - CPU temperature delta tracking

#### **Semantic Signature Generator** (`semantic_signature_generator.rs`)
- **4-layer analysis**: ABI + Security + Type + Meaning signatures
- **153 binaries processed** with full semantic profiles
- **97.3% unique code** - Only 2.7% duplication (mostly stdlib)
- **88.4% more novel functions** than standard rustc components

### 🔧 Infrastructure Tools

#### **Batch Job Runner** (`batch_runner.rs`)
- **Generic job execution** with JSON configuration
- **Timeout handling** and output redirection
- **Dependency tracking** for complex workflows
- **Summary statistics** and timing analysis

#### **Declaration Archiver** (`archive_declarations.rs`)
- **Individual declarations** saved as separate JSON files
- **Nice filenames**: `043_fn_drop_113_120_176b_to_16b.json`
- **Tar.gz packaging** to save inodes (52 files → 1 archive)
- **Real string names** extracted from syn parsing

## Directory Structure

### Top-Level Domains (TLDs)
- `com/` - Commercial domains (98.3% - 56,155 repos)
- `org/` - Organizations (1.4% - 775 repos)
- `co/` - Modern startups (0.2% - 123 repos)
- `fr/`, `cz/`, `de/` - Regional domains
- `io/`, `dev/`, `net/` - Tech-focused domains
- `edu/`, `us/` - Educational and government

### Major Repository Hosts
- `com/github/` - GitHub (55,752 repositories - 97.6%)
- `com/googlesource/` - Google projects (Chromium, Android)
- `co/huggingface/` - AI/ML models (115 repositories)
- `org/freedesktop/` - Desktop Linux (472 repositories)
- `org/gitlab/` - GitLab projects (90 repositories)

### Analysis Results
- `analysis/` - Comprehensive analysis reports with parallel processing
- `split-decls/` - Split declarations projects (13 found!)
- `rust-ecosystem/` - Rust-specific analysis (42K Cargo.toml, 1.47M .rs files)
- `tld-stats/` - Domain statistics and breakdowns
- `compressed_declarations/` - Grammar-compressed Rust declarations
- `syn_compressed_declarations/` - Syn-based AST compression results

## Key Findings

- **GitHub Dominance**: 97.6% of repositories hosted on GitHub
- **Split-Decls Active**: 13 repositories using split-decls-rs
- **Massive Rust Ecosystem**: 1.47M Rust files, 42K projects
- **Enterprise Presence**: Google, GNU, Freedesktop integration
- **Semantic Richness**: 21,349 AST nodes vs 1,990 in standard rustc (10x more)
- **Compression Breakthrough**: 93-96% space savings with queryable grammar compression
- **Real-time Processing**: Cargo build interception for seamless compression

### 2025 Activity Highlights (from Investor Report)
- **Total Commits**: 337
- **Files Changed**: 2,689
- **Lines Added**: +693K
- **Lines Removed**: -95K
- **Top Repository**: ai-agent-terraform (143 commits)

## Analysis Programs

### Compression & Grammar Systems
- `grammar_rust_compressor.rs` - Sequitur-based queryable compression (93.3% savings)
- `syn_compressor.rs` - AST-level compression with real names (97.2% savings)
- `rustc_interceptor.rs` - Cargo build hijacking for real-time compression
- `archive_declarations.rs` - Declaration packaging with nice names
- `prove_compression.rs` - Compression proof on 1000 files
- `batch_runner.rs` - Generic job execution system

### Core Analyzers
- `crossbeam_rustc_analyzer_complete.rs` - 20-core parallel analyzer with protections
- `semantic_signature_generator.rs` - 4-layer semantic analysis (ABI+Security+Type+Meaning)
- `split_decls_applicator.rs` - Automatic code layer separation system
- `duplicate_block_detector.rs` - Code duplication analysis across binaries
- `basic_block_analyzer.rs` - Instruction block novelty analysis

### Specialized Tools
- `monster_group_connection.rs` - Mathematical analysis connecting rustc to Monster Group theory
- `value_lattice_streaming.rs` - Memory-optimized streaming analyzer for massive codebases
- `thermal_monitor.sh` - CPU temperature-based computational work measurement
- `run_job_queue.sh` - Parallel job queue for analyzing multiple repositories

### Data Collection
- `recent_commits_scanner.rs` - Scan repositories by recent activity
- `commits_by_user.rs` - Analyze commit patterns by user
- `local_commit_cache.rs` - Fast local-only commit caching
- `https_commit_fetcher.rs` - Remote commit fetching with SSH to HTTPS conversion

### Reporting and Visualization
- `investor-report-2025.rs` - A Rust program that aggregates Git activity data, calculates statistics (total commits, files changed, insertions, deletions, monthly repo matrix, top repositories), and generates a JSON report for 2025.
- `investor-report-2025.html` - An HTML document that visually presents the summarized 2025 Git activity data, including overall statistics, a month-by-repository activity matrix, and top 10 repositories, styled as an annual report.

### Results
- **289,795 unique instruction blocks** catalogued across 153 binaries
- **97.3% unique code** with minimal duplication
- **88.4% more novel functions** than standard rustc components
- **Thermal work measurement** - +5°C temperature delta from intensive analysis
- **Grammar compression** - 93-96% space savings with direct querying
- **Declaration archives** - Individual compressed declarations with real names
