# Perf Documentation Index

Consolidated documentation for performance analysis and telemetry.

## Core Concepts

### Perf Recording
- **perf record**: Capture performance data during execution
- **perf script**: Extract instruction pointers and traces
- **perf report**: Analyze performance bottlenecks

### Meta-Perf
- Recording perf analyzing perf reveals functional vocabulary
- Self-referential convergence creates universal labeling model
- See: `const_71_test/meta-perf/`

## Tools

### 1. perf-recorder
Location: `perf-recorder/`

Capture perf data from any Nix build.

```bash
nix run ./perf-recorder#perf-build -- .#default
```

Output: `perf_build_YYYYMMDD_HHMMSS.data`

### 2. perf-wrapper
Location: `perf-wrapper/`

Wrap any Nix derivation to record perf during build.

```nix
perf-wrapper.lib.wrapWithPerf {
  inherit pkgs;
  package = pkgs.hello;
}
```

Output: `/nix/store/xxx-package-with-perf/perf/build.perf.data`

### 3. perf-macros
Location: `docs/perf-macros/`

Rust proc macros for inline perf recording.

```rust
#[perf_auto]
fn my_function() { }

#[perf_probe]
fn capture_to_parquet() { }
```

### 4. mes-perf-recorder
Location: `mes-perf-recorder/`

Record GNU Mes bootstrap chain with perf.

```bash
nix run ./mes-perf-recorder#record-mes
```

Output: `mes-bootstrap.perf.data`

### 5. perf-complexity
Location: `const_71_test/perf-complexity/`

Auto-label instruction data via orthogonal projection.

```bash
perf-complexity --perf-data build.perf.data --mes-store /nix/store/xxx-meta-perf
```

## Data Flow

```
Build → perf record → perf.data
                         ↓
                    perf script → IPs
                         ↓
                  perf-complexity → Labels
                         ↓
                  Topological Matrix → Positions
```

## Nix Integration

See: `docs/nix/perf/` for Nix-specific perf integration.

### Build with Perf
```bash
nix build .#package-with-perf
ls result/perf/build.perf.data
```

### Training Derivations
Each nix build = training batch:
- Perf data captured during build
- Stored in `/nix/store/xxx/perf/`
- Reproducible training data

See: `const_71_test/mes-transformer-gpu/flake.nix`

## Analysis

### 71 Flakes Perf Collection
Location: `data/71_flakes_perf/`

4GB of real perf data from 71 language builds.

```bash
ls data/71_flakes_perf/*.perf.data
```

### Harmonic Analysis
```bash
cargo run --release --bin harmonic_analyzer -- data/71_flakes_perf/rust_build.perf.data
```

Output: Galois field coverage (GF(2^n))

## References

- Main README: `README.md` (section: Galois Field Analysis)
- Perf Macros: `docs/perf-macros/README.md`
- Perf Recorder: `perf-recorder/README.md`
- Perf Wrapper: `perf-wrapper/README.md`
- Mes Perf: `mes-perf-recorder/README.md`
