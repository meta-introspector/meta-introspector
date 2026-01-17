# File I/O Operations Inventory

**Generated**: 2026-01-17  
**Purpose**: Central documentation of all file read/write operations across the codebase

## Overview

This document catalogs all file I/O operations to:
- Track data dependencies
- Identify temporary files
- Document configuration files
- Map input/output flows
- Support reproducibility audits

## File I/O by Module

### 1. minimal_build_server.rs (14 operations)

**Reads:**
- `static/index.html` - HTML template
- `/home/{user}/.ssh/id_ed25519.pub` - SSH public key
- Error files for diagnostics

**Writes:**
- `/tmp/gpg-batch` - GPG batch configuration
- `/tmp/gpg-qa` - QA GPG configuration
- `/tmp/vault.json` - Vault data
- `/etc/systemd/system/qa-build-server.service` - Systemd service file

**Operations:**
- File copying for deployment
- In-place file modifications

### 2. markov_resonance_analyzer/src/main.rs (13 operations)

**Reads:**
- Cached file lists
- `failed_files_exclude.txt` - Exclusion list
- Source files for analysis

**Writes:**
- `markov_symbol_scores_partial.json` - Partial results
- `markov_symbol_scores.json` - Final scores
- `markov_file_index_mapping.json` - File index mapping
- `markov_similarity_matrix.bin` - Binary matrix data
- `markov_similarity_matrix_meta.json` - Matrix metadata
- `failed_files_exclude.txt` - Failed file list

### 3. nixso2wrap/src/main.rs (12 operations)

**Reads:**
- ABI JSON files

**Writes:**
- JSON output files (various)
- Analysis files
- ABI files for libraries
- MCP directory files

**Operations:**
- `create_dir_all` for output directories

### 4. ldd2wrap_all_calls.rs (10 operations)

**Reads:**
- Build data files
- Directory scanning
- Script files
- Binary files

**Writes:**
- Wrapper files
- Master files
- JSON files
- Telemetry configuration

**Operations:**
- Dynamic wrapper generation
- Binary analysis

### 5. generate-monthly-reports.rs (9 operations)

**Reads:**
- Activity directories (platform/user/year/month structure)
- Activity JSON files

**Writes:**
- `output_file` - Monthly reports (JSON)

**Operations:**
- Recursive directory traversal
- Activity aggregation

### 6. backend_equivalence.rs (8 operations)

**Reads:**
- `/tmp/llvm_out.ll` - LLVM IR
- `/tmp/llvm_out.s` - LLVM assembly
- `/tmp/gcc_out.s` - GCC assembly

**Writes:**
- Temporary source files
- `/tmp/gcc_input.c` - GCC input

**Operations:**
- Temporary file cleanup
- Compiler output comparison

### 7. eigenvector_word_model.rs (7 operations)

**Reads:**
- Binary directory scanning
- Binary files
- `perf_json` - Performance data
- Source files

**Writes:**
- `data/eigenvectors/word_eigenvectors.json` - Eigenvector data
- `bott8-layout-solver/word_eigenvectors.dzn` - MiniZinc format

### 8. source2test.rs (7 operations)

**Reads:**
- Trace files
- Sample files

**Writes:**
- Split data files
- Test code files
- JSON output files
- Sample test files

### 9. conformal_structure_analyzer.rs (6 operations)

**Reads:**
- Directory scanning
- Source files
- Binary files

**Writes:**
- `automorphism_analysis.json`
- `conformal_boundary.json`
- `rustjunk_eigenvector.json`

### 10. bits_to_rust.rs (6 operations)

**Reads:**
- WASM files

**Writes:**
- Temporary source files
- WASM output files

**Operations:**
- Temporary file cleanup

### 11. report_generator.rs (5 operations)

**Reads:**
- `UNIFIED_MASTER_TABLE.md`
- `NOVELTY_REPORT.md`
- `MULTI_PROJECT_FUNCTION_TABLE.md`

**Writes:**
- `CODE_ANALYSIS.parquet` - Parquet format
- `CODE_ANALYSIS.html` - HTML report

### 12. full_rustc_compressor.rs (5 operations)

**Reads:**
- Source files
- `/mnt/data1/files.txt` - File list
- Directory scanning

**Writes:**
- Decompressed content files
- `rust_build_full_compressed.json` - Compressed data

### 13. metis-partition-markov.rs (5 operations)

**Reads:**
- Matrix files
- Partition files

**Writes:**
- METIS format files
- `hf-markov-analysis-upload/partition_{:04}.json` - Partition data

### 14. rustc_perf_scanner.rs (5 operations)

**Reads:**
- Source files
- Directory scanning

**Writes:**
- Temporary source files

**Operations:**
- Temporary file cleanup (`/tmp/perf.data`)

### 15. binary_symbol_study.rs (5 operations)

**Reads:**
- Performance data files
- `data/build_analysis/real_build_1768332029_binaries.json`
- Binary files

**Writes:**
- JSON output files

### 16. rustc_tracer.rs (5 operations)

**Reads:**
- Input files
- Artifact files
- Binary files

**Writes:**
- JSON trace files
- Sample test files

### 17. untracked_risk_analyzer.rs (5 operations)

**Reads:**
- Source files
- `FILE_GIT_MAPPING.csv`

**Writes:**
- `UNTRACKED_HIGH_RISK.md` - Risk report

### 18. reach_tracer.rs (5 operations)

**Reads:**
- Trace files
- Source files
- Output files

**Writes:**
- JSON trace files
- Sample test files

### 19. audit_macro_applier.rs (5 operations)

**Reads:**
- Parquet files
- Labeled files
- Source files

**Writes:**
- Modified source files

### 20. save_compressed_data.rs (5 operations)

**Reads:**
- Source files

