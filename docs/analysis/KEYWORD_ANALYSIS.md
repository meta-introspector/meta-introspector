# Keyword Analysis - Meta-Introspector Codebase

**Generated**: 2026-01-18T13:46:01-05:00  
**Purpose**: Understand file indexing infrastructure and git repository mapping

## Core Infrastructure

### File Indexing System

**Primary Components**:
- `file_index_service.rs` - Centralized HTTP/CLI service with Parquet cache
- `file_index_server.rs` - Server implementation
- `file_index_cli.rs` - CLI interface
- `build_incremental_index.sh` - SQLite-based incremental indexer

**Key Features**:
- In-memory index with priority scoring
- Access pattern profiling
- Parquet persistence
- Sub-millisecond cached queries
- Predictive pre-fetching

**Data Sources**:
- `~/nix/index/allrs.txt` - 136MB, all Rust files (3M+ files)
- `~/nix/index/githubrs.txt` - GitHub Rust files
- `~/nix/index/octocrab.txt` - Octocrab related files
- `FILE_GIT_MAPPING.csv` - File to git object mapping

### Git Repository Management

**Discovery Scripts**:
- `ingest_git_data.sh` - Ingest git configs and .gitmodules
- `find-active-repos.sh` - Find repos with recent activity
- `cross_reference_repos.sh` - Cross-reference local vs GitHub repos
- `check_parent_git.sh` - Check parent directories for git repos
- `scan-all-submodules.sh` - Scan all .gitmodules files

**Analysis Tools**:
- `analyze_repos.sh` - Analyze repository structure
- `quick_git_stat.sh` - Quick stat and sort by date
- `git_pack_scanner.rs` - Scan git pack files
- `all_commits_collector.rs` - Collect all commits

**Registry**:
- `~/nix/index/github_meta-introspector_repos.json` - GitHub repos
- `~/nix/index/github_metaintrospector_repos.json` - Alternative naming

### Search Utilities

**Shell Libraries**:
- `lib/search_utils.sh` - Original search utilities
- `lib/search_utils_v2.sh` - V2 with file-index service integration
- `test_search_utils.sh` - Test suite

**Key Functions** (from search_utils.sh):
- `find_git_repos()` - Find all git repositories
- File-index service integration via HTTP API

## Data Pipeline Architecture

### Stage 1: Discovery
```
/mnt/data1/ → find .git dirs → git repo list
```

### Stage 2: File Enumeration
```
git repos → git ls-files → file lists → ~/nix/index/allrs.txt
```

### Stage 3: Git Object Mapping
```
files → git hash-object → FILE_GIT_MAPPING.csv
```

### Stage 4: Symbol Extraction
```
files → parse symbols → symbol locations in git objects
```

### Stage 5: Process Tracking
```
nix build → strace/perf → process telemetry
```

### Stage 6: Intelligence
```
telemetry → analysis → insights
```

## Key Rust Components

### Telemetry & Tracing
- `master_telemetry.rs` - Master telemetry coordinator
- `telemetry_lib.rs` - Telemetry library
- `rust_preload_interceptor/` - LD_PRELOAD interceptor
- `nix-telemetry/` - Nix build telemetry
- `parquet_abi_interceptor.rs` - ABI call interceptor
- `rustc_perf_scanner.rs` - Rustc performance scanner

### Analysis Tools
- `symbol_similarity/` - Symbol analysis suite
  - `moonshine.rs` - ELF binary analysis
  - `eigenvector.rs` - Eigenvector analysis
  - `markov_labels.rs` - Markov chain labeling
  - `lmfdb_orbits.rs` - LMFDB orbit mapping
- `markov_chain_miner.rs` - Markov chain mining
- `duplicate_finder.rs` - Find duplicate code
- `intrinsic_complexity.rs` - Complexity analysis

### Build & Compilation
- `nix_cargo_interceptor.rs` - Intercept cargo builds
- `real_compile_proof.rs` - Compilation proofs
- `build_order_pipeline.rs` - Build order tracking
- `compiler_auto_labeler.rs` - Auto-label compiler output

### Data Management
- `push_to_hf.rs` - Push to HuggingFace datasets
- `query-parquet/` - Query Parquet files
- `access_pattern_profiler.rs` - Profile access patterns
- `project_hierarchy.rs` - Project structure analysis

## Python Analysis Tools

### Nix & Build Analysis
- `classify_nix_failures.py` - Classify build failures
- `extract_built_packages.py` - Extract successful builds
- `list_successful_packages.py` - List successful packages
- `analyze_project_ownership.py` - Project ownership analysis

