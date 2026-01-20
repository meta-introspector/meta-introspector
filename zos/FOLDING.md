# Trace Folding System

Combines all analysis traces into unified parquet files.

## Folding Dimensions

```
File Path (key)
  ├── Markov: score, neighbors
  ├── Context: usage patterns, frequency
  ├── Compile: time, dependencies
  ├── Perf: cycles, instructions, cache misses
  ├── Strace: syscalls, counts
  ├── Network: hosts, bytes
  └── Build Logs: success, warnings
```

## Input Sources

### From Build Logs
- `nix_build_logs.parquet` - Compile traces
- `rustc_trace_schema.parquet` - Compiler telemetry

### From Validation
- `markov_symbol_scores.parquet` - Symbol analysis
- `perf.data` - Performance traces
- `strace.log` - Syscall traces
- `network.log` - Network access

### From 3M Index
- `files.parquet` - File metadata
- `repos.parquet` - Repository context

## Folding Process

```rust
// Load all traces
let markov = load_parquet("markov_symbol_scores.parquet");
let compile = load_parquet("nix_build_logs.parquet");
let perf = parse_perf("perf.data");
let strace = parse_strace("strace.log");
let network = parse_network("network.log");

// Fold by file path
let folded = fold_traces(markov, compile, perf, strace, network);

// Save compressed
save_parquet("folded_analysis.parquet", folded);
```

## Output Schema

```
folded_analysis.parquet:
  - file_path: string
  - markov_score: float64
  - markov_neighbors: list<string>
  - context_usage: list<string>
  - compile_time_ms: uint64
  - compile_dependencies: list<string>
  - perf_cycles: uint64
  - perf_instructions: uint64
  - perf_cache_misses: uint64
  - syscalls: list<string>
  - syscall_count: uint64
  - network_hosts: list<string>
  - network_bytes: uint64
  - build_success: bool
  - build_warnings: uint64
```

## Compression

Raw traces → Folded parquet:
- 100MB logs → 5MB parquet
- Indexed by file path
- Queryable with DuckDB/Arrow

## Usage

```bash
# Fold all traces
cargo run --bin fold_traces zos-validation/layer-2

# Query folded data
duckdb -c "
  SELECT file_path, markov_score, compile_time_ms, syscall_count
  FROM 'folded_analysis.parquet'
  WHERE markov_score > 0.8
  ORDER BY compile_time_ms DESC
  LIMIT 10
"
```

## Analysis Queries

### Find expensive files
```sql
SELECT file_path, compile_time_ms, perf_cycles
FROM folded_analysis
WHERE compile_time_ms > 1000
ORDER BY perf_cycles DESC;
```

### Find network-heavy files
```sql
SELECT file_path, network_hosts, network_bytes
FROM folded_analysis
WHERE network_bytes > 1000000;
```

### Find suspicious syscalls
```sql
SELECT file_path, syscalls
FROM folded_analysis
WHERE list_contains(syscalls, 'execve')
   OR list_contains(syscalls, 'ptrace');
```

## Integration

```bash
# In validation pipeline
./validate-layer.sh 2 /nix/store/.../binary
  ↓
fold_traces zos-validation/layer-2
  ↓
query_folded zos-validation/layer-2/folded_analysis.parquet
  ↓
Graduate or Quarantine
```

This creates a **unified view** of all analysis dimensions for each file.