**Writes:**
- Compressed files (multiple formats)
- Gzip files
- XZ files
- Bzip2 files

## File Patterns

### Temporary Files
- `/tmp/gpg-batch`
- `/tmp/gpg-qa`
- `/tmp/vault.json`
- `/tmp/gcc_input.c`
- `/tmp/llvm_out.ll`
- `/tmp/llvm_out.s`
- `/tmp/gcc_out.s`
- `/tmp/perf.data`
- `/tmp/metadata.txt`
- `/tmp/Dockerfile`

### Configuration Files
- `/etc/systemd/system/qa-build-server.service`
- `static/index.html`
- `/home/{user}/.ssh/id_ed25519.pub`

### Data Files
- `*.json` - Primary data format
- `*.parquet` - Columnar data
- `*.bin` - Binary matrices
- `*.csv` - Tabular data
- `*.md` - Reports

### Output Directories
- `data/eigenvectors/`
- `hf-markov-analysis-upload/`
- `bott8-layout-solver/`
- Build directories (dynamic)

## I/O Patterns

### Read Patterns
1. **Directory Scanning**: `fs::read_dir()` for recursive traversal
2. **Text Files**: `fs::read_to_string()` for source/config
3. **Binary Files**: `fs::read()` for binaries/artifacts
4. **Structured Data**: JSON/Parquet parsing

### Write Patterns
1. **JSON Output**: `serde_json::to_string_pretty()`
2. **Binary Output**: Direct byte writes
3. **Reports**: Markdown/HTML generation
4. **Temporary Files**: `/tmp/` prefix

### Error Handling
- `.ok()` - Silent failure
- `.unwrap()` - Panic on error
- `.unwrap_or_default()` - Fallback values
- `?` operator - Error propagation

## Recommendations

### Security
- [ ] Audit all `/tmp/` file usage for race conditions
- [ ] Review SSH key access patterns
- [ ] Validate systemd service file writes (requires root)

### Reliability
- [ ] Replace `.unwrap()` with proper error handling
- [ ] Add file locking for concurrent access
- [ ] Implement atomic writes for critical files

### Performance
- [ ] Cache frequently read files
- [ ] Use memory-mapped I/O for large files
- [ ] Batch directory operations

### Reproducibility
- [ ] Document all input file formats
- [ ] Version output file schemas
- [ ] Add checksums for data files

## Statistics

- **Total files analyzed**: 277
- **Total I/O operations**: 742+
- **Modules with I/O**: 30+
- **Temporary files**: 10+
- **Output formats**: JSON, Parquet, HTML, Markdown, Binary

## Data Directory Producers

### data-eigenvectors/
**Producer**: `eigenvector_word_model.rs`
**Outputs**:
- `data/eigenvectors/word_eigenvectors.json` - Word eigenvector analysis
- `bott8-layout-solver/word_eigenvectors.dzn` - MiniZinc format

**Purpose**: Extracts eigenvector representations from binary symbols and performance data

### data-markov-analysis/
**Producer**: `metis-partition-markov.rs`
**Outputs**:
- `hf-markov-analysis-upload/partition_{:04}.json` - Markov chain partitions

**Purpose**: METIS-based partitioning of Markov similarity matrices

### data-moonshine/
**Producer**: `elf_moonshine_detector.rs`, `symbol_similarity/src/moonshine.rs`
**Outputs**:
- `elf_moonshine_map.txt` - Modular forms in codec resonance

**Consumers**: `codec_binary_extractor.rs`, `symbol_similarity/src/codec_extractor.rs`

**Purpose**: Detects modular forms and periodic patterns in ELF binary codec switches

### data-telemetry/
**Producers**: Multiple telemetry systems
- `nix_telemetry_preload.rs` → `/mnt/data1/meta-introspector/data/parquet_telemetry/{session}.csv`
- `rust-telemetry-driver/src/main.rs` → `/tmp/rust_telemetry_{session}.jsonl`
- `parquet_telemetry_proof.rs` → `/tmp/nix_bootstrap_telemetry_{pid}.csv`
- `telemetry_server.rs` - Structured telemetry capture server

**Purpose**: Comprehensive system call and build process telemetry collection

### data-blockchain/
**Producer**: `universal_client_node.rs`
**Data Structure**: `BlockchainSO` - ZOS server as blockchain

**Purpose**: Blockchain-based shared object registry with ZK proofs

### data-const71/
**Producer**: `flake-71-perf-collector/src/main.rs`
**Input**: `/mnt/data1/meta-introspector/const_71_test/` - 71 language test directories
**Outputs**: Performance data from multi-language Nix builds

**Languages tested**: xml, prolog, vhdl, whitespace, tcl, z3, metacoq, jax_gpu, pytorch, fish, neo4j, qiskit, verilog, asm_riscv, php, node, json, rust, genetic, idris2, redis, brainfuck, python, agda, julia, unlambda, asm_x86_64, graph_partition, asm_aarch64, intercal, smt2, isabelle, datalog, perl, bash, ook, cirq, graphql, move, asm_mips, lean4, mcts, solidity, ini, lua, nix_derivation, ocaml, scheme, rockstar, r, zsh, mongodb, asm_wasm, minizinc, tensorflow, sparql, asm, nix_expr, befunge, malbolge, haskell, piet, toml, coq, gcc, llvm, ruby, chisel, sql, vyper, yaml

**Purpose**: Collect performance metrics across 71 different programming languages and toolchains

## Related Documentation

- [FILE_INDEX.md](FILE_INDEX.md) - Complete file listing
- [QUALITY_AUDIT.md](../QUALITY_AUDIT.md) - Quality status
- [DEMO2CODE_POLICY.md](../DEMO2CODE_POLICY.md) - Production standards
