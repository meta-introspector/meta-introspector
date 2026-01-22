# CRQ-004: llama.cpp-clean - Instrumented Build & Trace Collection

**Status**: Planning  
**Priority**: High  
**Branch**: feature/CRQ-004-llama-cpp-clean  
**Location**: /mnt/data1/2023/11/09/llama.cpp

## Objective

Clean up and systematize llama.cpp instrumentation:
1. Nix-based instrumented builds
2. Perf trace collection for all models
3. Organize existing dirty code
4. Create reproducible trace pipeline
5. Analyze traces for optimization

## Current State

**Location**: `/mnt/data1/2023/11/09/llama.cpp`  
**Status**: Dirty code, ton of work sunk in  
**Branches**: Multiple feature branches (zos, save_temps, etc.)  
**Problem**: Needs systematic cleanup and organization

## Why Clean This Up?

1. **Trace Collection** - Systematic perf data from all models
2. **Nix Builds** - Reproducible instrumented builds
3. **Analysis** - Understand model performance patterns
4. **Optimization** - Find bottlenecks across models
5. **Integration** - Feed traces to lifting pipeline

## Architecture

```
llama.cpp-clean/
├── flake.nix                    # Main Nix build
├── nix/
│   ├── instrumented.nix        # Perf-instrumented build
│   ├── models.nix              # Model definitions
│   └── trace-collector.nix     # Trace collection automation
├── scripts/
│   ├── build-instrumented.sh   # Build with perf
│   ├── run-all-models.sh       # Run all models with tracing
│   ├── collect-traces.sh       # Collect and organize traces
│   └── analyze-traces.sh       # Analyze collected data
├── traces/
│   ├── llama-7b/
│   ├── llama-13b/
│   ├── mistral-7b/
│   └── ...
└── analysis/
    ├── syscalls.json           # Syscall analysis
    ├── hotspots.json           # Performance hotspots
    └── galois-coverage.json    # GF coverage per model
```

## Migration Plan

### Phase 1: Audit & Organize (Week 1)

**Audit existing work:**
```bash
cd /mnt/data1/2023/11/09/llama.cpp

# Check all branches
git branch -a

# Find instrumentation code
find . -name "*trace*" -o -name "*perf*" -o -name "*instrument*"

# Identify dirty code
find . -name "*.py~" -o -name "#*#"
```

**Organize:**
- Document what works
- Identify what's broken
- List all models tested
- Catalog existing traces

### Phase 2: Nix Build System (Week 2)

**Create clean Nix builds:**

```nix
# nix/instrumented.nix
{ pkgs, ... }:

pkgs.stdenv.mkDerivation {
  name = "llama-cpp-instrumented";
  
  src = ./.;
  
  nativeBuildInputs = with pkgs; [
    cmake
    pkg-config
    linuxPackages.perf
  ];
  
  cmakeFlags = [
    "-DCMAKE_BUILD_TYPE=RelWithDebInfo"
    "-DLLAMA_PERF=ON"
  ];
  
  # Wrap binaries with perf
  postInstall = ''
    for bin in $out/bin/*; do
      mv $bin $bin.real
      cat > $bin << 'WRAPPER'
#!/bin/sh
perf record -o trace.perf.data $0.real "$@"
WRAPPER
      chmod +x $bin
    done
  '';
}
```

**Model definitions:**
```nix
# nix/models.nix
{
  models = [
    { name = "llama-7b"; url = "..."; }
    { name = "llama-13b"; url = "..."; }
    { name = "mistral-7b"; url = "..."; }
    { name = "mixtral-8x7b"; url = "..."; }
    { name = "phi-2"; url = "..."; }
  ];
}
```

### Phase 3: Trace Collection Pipeline (Week 3)

