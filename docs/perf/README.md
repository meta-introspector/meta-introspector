# Perf Recording Guide

## Canonical Patterns

We have **3 canonical patterns** for perf recording. Use these, don't duplicate!

### 1. Nix Derivation (Immutable Storage)

**Use when:** Building reproducible perf data in /nix/store

**Pattern:**
```nix
buildPhase = ''
  mkdir -p $out/perf
  perf record -o $out/perf/build.perf.data -F 99 -g nix build
'';
```

**Examples:**
- `mes-bootstrap-proof/flake.nix`
- `nix/script-complexity.nix`
- `instrumentation-overlay/flake.nix`

### 2. Perf-Recorder (Interactive)

**Use when:** Quick interactive recording

**Usage:**
```bash
nix run ./perf-recorder#perf-build -- .#target
```

**Features:**
- Timestamped output files
- Call graph recording
- Stats collection

**See:** `perf-recorder/README.md`

### 3. Script Pattern (One-off Analysis)

**Use when:** Custom analysis workflow

**Pattern:**
```bash
OUTPUT="data/analysis"
mkdir -p "$OUTPUT"
perf record -e cycles,instructions -o "$OUTPUT/build.data" nix build
```

**Example:** `scripts/build/build_and_analyze_const71.sh`

## Common Use Cases

### Record Bootstrap
```bash
./scripts/build/bootstrap.sh
# Perf data stored in /nix/store
```

### Record 71 Languages
```bash
./execute_workflows.sh
# Calls: nix build ./nix/flakes/const_71_test#perf-all
```

### Interactive Recording
```bash
nix run ./perf-recorder#perf-build -- .#default
```

## Analysis

```bash
# Interactive report
perf report -i perf.data

# Text dump
perf script -i perf.data > trace.txt

# Flamegraph
perf script -i perf.data | stackcollapse-perf.pl | flamegraph.pl > flame.svg
```

## Policy

**MUST:**
- Store perf data in /nix/store (via derivations)
- Use one of the 3 canonical patterns
- Reference this guide, don't duplicate examples

**MUST NOT:**
- Create loose files in data/ or zos-results/
- Use ../../ relative paths
- Duplicate perf record commands

## See Also

- `docs/nix/perf/CONSOLIDATION_NIX_STORE_PERF.md` - Policy details
- `perf-recorder/README.md` - Interactive tool
- `docs/perf/CLEANUP_PLAN.md` - Consolidation plan
