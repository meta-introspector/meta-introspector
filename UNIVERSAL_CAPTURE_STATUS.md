# Universal Data Capture - Architecture Sketch

## Vision
Capture every function call during Nix Rust bootstrap with complete provenance, structured in Parquet for analysis.

## What We Have ✅

### 1. Symbol Extraction (build.rs)
- ✅ Reads 92 .so files from real Nix build
- ✅ Extracts 37,756 symbols with goblin
- ✅ LMFDB conductor ranking (3000-10000+)
- ✅ Harmonic filtering (strings, memory, io, crypto, etc.)
- ✅ Environment vars: LMFDB_HARMONIC_FILTER, LMFDB_FILTER_PERCENT

### 2. Safe Telemetry (nix-telemetry)
- ✅ Feature-gated wrappers
- ✅ Safe print library (raw SYS_write, no recursion)
- ✅ Proven: 19,000+ calls without segfault
- ✅ Recursion guard

### 3. Interception Macros (nix_cargo_interceptor.rs)
- ✅ `intercept_rustc!` - crate, version, source, flags
- ✅ `intercept_cargo!` - dependencies, metadata
- ✅ `intercept_nix!` - derivation, store paths, build inputs
- ✅ `intercept_linker!` - linked libraries, ELF data

### 4. Symbol Dissolution (symbol_dissolver_macros.rs)
- ✅ `dissolve_abi!` - Parse ABI from ELF
- ✅ `dissolve_bytes!` - Extract bytes + SHA256
- ⚠️  `dissolve_source!` - DWARF parsing (TODO)
- ⚠️  `dissolve_docs!` - Documentation extraction (TODO)
- ⚠️  `dissolve_usage!` - Usage pattern analysis (TODO)

### 5. Logistical Graph (logistical_graph.rs)
- ✅ Graph structure defined
- ✅ Node: symbol + crate + derivation + conductor
- ✅ Edges: Build, Link, Call, Nix dependencies
- ⚠️  Topological sort (skeleton)
- ⚠️  Critical path analysis (TODO)
- ⚠️  LMFDB clustering (skeleton)

## What's Missing ❌

### 1. Actual Wrapper Generation
- ❌ Generate 37k wrappers from build.rs
- ❌ Use macros to dissolve each symbol
- ❌ Handle different function signatures automatically
- ❌ Type-safe argument serialization

### 2. Parquet Streaming
- ❌ Real-time streaming to Parquet (currently CSV/JSONL)
- ❌ Arrow schema definition
- ❌ Compression and batching
- ❌ Partition by crate/derivation

### 3. Runtime Capture
- ❌ LD_PRELOAD that doesn't segfault
- ❌ Handle malloc/free without recursion
- ❌ Capture call stacks
- ❌ Thread-safe logging

### 4. Graph Construction
- ❌ Parse actual call chains from runtime data
- ❌ Build complete dependency graph
- ❌ Compute critical path
- ❌ Identify bottlenecks

### 5. Integration
- ❌ Hook into Nix build process
- ❌ Capture full Rust bootstrap
- ❌ Correlate symbols across crates
- ❌ Generate final Parquet dataset

## Next Steps

1. **Fix LD_PRELOAD recursion** - Use dlsym to get real functions
2. **Generate all 37k wrappers** - Expand build.rs to create full set
3. **Add Parquet writer** - Replace CSV with arrow/parquet
4. **Test on small build** - Capture single crate first
5. **Scale to bootstrap** - Full Nix Rust build capture

## The Golden Grail

```
Nix Rust Bootstrap
    ↓
Intercept all callbacks (rustc, cargo, nix, linker)
    ↓
Dissolve 37k symbols (ABI, bytes, source, docs, usage)
    ↓
Build logistical graph (dependencies, build order, call chains)
    ↓
Stream to Parquet (structured, typed, compressed)
    ↓
Universal dataset: Every function call with complete provenance
```

## Current Status

**Sketch complete**: Architecture designed, key components prototyped  
**Proof of concept**: LMFDB filtering works, safe print prevents recursion  
**Ready for**: Implementation of full wrapper generation and Parquet streaming

The foundation is solid - now we build! 🚀
