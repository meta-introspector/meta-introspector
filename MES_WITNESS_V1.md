# Mes Bootstrap Witness v1

## Objective

Capture the **complete bootstrap of GNU Mes** as a verifiable witness using:
- **strace**: Every syscall with arguments, results, and timing
- **perf**: CPU cycles, instructions, cache misses, call graphs
- **Build logs**: Complete compilation output

## Files

### Input
- `/mnt/data1/nix/time/2024/05/30/mes/` - GNU Mes source

### Output (in `/mnt/data1/meta-introspector/witnesses/mes-v1/`)
- `mes_bootstrap.strace` - Raw strace output
- `mes_bootstrap.perf.data` - Raw perf data
- `mes_build.log` - Build output
- `syscall_summary.txt` - Syscall frequency
- `mes_perf_report.txt` - Perf analysis
- `mes_bootstrap_witness_v1.parquet` - Strace in parquet
- `mes_perf_witness_v1.parquet` - Perf in parquet

## Schema

### mes_bootstrap_witness_v1.parquet
```
timestamp: float64    # Unix timestamp with microseconds
pid: uint32          # Process ID
syscall: string      # Syscall name (open, read, write, etc.)
args: string         # Syscall arguments
result: string       # Return value
duration: float64    # Syscall duration in seconds
```

### mes_perf_witness_v1.parquet
```
timestamp: float64   # Sample timestamp
pid: uint32         # Process ID
event: string       # cycles, instructions, cache-misses, etc.
count: uint64       # Event count
callchain: string   # Call stack (dwarf)
```

## Build

```bash
./build_mes_witness_v1.sh
```

## Analysis

```bash
# Analyze witness
cargo run --bin analyze_witness witnesses/mes-v1/

# Classify with LMFDB
cargo run --bin classify_witness witnesses/mes-v1/

# Push to HuggingFace
cargo run --bin push_witness_to_hf witnesses/mes-v1/
```

## What This Captures

1. **Every syscall** during Mes bootstrap
2. **CPU performance** metrics
3. **Call graphs** showing execution flow
4. **Timing** for every operation
5. **Complete provenance** from source to binary

## The Witness

This is **witness v1** - the first layer of the bootstrap:
- Complexity: 0.001
- LMFDB orbit: 1.a1
- Size: ~5 KB
- Dependencies: None (seed)

Each subsequent build (TinyCC, GCC, Nix, etc.) will be witness v2, v3, etc., with increasing complexity.

## Verification

The witness can be verified by:
1. Replaying the strace
2. Comparing perf metrics
3. Checking LMFDB classification
4. Verifying homotopy from source to binary

## Next Witnesses

- v2: TinyCC (complexity 1.0, orbit 23.a1)
- v3: GCC (complexity 5.0, orbit 47.a1)
- v4: Nix (complexity 10.0, orbit 71.a1)
- v5: Postgres (complexity 15.0, orbit 71.a2)
- v6: Rustc (complexity 50.0, orbit 71.a3)
- v7: Lean4 (complexity 30.0, orbit 71.a4)
- v8: MiniZinc (complexity 20.0, orbit 71.a5)
- v9: Singularity (complexity 100.0, orbit 71.a6)
