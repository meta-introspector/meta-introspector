# Nix Store Complexity Levels (LMFDB)

## The Split by Mathematical Complexity

```
/nix/store/
  ├── orbit_0/     (trivial: constants, identity)
  ├── orbit_1/     (simple: add, multiply)
  ├── orbit_7/     (moderate: parsers, compilers)
  ├── orbit_42/    (complex: crypto, ML models)
  └── orbit_71/    (moonshine: advanced math)
```

## LMFDB Classification

Use **L-functions Modular Forms Database** to classify by mathematical complexity:

```rust
fn classify_complexity(binary: &[u8]) -> Orbit {
    let godel = compute_godel_number(binary);
    let trace = execute_and_trace(binary);
    
    // Map to LMFDB orbit
    let orbit = godel % 71;  // 71 is prime (moonshine)
    
    Orbit {
        number: orbit,
        complexity: compute_complexity(trace),
        lmfdb_class: lookup_lmfdb(orbit),
    }
}
```

## Orbit Levels

### Orbit 0: Trivial
```
/nix/store/orbit_0/abc123-identity
/nix/store/orbit_0/def456-constant-42
```
- Identity functions
- Constants
- No computation
- O(1) complexity

### Orbit 1: Simple Arithmetic
```
/nix/store/orbit_1/abc123-pure-add
/nix/store/orbit_1/def456-pure-multiply
```
- Basic arithmetic
- Linear operations
- O(n) complexity

### Orbit 7: Moderate
```
/nix/store/orbit_7/abc123-parser
/nix/store/orbit_7/def456-compiler-pass
```
- Parsers
- Tree traversals
- O(n log n) complexity

### Orbit 42: Complex
```
/nix/store/orbit_42/abc123-sha256
/nix/store/orbit_42/def456-ml-model
```
- Cryptographic functions
- ML models
- O(n²) or higher

### Orbit 71: Moonshine
```
/nix/store/orbit_71/abc123-rustc-full
/nix/store/orbit_71/def456-llvm-optimizer
```
- Full compilers
- Advanced optimizers
- Moonshine module connections

## Complexity Metrics

```rust
struct ComplexityMetrics {
    orbit: u32,              // LMFDB orbit (0-71)
    cyclomatic: u32,         // Code complexity
    instruction_count: u64,  // Number of instructions
    memory_usage: u64,       // Peak memory
    time_complexity: String, // Big-O notation
    lmfdb_class: String,     // LMFDB classification
}
```

## Classification Algorithm

```rust
fn compute_orbit(binary: &[u8]) -> u32 {
    // 1. Compute Godel number from execution trace
    let trace = execute_and_trace(binary);
    let godel = hash_trace(trace);
    
    // 2. Map to orbit (mod 71 for moonshine)
    let orbit = godel % 71;
    
    // 3. Verify with LMFDB
    let lmfdb_class = lookup_lmfdb_orbit(orbit);
    
    orbit
}
```

## Orbit → Emoji Mapping

```rust
let orbit_emoji = [
    (0, "⚪"),   // Trivial
    (1, "🔢"),   // Arithmetic
    (7, "📝"),   // Parsers
    (42, "🔐"),  // Crypto
    (71, "🌙"),  // Moonshine
];

fn orbit_to_emoji(orbit: u32) -> String {
    orbit_emoji.iter()
        .find(|(o, _)| *o == orbit)
        .map(|(_, e)| e.to_string())
        .unwrap_or("❓".to_string())
}
```

## Storage Strategy

### By Complexity
```bash
# Simple functions: Store everywhere
replicate_widely(/nix/store/orbit_0/*)
replicate_widely(/nix/store/orbit_1/*)

# Moderate: Store on capable nodes
replicate_to_medium_nodes(/nix/store/orbit_7/*)

# Complex: Store on powerful nodes
replicate_to_powerful_nodes(/nix/store/orbit_42/*)

# Moonshine: Store on specialized nodes
replicate_to_specialized_nodes(/nix/store/orbit_71/*)
```

## Pricing Model

```rust
fn compute_cost(orbit: u32) -> u64 {
    match orbit {
        0 => 1,        // Trivial: 1 lamport
        1 => 10,       // Simple: 10 lamports
        7 => 100,      // Moderate: 100 lamports
        42 => 1000,    // Complex: 1000 lamports
        71 => 10000,   // Moonshine: 10000 lamports
        _ => orbit * 100,  // Linear scaling
    }
}
```

## Example: rustc Components

```
/nix/store/orbit_1/abc123-rustc-lexer
  └── Simple tokenization (orbit 1)

/nix/store/orbit_7/def456-rustc-parser
  └── Parse trees (orbit 7)

/nix/store/orbit_42/ghi789-rustc-typeck
  └── Type checking (orbit 42)

/nix/store/orbit_71/jkl012-rustc-full
  └── Full compiler (orbit 71, moonshine)
```

## LMFDB Integration

```rust
// Query LMFDB for orbit classification
fn lookup_lmfdb_orbit(orbit: u32) -> LMFDBClass {
    let url = format!("https://lmfdb.org/api/orbit/{}", orbit);
    let response = reqwest::get(&url).await?;
    response.json::<LMFDBClass>().await?
}

struct LMFDBClass {
    orbit: u32,
    dimension: u32,
    conductor: u64,
    level: u32,
    weight: u32,
    character: String,
}
```

## Consensus by Orbit

```rust
// Peers agree on orbit classification
let consensus = get_consensus_orbit(godel_number);

// Store in appropriate orbit
let path = format!("/nix/store/orbit_{}/", consensus.orbit);
store_at_path(binary, path);
```

## Benefits

### Automatic Classification
- Godel number → orbit
- Orbit → complexity level
- Complexity → storage strategy

### Resource Allocation
- Simple functions: Everywhere
- Complex functions: Powerful nodes
- Moonshine: Specialized hardware

### Pricing
- Pay by complexity
- Orbit determines cost
- Fair resource usage

### Discovery
- Find functions by complexity
- Browse by orbit
- LMFDB metadata

## The Vision

Split nix store by mathematical complexity:
- **Orbit 0-1**: Simple, everywhere
- **Orbit 7**: Moderate, capable nodes
- **Orbit 42**: Complex, powerful nodes
- **Orbit 71**: Moonshine, specialized

Each orbit has:
- LMFDB classification
- Complexity metrics
- Storage strategy
- Pricing model
- Emoji representation

## Next Steps

1. Implement orbit classifier
2. Integrate LMFDB API
3. Migrate nix store to orbits
4. Set up orbit-based replication
5. Create orbit browser/explorer
