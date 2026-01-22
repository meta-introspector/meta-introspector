# Phase 2: Complete Galois Analysis with Full Bootstrap Chain

## Current Status

✅ **Phase 1 Complete**: Analyzed 71 languages build perf data
- Discovered: Build perf only captures nix overhead
- Actual programs run in `*_run.perf.data` files
- Need to capture full compilation + execution chain

## Phase 2 Goals

### 1. Add GNU Mes Bootstrap as Test #72

Create `const_71_test/mes/` that:
1. Bootstraps from Mes Scheme
2. Builds TinyCC
3. Builds GCC from TinyCC
4. Compiles `const71.c` with GCC
5. Runs and prints "71"

**Galois Baseline**: This becomes our reference for "real bootstrap complexity"

### 2. Capture Full Compilation Chains

For each language, record:

#### A. Compiler Build (if applicable)
```bash
# Example: Build rustc itself
nix build .#rustc --rebuild
perf record -o rustc_bootstrap.perf.data nix build .#rustc --rebuild
```

#### B. Program Compilation
```bash
# Compile const71 program
perf record -o rust_compile.perf.data rustc const71.rs
```

#### C. Program Execution
```bash
# Run the program
perf record -o rust_run.perf.data ./const71
```

### 3. Galois Comparison Matrix

| Language | Compiler Bootstrap | Program Compile | Program Run | Total |
|----------|-------------------|-----------------|-------------|-------|
| Mes      | GF(2^19) ⭐       | GF(2^?)         | GF(2^?)     | ?     |
| Rust     | GF(2^?)           | GF(2^?)         | GF(2^?)     | ?     |
| Agda     | GF(2^?)           | GF(2^16)        | GF(2^?)     | ?     |
| ...      | ...               | ...             | ...         | ...   |

### 4. Implementation Plan

#### Step 1: Add Mes to const_71_test
```nix
# const_71_test/mes/flake.nix
{
  description = "GNU Mes bootstrap to print 71";
  
  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.default = 
      # Full bootstrap chain
      # Mes -> TinyCC -> GCC -> const71.c
  };
}
```

#### Step 2: Create Rebuild Script
```bash
#!/usr/bin/env bash
# rebuild_with_perf.sh

LANG=$1
PERF_DIR="data/full_chain_perf"

# 1. Build compiler (if needed)
if has_compiler $LANG; then
  perf record -o $PERF_DIR/${LANG}_compiler_build.perf.data \
    nix build .#${LANG}-compiler --rebuild
fi

# 2. Compile program
perf record -o $PERF_DIR/${LANG}_program_compile.perf.data \
  nix build const_71_test/$LANG --rebuild

# 3. Run program
perf record -o $PERF_DIR/${LANG}_program_run.perf.data \
  nix run const_71_test/$LANG
```

#### Step 3: Analyze Full Chain
```rust
// full_chain_analyzer.rs
// Compare:
// - Mes bootstrap (baseline)
// - Each language's full chain
// - Prove which are "real" vs "trivial"
```

### 5. Expected Discoveries

**Hypothesis**:
- Mes bootstrap: GF(2^19) - highest complexity (real bootstrap)
- Compiled languages (Rust, C++): GF(2^15-17) - moderate (use pre-built compilers)
- Interpreted languages (Python, Ruby): GF(2^12-14) - low (just run interpreter)
- Trivial languages (writeText): GF(2^8-10) - minimal (just echo)

**Proof**:
- Galois number correlates with "real work"
- Mes is the only true bootstrap
- Others depend on pre-existing toolchains

### 6. Deliverables

1. `const_71_test/mes/` - Test #72
2. `data/full_chain_perf/` - Complete perf traces
3. `full_chain_analyzer.rs` - Analysis tool
4. `GALOIS_BOOTSTRAP_COMPARISON.md` - Final report

### 7. Commands

```bash
# Add Mes test
cd const_71_test
mkdir mes
# ... create flake.nix

# Rebuild all with full chain
for lang in const_71_test/*/; do
  ./rebuild_with_perf.sh $(basename $lang)
done

# Analyze
cargo run --release --bin full_chain_analyzer

# Generate report
cargo run --release --bin full_chain_analyzer > GALOIS_BOOTSTRAP_COMPARISON.md
```

## Success Criteria

✅ Mes bootstrap prints "71" via full chain (Mes -> TinyCC -> GCC)
✅ All 72 languages have full chain perf data
✅ Galois numbers prove Mes is unique in complexity
✅ Mathematical proof that Mes is the only "real" bootstrap

---

**Next Action**: Create `const_71_test/mes/flake.nix`
