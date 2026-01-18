# Code Flow Analysis Workflow

## Objective
Trace code from source → compiler → runtime → binary, model as vectors, find duplicates

## Pipeline Stages

### 1. Source Code Analysis
**Input**: Rust source files (`.rs`)
**Tools**: 
- `markov_chain_miner.rs` - Character-level Markov modeling
- `code_grammar_analyzer.rs` - Grammar pattern extraction

**Output**:
- `markov_symbol_scores.parquet` - Symbol transition probabilities
- Character n-gram frequencies
- Code pattern signatures

**Vector representation**: Character transition matrices

---

### 2. Syn AST Collection
**Input**: Rust source files
**Tools**:
- `syn` crate - Parse to AST
- `ast_extractor.rs` - Extract AST nodes
- `syn_visitor.rs` - Visit all nodes

**Output**:
- `data/syn_asts/*.json` - Serialized ASTs
- Function signatures
- Type definitions
- Macro expansions

**Vector representation**: AST node type frequencies, depth, branching factor

---

### 3. HIR (High-level IR) Collection
**Input**: Rust source + rustc
**Tools**:
- `rustc -Z unpretty=hir` - Dump HIR
- `hir_collector.rs` - Parse HIR output
- `nix_cargo_interceptor.rs` - Intercept rustc calls

**Output**:
- `data/hir/*.hir` - HIR dumps
- Type resolution info
- Trait bounds
- Lifetime annotations

**Vector representation**: Type graph embeddings, trait usage patterns

---

### 4. MIR (Mid-level IR) Collection
**Input**: Rust source + rustc
**Tools**:
- `rustc -Z unpretty=mir` - Dump MIR
- `mir_collector.rs` - Parse MIR output
- Control flow graph extraction

**Output**:
- `data/mir/*.mir` - MIR dumps
- Basic blocks
- Control flow graphs
- Borrow checker info

**Vector representation**: CFG topology, basic block sizes, branch patterns

---

### 5. Compiler Trace Collection
**Input**: Build process
**Tools**:
- `nix-telemetry` - LD_PRELOAD hook
- `rust_preload_interceptor` - Intercept syscalls
- `telemetry_hook_test_driver.rs` - Test harness
- `full_rustc_compressor.rs` - Compress traces

**Output**:
- `data/telemetry/rustc_trace.parquet` - Syscall traces
- Compilation time per file
- Memory usage
- File I/O patterns

**Vector representation**: Syscall sequence embeddings, resource usage vectors

---

### 6. Binary Analysis
**Input**: Compiled binaries
**Tools**:
- `moonshine_analysis.rs` - ELF analysis
- `elf_moonshine_map.txt` - Symbol mapping
- `real_compile_proof.rs` - Verify compilation

**Output**:
- `data/binaries/elf_analysis.json` - Binary metadata
- Symbol tables
- Section sizes
- Dependencies

**Vector representation**: Symbol frequency, section size ratios, dependency graphs

---

### 7. Runtime Tracing
**Input**: Running binaries
**Tools**:
- `strace` wrapper
- `perf` integration
- `telemetry_lib.rs` - Runtime hooks

**Output**:
- `data/runtime/syscall_traces.parquet` - Runtime syscalls
- Performance counters
- Memory allocations
- I/O operations

**Vector representation**: Execution path embeddings, resource consumption patterns

---

### 8. Vector Modeling
**Input**: All collected data
**Tools**:
- `eigenvector_analysis.rs` - Compute eigenvectors
- `markov_similarity.rs` - Compare Markov models
- `code_similarity_detector.rs` - Semantic similarity

**Process**:
1. **Character-level**: Markov transition matrices → eigenvectors
2. **AST-level**: Node type frequencies → TF-IDF vectors
3. **IR-level**: CFG topology → graph embeddings
4. **Binary-level**: Symbol distributions → frequency vectors
5. **Runtime-level**: Syscall sequences → sequence embeddings

**Output**:
- `data/vectors/code_embeddings.parquet` - Unified vector space
- Similarity matrices
- Cluster assignments

---

### 9. Duplicate Detection
**Input**: Vector embeddings
**Tools**:
- `duplicate_finder.rs` - Hash-based detection
- `parallel_duplication_scanner.rs` - Parallel scanning
- `code_duplication_scanner.rs` - Semantic duplicates

**Methods**:
1. **Exact duplicates**: SHA256 hashing
2. **Near duplicates**: Cosine similarity > 0.95
3. **Semantic duplicates**: Edit distance on AST
4. **Functional duplicates**: Same MIR/binary output

**Output**:
- `DUPLICATE_CODE_REPORT.md` - Duplicate clusters
- `data/duplicates/clusters.json` - Grouped duplicates
- Uniqueness scores per file

---

### 10. Uniqueness Analysis
**Input**: Duplicate detection results
**Tools**:
- `untracked_report.rs` - Report by repo
- `novel_code_detector.rs` - Find unique patterns

