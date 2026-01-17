# Source2Test: Profile-Based Test Generation

Automatically split source files into minimal test cases based on execution profiles.

## Quick Start

```bash
# Build
cargo build --release --bin source2test

# Run
./target/release/source2test
```

## What It Does

1. **Profiles** each output byte by its instruction sequence
2. **Clusters** input bytes that share the same profile
3. **Splits** source into minimal test inputs per cluster
4. **Generates** test skeletons for each cluster

## Example

**Input:** `sample.rs` (100 bytes)
```rust
fn add(a: i32, b: i32) -> i32 { a + b }
fn mul(a: i32, b: i32) -> i32 { a * b }
```

**Output:** 2 clusters
- Cluster 0: bytes 0-30 (add function) → profile `a1b2c3d4`
- Cluster 1: bytes 31-60 (mul function) → profile `e5f6g7h8`

**Generated:**
- `source2test_splits/sample_000_profile_a1b2c3d4.bin` (30 bytes)
- `source2test_splits/sample_001_profile_e5f6g7h8.bin` (30 bytes)
- `source2test_tests/test_cluster_000.rs`
- `source2test_tests/test_cluster_001.rs`

## Profile Fingerprint

Each profile is a hash of:
- Number of input bytes that contributed
- Number of instructions executed
- SHA256 of instruction sequence

Same profile = same execution path = can be tested together

## Use Cases

### Minimal Test Cases
Test only the code you need:
```bash
# Instead of testing entire file
rustc test.rs

# Test specific cluster
rustc source2test_splits/sample_000_profile_a1b2c3d4.bin
```

### Targeted Fuzzing
Fuzz specific execution paths:
```bash
afl-fuzz -i source2test_splits/ -o findings/ -- rustc @@
```

### Regression Detection
Detect when code changes affect specific paths:
```bash
# Before: profile a1b2c3d4
# After:  profile xxxxxxxx
# → Execution path changed!
```

### Coverage Analysis
See which source regions exercise which compiler paths:
```json
{
  "cluster_0": {
    "input_bytes": [0, 1, 2],
    "profile": "simple_codegen"
  },
  "cluster_1": {
    "input_bytes": [100, 101, 102],
    "profile": "heavy_optimization"
  }
}
```

## Output Files

### Splits
`source2test_splits/` - Binary files with input bytes per cluster

### Tests
`source2test_tests/` - Generated test skeletons:
```rust
#[test]
fn test_cluster_0() {
    // Input bytes: [0, 1, 2, 3, 4, 5]
    // Output bytes: [0, 1, 2, ..., 50]
    // Instructions: [0x400500, 0x400510]
    
    // TODO: Add assertions
}
```

### JSON
`source2test_clusters.json` - Full clustering data

## Integration

### With Cargo
```toml
[dev-dependencies]
source2test = { path = "../source2test" }
```

### With Fuzzing
```bash
# Generate clusters
./target/release/source2test

# Fuzz each cluster
for split in source2test_splits/*.bin; do
    afl-fuzz -i $split -o findings_$(basename $split) -- rustc @@
done
```

### With CI
```yaml
- run: cargo build --release --bin source2test
- run: ./target/release/source2test
- run: cargo test --test-threads=1 source2test_tests/*.rs
```

## Performance

- Clustering: ~1ms per 1000 output bytes
- Splitting: ~0.1ms per 1000 input bytes
- Memory: ~200 bytes per output byte

## Limitations

- Requires reachability trace (run `reach_tracer` first)
- Profile hash collisions possible (use longer hash)
- No control flow tracking (only data flow)

## Advanced

### Custom Profiles
```rust
impl Source2Test {
    fn custom_profile(&self, reach: &ByteReach) -> ReachProfile {
        // Add your own profiling logic
        ReachProfile {
            input_count: reach.input_offsets.len(),
            insn_count: reach.instruction_addrs.len(),
            insn_hash: custom_hash(&reach.instruction_addrs),
            // Add custom fields
        }
    }
}
```

### Filter Clusters
```rust
let important_clusters = s2t.clusters.iter()
    .filter(|c| c.output_bytes.len() > 100)
    .collect();
```

### Merge Clusters
```rust
// Merge clusters with similar profiles
let merged = merge_similar_profiles(&s2t.clusters, threshold);
```
