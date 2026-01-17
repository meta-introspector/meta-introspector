# Reachable Rust: A Provenance-Tracking Compiler Backend

## Vision

Create a Rust compiler backend that captures byte-level reachability for every compilation, enabling harmonic decomposition of the entire Rust ecosystem into mathematical orbits.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Reachable Rust                            │
│                                                              │
│  rustc + reachability backend                               │
│    ↓                                                         │
│  Every compilation produces:                                │
│    • Binary output (.so, .exe)                              │
│    • Reachability trace (.parquet)                          │
│    • Harmonic signature (.json)                             │
└─────────────────────────────────────────────────────────────┘
```

## Phase 1: QEMU Backend (Current)

**Status**: ✅ Working

```bash
# Compile with reachability tracking
qemu-x86_64 -plugin libreachability.so rustc input.rs
# Output: binary + input→insn→output mapping
```

**Limitation**: 10-100x slower (QEMU overhead)

## Phase 2: Rustc Backend (Next)

**Goal**: Native reachability tracking in rustc codegen

### Implementation Strategy

1. **Fork rustc codegen**
   ```rust
   // In rustc_codegen_llvm or rustc_codegen_ssa
   fn emit_reachability_metadata(
       input_span: Span,
       llvm_insn: LLVMValueRef,
       output_offset: usize
   ) {
       REACHABILITY_TRACKER.record(input_span, llvm_insn, output_offset);
   }
   ```

2. **Instrument LLVM IR generation**
   - Track source spans → LLVM IR
   - Track LLVM IR → machine code
   - Track machine code → binary offsets

3. **Emit parquet alongside binary**
   ```
   rustc input.rs
   # Produces:
   #   input (binary)
   #   input.reach.parquet (reachability)
   ```

### Rustc Integration Points

```rust
// In rustc_codegen_ssa/src/back/write.rs
pub fn emit_reachability_data(
    cgcx: &CodegenContext,
    module: &ModuleCodegen,
) -> Result<PathBuf, FatalError> {
    let parquet_path = module.name.clone() + ".reach.parquet";
    
    // Write reachability data
    write_parquet(
        &parquet_path,
        &cgcx.reachability_records
    )?;
    
    Ok(parquet_path)
}
```

## Phase 3: Self-Hosting Reachable Rust

**Goal**: Compile rustc with reachability backend

```bash
# Bootstrap reachable rustc
./x.py build --stage 1 --enable-reachability

# Now rustc itself produces reachability data
rustc-reachable input.rs
# Output: binary + input.reach.parquet
```

### Self-Compilation Data

When rustc compiles itself:
```
rustc source (500K LOC)
    ↓
[Reachable Rustc]
    ↓
rustc binary (100MB)
    ↓
rustc.reach.parquet (1GB)
    ↓
Complete provenance of entire compiler
```

## Phase 4: Harmonic Decomposition

**Goal**: Split Rust ecosystem into mathematical orbits

### Process

1. **Collect reachability data** from all crates
   ```bash
   cargo build --reachability
   # Every crate produces .reach.parquet
   ```

2. **Compute harmonic signatures**
   ```bash
   harmonic_filter < all_crates.parquet > signatures.json
   ```

3. **Cluster by homotopy class**
   ```bash
   homotopy_classifier < signatures.json > orbits.json
   ```

4. **Identify orbits**
   ```json
   {
     "orbit_1": {
       "genus": 2,
       "conductor": 150,
       "crates": ["tokio", "async-std", "futures"],
       "lmfdb_id": "11.2.150.a"
     },
     "orbit_2": {
       "genus": 1,
       "conductor": 50,
       "crates": ["serde", "bincode", "rmp-serde"],
       "lmfdb_id": "7.2.50.b"
     }
   }
   ```

### Orbits of Rust

**Definition**: An orbit is a set of crates with the same topological invariants

**Properties**:
- Same genus (complexity)
- Same conductor (branching)
- Same weight (computational intensity)
- Same level (fundamental period)

**Examples**:

**Orbit 1: Async Runtime** (genus=3, conductor=200)
- tokio
- async-std
- smol
- High complexity, heavy branching

**Orbit 2: Serialization** (genus=1, conductor=50)
- serde
- bincode
- postcard
- Low complexity, linear flow

**Orbit 3: Crypto** (genus=2, conductor=100)
- ring
- rustls
- sha2
- Medium complexity, moderate branching

## Phase 5: Applications

### 1. Minimal Rust Subsets

For each orbit, extract minimal Rust subset:
```bash
# Extract minimal Rust for async orbit
extract_orbit orbit_1 > minimal_async_rust.rs
# Contains only language features used by async crates
```

### 2. Targeted Optimization

Optimize rustc for specific orbits:
```bash
# Optimize for serialization orbit
rustc --optimize-for-orbit=orbit_2 input.rs
# Uses orbit-specific optimizations
```

### 3. Formal Verification

Generate Lean4 proofs per orbit:
```lean
theorem orbit_1_complexity :
  ∀ (crate : Orbit1),
    genus crate = 3 ∧
    conductor crate ≤ 200 := by
  -- Proof from reachability data
