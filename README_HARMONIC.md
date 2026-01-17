# Harmonic Test Filtering

Mathematical test selection using frequency analysis.

## Quick Start

```bash
cargo build --release --bin harmonic_filter
./target/release/harmonic_filter
```

## Concept

Tests as waveforms:
- Input byte positions → signal
- FFT → frequency signature
- Select by mathematical properties

## Selection Methods

### 1. Orthogonal (Diverse)
```rust
filter.select_orthogonal(10)
// Returns 10 most different tests
```

### 2. Resonance (Important)
```rust
filter.select_by_resonance(5.0, f64::MAX)
// Returns high-amplitude tests
```

### 3. Harmonic (Frequency)
```rust
filter.filter_harmonics(0.2, &[0.4, 0.6])
// Returns tests matching frequency bands
```

### 4. Similar (Pattern)
```rust
filter.select_tests(&target_signature, 3.0)
// Returns tests similar to target
```

## Example

```rust
let mut filter = HarmonicFilter::new();

// Add test clusters
filter.add_test(0, &[0,1,2,3,4], &[0x400500]);
filter.add_test(1, &[10,11,12,13], &[0x400600]);
filter.add_test(2, &[0,5,10,15,20], &[0x400700]);

// Select 2 most diverse
let diverse = filter.select_orthogonal(2);
// → [0, 1] (different patterns)

// Select high resonance
let important = filter.select_by_resonance(5.0, f64::MAX);
// → [2] (high amplitude)
```

## Integration

```rust
// With Source2Test
let s2t = Source2Test::from_trace("reach.txt")?;
let mut filter = HarmonicFilter::new();

for cluster in &s2t.clusters {
    filter.add_test(cluster.id, &cluster.input_bytes, &cluster.insns);
}

let selected = filter.select_orthogonal(10);
// Run only these 10 tests
```

## Use Cases

1. **CI/CD**: Select 10 diverse tests for fast feedback
2. **Nightly**: Select all high-resonance tests
3. **Regression**: Find tests similar to changed code
4. **Fuzzing**: Select diverse seeds
5. **Mutation**: Target relevant tests per mutation

## Why Harmonics?

- **Mathematical**: Reproducible, unbiased
- **Efficient**: O(n²) for n tests
- **Diverse**: Orthogonal selection guarantees coverage
- **Targeted**: Frequency bands for specific paths

## Output

```json
{
  "cluster_id": 0,
  "signature": {
    "frequencies": [0.0, 0.2, 0.4],
    "amplitudes": [5.0, 3.0, 1.0],
    "phase": [0.0, 1.57, 3.14]
  },
  "resonance": 3.0
}
```

## Performance

- Signature: O(n log n) per test
- Selection: O(n²) for orthogonal, O(n) for others
- Memory: ~100 bytes per test

## Complete Pipeline

```bash
# 1. Trace reachability
./target/release/reach_tracer

# 2. Cluster by profile
./target/release/source2test

# 3. Filter harmonically
./target/release/harmonic_filter

# 4. Run selected tests
for test in $(jq -r '.[]' selected.json); do
    cargo test cluster_$test
done
```
