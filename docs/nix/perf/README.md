# Nix + Perf Integration

How to capture perf data during Nix builds and use it for training.

## Architecture

```
Nix Build → perf record → /nix/store/xxx/perf/build.perf.data
                              ↓
                         Training Input
                              ↓
                         Model Checkpoint → /nix/store/xxx/model/
```

## Methods

### 1. perf-wrapper (Recommended)

Wrap any derivation to add perf recording:

```nix
{
  inputs.perf-wrapper.url = "path:./perf-wrapper";
  
  outputs = { self, nixpkgs, perf-wrapper }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in {
      packages.default = perf-wrapper.lib.wrapWithPerf {
        inherit pkgs;
        package = pkgs.myPackage;
      };
    };
}
```

Output structure:
```
/nix/store/xxx-myPackage-with-perf/
├── bin/myPackage
└── perf/
    ├── build.perf.data
    └── trace.txt
```

### 2. Manual Integration

Add perf recording to any derivation:

```nix
pkgs.stdenv.mkDerivation {
  name = "my-package";
  
  nativeBuildInputs = [ pkgs.linuxPackages.perf ];
  
  preBuild = ''
    mkdir -p $out/perf
    perf record -o $out/perf/build.perf.data -F 997 -g &
    PERF_PID=$!
  '';
  
  postBuild = ''
    kill -INT $PERF_PID 2>/dev/null || true
    wait $PERF_PID 2>/dev/null || true
    
    # Extract IPs
    perf script -i $out/perf/build.perf.data -F ip \
      > $out/perf/ips.txt
  '';
}
```

### 3. Training Derivation

Build + train in one derivation:

```nix
pkgs.rustPlatform.buildRustPackage {
  pname = "mes-transformer-train";
  
  preBuild = ''
    perf record -o $out/perf/build.perf.data ...
  '';
  
  postBuild = ''
    # Train on perf data from THIS build
    $out/bin/train-model \
      --perf-data $out/perf/build.perf.data \
      --output $out/model/checkpoint.bin
  '';
}
```

See: `const_71_test/mes-transformer-gpu/flake.nix`

## Data Flow

### Single Build
```
nix build .#package
  ↓
perf record during build
  ↓
/nix/store/abc-package/perf/build.perf.data
  ↓
Extract IPs
  ↓
Train model
  ↓
/nix/store/abc-package/model/checkpoint.bin
```

### Distributed Training
```
Node 1: nix build .#rust  → /nix/store/aaa-rust/model/
Node 2: nix build .#python → /nix/store/bbb-python/model/
Node 3: nix build .#haskell → /nix/store/ccc-haskell/model/
  ↓
Merge checkpoints
  ↓
Final model
```

## Meta-Perf

Recording perf analyzing perf:

```nix
pkgs.stdenv.mkDerivation {
  buildPhase = ''
    # Level 0: Record initial program
    perf record -o level0.perf.data program
    
    # Level 1: Record perf analyzing level 0
    perf record -o level1.perf.data \
      perf script -i level0.perf.data
    
    # Level 2: Record perf analyzing level 1
    perf record -o level2.perf.data \
      perf script -i level1.perf.data
  '';
}
```

See: `const_71_test/meta-perf/flake.nix`

## Topological Training

Build order = training order:

```
mes.perf.data → Layer 0 (GF(2^19))
  ↓
tcc.perf.data → Layer 1 (GF(2^20))
  ↓
gcc.perf.data → Layer 2 (GF(2^21))
  ↓
rust.perf.data → Layer 3 (GF(2^22))
```

Each layer labels the previous orthogonally.

See: `const_71_test/topological-function-matrix/`

## Reproducibility

### Same Derivation = Same Perf Data
```bash
# Build 1
nix build .#package
HASH1=$(nix-store -q result)

# Build 2 (same inputs)
nix build .#package
HASH2=$(nix-store -q result)

# $HASH1 == $HASH2
# Same perf data, same training data
```

### Snapshot = Nix Store State
```bash
# Snapshot 1
nix build .#mes-level0
LEVEL0=$(nix-store -q result)

# Snapshot 2
nix build .#mes-level1
LEVEL1=$(nix-store -q result)

# Both snapshots preserved in store
# Can train on either snapshot
```

## Tools

### perf-complexity
Auto-label using orthogonal projection:

```bash
perf-complexity \
  --basis-system /nix/store/xxx-mes/perf/build.perf.data \
  --target-system /nix/store/yyy-rust/perf/build.perf.data \
  --output labels.json
```

### harmonic_analyzer
Analyze Galois field coverage:

```bash
harmonic_analyzer /nix/store/xxx/perf/build.perf.data
```

## Examples

### 71 Languages
Each language build captures perf:

```bash
cd const_71_test/rust
nix build --impure
ls result/perf/  # (if wrapped with perf)
```

### MES Transformer GPU
Train on nix build perf data:

```bash
cd const_71_test/mes-transformer-gpu
NIXPKGS_ALLOW_UNFREE=1 nix build --impure
ls result/model/checkpoint.bin
```

## References

- Perf Wrapper: `perf-wrapper/README.md`
- Perf Recorder: `perf-recorder/README.md`
- Meta-Perf: `const_71_test/meta-perf/flake.nix`
- Training: `const_71_test/mes-transformer-gpu/flake.nix`
