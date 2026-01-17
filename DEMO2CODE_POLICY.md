# Demo2Code Policy

## Core Principle

**NO MORE DEMOS. NO MORE FAKE DATA. NO MORE PROXIES.**

Every component must be production-ready, fully integrated, and use real data.

## Banned Patterns

### ❌ Prohibited
```rust
// Mock data
let mock_data = vec![1, 2, 3];

// Fake implementations
fn fake_analysis() -> Result { Ok(FakeResult) }

// Placeholder returns
return Some("placeholder".to_string());

// Demo-only code
if cfg!(demo) { ... }

// Hardcoded test values
let test_value = 42;

// Simplified proxies
struct SimpleProxy; // Real implementation required
```

### ✅ Required
```rust
// Real data from actual sources
let data = fs::read(path)?;

// Full implementations
fn real_analysis(input: &[u8]) -> Result<Analysis> {
    // Complete implementation
}

// Actual computed values
return compute_from_real_data(input);

// Production code only
// No demo branches

// Dynamic values
let value = measure_actual_metric()?;

// Complete implementations
struct FullImplementation {
    // All fields, all methods
}
```

## Code Classification

### Fake Detection Metrics

1. **Constant Ratio**: `constants / total_tokens`
   - Threshold: > 0.1 = likely fake

2. **Hardcoded Strings**: Count of literal strings
   - Threshold: > 5 = suspicious

3. **Mock Patterns**: Regex matches for "mock", "fake", "demo", "test", "placeholder"
   - Threshold: > 0 = banned

4. **Return Complexity**: Lines in function / return statements
   - Threshold: < 3 = too simple

5. **Error Handling**: try/catch or ? operators
   - Threshold: 0 = incomplete

## Implementation Layers

### Layer 1: Core Infrastructure (DONE)
- ✅ QEMU plugin (Rust)
- ✅ Reachability tracer
- ✅ Harmonic filter
- ✅ Homotopy classifier

### Layer 2: Real Integration (NOW)
- [ ] Actual QEMU traces (not mocked)
- [ ] Real file I/O (not test data)
- [ ] Complete error handling
- [ ] Full database integration (LMFDB, OEIS, Wikidata)

### Layer 3: Production Pipeline (NEXT)
- [ ] Nix builds with real tests
- [ ] Lean4 proofs from actual analysis
- [ ] CI/CD with real metrics
- [ ] Performance benchmarks

### Layer 4: Verification (FINAL)
- [ ] All code passes fake detector
- [ ] All functions have real implementations
- [ ] All data from actual sources
- [ ] All proofs verified

## Enforcement

### Pre-commit Hook
```bash
#!/bin/bash
# Reject commits with fake patterns
if grep -r "mock\|fake\|demo\|placeholder\|TODO" --include="*.rs" .; then
    echo "❌ REJECTED: Contains fake/demo code"
    exit 1
fi
```

### CI Check
```yaml
- name: Detect fake code
  run: |
    cargo build --release --bin fake_detector
    ./target/release/fake_detector src/
    if [ $? -ne 0 ]; then
      echo "❌ Fake code detected"
      exit 1
    fi
```

## Migration Plan

### Phase 1: Audit (Week 1)
- Run fake detector on all code
- Tag all fake/demo code
- Create issues for each

### Phase 2: Replace (Week 2-4)
- Replace mocks with real implementations
- Remove all demo branches
- Integrate actual data sources

### Phase 3: Verify (Week 5)
- All tests use real data
- All functions fully implemented
- Zero fake detector warnings

### Phase 4: Lock (Week 6)
- Enable pre-commit hooks
- Enforce in CI
- Document real implementations

## Exceptions

**NONE.**

If something cannot be implemented fully, it should not be committed.

## Success Criteria

- [ ] Fake detector score: 0/100 (zero fake code)
- [ ] All functions > 10 lines (no trivial stubs)
- [ ] All data from files/network (no hardcoded)
- [ ] All errors handled (no unwrap without justification)
- [ ] All tests use real scenarios (no mock data)

## Review Checklist

Before merging:
- [ ] No constants except configuration
- [ ] No "demo", "mock", "fake", "test" in names
- [ ] No placeholder returns
- [ ] No simplified implementations
- [ ] Full error handling
- [ ] Real data sources documented
- [ ] Integration tests pass with real data

## Commitment

**We build production systems, not demos.**

Every line of code must be production-ready or it doesn't get committed.
