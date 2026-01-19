# Lattice Decomposition Specification

## Goal
Decompose the entire Rust ecosystem into a complexity lattice, enabling:
- Topological build ordering
- Source deduplication across 5000+ crates
- Git object sharing via `/mnt/data1/git`
- Single-version flattening of dependencies
- Reproducible builds from constants → rustc → cargo2nix → ecosystem

## Lattice Layers

### Layer 0: Constants
- Primitive types, literals, static data
- Zero dependencies
- Examples: version strings, feature flags, const values

### Layer 1: Pure Functions
- Functions with only Layer 0 dependencies
- No external crate dependencies
- Examples: hash functions, encoders, math utilities

### Layer 2: Single-Dependency Modules
- Depend only on Layer 0 or Layer 1
- Examples: error types, simple traits, basic data structures

### Layer N: Complexity Hierarchy
Each layer L_n depends only on layers L_0 through L_{n-1}

## Build Targets

### Phase 1: Core Compiler (Layers 0-50)
```
constants → types → traits → macros → rustc_ast → rustc_middle → rustc
```

### Phase 2: Build Tools (Layers 51-100)
```
rustc → cargo → cargo2nix
```

### Phase 3: Ecosystem (Layers 101-N)
```
cargo2nix → minimal-build-server → 5000+ crates
```

## Implementation Strategy

### 1. Dependency Graph Analysis
```rust
// For each crate in ~/nix/vendor/rust/cargo2nix/submodules
// Extract: name, version, dependencies, source location
// Build: directed acyclic graph (DAG)
```

### 2. Topological Sort
```
L_0 = {crates with 0 deps}
L_1 = {crates depending only on L_0}
L_n = {crates depending only on L_0..L_{n-1}}
```

### 3. Source Deduplication
```
/mnt/data1/git/
  ├── github.com/rust-lang/rust/
  │   └── library/core/  → shared by all crates
  ├── github.com/serde-rs/serde/
  │   └── serde/  → single version, all dependents use this
  └── crates.io/
      └── <crate>/<version>/  → canonical source
```

### 4. Build Orchestration
```nix
# For each layer L_n:
buildLayer = n: prevLayers:
  let
    cratesInLayer = filterByMaxDepth n;
    inputs = flatten prevLayers;
  in
    buildCrates cratesInLayer inputs;
```

## Data Structures

### Crate Node
```rust
struct CrateNode {
    name: String,
    version: String,
    source: PathBuf,  // /mnt/data1/git/<domain>/<path>
    deps: Vec<CrateNode>,
    layer: usize,
    complexity: usize,  // transitive dep count
}
```

### Lattice
```rust
struct Lattice {
    layers: Vec<Vec<CrateNode>>,
    max_depth: usize,
    total_crates: usize,
}
```

## Metrics

### Complexity Score
```
complexity(crate) = 1 + Σ complexity(dep) for dep in deps
```

### Deduplication Ratio
```
dedup_ratio = unique_sources / total_source_references
```

### Build Parallelism
```
max_parallel = max(|L_n|) for all layers n
```

## Template: cargo2nix/submodules

Current state:
- 500 Rust submodules in `~/nix/vendor/rust/cargo2nix/submodules/`
- All use local sources (no network fetches)
- Single flattened dependency tree

Target:
- Extend to 5000+ crates
- Maintain local-only builds
- Preserve topological ordering
- Enable incremental layer builds

## Output Artifacts

### 1. Lattice Map
```
spec/lattice.json
{
  "layers": [
    {"id": 0, "crates": ["const_values", "primitives"], "complexity": 1},
    {"id": 1, "crates": ["hash", "encode"], "complexity": 2},
    ...
  ]
}
```

### 2. Build Order
```
spec/build_order.txt
# Layer 0
const_values
primitives

# Layer 1
hash
encode
...
```

### 3. Deduplication Report
```
spec/dedup_report.md
- Total crates: 5000
- Unique sources: 3200
- Dedup ratio: 64%
- Shared git objects: 2.1GB saved
```

## Next Steps

1. **Analyze existing submodules** - Extract dependency graph from 500 crates
2. **Compute lattice layers** - Topological sort by dependency depth
3. **Identify deduplication opportunities** - Find duplicate sources across crates
4. **Generate build specification** - Create Nix expressions for each layer
5. **Validate with cargo2nix** - Build cargo2nix using lattice approach
6. **Scale to ecosystem** - Apply to all 5000+ crates

## Success Criteria

- ✓ Zero network fetches during build (all sources in `/mnt/data1/git`)
- ✓ Deterministic layer assignment for all crates
- ✓ Parallel builds within each layer
- ✓ <10% storage overhead vs naive approach
- ✓ Reproducible builds from layer 0 → N
