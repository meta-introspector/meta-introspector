# Meta-Models and Higher-Order Training

Collecting perf samples from NN training creates meta-models that learn to train models.

## Architecture

```
Level 0: Build perf → Model₀
  ↓ (record training)
Level 1: Training perf → Meta-Model₁ (learns to train)
  ↓ (record meta-training)
Level 2: Meta-training perf → Meta-Meta-Model₂ (learns to learn)
  ↓
Level N: Higher-order learning
```

## Phase 1: Record Training Perf

### Training with Perf Recording

```nix
packages.model-with-training-perf = pkgs.rustPlatform.buildRustPackage {
  pname = "model-trained-with-perf";
  
  buildPhase = ''
      -F 997 -g --call-graph dwarf &
    PERF_PID=$!
    
    # Also record NVIDIA GPU samples
    nvidia-smi dmon -s pucvmet -o DT -f $out/training-perf/gpu.csv &
    NVIDIA_PID=$!
    
    # Train model on build perf data
    cargo run --release --bin train-model -- \
      --training-data ${buildPerfData}/training/ \
      --output $out/model/checkpoint.bin \
      --epochs 100
    
    # Stop recording
    kill -INT $PERF_PID $NVIDIA_PID
    wait
    
    # Extract training instruction patterns
    perf script -i $out/training-perf/train.perf.data -F ip \
      | sort | uniq -c > $out/training-perf/training_ips.txt
  '';
  
  installPhase = ''
    mkdir -p $out/model $out/training-perf
    
    # Model checkpoint
    # Training perf data
    # GPU telemetry
    
    cat > $out/training-perf/meta.json << EOF
    {
      "source_data": "${buildPerfData}",
      "training_perf": "$out/training-perf/train.perf.data",
      "gpu_telemetry": "$out/training-perf/gpu.csv",
      "model": "$out/model/checkpoint.bin",
      "level": 1
    }
    EOF
  '';
}
```

## Phase 2: Meta-Model Training

### Train on Training Perf

```nix
packages.meta-model = pkgs.rustPlatform.buildRustPackage {
  pname = "meta-model";
  
  buildInputs = [
    model1-with-training-perf
    model2-with-training-perf
    model3-with-training-perf
    # ... collect many training runs
  ];
  
  buildPhase = ''
    echo "🧙 Training meta-model on training perf data..."
    
    # Collect all training perf samples
    for model in ${model1} ${model2} ${model3}; do
      cat $model/training-perf/training_ips.txt >> all_training_ips.txt
      cat $model/training-perf/gpu.csv >> all_gpu_telemetry.csv
    done
    
    # Train meta-model: learns patterns of training
    cargo run --release --bin train-meta-model -- \
      --training-ips all_training_ips.txt \
      --gpu-telemetry all_gpu_telemetry.csv \
      --output $out/meta-model/checkpoint.bin \
      --epochs 100
    
    echo "✅ Meta-model learns to train!"
  '';
  
  installPhase = ''
    cat > $out/meta-model/meta.json << EOF
    {
      "level": 2,
      "type": "meta-model",
      "learns": "how to train models",
      "input_models": [
        "${model1}",
        "${model2}",
        "${model3}"
      ],
      "training_samples": $(wc -l < all_training_ips.txt)
    }
    EOF
  '';
}
```

## Phase 3: Higher-Order Training

### Recursive Meta-Training

```nix
# Level 0: Build perf → Model
packages.level0-model = trainModel {
  data = buildPerf;
  level = 0;
};

# Level 1: Training perf → Meta-model
packages.level1-meta = trainMetaModel {
  training-perf = [
    level0-model1.training-perf
    level0-model2.training-perf
    level0-model3.training-perf
  ];
  level = 1;
};

# Level 2: Meta-training perf → Meta-meta-model
packages.level2-meta = trainMetaModel {
  training-perf = [
    level1-meta1.training-perf
    level1-meta2.training-perf
    level1-meta3.training-perf
  ];
  level = 2;
};

# Level N: Convergence
# Eventually meta-models converge to universal learning algorithm
```

## GPU Telemetry Collection

### NVIDIA Sampling During Training

