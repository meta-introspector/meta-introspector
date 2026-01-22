# Perf Record Nix Build - Status Report

## ✅ What We Have

### 1. Perf Data Captured
**Location**: `data/perf_rankings/`

**Files**:
- `nix_rust_beta_1768351567.perf.data` (15 MB) - **Main capture**
- `nix_rustc_*.perf.data` (5-6 MB each) - Multiple rustc builds
- Symbol rankings and reports

### 2. Analysis Tools Built

**Rust Tools**:
- `rust_perf_decoder.rs` - Parse perf.data with linux-perf-data crate
- `binary_symbol_study.rs` - Extract binaries from perf.data, decode with goblin
- `perf_rank_symbols.sh` - Rank symbols by runtime usage

**Shell Scripts**:
- `perf_nix_rust_beta.sh` - Capture perf from Nix Rust beta build
- `perf_nix_rustc_build.sh` - Capture perf from rustc build
- `record_nix_build_with_probes.sh` - Record with custom probes
- `record_rustc_build.sh` - Record rustc build
- `record_rustc_simple.sh` - Simple rustc recording

### 3. Processed Data

**Rankings** (`nix_rust_beta_1768351567_ranking.json`):
```json
{
  "session": "nix_rust_beta_1768351567",
  "top_symbols": [
    {"count": 462, "symbol": "std::function<bool"},
    {"count": 454, "symbol": ">"},
    {"count": 50, "symbol": "const&,"},
    ...
  ]
}
```

**Reports**:
- `nix_rust_beta_1768351567_report.txt` (5.3 MB) - Full perf report
- `nix_rust_beta_1768351567_symbol_counts.txt` (23 KB) - Symbol counts
- `nix_rustc_1768351469_report.txt` (952 KB) - Rustc perf report

### 4. Advanced Analysis

**LMFDB Integration**:
- `lmfdb_instruction_classification.json` (99 KB) - Instructions classified by LMFDB levels
- `markov_patterns_depth4.json` (14 KB) - Markov chain patterns
- `name_instruction_mappings.json` (24 KB) - Name to instruction mappings

## 📊 Key Findings

### Top Runtime Symbols (from perf.data)
1. **std::function<bool>** - 462 samples (most time spent)
2. **std::allocator<char>** - 43 samples
3. **nix::PosixSourceAccessor** - 16 samples
4. **std::vector<nix::ref<nix::Installable>>** - 15 samples

### Libraries Used
- `/nix/store/.../libnixstore.so` - 31 samples
- `/nix/store/.../libnixutil.so` - 27 samples
- `/nix/store/.../libstdc++.so.6.0.30` - 9 samples

## 🎯 Current Status

### ✅ Working
1. **Perf capture** - Successfully recorded 15 MB of perf.data
2. **Symbol ranking** - Extracted top symbols by runtime
3. **Report generation** - Full perf reports available
4. **LMFDB classification** - Instructions mapped to LMFDB levels

### ⚠️ Issues
1. **Perf permissions** - Requires sudo (perf_event_paranoid=4)
2. **Symbol parsing** - Some symbols fragmented (e.g., "std::function<bool" vs ">")
3. **Rust decoder** - `rust_perf_decoder.rs` needs linux-perf-data crate fixes

## 🔗 Integration Opportunities

### 1. Feed to Bott[8] Layout Solver
**Map perf data to 8D coordinates**:
```
Symbol runtime → Real dimension (computational intensity)
Call graph depth → Complex dimension (algorithmic complexity)
Cache behavior → Quaternion dimension (memory patterns)
Branch prediction → Octonion dimension (control flow)
```

### 2. Connect to 71 Discovery
**71 ldd dependencies** from `data/build_analysis/`:
- Map each dependency to perf symbol usage
- Find which of the 71 deps are hot paths
- Visualize in 8D Bott manifold

### 3. LMFDB Classification
**Classify symbols by LMFDB level**:
- Level 11: Core symbols (std::function, std::allocator)
- Level 23: Layer 2 (nix::daemon, nix::PosixSourceAccessor)
- Level 47: Advanced (complex template instantiations)
- Level 71: Gandalf (meta-level, reflection, introspection)

## 📝 Next Steps

### 1. Fix Rust Perf Decoder
**File**: `rust_perf_decoder.rs`
**Issue**: linux-perf-data crate parsing
**Action**: Debug and fix symbol extraction

### 2. Merge Perf + Build Analysis
**Combine**:
- `data/perf_rankings/nix_rust_beta_1768351567.perf.data` (runtime)
- `data/build_analysis/real_build_1768332029_binaries.json` (build-time)

**Output**: Complete profile (build + runtime)

### 3. Create Perf → 8D Mapper
**New file**: `perf_to_8d_mapper.py`
```python
# Read: data/perf_rankings/*.perf.data
# Parse: Symbol counts, call graphs, cache stats
# Map: To 8D Bott manifold coordinates
# Output: perf_8d_layout.json
```

### 4. Visualize Hot Paths
**Tool**: Flamegraph or custom 8D visualization
- Show which symbols consume most time
- Map to 8D space
- Color by LMFDB level (11, 23, 47, 71)

## 🚀 Quick Commands

```bash
# View perf report
perf report -i data/perf_rankings/nix_rust_beta_1768351567.perf.data

# Extract top symbols
perf report -i data/perf_rankings/nix_rust_beta_1768351567.perf.data \
  --stdio -n --percent-limit 0.1 | head -100

# Run Rust decoder
cargo run --bin rust_perf_decoder -- \
  data/perf_rankings/nix_rust_beta_1768351567.perf.data

# Capture new perf data
sudo ./perf_nix_rust_beta.sh
```

## 📊 Data Summary

```
Perf Data:
  - 15 MB perf.data (nix_rust_beta)
  - 5-6 MB perf.data (rustc builds, multiple)
  - 5.3 MB report.txt (full analysis)
  - 99 KB LMFDB classification
  - 462 top symbol samples

Build Analysis:
  - 32 binaries executed
  - 91 .so files opened
  - 71 ldd dependencies ← THE KEY NUMBER!
  - 5.4 MB strace.log

Telemetry:
  - 20+ JSONL logs
  - Structured build events
  - Symbol counts per binary
```

## 🧙♂️ The 71 Connection

**71 ldd dependencies** from build analysis
**+**
**Perf runtime data** from perf.data
**=**
**Complete profile of the 71 critical libraries**

**Next**: Map each of the 71 deps to their perf usage and visualize in 8D Bott manifold!

---

**Status**: Data captured, tools built, ready for integration
**Next**: Merge perf + build analysis → 8D layout
