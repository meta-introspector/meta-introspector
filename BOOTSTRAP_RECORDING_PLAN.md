# Complete Toolchain Bootstrap Recording Plan

## Goal
Record the **entire rebuild** of each language toolchain from scratch, capturing every compilation step.

## What We Need to Record

### Rust Bootstrap (20+ minutes)
```
Stage 0: Download bootstrap compiler
Stage 1: Build rustc with bootstrap
Stage 2: Build rustc with stage1
Stage 3: Build stdlib, cargo, clippy, rustfmt
```
**Expected**: 10,000+ rustc invocations, millions of samples

### GHC Bootstrap (30+ minutes)
```
Stage 0: Boot GHC
Stage 1: Build GHC with boot
Stage 2: Build libraries
```
**Expected**: 5,000+ ghc invocations

### OCaml Bootstrap (10+ minutes)
```
Build ocamlc (bytecode compiler)
Build ocamlopt (native compiler)
Build stdlib
```

### Agda Bootstrap (15+ minutes)
```
Build Agda with GHC
Compile Agda stdlib
Type-check everything
```

## Recording Strategy

### Option 1: Nix with --no-substitute
```nix
{
  rustc-bootstrap = pkgs.rustc.overrideAttrs (old: {
    nativeBuildInputs = old.nativeBuildInputs ++ [ pkgs.perf ];
    
    buildPhase = ''
      perf record -o $out/rustc_bootstrap.perf.data -F 99 -g \
        ${old.buildPhase}
    '';
  });
}
```

Build with: `nix build --no-substitute --rebuild`

### Option 2: Wrapper Script
```bash
# Wrap all compiler invocations
export CC="perf record -o cc_$$.perf.data gcc"
export RUSTC="perf record -o rustc_$$.perf.data rustc"
```

### Option 3: System-wide Recording
```bash
# Record everything during build
perf record -a -F 99 -g -o full_bootstrap.perf.data &
PERF_PID=$!

nix build --no-substitute rustc

kill $PERF_PID
```

## Data Size Estimates

### Per Language Bootstrap
- **Rust**: ~500MB perf data (20 min @ 99Hz)
- **GHC**: ~400MB perf data (30 min @ 99Hz)
- **OCaml**: ~150MB perf data (10 min @ 99Hz)
- **Agda**: ~200MB perf data (15 min @ 99Hz)

### Total for 7 Languages
- **~2GB perf data**
- **~10GB parquet** (after conversion)

## Implementation Plan

### Phase 1: Single Language (Rust)
1. Create flake that forces full rebuild
2. Record with perf wrapper
3. Convert to parquet
4. Analyze instruction spectrum
5. Label all compilation phases

### Phase 2: All Compiled Languages
- Rust, GHC, OCaml, Coq, Agda, Lean, Idris

### Phase 3: Interpreters
- Python, Ruby, Lua (build the interpreter itself)

## Labeling Strategy

From perf data, extract:
1. **Phase labels**: "stage1", "stage2", "stdlib"
2. **Component labels**: "rustc", "cargo", "clippy"
3. **Function labels**: "codegen", "typeck", "borrow_check"
4. **Instruction patterns**: Which CPU instructions used where

## Storage

All in `/nix/store`:
```
/nix/store/xxx-rustc-bootstrap-perf/
  ├── rustc_bootstrap.perf.data (500MB)
  ├── rustc_bootstrap.parquet (2GB)
  ├── phase_labels.json
  └── instruction_spectrum.json
```

## Query Examples

```sql
-- Which phase uses most instructions?
SELECT phase, COUNT(*) FROM rustc_bootstrap 
GROUP BY phase ORDER BY COUNT(*) DESC;

-- Which functions are hottest?
SELECT symbol, COUNT(*) FROM rustc_bootstrap
WHERE phase = 'codegen' 
GROUP BY symbol ORDER BY COUNT(*) DESC LIMIT 100;

-- Instruction diversity by phase
SELECT phase, COUNT(DISTINCT ip) as unique_ips
FROM rustc_bootstrap GROUP BY phase;
```

## Timeline

- **Phase 1 (Rust)**: 2 hours (build + record + analyze)
- **Phase 2 (7 langs)**: 1 day (parallel builds)
- **Phase 3 (Analysis)**: 2 days (label + query + visualize)

## Next Step

Start with Rust bootstrap recording using `--no-substitute` flag.
