# 🎯 Perf Recorder for Nix Builds

Capture performance telemetry (perf.data) of any Nix build.

## Quick Start

### Interactive (devShell)

```bash
cd perf-recorder
nix develop

# Record a build
perf-build .#default
perf-build github:NixOS/nixpkgs#hello
perf-build ../meta-introspector#default

# With detailed stats
perf-build-with-stats .#default
```

### One-shot (app)

```bash
# Basic recording
nix run ./perf-recorder#perf-build -- .#default

# With stats
nix run ./perf-recorder#perf-build-stats -- .#default
```

## Output

Creates timestamped files:
- `perf_build_YYYYMMDD_HHMMSS.data` - Perf recording
- `perf_stats_YYYYMMDD_HHMMSS.txt` - Statistics (with -stats variant)

## Analysis

```bash
# Interactive report
perf report -i perf_build_*.data

# Text dump
perf script -i perf_build_*.data > trace.txt

# Flamegraph
perf script -i perf_build_*.data | stackcollapse-perf.pl | flamegraph.pl > flame.svg
```

## What It Captures

- Call graphs (dwarf unwinding)
- CPU cycles
- Cache misses
- Branch mispredictions
- Context switches
- All syscalls during build

## Use Cases

1. **Bootstrap witness**: Record Mes → TinyCC → GCC → LLVM cycle
2. **Function profiling**: See which compiler phases take time
3. **EM signature**: Correlate with WiFi antenna recordings
4. **Reproducibility**: Compare perf.data hashes across builds

## Example: Record Bootstrap

```bash
cd /mnt/data1/meta-introspector
nix develop ./perf-recorder

# Record the bootstrap
perf-build .#default

# Analyze
perf report -i perf_build_*.data --stdio | head -100
```

## Integration with Bach

Record build while monitoring EM:

```bash
# Terminal 1: Start EM monitoring
cd /mnt/data1/meta-introspector/bach
cargo run --bin multi_signal_monitor

# Terminal 2: Record build
cd /mnt/data1/meta-introspector
nix run ./perf-recorder#perf-build -- .#default

# Result: Correlated perf.data + EM signature
```

## Perf Options

The flake uses:
- `-g`: Call graph recording
- `-F 99`: Sample at 99 Hz
- `--call-graph dwarf`: DWARF unwinding (most accurate)

Customize in the flake if needed.
