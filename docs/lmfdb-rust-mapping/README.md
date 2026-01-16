# LMFDB Rust Mapping Library

Unified library for mapping Rust binaries, symbols, and performance data to LMFDB mathematical structures.

## 🎯 What We Learned

### Self-Analysis Results

We successfully used the LMFDB mapping library to analyze itself! 

**Analyzed**: `libserde_derive.so` (8174 symbols)

**Key Findings**:
- **Conductor**: 618
- **Orbit Distribution**:
  - Genesis (11): 3715 symbols (45%)
  - Trinity (23): 3826 symbols (47%)
  - Completeness (47): 633 symbols (8%)
  - Return (71): 0 symbols (0%)

**Insight**: Most Rust symbols fall into Genesis/Trinity orbits, indicating foundational/stable complexity. Only 8% reach Completeness level.

## 📦 Structure

```
lmfdb-rust-mapping/
├── lmfdb-types/        # Core data types (LMFDBLabel, OrbitLevel, etc.)
├── lmfdb-traits/       # Trait definitions (LMFDBClient, LMFDBMapper, etc.)
└── src/lib.rs          # Main implementation
```

## 🚀 Usage

```rust
use lmfdb_rust_mapping::*;

let mut mapper = LMFDBMapper::new();
let analysis = mapper.analyze_binary("path/to/binary.so")?;

println!("Total symbols: {}", analysis.total_symbols);
println!("Conductor: {}", analysis.conductor);

for (orbit, count) in &analysis.orbit_distribution {
    println!("{:?}: {} symbols", orbit, count);
}
```

## 🔬 Self-Analysis

Run the self-analyzer:

```bash
cargo run --bin analyze
```

Output: `lmfdb_self_analysis.json`

## 🎯 The 71 Pattern

Orbit levels based on mathematical progression:
- **11 (Genesis)**: Foundation/Core
- **23 (Trinity)**: Stability/Structure  
- **47 (Completeness)**: Advanced/Complete
- **71 (Return)**: Mastery/Transcendence

## 📊 Integration

- **Binary Analysis**: Map ELF symbols to LMFDB labels
- **Perf Data**: Classify performance samples by orbit
- **71 Flakes**: Analyze all 71 language implementations
- **Duplicate Detection**: Find similar functions via LMFDB signatures

## 🔗 Next Steps

1. Analyze all 71 flakes with LMFDB mapping
2. Find duplicate code via modular signatures
3. Build LMFDB server for storing mappings
4. Integrate with perf canonical reader
