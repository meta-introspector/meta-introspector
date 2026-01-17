# Harmonic Test Filtering

Mathematical test selection using frequency analysis and harmonic resonance.

## Concept

Treat test patterns as waveforms:
- **Input byte positions** → signal samples
- **Instruction sequences** → frequency components
- **FFT transform** → harmonic signature
- **Resonance** → test importance
- **Distance** → test similarity

## Why Harmonics?

Traditional test selection:
- Random sampling → poor coverage
- Exhaustive → too slow
- Heuristic → biased

Harmonic filtering:
- **Orthogonal selection** → maximum diversity
- **Frequency bands** → specific code paths
- **Resonance** → prioritize important tests
- **Mathematical** → reproducible, unbiased

## Harmonic Signature

Each test cluster gets a signature:

```rust
struct HarmonicSignature {
    frequencies: Vec<f64>,  // Frequency components (0.0-1.0)
    amplitudes: Vec<f64>,   // Strength of each frequency
    phase: Vec<f64>,        // Phase shift
}
```

Computed via discrete Fourier transform of input byte positions.

## Selection Methods

### 1. Orthogonal Selection (Maximum Diversity)

Select tests that are maximally different:

```rust
let diverse_tests = filter.select_orthogonal(10);
// Returns 10 most diverse test clusters
```

**Use case**: Broad coverage with minimal tests

### 2. Resonance Filtering (Importance)

Select by amplitude (test "strength"):

```rust
let important = filter.select_by_resonance(10.0, f64::MAX);
// Returns tests with high resonance (>= 10.0)
```

**Use case**: Prioritize complex/important code paths

### 3. Harmonic Matching (Frequency Bands)

Select tests matching specific frequencies:

```rust
let harmonic = filter.filter_harmonics(0.2, &[0.4, 0.6, 0.8]);
// Returns tests with fundamental 0.2 and overtones
```

**Use case**: Target specific execution patterns

### 4. Similarity Search

Find tests similar to a target pattern:

```rust
let target = HarmonicFilter::compute_signature(&[0, 2, 4, 6], &[0x400500]);
let similar = filter.select_tests(&target, 5.0);
// Returns tests within distance 5.0 of target
```

**Use case**: Regression testing, mutation testing

## Example

### Input Clusters
```
Cluster 0: bytes [0,1,2,3,4]     → signature (freq=[0.0,0.2,0.4], amp=[5.0,3.0,1.0])
Cluster 1: bytes [10,11,12,13]   → signature (freq=[0.0,0.25,0.5], amp=[4.0,2.0,0.5])
Cluster 2: bytes [0,5,10,15,20]  → signature (freq=[0.0,0.2,0.4], amp=[10.0,5.0,2.0])
```

### Orthogonal Selection
```
Select 2 most diverse:
→ Cluster 0 and Cluster 1 (different frequency patterns)
```

### Resonance Filtering
```
Select resonance >= 5.0:
→ Cluster 0 (resonance 3.0) ✗
→ Cluster 2 (resonance 5.7) ✓
```

### Harmonic Matching
```
Fundamental 0.2, overtones [0.4]:
→ Cluster 0 ✓ (has 0.2 and 0.4)
→ Cluster 1 ✗ (has 0.25 and 0.5)
→ Cluster 2 ✓ (has 0.2 and 0.4)
```

## Integration with Source2Test

```rust
use source2test::Source2Test;
use harmonic_filter::HarmonicFilter;

// 1. Cluster tests
let s2t = Source2Test::from_trace("reach.txt")?;

// 2. Build harmonic filter
let mut filter = HarmonicFilter::new();
for cluster in &s2t.clusters {
    filter.add_test(
        cluster.id,
        &cluster.input_bytes,
        &cluster.example_insns
    );
}

// 3. Select tests
let selected = filter.select_orthogonal(10);

// 4. Run only selected tests
for cluster_id in selected {
    run_test(&s2t.clusters[cluster_id]);
}
```

## Mathematical Properties

### Frequency Domain
- Low frequencies → coarse patterns (function-level)
- High frequencies → fine patterns (byte-level)

### Amplitude
- High amplitude → strong pattern (many bytes)
- Low amplitude → weak pattern (few bytes)

### Phase
- Phase shift → pattern offset
- Phase coherence → pattern alignment

### Distance Metric
```
distance(A, B) = Σ |amp_A[i] - amp_B[i]| + 0.1 * |phase_A[i] - phase_B[i]|
```

## Use Cases

### 1. CI/CD Test Selection
```bash
# Select 10 diverse tests for quick CI
harmonic_filter --orthogonal 10 --output ci_tests.json

# Run only selected
for test in $(jq -r '.[]' ci_tests.json); do
    cargo test cluster_$test
done
```

### 2. Nightly Full Suite
```bash
# Select all high-resonance tests
harmonic_filter --resonance-min 5.0 --output nightly_tests.json
```

### 3. Regression Testing
```bash
# Find tests similar to changed code
harmonic_filter --similar-to changed_cluster.json --threshold 3.0
```

### 4. Mutation Testing
```bash
# For each mutation, select relevant tests
for mutation in mutations/*; do
    harmonic_filter --similar-to $mutation --output tests_$mutation.json
done
```

### 5. Fuzzing Seed Selection
```bash
# Select diverse seeds for fuzzing
harmonic_filter --orthogonal 100 --output fuzz_seeds.json
```

## Performance

- Signature computation: O(n log n) per cluster
- Orthogonal selection: O(n²) for n tests
- Resonance filtering: O(n)
- Similarity search: O(n)

## Advantages

1. **Mathematical**: Reproducible, no heuristics
2. **Efficient**: Select k tests from n in O(n²)
3. **Diverse**: Orthogonal selection guarantees coverage
4. **Targeted**: Frequency filtering for specific paths
5. **Prioritized**: Resonance for importance

## Limitations

- Assumes byte positions encode meaningful patterns
- FFT may not capture all semantic relationships
- Distance metric is approximate
- Requires tuning thresholds

## Future Work

- [ ] Wavelet transforms for multi-scale analysis
- [ ] Machine learning for optimal thresholds
- [ ] Real-time adaptive filtering
- [ ] Integration with coverage metrics
- [ ] Visualization of harmonic space

## References

- Fourier Analysis: https://en.wikipedia.org/wiki/Fourier_analysis
- Signal Processing: https://en.wikipedia.org/wiki/Digital_signal_processing
- Test Selection: https://en.wikipedia.org/wiki/Test_selection
