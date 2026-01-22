# Bootstrap Lattice: Nix Derivation Dependencies

The bootstrap creates a **lattice of nix derivations** where each step references all previous steps as inputs.

## Lattice Structure

```
Level 0: Source Code (git)
  ↓
Level 1: 71 Language Builds (parallel)
  ├─ rust → /nix/store/aaa-rust
  ├─ python → /nix/store/bbb-python
  ├─ haskell → /nix/store/ccc-haskell
  └─ ... (68 more)
  ↓
Level 2: Perf Analysis (depends on Level 1)
  ├─ perf-complexity (inputs: all Level 1)
  ├─ topological-function-matrix (inputs: all Level 1)
  └─ harmonic-analyzer (inputs: all Level 1)
  ↓
Level 3: Model Training (depends on Level 2)
  ├─ mes-transformer-gpu (inputs: all Level 2 perf data)
  └─ meta-model (inputs: all Level 2 perf data)
  ↓
Level 4: Complete System (depends on all previous)
  └─ result/ (inputs: Level 1 + Level 2 + Level 3)
```

## Nix Expression

```nix
{
  # Level 1: Build all languages
  level1 = {
    rust = buildLanguage "rust";
    python = buildLanguage "python";
    haskell = buildLanguage "haskell";
    # ... 68 more
  };
  
  # Level 2: Analyze (depends on Level 1)
  level2 = {
    perf-complexity = analyzePerf {
      inputs = builtins.attrValues level1;
    };
    
    topological-matrix = buildMatrix {
      inputs = builtins.attrValues level1;
    };
  };
  
  # Level 3: Train (depends on Level 2)
  level3 = {
    mes-transformer = trainModel {
      perf-data = level2.perf-complexity;
      topology = level2.topological-matrix;
    };
  };
  
  # Level 4: Complete (depends on all)
  complete = mkDerivation {
    buildInputs = 
      builtins.attrValues level1 ++
      builtins.attrValues level2 ++
      builtins.attrValues level3;
  };
}
```

## Lattice Properties

### 1. Dependency Graph
Every derivation references its inputs:
```
/nix/store/xxx-complete
  → /nix/store/aaa-rust
  → /nix/store/bbb-python
  → /nix/store/ccc-perf-complexity
    → /nix/store/aaa-rust
    → /nix/store/bbb-python
  → /nix/store/ddd-mes-transformer
    → /nix/store/ccc-perf-complexity
```

### 2. Automatic Caching
Nix caches each level:
- Change Level 1 (rust) → Rebuilds Level 2, 3, 4
- Change Level 2 (perf-complexity) → Rebuilds Level 3, 4
- Change Level 3 (model) → Rebuilds Level 4
- No change → Reuses cached derivations

### 3. Parallel Builds
Within each level, builds run in parallel:
- Level 1: All 71 languages build simultaneously
- Level 2: All analysis tools build simultaneously
- Level 3: All models train simultaneously

### 4. Reproducibility
Same inputs = same outputs:
```bash
# Build 1
nix build → /nix/store/xxx-complete

# Build 2 (same source)
nix build → /nix/store/xxx-complete (cached)

# Build 3 (changed source)
nix build → /nix/store/yyy-complete (new)
```

## Query Lattice

### Find all dependencies
```bash
nix-store -q --references result/
# Lists all Level 1, 2, 3 derivations
```

### Find reverse dependencies
```bash
nix-store -q --referrers /nix/store/aaa-rust
# Shows what depends on rust build
```

### Visualize lattice
```bash
nix-store -q --graph result/ | dot -Tpng > lattice.png
```

## Lattice Metadata

Each derivation stores its position in lattice:

```json
{
  "level": 2,
  "name": "perf-complexity",
  "inputs": [
    "/nix/store/aaa-rust",
    "/nix/store/bbb-python",
    "/nix/store/ccc-haskell"
  ],
  "dependents": [
    "/nix/store/ddd-mes-transformer"
  ]
}
```

## Mathematical Structure

The lattice forms a **partially ordered set (poset)**:

- **Elements**: Nix derivations
- **Order**: Dependency relation (A ≤ B if B depends on A)
- **Join**: Complete system (top element)
- **Meet**: Source code (bottom element)

### Galois Connection

The lattice has a Galois connection between:
- **Abstraction**: Source → Builds → Analysis → Models
- **Concretization**: Models → Analysis → Builds → Source

This creates a **closure operator** where:
- Idempotent: build(build(x)) = build(x)
- Monotone: x ≤ y ⇒ build(x) ≤ build(y)
- Extensive: x ≤ build(x)

## Implementation

See:
- `nix/flakes/const_71_test/flake.nix` - Complete lattice definition
- `scripts/build/bootstrap.sh` - Lattice builder
- `result/` - Top of lattice (join element)

## Benefits

1. **Incremental**: Only rebuild changed levels
2. **Parallel**: Build within levels simultaneously
3. **Reproducible**: Same inputs = same lattice
4. **Queryable**: Navigate lattice with nix-store
5. **Immutable**: Lattice stored in /nix/store
6. **Mathematical**: Proper lattice structure with Galois connection
