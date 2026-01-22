# Perf + Rust + Nix Integration

Complete integration: Rust tools analyzing perf data from Nix builds.

## Architecture

```
Nix Build → perf.data → Rust Analyzer → Labels/Matrix → Training
```

## Rust Tools

### 1. perf-complexity
Location: `const_71_test/perf-complexity/`

Auto-label instruction data via orthogonal projection.

```rust
use perf_complexity::PerfComplexity;

let mut analyzer = PerfComplexity::new();
analyzer.load_basis_system(mes_perf_path)?;
analyzer.load_target_system(rust_perf_path)?;
analyzer.compute_orthogonal_labels();
```

### 2. topological-function-matrix
Location: `const_71_test/topological-function-matrix/`

Build topology → function matrix.

```rust
use topological_function_matrix::FunctionMatrix;

let mut matrix = FunctionMatrix::new();
// Add nodes from nix build graph
matrix.compute_matrix()?;
```

### 3. harmonic_analyzer
Location: `src/bin/harmonic_analyzer.rs`

Analyze Galois field coverage from perf data.

```bash
cargo run --release --bin harmonic_analyzer -- \
  /nix/store/xxx/perf/build.perf.data
```

### 4. mes-transformer-gpu
Location: `const_71_test/mes-transformer-gpu/`

Train transformer on nix build perf data.

```rust
// Load perf data from nix store
let perf_data = load_from_nix_store(derivation_path)?;

// Train on GPU
train_transformer(&perf_data, &device)?;
```

## Data Flow

```
Nix Build
  ↓
/nix/store/xxx/perf/build.perf.data
  ↓
Rust: linux-perf-data crate
  ↓
Parse perf.data → Extract IPs
  ↓
perf-complexity → Orthogonal labels
  ↓
topological-function-matrix → Positions
  ↓
mes-transformer-gpu → Train model
  ↓
/nix/store/xxx/model/checkpoint.bin
```

## Dependencies

### Rust Crates
```toml
[dependencies]
linux-perf-data = "0.8"  # Parse perf.data
burn = { git = "...", features = ["cuda"] }  # GPU training
burn-cuda = { git = "..." }
petgraph = "0.6"  # Topological sort
serde = "1.0"
```

### Nix Packages
```nix
nativeBuildInputs = [
  linuxPackages.perf
  cudaPackages.cuda_nvcc
  makeWrapper
];
```

## Build Integration

### Rust + Nix + Perf
```nix
pkgs.rustPlatform.buildRustPackage {
  pname = "analyzer";
  
  nativeBuildInputs = [ pkgs.linuxPackages.perf ];
  
  preBuild = ''
  '';
  
  postBuild = ''
    # Analyze own build perf data
    $out/bin/analyzer --perf $out/perf/build.perf.data
  '';
}
```

## Examples

### Analyze Mes Bootstrap
```bash
# Build mes with perf
nix build .#mes-level0

# Analyze with Rust
cargo run --release --bin perf-complexity -- \
  --perf-data $(nix-store -q result)/perf/mes-bootstrap.perf.data
```

### Train on 71 Languages
```bash
# Build all 71 languages with perf
for lang in const_71_test/*/; do
  nix build --impure "$lang"
done

# Train transformer
cd const_71_test/mes-transformer-gpu
NIXPKGS_ALLOW_UNFREE=1 nix build --impure
```

### Meta-Perf Analysis
```bash
# Generate meta-perf data
nix build .#meta-perf

# Analyze convergence
cargo run --release --bin analyze-convergence -- \
  $(nix-store -q result)/convergence/
```

## GPU Training

### CUDA Setup
```nix
buildInputs = [
  cudaPackages.cuda_cudart
  cudaPackages.libcublas
  linuxPackages.nvidia_x11
];

CUDA_PATH = "${pkgs.cudaPackages.cuda_cudart}";
```

### Run with CUDA 13
```bash
LD_LIBRARY_PATH=/nix/store/.../glibc-2.40-66/lib:/usr/local/cuda-13.0/lib64:/usr/lib/x86_64-linux-gnu \
  ./result/bin/mes-transformer-gpu
```

## Reproducibility

### Same Inputs = Same Outputs
```bash
# Build 1
nix build .#analyzer
HASH1=$(nix-store -q result)

# Build 2
nix build .#analyzer
HASH2=$(nix-store -q result)

# $HASH1 == $HASH2
# Same perf data, same analysis, same model
```

## References

- Perf: `docs/perf/README.md`
- Nix: `docs/nix/README.md`
- Nix+Perf: `docs/nix/perf/README.md`
- Rust Tools: `const_71_test/perf-complexity/`, `const_71_test/mes-transformer-gpu/`