**Automated collection:**
```bash
#!/bin/bash
# scripts/run-all-models.sh

MODELS=(llama-7b llama-13b mistral-7b mixtral-8x7b phi-2)
PROMPTS=(
  "Hello world"
  "Explain quantum computing"
  "Write a Python function"
)

for model in "${MODELS[@]}"; do
  for prompt in "${PROMPTS[@]}"; do
    echo "Running $model with: $prompt"
    
    perf record -o "traces/$model/$(date +%s).perf.data" \
      ./llama-cli -m "models/$model.gguf" -p "$prompt"
    
    # Extract syscalls
    perf script -i "traces/$model/$(date +%s).perf.data" > \
      "traces/$model/$(date +%s).perf.script"
  done
done
```

### Phase 4: Analysis Tools (Week 4)

**Trace analysis:**
```python
# scripts/analyze-traces.py
import json
from pathlib import Path

def analyze_model_traces(model_name):
    traces = Path(f"traces/{model_name}").glob("*.perf.script")
    
    syscalls = {}
    hotspots = []
    
    for trace in traces:
        # Parse perf script
        # Extract syscalls
        # Find hotspots
        # Compute GF coverage
        pass
    
    return {
        "model": model_name,
        "syscalls": syscalls,
        "hotspots": hotspots,
        "gf_coverage": compute_gf_coverage(syscalls)
    }

# Analyze all models
results = {}
for model in MODELS:
    results[model] = analyze_model_traces(model)

# Save
with open("analysis/all-models.json", "w") as f:
    json.dump(results, f, indent=2)
```

### Phase 5: Integration with Lifting Pipeline (Week 5)

**Feed traces to lifting:**
```bash
# For each model, lift C++ → Rust
for model in llama-7b llama-13b mistral-7b; do
  python3 scripts/build/lift_python.py \
    --traces "traces/$model/*.perf.data" \
    --source "llama.cpp/src/llama.cpp" \
    --output "rust-llama/$model.rs"
done
```

## Cleanup Tasks

### Immediate
- [ ] Remove all `*~` and `#*#` files
- [ ] Consolidate branches (merge useful work)
- [ ] Document existing instrumentation
- [ ] List all collected traces

### Short-term
- [ ] Create clean Nix build
- [ ] Automate trace collection
- [ ] Organize traces by model
- [ ] Build analysis tools

### Long-term
- [ ] Lift llama.cpp → Rust
- [ ] Optimize based on traces
- [ ] Integrate with ZOS
- [ ] Add ZK proofs for inference

## Expected Outputs

### Traces
```
traces/
├── llama-7b/
│   ├── 1234567890.perf.data
│   ├── 1234567890.perf.script
│   └── analysis.json
├── llama-13b/
├── mistral-7b/
└── ...
```

### Analysis
```json
{
  "llama-7b": {
    "syscalls": {
      "read": 1234,
      "write": 567,
      "mmap": 89
    },
    "hotspots": [
      {"function": "ggml_mul_mat", "samples": 45000},
      {"function": "ggml_rope", "samples": 12000}
    ],
    "gf_coverage": {
      "GF(2^20)": 0.85,
      "GF(2^21)": 0.42
    }
  }
}
```

## Benefits

✅ **Reproducible** - Nix builds guarantee consistency  
✅ **Systematic** - All models traced uniformly  
✅ **Analyzable** - Structured trace data  
✅ **Optimizable** - Find bottlenecks across models  
✅ **Liftable** - Feed to Rust migration pipeline  

## Timeline

- Week 1: Audit & organize existing work
- Week 2: Clean Nix build system
- Week 3: Automated trace collection
- Week 4: Analysis tools
- Week 5: Integration with lifting

## Success Criteria

- [ ] Clean Nix build with instrumentation
- [ ] Traces collected for 10+ models
- [ ] Analysis tools working
- [ ] Hotspots identified
- [ ] GF coverage computed
- [ ] Ready for Rust lifting

---

**CRQ-004: Clean up llama.cpp instrumentation and systematize trace collection**