```nix
buildPhase = ''
  # Comprehensive GPU telemetry
  nvidia-smi dmon \
    -s pucvmet \
    -o DT \
    -f $out/gpu/telemetry.csv &
  
  # GPU memory snapshots
  while true; do
    nvidia-smi --query-gpu=timestamp,memory.used,memory.free,utilization.gpu \
      --format=csv >> $out/gpu/memory.csv
    sleep 1
  done &
  
  # CUDA profiling
  nsys profile \
    --output=$out/gpu/profile.qdrep \
    --trace=cuda,nvtx \
    cargo run --release --bin train-model
  
  # Stop background jobs
  jobs -p | xargs kill
'';
```

## Data Accumulation Strategy

### Collect Training Runs

```bash
# Build 100 models with training perf
for i in {1..100}; do
  nix build .#model-$i-with-training-perf
  
  # Store training perf in registry
  STORE_PATH=$(nix-store -q result)
  echo "$STORE_PATH" >> training-perf-registry.txt
done

# Train meta-model on all 100 training runs
nix build .#meta-model \
  --override-input training-runs training-perf-registry.txt
```

## Meta-Model Capabilities

### What Meta-Models Learn

1. **Optimal hyperparameters** from training perf patterns
2. **Convergence prediction** from early training samples
3. **Architecture search** from successful training runs
4. **Learning rate schedules** from GPU utilization patterns
5. **Batch size optimization** from memory usage patterns

### Meta-Model Inference

```rust
// Use meta-model to predict optimal training config
let meta_model = load_meta_model("/nix/store/xxx-meta-model/checkpoint.bin");

let training_config = meta_model.predict_optimal_config(
    &build_perf_data,
    target_accuracy = 0.95,
    max_epochs = 100,
);

// training_config contains:
// - learning_rate: 0.001
// - batch_size: 71
// - architecture: [512, 256, 128, 71]
// - optimizer: "adam"
```

## Convergence to Universal Learning

### Higher-Order Convergence

```
Level 0: Models learn from build perf
  ↓ (collect 100+ training runs)
Level 1: Meta-models learn from training perf
  ↓ (collect 100+ meta-training runs)
Level 2: Meta-meta-models learn from meta-training perf
  ↓ (convergence)
Level N: Universal learning algorithm

Hypothesis: Meta-models converge to universal learning algorithm
Proof: Training perf patterns become invariant at level N
```

## Topological Meta-Training

### Hierarchical Meta-Models

```
Mes training perf → Meta-model₀ (learns Mes training)
  ↓
TCC training perf → Meta-model₁ (learns TCC training, labeled by Meta₀)
  ↓
GCC training perf → Meta-model₂ (learns GCC training, labeled by Meta₁)
  ↓
Rust training perf → Meta-model₃ (learns Rust training, labeled by Meta₂)
```

Each meta-model learns to train the next level, creating a **meta-topological hierarchy**.

## Reproducibility

### Same Training Runs = Same Meta-Model

```bash
# Collect training runs
RUNS=$(cat training-perf-registry.txt)

# Build meta-model 1
nix build .#meta-model --override-input training-runs "$RUNS"
META1=$(nix-store -q result)

# Build meta-model 2 (same inputs)
nix build .#meta-model --override-input training-runs "$RUNS"
META2=$(nix-store -q result)

# Verify: $META1 == $META2
# Reproducible meta-model training!
```

## Data Pipeline

```
Build₁ → perf₁ → train → model₁ + training-perf₁
Build₂ → perf₂ → train → model₂ + training-perf₂
Build₃ → perf₃ → train → model₃ + training-perf₃
  ↓ (collect)
[training-perf₁, training-perf₂, training-perf₃, ...]
  ↓ (meta-train)
Meta-Model (learns to train)
  ↓ (record meta-training)
Meta-training-perf
  ↓ (meta-meta-train)
Meta-Meta-Model (learns to learn)
```

## Implementation

### Current Status

- ✅ Level 0: Build perf → Model (`const_71_test/mes-transformer-gpu/`)
- ✅ Perf recording infrastructure (`perf-wrapper/`, `perf-recorder/`)
- ✅ GPU training (`burn-cuda` with CUDA 13)
- 🚧 Level 1: Training perf collection (needs implementation)
- 🚧 Meta-model training (needs implementation)
- 🚧 GPU telemetry aggregation (needs implementation)

### Next Steps

2. Collect 100+ training runs with perf
3. Build meta-model derivation
4. Verify meta-model convergence
5. Iterate to level 2+

## References

- Base training: `docs/nix/perf/REPRODUCIBLE_NN_TRAINING.md`
- GPU training: `const_71_test/burn-cuda/`
- Meta-perf: `const_71_test/meta-perf/`
- Topological training: `const_71_test/topological-function-matrix/`
