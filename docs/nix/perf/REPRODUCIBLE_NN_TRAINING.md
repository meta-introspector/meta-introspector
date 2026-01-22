# Reproducible Neural Networks from Nix Build Perf Samples

Complete pipeline: Nix builds → perf samples → training data → NN models as derivations.

## Architecture

```
Nix Build (Level N)
Perf Samples → /nix/store/xxx-build/perf/build.perf.data
  ↓ extract IPs
Training Data → /nix/store/xxx-build/training/ips.txt
  ↓ train (GPU)
NN Model → /nix/store/xxx-model/checkpoint.bin
  ↓ (reproducible)
Same Build = Same Model
```

## Phase 1: Perf Sampling During Build

### Method 1: perf-wrapper (Automatic)


```nix
{
  inputs.perf-wrapper.url = "path:./perf-wrapper";
  
  outputs = { self, nixpkgs, perf-wrapper }:
    let pkgs = nixpkgs.legacyPackages.x86_64-linux;
    in {
      packages.default = perf-wrapper.lib.wrapWithPerf {
        inherit pkgs;
        package = pkgs.myPackage;
      };
    };
}
```

Output:
```
/nix/store/abc-myPackage-with-perf/
├── bin/myPackage
└── perf/
    ├── build.perf.data      # Raw perf samples
    └── trace.txt            # perf script output
```

### Method 2: Manual Integration

```nix
pkgs.stdenv.mkDerivation {
  name = "my-package";
  
  nativeBuildInputs = [ pkgs.linuxPackages.perf ];
  
  preBuild = ''
    mkdir -p $out/perf
      -F 997 -g --call-graph dwarf &
    PERF_PID=$!
  '';
  
  postBuild = ''
    kill -INT $PERF_PID 2>/dev/null || true
    wait $PERF_PID 2>/dev/null || true
    
    # Extract instruction pointers
    perf script -i $out/perf/build.perf.data -F ip \
      | grep -v '^#' | sort -u > $out/perf/ips.txt
    
    echo "Captured $(wc -l < $out/perf/ips.txt) unique IPs"
  '';
}
```

## Phase 2: Process Perf Samples into Training Data

### Extract Features from Perf Data

```nix
pkgs.rustPlatform.buildRustPackage {
  pname = "perf-to-training";
  
  buildPhase = ''
    # Input: perf.data from previous build
    PERF_DATA="${previousBuild}/perf/build.perf.data"
    
    # Extract features using perf-complexity
    perf-complexity \
      --perf-data "$PERF_DATA" \
      --output $out/training/features.json
    
    # Extract IPs with frequencies
    perf script -i "$PERF_DATA" -F ip \
      | sort | uniq -c | sort -rn \
      > $out/training/ip_frequencies.txt
    
    # Extract call graphs
    perf script -i "$PERF_DATA" -F ip,sym \
      > $out/training/call_graph.txt
  '';
  
  installPhase = ''
    mkdir -p $out/training
    
    # Create training metadata
    cat > $out/training/meta.json << EOF
    {
      "source_build": "${previousBuild}",
      "perf_data": "$PERF_DATA",
      "unique_ips": $(wc -l < $out/training/ip_frequencies.txt),
      "timestamp": "$(date -Iseconds)"
    }
    EOF
  '';
}
```

## Phase 3: Train NN Model as Nix Derivation