**Metrics**:
- **Unique code %**: Lines not duplicated elsewhere
- **Novel patterns**: AST patterns not in other repos
- **Unique algorithms**: MIR patterns unique to Mike's repos
- **Original implementations**: No similar code in 3.66M file index

**Output**:
- `MIKE_UNIQUE_CODE_REPORT.md` - Uniqueness analysis
- `data/unique/novel_patterns.json` - Novel code patterns
- Comparison vs all indexed repos

---

## Data Flow Diagram

```
Source Code (.rs)
    ↓
[Markov Model] → Character vectors
    ↓
[Syn Parser] → AST vectors
    ↓
[rustc -Z unpretty=hir] → HIR vectors
    ↓
[rustc -Z unpretty=mir] → MIR vectors
    ↓
[Compiler Trace] → Build vectors
    ↓
[Binary Analysis] → Symbol vectors
    ↓
[Runtime Trace] → Execution vectors
    ↓
[Vector Space] → Unified embeddings
    ↓
[Similarity Search] → Duplicate detection
    ↓
[Uniqueness Score] → Novel code identification
```

---

## Canonical Data Store

All outputs stored in Parquet format:

```
data/
├── indexes/
│   └── files.parquet              # 3.66M files with git provenance
├── markov-analysis/
│   └── markov_symbol_scores.parquet
├── syn-asts/
│   └── ast_nodes.parquet
├── hir/
│   └── hir_dumps.parquet
├── mir/
│   └── mir_dumps.parquet
├── telemetry/
│   └── rustc_trace.parquet
├── binaries/
│   └── elf_analysis.parquet
├── runtime/
│   └── syscall_traces.parquet
├── vectors/
│   └── code_embeddings.parquet
└── duplicates/
    └── clusters.parquet
```

---

## Execution Plan

### Phase 1: Collection (Parallel)
```bash
# Run all collectors in parallel
cargo run --release --bin markov_chain_miner -- mike_repos_rust_files.txt &
cargo run --release --bin ast_extractor -- mike_repos_rust_files.txt &
cargo run --release --bin hir_collector -- mike_repos_rust_files.txt &
cargo run --release --bin mir_collector -- mike_repos_rust_files.txt &
```

### Phase 2: Compilation Tracing
```bash
# Intercept rustc during build
LD_PRELOAD=./target/release/libnix_telemetry.so cargo build --release
```

### Phase 3: Binary Analysis
```bash
cargo run --release --bin moonshine_analysis -- /opt/zos-production/target/release/*
```

### Phase 4: Vector Modeling
```bash
cargo run --release --bin eigenvector_analysis
cargo run --release --bin markov_similarity
```

### Phase 5: Duplicate Detection
```bash
cargo run --release --bin parallel_duplication_scanner -- mike_repos_rust_files.txt
```

### Phase 6: Uniqueness Report
```bash
cargo run --release --bin novel_code_detector -- mike_repos_rust_files.txt
```

---

## GitHub Actions Integration

Workflow: `.github/workflows/code-flow-analysis.yml`

```yaml
- name: Collect all stages
  run: |
    cargo run --release --bin markov_chain_miner
    cargo run --release --bin ast_extractor
    cargo run --release --bin eigenvector_analysis
    
- name: Upload artifacts
  uses: meta-introspector/upload-artifact@v4
  with:
    name: code-flow-analysis
    path: data/**/*.parquet
```

---

## Mathematical Representation

Each code file becomes a vector in high-dimensional space:

```
v(file) = [
    markov_eigenvalues[0..100],      # Character patterns
    ast_node_frequencies[0..50],      # Syntax patterns
    hir_type_embeddings[0..100],      # Type patterns
    mir_cfg_features[0..50],          # Control flow
    binary_symbol_dist[0..100],       # Binary patterns
    runtime_syscall_seq[0..100]       # Execution patterns
]
```

**Similarity**: `sim(v1, v2) = cosine(v1, v2)`

**Uniqueness**: `unique(v) = 1 - max(sim(v, all_other_vectors))`

---

## Expected Outputs

1. **Mike's Code Fingerprint**: Unique vector signature
2. **Duplicate Report**: What code is shared/unique
3. **Novel Patterns**: Algorithms/patterns not found elsewhere
4. **Compilation Flow**: Source → Binary trace
5. **Uniqueness Score**: % of truly original code

---

## Tools Status

✅ Built and working:
- markov_chain_miner
- eigenvector_analysis
- moonshine_analysis
- telemetry collectors
- duplicate_finder

🚧 Need to build:
- ast_extractor (use syn)
- hir_collector (rustc wrapper)
- mir_collector (rustc wrapper)
- novel_code_detector

---

**Next Step**: Run Phase 1 collectors on Mike's 496 Rust files
