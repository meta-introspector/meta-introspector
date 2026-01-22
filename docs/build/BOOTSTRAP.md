# Bootstrap System

## Overview

`bootstrap.sh` is the single command to build and iterate the entire ZOS system. It uses Nix for reproducible builds and stores all performance data in the Nix store.

## Usage

```bash
./bootstrap.sh
```

Run repeatedly to iterate. Each run:
2. Generates self-metadata
3. Commits changes
4. Reports status

## What It Does

### Phase 1: Nix Build
- Builds all packages via `nix build`
- Records perf data during build
- Stores results in `/nix/store/hash-package/`
- Perf data stored at `/nix/store/hash-package/perf/`

### Phase 2: Self-Metadata
- Runs `collect-repo-metadata.sh`
- Generates `zos/zos.toml` with repo stats
- Applies ZOS to itself

### Phase 3: Commit
- Stages all changes
- Excludes perf data (stored in Nix store only)
- Commits with timestamp

### Phase 4: Status
- Reports commit count
- Reports tool count
- Ready for next iteration

## Perf Data Storage

**NOT in git** - stored in Nix store as build artifacts:

```
/nix/store/abc123-package/
├── pkg/              # Build outputs
└── perf/             # Performance data
    ├── build.perf.data
    └── report.txt
```

### Query Perf Data

```bash
# Find all perf data
ls /nix/store/*-wasm/perf/

# View specific build
perf report -i /nix/store/abc123-wasm/perf/build.perf.data

# Compare across builds
for p in /nix/store/*-wasm/perf/report.txt; do
  echo "=== $p ==="
  head -20 "$p"
done
```

## Benefits

- **Reproducible**: Same inputs → same outputs (including perf data)
- **Immutable**: Can't modify after build
- **Content-addressed**: Hash includes all artifacts
- **Time-series**: Each build is a snapshot
- **No git bloat**: Perf data not in version control

## The Bootstrap Chain

```
357 bytes (seed)
  ↓
MES (Scheme interpreter)
  ↓
TCC (C compiler)
  ↓
GCC (optimizing compiler)
  ↓
LLVM (modern optimizer)
  ↓
Rust (systems language)
  ↓
ZOS (self-analysis)
```

Each stage recorded with perf, stored in Nix store.

## Modular Form Analysis

After multiple iterations, analyze the bootstrap as a modular form:

```bash
# Build the analyzer
rustc modular_form_curve.rs -O

# Run on stored perf data
./modular_form_curve
```

Shows:
- Instruction spectrum at each stage
- Similarity between stages (fractal self-similarity)
- Resonances at ZOS primes
- Entropy decrease backwards in time

## Integration with Complete Bootstrap

For full chain from MES:

```bash
# Use perf-recorder or bootstrap.sh instead
nix run ./perf-recorder#perf-build -- .#default
```

Records entire chain: MES → Nix → LLVM → Rust → ZOS

All perf data stored in `zos-results/complete-bootstrap/` (local) and `/nix/store/` (immutable).

## Continuous Iteration

```bash
# Run until convergence
while true; do
  ./bootstrap.sh
  sleep 60
done
```

Each iteration:
- Builds from latest git
- Records performance
- Stores in Nix store
- Commits metadata
- System evolves

## See Also

- `zos/BOOTSTRAP_MODULAR_FORM.md` - Mathematical theory
- `zos/MES_BOOTSTRAP.md` - Full bootstrap chain
- `zos/NEURAL_PHASE_TRANSITION.md` - ML on phase transitions
- `modular_form_curve.rs` - Analysis tool