### Training Derivation

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    training-data.url = "path:./training-data-derivation";
  };
  
  outputs = { self, nixpkgs, training-data }:
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
        config.allowUnfree = true;  # For CUDA
      };
    in {
      packages.default = pkgs.rustPlatform.buildRustPackage {
        pname = "nn-model-trained";
        version = "0.1.0";
        src = ./.;
        
        # Training dependencies
        nativeBuildInputs = with pkgs; [
          cudaPackages.cuda_nvcc
          makeWrapper
        ];
        
        buildInputs = with pkgs; [
          cudaPackages.cuda_cudart
          cudaPackages.libcublas
          linuxPackages.nvidia_x11
        ];
        
        # Training happens during build
        buildPhase = ''
          # Load training data from previous derivation
          TRAINING_DATA="${training-data}/training"
          
          echo "🚀 Training NN model on perf samples..."
          
          # Train on GPU
          cargo run --release --bin train-model -- \
            --training-data "$TRAINING_DATA/features.json" \
            --ip-frequencies "$TRAINING_DATA/ip_frequencies.txt" \
            --output $out/model/checkpoint.bin \
            --epochs 100 \
            --batch-size 71 \
            --learning-rate 0.001
          
          echo "✅ Training complete"
        '';
        
        installPhase = ''
          mkdir -p $out/model $out/bin
          
          # Model checkpoint already in $out/model/
          
          # Install inference binary
          install -Dm755 target/*/release/inference $out/bin/
          
          # Wrap with CUDA libraries
          wrapProgram $out/bin/inference \
            --set MODEL_PATH "$out/model/checkpoint.bin" \
            --prefix LD_LIBRARY_PATH : "${pkgs.lib.makeLibraryPath [ 
              pkgs.linuxPackages.nvidia_x11 
            ]}"
          
          # Create model metadata
          cat > $out/model/meta.json << EOF
          {
            "training_data": "${training-data}",
            "source_build": "$(cat $TRAINING_DATA/meta.json | jq -r .source_build)",
            "epochs": 100,
            "batch_size": 71,
            "model_hash": "$(sha256sum $out/model/checkpoint.bin | cut -d' ' -f1)",
            "derivation": "$out",
            "timestamp": "$(date -Iseconds)"
          }
          EOF
        '';
      };
    };
}
```

## Phase 4: Reproducibility

### Same Build = Same Model

```bash
# Build 1
nix build .#my-package-with-perf
BUILD1=$(nix-store -q result)

# Build 2 (same inputs)
nix build .#my-package-with-perf
BUILD2=$(nix-store -q result)

# Verify: $BUILD1 == $BUILD2
# Same perf data, same training data, same model

# Train model from build 1
nix build .#trained-model --override-input training-data $BUILD1
MODEL1=$(nix-store -q result)

# Train model from build 2
nix build .#trained-model --override-input training-data $BUILD2
MODEL2=$(nix-store -q result)

# Verify: $MODEL1 == $MODEL2
# Reproducible NN training!
```

## Topological Training Pipeline

### Level 0: GNU Mes (Base)

```nix
packages.mes-level0 = pkgs.stdenv.mkDerivation {
  name = "mes-bootstrap-perf";
  src = pkgs.fetchurl {
    url = "mirror://gnu/mes/mes-0.26.tar.gz";
    sha256 = "...";
  };
  
  buildPhase = ''
      -F 997 -g -- make
  '';
};
```

### Level 1: Train on Mes

```nix
packages.model-level0 = trainModel {
  training-data = packages.mes-level0;
  name = "mes-model";
};
```

### Level 2: TCC (depends on Mes)

```nix
packages.tcc-level1 = buildWithPerf {
  package = pkgs.tcc;
  basis-model = packages.model-level0;  # Use Mes model for labeling
};
```

### Level 3: Train on TCC

```nix
packages.model-level1 = trainModel {
  training-data = packages.tcc-level1;
  basis-model = packages.model-level0;  # Orthogonal labeling
};
```

### Complete Hierarchy

```
mes.perf.data → model-level0.bin
  ↓ (labels)
tcc.perf.data → model-level1.bin
  ↓ (labels)
gcc.perf.data → model-level2.bin
  ↓ (labels)
rust.perf.data → model-level3.bin
```

## Distributed Training

### Multiple Nodes

```bash
# Node 1: Train on Rust
nix build .#rust-model

# Node 2: Train on Python
nix build .#python-model

# Node 3: Train on Haskell
nix build .#haskell-model

# Merge models
nix build .#merged-model \
  --override-input rust-model /nix/store/aaa-rust-model \
  --override-input python-model /nix/store/bbb-python-model \
  --override-input haskell-model /nix/store/ccc-haskell-model
```

### Merge Derivation

```nix
packages.merged-model = pkgs.stdenv.mkDerivation {
  name = "merged-nn-model";
  
  buildInputs = [ rust-model python-model haskell-model ];
  
  buildPhase = ''
    # Merge model checkpoints
    merge-models \
      --input ${rust-model}/model/checkpoint.bin \
      --input ${python-model}/model/checkpoint.bin \
      --input ${haskell-model}/model/checkpoint.bin \
      --output $out/model/merged.bin
  '';
};
```

## Meta-Perf: Self-Referential Training

### Record perf analyzing perf

```nix
packages.meta-perf-model = pkgs.stdenv.mkDerivation {
  buildPhase = ''
    # Level 0: Record program
    
    # Level 1: Record perf analyzing level 0
      perf script -i level0.perf.data
    
    # Level 2: Record perf analyzing level 1
      perf script -i level1.perf.data
    
    # Train on convergence
    train-model \
      --level0 level0.perf.data \
      --level1 level1.perf.data \
      --level2 level2.perf.data \
      --output $out/model/meta-perf.bin
  '';
};
```

## Verification

### Test Reproducibility

```bash
# Build twice
nix build .#trained-model
HASH1=$(nix-store -q --hash result)

nix build .#trained-model
HASH2=$(nix-store -q --hash result)

# Verify
if [ "$HASH1" = "$HASH2" ]; then
  echo "✅ Reproducible NN training!"
else
  echo "❌ Non-deterministic training"
fi
```

## Current Implementation

See:
- `const_71_test/mes-transformer-gpu/flake.nix` - Training derivation
- `const_71_test/meta-perf/flake.nix` - Meta-perf convergence
- `const_71_test/perf-complexity/` - Feature extraction
- `const_71_test/topological-function-matrix/` - Hierarchical training

## References

- Nix+Perf: `docs/nix/perf/README.md`
- Perf Tools: `docs/perf/README.md`
- Complete Stack: `docs/perf/rust/nix/README.md`
