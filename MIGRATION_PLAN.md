# Demo2Code Migration Plan

## Current State Audit

### Files to Review
```bash
# Find all demo/mock code
grep -r "demo\|mock\|fake" --include="*.rs" . | wc -l

# Find trivial functions
grep -A 3 "^fn " **/*.rs | grep -B 3 "^}" | wc -l

# Find hardcoded values
grep -r "= 42\|= 123\|\"test\"\|\"example\"" --include="*.rs" .
```

### Current Violations

#### High Priority (Block Merge)
- [ ] `demo_*.rs` files - Remove or make production-ready
- [ ] Mock data in tests - Replace with real data
- [ ] Placeholder returns - Implement fully
- [ ] TODO/FIXME comments - Resolve or remove

#### Medium Priority (Fix in Sprint)
- [ ] Hardcoded constants - Move to config
- [ ] Trivial functions - Expand or inline
- [ ] Missing error handling - Add ? operators
- [ ] Simplified implementations - Complete

#### Low Priority (Technical Debt)
- [ ] Magic numbers - Document or extract
- [ ] Test-only code - Separate or remove
- [ ] Unused code - Delete

## Migration Strategy

### Week 1: Audit & Tag
```bash
# Run detector on all code
cargo build --release --bin fake_detector
./target/release/fake_detector src/ > audit_report.txt

# Create issues for each violation
# Tag with "demo2code" label
```

### Week 2-3: Core Replacements

#### Replace Demo Files
```bash
# For each demo_*.rs:
# 1. Identify real use case
# 2. Implement production version
# 3. Add integration tests with real data
# 4. Delete demo file
```

#### Real Data Integration
```rust
// Before (BANNED)
let data = vec![1, 2, 3];

// After (REQUIRED)
let data = fs::read("data/input.bin")?;
```

#### Complete Implementations
```rust
// Before (BANNED)
fn analyze() -> Result<String> {
    Ok("placeholder".to_string())
}

// After (REQUIRED)
fn analyze(input: &[u8]) -> Result<Analysis> {
    let trace = qemu_trace(input)?;
    let clusters = cluster_tests(&trace)?;
    let harmonics = compute_harmonics(&clusters)?;
    let classification = classify_homotopy(&harmonics)?;
    Ok(classification)
}
```

### Week 4: Integration Testing

#### Real QEMU Traces
```bash
# No more mock traces
# All tests use actual QEMU plugin
qemu-x86_64 -plugin libreachability_rust.so rustc test.rs
```

#### Real Database Queries
```rust
// No more fake lookups
// Query actual LMFDB, OEIS, Wikidata
let lmfdb_result = query_lmfdb(level, weight, conductor).await?;
```

#### Real Lean4 Proofs
```bash
# No more placeholder proofs
# Generate and verify actual theorems
lean --make generated_proof.lean
```

### Week 5: Verification

#### Run Fake Detector
```bash
./target/release/fake_detector src/
# Must return: ✅ POLICY COMPLIANT
```

#### CI Must Pass
```bash
# All checks green
- Fake detector: ✅
- Pattern check: ✅
- Error handling: ✅
- Integration tests: ✅
```

#### Manual Review
- [ ] No demo files remain
- [ ] All data from real sources
- [ ] All functions fully implemented
- [ ] All errors handled properly

### Week 6: Lock Down

#### Enable Enforcement
```bash
# Pre-commit hook active
git commit  # Will reject fake code

# CI enforcement active
# PRs with fake code auto-rejected
```

#### Documentation
- [ ] Update README with real examples
- [ ] Document all data sources
- [ ] Remove demo instructions
- [ ] Add production deployment guide

## File-by-File Plan

### Remove Entirely
- `demo_*.rs` - All demo files
- `*_mock.rs` - All mock implementations
- `test_data.rs` - Hardcoded test data

### Rewrite Completely
- `reach_tracer.rs` - Remove mock data, use real QEMU
- `source2test.rs` - Remove example data
- `harmonic_filter.rs` - Remove demo signatures
- `homotopy_classifier.rs` - Real database queries

### Enhance
- `qemu-plugin/src/lib.rs` - Already production-ready ✅
- `intrinsic_complexity.rs` - Remove mock compression
- `fake_detector.rs` - Already production-ready ✅

## Success Metrics

### Code Quality
- Fake detector score: 100/100
- Zero banned patterns
- Error handling ratio: > 1.0
- Function complexity: > 5 lines average

### Functionality
- All QEMU traces real
- All database queries real
- All Lean4 proofs verified
- All tests use production data

### Process
- Pre-commit hook: 100% enforcement
- CI: 100% pass rate
- Code review: Zero fake code merged
- Documentation: Production-ready

## Rollout

### Phase 1: Soft Launch (Week 1-2)
- Warnings only
- Educate team
- Fix obvious violations

### Phase 2: Hard Enforcement (Week 3-4)
- Block commits with fake code
- Require fixes before merge
- Daily fake detector runs

### Phase 3: Zero Tolerance (Week 5+)
- Auto-reject PRs with violations
- Revert commits that slip through
- Public shame for fake code 😄

## Communication

### Team Announcement
```
🚨 NEW POLICY: Demo2Code

Starting today, we no longer accept:
- Demo files
- Mock data
- Fake implementations
- Placeholder code

All code must be production-ready.

See DEMO2CODE_POLICY.md for details.
```

### PR Template Update
```markdown
## Demo2Code Checklist
- [ ] No demo/mock/fake patterns
- [ ] Real data sources documented
- [ ] Full error handling
- [ ] Fake detector passes
```

## Support

### Questions?
- Read: DEMO2CODE_POLICY.md
- Ask: #demo2code channel
- Tool: fake_detector

### Need Help?
- Pair programming sessions
- Code review assistance
- Architecture guidance

## Timeline

```
Week 1: Audit complete
Week 2: 50% violations fixed
Week 3: 80% violations fixed
Week 4: 95% violations fixed
Week 5: 100% compliant
Week 6: Enforcement locked
```

## Commitment

**We build production systems, not demos.**

Every line of code is production-ready or it doesn't exist.