### Performance & Tracing
- `setup_perf_probes.py` - Setup perf probes
- `trace_jupiter.py` - Trace Jupiter protocol
- `merge_perf_lmfdb.py` - Merge perf with LMFDB data
- `bott8-layout-solver/parse_perf.py` - Parse perf output
- `bott8-layout-solver/map_perf_to_8d.py` - Map to 8D space

### Blockchain & Contracts
- `fetch_top_contracts.py` - Fetch Solana contracts
- `fetch_recent_blocks.py` - Fetch recent blocks
- `predict_jupiter_branches.py` - Predict Jupiter branches
- `branch_prediction_market.py` - Branch prediction market
- `build_jupiter_cfg.py` - Build Jupiter CFG

### Data Analysis
- `analyze_error_patterns.py` - Error pattern analysis
- `compare_orbits.py` - Compare mathematical orbits
- `extract_nix_store_frequencies.py` - Nix store frequency analysis

## Shell Script Categories

### Build & Compilation (18 scripts)
- `build_all.sh`, `build_all_crates.sh`, `build-all-hot.sh`
- `build_with_logger.sh` - Build with telemetry
- `nix_build_real_telemetry.sh` - Nix build with telemetry
- `record_rustc_build.sh` - Record rustc compilation
- `capture_rustc_build_order.sh` - Capture build order
- `build_ziggurat.sh` - Build Ziggurat project

### Indexing & Discovery (12 scripts)
- `build_incremental_index.sh` - Incremental file index
- `update_indexes.sh` - Refresh file indexes
- `ingest_git_data.sh` - Ingest git metadata
- `find_solana_sources.sh` - Find Solana sources
- `mike_repos_quick_index.sh` - Quick repo index
- `scan-all-submodules.sh` - Scan submodules

### Analysis & Monitoring (15 scripts)
- `analyze_repos.sh` - Analyze repositories
- `run_core_analysis.sh` - Core analysis
- `run_full_analysis.sh` - Full analysis
- `run_markov_analysis.sh` - Markov analysis
- `thermal_monitor.sh` - Monitor system temperature
- `monitor_and_update.sh` - Monitor and update

### Tracing & Profiling (8 scripts)
- `strace_proof.sh` - Strace proof of execution
- `trace_self_compilation.sh` - Trace self-compilation
- `setup_perf_probes.sh` - Setup perf probes
- `record_nix_build_with_probes.sh` - Record with probes
- `capture_all.sh` - Capture all telemetry
- `capture_multidimensional_messages.sh` - Multi-dimensional capture

### Flake Management (10 scripts)
- `fix_composite_flakes.sh` - Fix composite flakes
- `fix_flake_false_to_true.sh` - Fix flake boolean
- `fix_self_attribute_errors.sh` - Fix self attribute errors
- `create_remaining_flakes.sh` - Create missing flakes
- `generate_contract_flakes.sh` - Generate contract flakes
- `create_proof_system_flakes.sh` - Create proof system flakes

### Solana & Blockchain (4 scripts)
- `find_solana_sources.sh` - Find Solana sources
- `pull_solana_contracts.sh` - Pull contracts
- `decompile_solana_contracts.sh` - Decompile contracts
- `launch_trading_network.sh` - Launch trading network

### Proof & Verification (5 scripts)
- `prove-reproducibility.sh` - Prove reproducibility
- `complete_so_proof.sh` - Complete SO proof
- `final_so_proof.sh` - Final SO proof
- `prove_71_equivalence.sh` - Prove equivalence
- `verify_binaries.sh` - Verify binary integrity

## Data Files & Indexes

### JSON Data
- `arrow_matches.json` - Arrow pattern matches
- `conformal_arrows.json` - Conformal arrow mappings
- `emoji_frequencies.json` - Emoji frequency analysis
- `ngram_orbits.json` - N-gram orbit data
- `prime_harmonics.json` - Prime harmonic analysis
- `word_emoji_connections.json` - Word-emoji connections
- `successful_packages.json` - Successful package builds

### Text Indexes
- `untracked_meta_introspector.txt` - Untracked files
- `ticket_summary.txt` - Ticket summaries

### CSV Data
- `FILE_GIT_MAPPING.csv` - File to git object mapping

### Other Formats
- `metamemecoin.tt` - Metameme coin data

## Documentation Categories

