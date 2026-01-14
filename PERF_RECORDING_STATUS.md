# Perf Recording Status - Current State

## ✅ What's Working

### 1. Data Captured
**Location**: `data/perf_rankings/`
- ✅ **15 MB perf.data** from nix rust beta build
- ✅ **5-6 MB perf.data** from rustc builds (multiple)
- ✅ Symbol rankings and reports generated

### 2. Tools Built
**Recording**:
- ✅ `perf_canonical_recorder.rs` - Centralized recorder
- ✅ 11 shell scripts identified for migration

**Reading**:
- ✅ `rust_perf_decoder.rs` - Parse with linux-perf-data
- ✅ `binary_symbol_study.rs` - Extract binaries + symbols
- ⚠️ Need canonical reader (designed, not implemented)

**Instrumentation**:
- ✅ `#[perf_auto]` macro - Zero-overhead telemetry
- ✅ `#[perf_probe]` macro - Parquet capture
- ✅ `perf!()` and `probe!()` - Inline capture

### 3. Integration
- ✅ `nix_canonical_builder.rs` - Automatic perf during nix builds
- ✅ Trait-based executors (Shell + .so)
- ⚠️ Need to wire up automatic perf recording

## ⚠️ What's Missing

### 1. Automatic Perf Recording
**Issue**: `nix_canonical_builder` doesn't automatically start perf yet

**Solution**:
```rust
impl NixCanonicalBuilder {
    pub fn build(&self, request: NixBuildRequest) -> Result<NixBuildResult, String> {
        // Start perf recording
        let perf_session = if self.perf_enabled {
            Some(start_perf_recording())
        } else {
            None
        };
        
        // Execute nix build
        let result = self.executor.execute(&request)?;
        
        // Stop perf and save
        if let Some(session) = perf_session {
            save_perf_data(session, &result)?;
        }
        
        Ok(result)
    }
}
```

### 2. Canonical Perf Reader
**Status**: Designed but not implemented

**Need**:
- Single entry point for reading perf.data
- Export to JSON/Parquet
- MiniZinc .dzn format
- Batch analysis for 71 experiments

### 3. 71 Experiments
**Status**: Not started

**Plan**:
```rust
for i in 1..=71 {
    let result = nix_build_flake(&format!(".#experiment{}", i))?;
    // Perf data automatically captured
}
```

## 🎯 Next Steps

### Priority 1: Wire Up Automatic Perf
1. Add perf recording to `nix_canonical_builder.rs`
2. Test with simple build
3. Verify perf.data is captured

### Priority 2: Implement Canonical Reader
1. Create `perf_canonical_reader.rs`
2. Migrate `rust_perf_decoder.rs`
3. Add MiniZinc export

### Priority 3: Run 71 Experiments
1. Define 71 test packages
2. Run all with perf
3. Analyze results
4. Feed to Bott[8] solver

## 📊 Current Data

**Captured**:
- 15 MB perf.data (nix rust beta)
- 462 samples in `std::function<bool>` (hottest path)
- 71 ldd dependencies identified

**Processed**:
- Symbol rankings
- LMFDB classification
- Markov patterns

**Ready for**:
- MiniZinc export
- 8D visualization
- 71 experiments

## 🚀 Quick Test

```bash
# Test current perf recording
sudo ./perf_nix_rust_beta.sh

# Check output
ls -lh data/perf_rankings/*.perf.data

# Analyze
cargo run --bin rust_perf_decoder -- data/perf_rankings/*.perf.data
```

---

**Status**: Tools built, data captured, need to wire up automation
**Next**: Add automatic perf to nix_canonical_builder