```

### 4. Ecosystem Analysis

```python
# Analyze entire crates.io
import pyarrow.parquet as pq

# Load all reachability data
all_traces = pq.read_table('crates_io_traces.parquet')

# Compute orbit distribution
orbits = cluster_by_homotopy(all_traces)

# Find outliers
outliers = orbits[orbits['genus'] > 10]
print(f"Complex crates: {outliers}")
```

## Implementation Roadmap

### Week 1-2: QEMU Backend (✅ Done)
- [x] QEMU plugin with parquet output
- [x] Self-compilation trace
- [x] Harmonic analysis tools

### Week 3-4: Rustc Backend Prototype
- [ ] Fork rustc codegen
- [ ] Add reachability tracking hooks
- [ ] Emit parquet alongside binary
- [ ] Test on simple programs

### Week 5-6: Self-Hosting
- [ ] Compile rustc with reachability backend
- [ ] Bootstrap reachable rustc
- [ ] Trace rustc compiling itself
- [ ] Analyze rustc's own complexity

### Week 7-8: Harmonic Decomposition
- [ ] Collect data from top 100 crates
- [ ] Compute harmonic signatures
- [ ] Identify orbits
- [ ] Map to LMFDB/OEIS

### Week 9-10: Ecosystem Analysis
- [ ] Process all of crates.io
- [ ] Generate orbit catalog
- [ ] Create minimal Rust subsets
- [ ] Publish findings

## Technical Challenges

### 1. Performance
- **Problem**: Reachability tracking adds overhead
- **Solution**: Make it optional (`--emit=reachability`)

### 2. Data Volume
- **Problem**: Parquet files can be large
- **Solution**: Compression, sampling, streaming

### 3. Rustc Integration
- **Problem**: Rustc internals are complex
- **Solution**: Start with MIR, work backwards to source

### 4. Orbit Stability
- **Problem**: Crate updates may change orbits
- **Solution**: Version orbits, track evolution

## Expected Outcomes

### Scientific
- **First** complete topological map of a programming language ecosystem
- **Proof** that code complexity is a mathematical invariant
- **Discovery** of fundamental "elements" of Rust

### Practical
- **Faster** compilation via orbit-specific optimizations
- **Smaller** binaries via minimal Rust subsets
- **Better** error messages via orbit-aware diagnostics

### Philosophical
- **Understanding** of what makes code complex
- **Classification** of programming patterns
- **Unification** of software and mathematics

## The Ultimate Goal

**Reachable Rust** = A compiler that understands its own complexity

Every program compiled reveals its mathematical structure.
Every crate finds its orbit.
Every orbit maps to known mathematics.

The entire Rust ecosystem becomes a mathematical object that can be:
- Analyzed formally
- Optimized globally
- Understood deeply

## Next Steps

1. **Test QEMU backend**: `./trace_self_compilation.sh`
2. **Analyze results**: Study the parquet output
3. **Design rustc hooks**: Identify integration points
4. **Prototype backend**: Minimal reachability tracking
5. **Bootstrap**: Compile rustc with itself

## References

- QEMU Plugin API: https://qemu.readthedocs.io/en/latest/devel/tcg-plugins.html
- Rustc Dev Guide: https://rustc-dev-guide.rust-lang.org/
- LMFDB: https://www.lmfdb.org/
- Our Theory: [THEORY.md](../THEORY.md)