### System Design (8 docs)
- `SINGULARITY_DESIGN.md` - Singularity system design
- `COMPLETE_SINGULARITY.md` - Complete singularity spec
- `SELF_DESCRIBING_COMPLETE.md` - Self-describing system
- `MYCELIUM_NETWORK.md` - Mycelium network architecture
- `LLM_MYCELIUM_NETWORK.md` - LLM mycelium integration
- `MODEL_LATTICE_V1.md` - Model lattice design

### Indexing & Analysis (6 docs)
- `FILE_INDEX_COMPLETE.md` - Complete file index
- `FILE_INDEX_ANALYSIS.md` - File index analysis
- `PARQUET_FILE_INDEX.md` - Parquet-based index
- `CENTRALIZATION_INDEX.md` - Centralization index
- `FIND_GREP_COMPLETE_AUDIT.md` - Find/grep audit
- `AUDIT_SUMMARY.md` - Audit summary

### Mathematical & Theoretical (5 docs)
- `BYTE_HOMOTOPY.md` - Byte-level homotopy
- `HOMOMORPHIC_HOMOTOPY.md` - Homomorphic homotopy
- `HOMOTOPY_MONSTER_INDEX.md` - Homotopy monster group
- `BOTT_PERIODICITY_LABELING.md` - Bott periodicity
- `TICKET_ORBIT_MAPPING.md` - Ticket orbit mapping

### Compilation & Witness (4 docs)
- `COMPILATION_AS_WITNESS.md` - Compilation as proof
- `MES_WITNESS_V1.md` - MES witness system
- `MES_AS_LABELER.md` - MES as labeler
- `AUTOLABEL_BOOTSTRAP.md` - Auto-labeling bootstrap

### Infrastructure (7 docs)
- `CURRENT_SERVER_SPECS.md` - Server specifications
- `POWER_REQUIREMENTS.md` - Power requirements
- `ABSOLUTE_PATH_AUDIT.md` - Absolute path audit
- `PUBLIC_PRIVATE_SEPARATION.md` - Public/private separation
- `READY_TO_BUILD.md` - Build readiness
- `HISTORICAL_PROJECTS.md` - Historical project tracking
- `SOURCEFORGE_LINEAGE.md` - SourceForge lineage

### Advanced Concepts (4 docs)
- `INTENT_PREDICTION.md` - Intent prediction
- `PLATOS_CAVE_MINING.md` - Plato's cave mining
- `GITHUB_TRANSITIVE_CLOSURE.md` - GitHub transitive closure
- `EMACS_CREATED_GCC.md` - Emacs-created GCC

## Key Patterns & Concepts

### File → Git Object Mapping
```
file_path → git hash-object → blob_sha → git_repo + commit
```

### Symbol → Location Mapping
```
symbol_name → file_path → git_object → line_number
```

### Process → Build Mapping
```
nix build → process_tree → strace/perf → telemetry
```

### Telemetry → Intelligence
```
syscalls + perf → patterns → predictions → optimizations
```

## Service Architecture

### File Index Service
```
HTTP API (port 3030)
  ↓
In-Memory Index (priority-sorted)
  ↓
Parquet Cache (/mnt/data1/meta-introspector/indexes/)
  ↓
SQLite DB (file_index.db)
```

### Search Utils V2
```
Shell Script → HTTP Request → File Index Service → Cached Result
```

## Next Steps

1. **Start file_index_service** - Compile and run the service
2. **Ingest existing lists** - Load ~/nix/index/allrs.txt
3. **Scan /mnt/data1/** - Find all git repos
4. **Build FILE_GIT_MAPPING.csv** - Map files to git objects
5. **Extract symbols** - Parse and map symbol locations
6. **Track nix builds** - Monitor with strace/perf
7. **Analyze telemetry** - Generate insights

## Keywords by Category

### Core Infrastructure
- file_index, git_repo, git_object, symbol, location, mapping
- parquet, sqlite, cache, index, service, server, cli

### Discovery & Enumeration
- find, scan, discover, enumerate, list, collect, ingest

### Git Operations
- commit, branch, remote, hash-object, ls-files, pack, blob

### Analysis
- markov, eigenvector, similarity, orbit, complexity, pattern
- telemetry, trace, strace, perf, syscall, profile

### Build & Compilation
- nix, cargo, rustc, build, compile, intercept, witness, proof

### Data Formats
- parquet, json, csv, sqlite, txt, md

### Mathematical
- homotopy, homomorphic, bott, periodicity, lattice, orbit

### System
- process, memory, cpu, thermal, power, server, infrastructure
