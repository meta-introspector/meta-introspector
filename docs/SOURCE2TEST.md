# Source2Test: Reachability-Based Test Generation

Automatically clusters input bytes by their execution profile and generates targeted test cases.

## Concept

Instead of random fuzzing, use reachability analysis to:
1. **Profile each output byte** by the instructions that generated it
2. **Cluster input bytes** that share the same execution profile
3. **Split source files** into minimal test cases per cluster
4. **Generate tests** that exercise specific code paths

## How It Works

```
Input Source File
    ↓
[QEMU Reachability Trace]
    ↓
For each output byte:
  - Hash instruction sequence
  - Count input dependencies
  - Create profile fingerprint
    ↓
Group input bytes by profile
    ↓
Split input into clusters
    ↓
Generate test cases
```

## Example

### Input
```rust
fn add(a: i32, b: i32) -> i32 { a + b }
fn mul(a: i32, b: i32) -> i32 { a * b }
fn main() {
    println!("{}", add(1, 2));
    println!("{}", mul(3, 4));
}
```

### Clustering
```
Cluster 0: Profile(3 inputs, 2 insns, hash a1b2c3d4)
  Input bytes: [0, 1, 2, 3, 4, 5]  // "fn add"
  Output bytes: [0-50]
  
Cluster 1: Profile(3 inputs, 3 insns, hash e5f6g7h8)
  Input bytes: [50, 51, 52, 53, 54, 55]  // "fn mul"
  Output bytes: [100-150]
```

### Generated Tests
```rust
// test_cluster_000.rs
#[test]
fn test_cluster_0() {
    // Tests "fn add" path
    // Input bytes: [0, 1, 2, 3, 4, 5]
}

// test_cluster_001.rs
#[test]
fn test_cluster_1() {
    // Tests "fn mul" path
    // Input bytes: [50, 51, 52, 53, 54, 55]
}
```

## Usage

```bash
# 1. Run reachability trace
cargo build --release --bin reach_tracer
./target/release/reach_tracer

# 2. Cluster and generate tests
cargo build --release --bin source2test
./target/release/source2test
```

## Output

### `source2test_splits/`
Binary files containing input bytes for each cluster:
```
sample_000_profile_a1b2c3d4.bin  # Bytes that trigger profile a1b2c3d4
sample_001_profile_e5f6g7h8.bin  # Bytes that trigger profile e5f6g7h8
```

### `source2test_tests/`
Generated test skeletons:
```rust
test_cluster_000.rs
test_cluster_001.rs
test_cluster_002.rs
```

### `source2test_clusters.json`
Full clustering data:
```json
{
  "clusters": [
    {
      "profile": {
        "input_count": 3,
        "insn_count": 2,
        "insn_hash": "a1b2c3d4"
      },
      "input_bytes": [0, 1, 2, 3, 4, 5],
      "output_bytes": [0, 1, 2, ..., 50],
      "example_insns": [0x400500, 0x400510]
    }
  ]
}
```

## Applications

### 1. Minimal Test Cases
Extract only the source bytes needed to test a specific code path:
```bash
# Cluster 0 tests only the "add" function
# No need to include "mul" function in test
```

### 2. Coverage-Guided Fuzzing
Focus fuzzing on under-tested profiles:
```rust
let rare_profiles = clusters.iter()
    .filter(|c| c.output_bytes.len() < 10)
    .collect();
// Fuzz these more
```

### 3. Regression Testing
Detect when code changes affect specific clusters:
```bash
# Before change: Cluster 0 has profile a1b2c3d4
# After change: Cluster 0 has profile xxxxxxxx
# → Code path changed, investigate
```

### 4. Incremental Compilation
Only recompile clusters affected by source changes:
```rust
// If bytes 10-20 changed
let affected_clusters = clusters.iter()
    .filter(|c| c.input_bytes.iter().any(|&b| b >= 10 && b <= 20))
    .collect();
```

### 5. Code Understanding
See which source regions exercise which compiler paths:
```
Bytes 0-50 (function definitions) → Simple codegen
Bytes 100-150 (complex macros) → Heavy optimization passes
```

## Integration

### With Fuzzing
```rust
use source2test::Source2Test;

fn fuzz_cluster(cluster_id: usize, data: &[u8]) {
    let s2t = Source2Test::load("clusters.json")?;
    let cluster = &s2t.clusters[cluster_id];
    
    // Mutate only the input bytes in this cluster
    let mut input = original_input.clone();
    for (i, &byte_offset) in cluster.input_bytes.iter().enumerate() {
        if i < data.len() {
            input[byte_offset] = data[i];
        }
    }
    
    test_compilation(&input);
}
```

### With CI
```yaml
- name: Generate test clusters
  run: |
    ./target/release/reach_tracer
    ./target/release/source2test
    
- name: Run cluster tests
  run: |
    for test in source2test_tests/*.rs; do
      rustc --test $test && ./${test%.rs}
    done
```

## Performance

- **Clustering**: O(n) where n = number of output bytes
- **Splitting**: O(m) where m = number of input bytes
- **Memory**: ~200 bytes per output byte

## Limitations

- Requires QEMU trace (slow)
- Profile hash may collide (use longer hash if needed)
- Doesn't track control flow (only data flow)
- Max 10MB output tracking

## Future Enhancements

- [ ] Control flow clustering (branch coverage)
- [ ] Value-based profiles (not just instruction sequences)
- [ ] Automatic test oracle generation
- [ ] Differential clustering (compare versions)
- [ ] Machine learning for profile similarity
- [ ] Integration with property-based testing

## Example Workflow

```bash
# 1. Trace compilation
./target/release/reach_tracer

# 2. Cluster by profile
./target/release/source2test

# 3. Review clusters
cat source2test_clusters.json | jq '.clusters[] | {profile, input_count: (.input_bytes | length)}'

# 4. Test specific cluster
rustc --test source2test_tests/test_cluster_000.rs
./test_cluster_000

# 5. Fuzz specific cluster
afl-fuzz -i source2test_splits/ -o findings/ -- ./target
```
EOF
cat > docs/SOURCE2TEST.md
echo "✅ Source2Test documentation created"
