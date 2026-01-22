# Perf Recording Centralization Plan

## 🎯 Goal

## 📊 Current Perf Record Usage

### Shell Scripts (Found 11)
1. `perf_nix_rust_beta.sh` - Nix Rust beta build
2. `perf_nix_rustc_build.sh` - Nix rustc build
3. `record_nix_build_with_probes.sh` - Nix build with probes
4. `record_rustc_build.sh` - Rustc build
5. `record_rustc_simple.sh` - Simple rustc recording
6. `perf_rank_symbols.sh` - Symbol ranking
7. `extract_cost_attribution.sh` - Cost attribution per language
8. `prove_71_equivalence.sh` - 71 equivalence proof
9. `test_const_equivalence.sh` - Const equivalence test
10. `test_const_equivalence_nix.sh` - Nix const equivalence
11. `build_and_analyze_const71.sh` - Build and analyze const 71

### Rust Tools (Found 3)
1. `rust_perf_decoder.rs` - Parse perf.data with linux-perf-data
2. `binary_symbol_study.rs` - Extract binaries from perf.data
3. `cost_attribution_71.rs` - Cost attribution analyzer

## 🔧 New Canonical Tool

### `perf_canonical_recorder.rs`

**Features**:
- ✅ Standardized JSON output format
- ✅ Session types: nix, rustc, cargo, binary, custom
- ✅ Automatic report generation
- ✅ Binary and library extraction
- ✅ Top 100 symbols with samples and percentages

**Output Structure**:
```json
{
  "session_id": "perf_nix_1768404986",
  "timestamp": 1768404986,
  "total_samples": 12345,
  "top_symbols": [
    {
      "symbol": "std::function<bool>",
      "samples": 462,
      "percentage": 3.75,
      "binary": "libnixstore.so"
    }
  ],
  "binaries": ["/nix/store/.../rustc", ...],
  "libraries": ["/nix/store/.../libnixstore.so", ...],
  "raw_report_path": "data/perf_canonical/perf_nix_1768404986_report.txt"
}
```

**Canonical Output Directory**:
```
data/perf_canonical/
  perf_nix_1768404986.perf.data      # Raw perf data
  perf_nix_1768404986_report.txt     # Text report
  perf_nix_1768404986.json           # Canonical JSON
```

## 🔄 Migration Plan

### Phase 1: Build and Test (Current)
- [x] Create `perf_canonical_recorder.rs`
- [ ] Add to Cargo.toml
- [ ] Test with simple command
- [ ] Test with nix build
- [ ] Test with rustc build

### Phase 2: Migrate Shell Scripts
Replace each shell script with canonical tool:

**Before**:
```bash
sudo perf report -i output.perf.data --stdio > report.txt
# Manual parsing...
```

**After**:
```bash
sudo perf_canonical_recorder nix nix build .#hello
# Automatic JSON output in data/perf_canonical/
```

### Phase 3: Update Rust Tools
- Update `rust_perf_decoder.rs` to read canonical JSON
- Update `binary_symbol_study.rs` to use canonical format
- Update `cost_attribution_71.rs` to consume canonical data

### Phase 4: Integration
- Feed canonical JSON to Bott[8] layout solver
- Map to 8D coordinates
- Visualize in unified system

## 📝 Canonical JSON Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "PerfReport",
  "type": "object",
  "required": ["session_id", "timestamp", "total_samples", "top_symbols"],
  "properties": {
    "session_id": {
      "type": "string",
      "pattern": "^perf_(nix|rustc|cargo|binary|custom)_[0-9]+$"
    },
    "timestamp": {
      "type": "integer",
      "description": "Unix timestamp"
    },
    "total_samples": {
      "type": "integer",
      "minimum": 0
    },
    "top_symbols": {
      "type": "array",
      "items": {
        "type": "object",
        "required": ["symbol", "samples", "percentage", "binary"],
        "properties": {
          "symbol": {"type": "string"},
          "samples": {"type": "integer", "minimum": 0},
          "percentage": {"type": "number", "minimum": 0, "maximum": 100},
          "binary": {"type": "string"}
        }
      }
    },
    "binaries": {
      "type": "array",
      "items": {"type": "string"}
    },
    "libraries": {
      "type": "array",
      "items": {"type": "string"}
    },
    "raw_report_path": {"type": "string"}
  }
}
```

## 🚀 Usage Examples

### Nix Build
```bash
sudo perf_canonical_recorder nix nix build .#hello
```

### Rustc Build
```bash
sudo perf_canonical_recorder rustc rustc --version
```

### Cargo Test
```bash
sudo perf_canonical_recorder cargo cargo test --release
```

### Binary Execution
```bash
sudo perf_canonical_recorder binary ./my_binary arg1 arg2
```

### Custom Command
```bash
sudo perf_canonical_recorder custom my-complex-command --with-args
```

## 📊 Output Files

All output goes to `data/perf_canonical/`:

```
data/perf_canonical/
├── perf_nix_1768404986.perf.data       # Raw perf data (binary)
├── perf_nix_1768404986_report.txt      # Human-readable report
├── perf_nix_1768404986.json            # Canonical JSON ← MAIN OUTPUT
├── perf_rustc_1768404987.perf.data
├── perf_rustc_1768404987_report.txt
├── perf_rustc_1768404987.json
└── ...
```

## 🔗 Integration Points

### 1. Bott[8] Layout Solver
```bash
# Record perf data
sudo perf_canonical_recorder nix nix build .#hello

# Map to 8D
python3 bott8-layout-solver/perf_to_8d_mapper.py \
  data/perf_canonical/perf_nix_*.json \
  data/perf_canonical/perf_nix_*_8d.json

# Solve layout
cd bott8-layout-solver
nix run .#solve
```

### 2. 71 Discovery Analysis
```bash
# Record 71 builds
for i in {1..71}; do
  sudo perf_canonical_recorder nix nix build .#test$i
done

# Analyze all 71
python3 analyze_71_perf_sessions.py data/perf_canonical/perf_nix_*.json
```

### 3. LMFDB Classification
```bash
# Record and classify
sudo perf_canonical_recorder nix nix build .#hello
python3 classify_symbols_lmfdb.py \
  data/perf_canonical/perf_nix_*.json \
  --levels 11,23,47,71
```

## ✅ Benefits

1. **Standardization** - All perf data in same format
2. **Automation** - No manual parsing needed
3. **Integration** - Easy to feed into other tools
4. **Versioning** - JSON schema can evolve
5. **Debugging** - Raw data + report + JSON all saved
6. **Reproducibility** - Session ID tracks everything

## 🎯 Next Steps

1. **Add to Cargo.toml** and build
2. **Test with simple command** (rustc --version)
3. **Test with nix build** (actual build)
4. **Migrate one shell script** as proof of concept
5. **Document migration** for remaining scripts
6. **Create integration tools** (perf_to_8d_mapper.py, etc.)

---

**Status**: Tool created, ready for testing
**Next**: Add to Cargo.toml and run first test
