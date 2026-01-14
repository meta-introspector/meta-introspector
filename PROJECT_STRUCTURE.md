# Meta-Introspector Project Structure

## 📊 Overview

**Total Files**: 227 (68 markdown, 159 rust)
**Purpose**: Comprehensive analysis and instrumentation of Rust builds, nix systems, and performance data

## 🎯 Core Systems

### 1. The 71 Discovery System
**Purpose**: Seven convergent streams all pointing to 71

**Key Files**:
- `COMPLETE_71_DISCOVERY.md` - Complete documentation of 7 independent discoveries
- `71_IN_LMFDB.md` - LMFDB mathematical foundation
- `BOTT_PERIODICITY_71.md` - 10-periodic table theory
- `THE_71_OMEN.md` - Mathematical pattern as meme magic
- `BEYOND_71.md` - What lies beyond

**Pattern**: 11 → 23 → 47 → 71 (Genesis → Trinity → Completeness → Return)

### 2. Nix Build System
**Purpose**: Centralized, instrumented nix builds

**Key Files**:
- `nix_canonical_builder.rs` ⭐ - THE ONLY PLACE for nix builds
- `unified_nix_builder.rs` - Migrated to use canonical (27 lines)
- `nix_build_telemetry.rs` - Migrated (44 lines)
- `nix_telemetry_integration.rs` - Migrated (33 lines)
- `unified_nix_service.rs` - Async service (migrated)

**Documentation**:
- `NIX_BUILD_AUDIT.md` - Audit of 40 files calling nix
- `NIX_BUILD_INTEGRATION_PLAN.md` - Unified architecture
- `NIX_CENTRALIZATION_PROGRESS.md` - Migration progress (4/40 done)

**Progress**: 640 lines eliminated, 87% average reduction

### 3. Perf Recording System
**Purpose**: Capture and analyze performance data

**Key Files**:
- `perf_canonical_recorder.rs` ⭐ - Centralized perf recording
- `rust_perf_decoder.rs` - Parse perf.data with linux-perf-data
- `binary_symbol_study.rs` - Extract binaries + symbols with goblin

**Proc Macros**:
- `perf-macros/` - Proc macro crate
  - `#[perf_auto]` - Zero-overhead telemetry
  - `#[perf_probe]` - Parquet data capture
  - `perf!()` - Inline recording
  - `probe!()` - Inline parquet capture
- `perf-runtime/` - Runtime support

**Documentation**:
- `PERF_USAGE_AUDIT.md` - 11 shell scripts to migrate
- `PERF_CENTRALIZATION_PLAN.md` - Migration plan
- `PERF_RECORD_STATUS.md` - Data captured (15 MB)
- `PERF_RECORDING_STATUS.md` - Current state
- `PERF_READER_AUDIT.md` - Reader centralization
- `PERF_PROC_MACRO_DESIGN.md` - Macro design

**Data**: 15 MB perf.data, 462 samples in hottest path, 71 ldd dependencies

### 4. Bott[8] Layout Solver
**Purpose**: 8D constraint optimization for distributed systems

**Location**: `bott8-layout-solver/`

**Key Files**:
- `bott8_optimal_layout.mzn` - MiniZinc constraint model
- `bott8_layout_example.dzn` - Example data (24 nodes)
- `parse_perf.py` - Parse perf stat output
- `map_perf_to_8d.py` - Map perf metrics to 8D coordinates
- `flake.nix` - Nix integration

**Documentation**:
- `STATUS.md` - Current status and next steps
- `BOTT8_LAYOUT_README.md` - Usage guide

**8D Dimensions**: Real, Complex, Quaternion, Octonion, Time, Information, Social, Semantic

### 5. Telemetry System
**Purpose**: Centralized data collection

**Key Files**:
- `telemetry_lib/` - Core telemetry library
- `telemetry_server.rs` - Centralized server
- `telemetry_bootstrap.rs` - Bootstrap system
- `bootstrap_telemetry_system.rs` - Multi-layer collection

**Data Location**: `data/telemetry/*.jsonl`

### 6. Build Analysis
**Purpose**: Analyze real nix builds

**Key Files**:
- `real_build_analysis.sh` - Capture build with strace
- `analyze_real_build.py` - Parse strace output

**Data Location**: `data/build_analysis/`
- `real_build_1768332029_binaries.json` - 32 binaries
- `real_build_1768332029_libraries.json` - 91 .so files
- `real_build_1768332029_ldd_deps.json` - 71 dependencies ⭐
- `real_build_1768332029_strace.log` - 5.4 MB raw data

## 📁 Directory Structure

```
meta-introspector/
├── *.md (68 files)           # Documentation
├── *.rs (159 files)          # Rust source
├── perf-macros/              # Perf proc macros
│   ├── src/lib.rs
│   └── example/
├── perf-runtime/             # Perf runtime support
│   └── src/
├── bott8-layout-solver/      # 8D constraint solver
│   ├── *.mzn
│   ├── *.dzn
│   └── *.py
├── data/                     # Data storage
│   ├── telemetry/           # Telemetry logs
│   ├── perf_rankings/       # Perf data (15 MB)
│   ├── perf_canonical/      # Canonical perf output
│   ├── probes/              # Parquet probe data
│   ├── build_analysis/      # Build analysis (71 deps)
│   └── nix_builds/          # Nix build results
└── const_71_test/           # 71 language tests
```

## 🔧 Key Rust Programs

### Analysis Tools
- `crossbeam_rustc_analyzer_complete.rs` - 20-core parallel analyzer
- `semantic_signature_generator.rs` - 4-layer semantic analysis
- `duplicate_block_detector.rs` - Code duplication analysis
- `value_lattice_streaming.rs` - Memory-optimized streaming

### Compression Tools
- `grammar_rust_compressor.rs` - Sequitur compression (93% savings)
- `syn_compressor.rs` - AST-level compression (97% savings)
- `rustc_interceptor.rs` - Cargo build hijacking

### Build Tools
- `ldd2wrap_all_calls.rs` - Generate wrappers for all binaries
- `unified_nix_builder.rs` - Centralized nix builds
- `nix_canonical_builder.rs` - THE canonical builder

### Perf Tools
- `perf_canonical_recorder.rs` - Record perf data
- `rust_perf_decoder.rs` - Parse perf.data
- `binary_symbol_study.rs` - Extract symbols

## 📊 Data Summary

### Captured
- **15 MB perf.data** (nix rust beta build)
- **71 ldd dependencies** (real build analysis)
- **32 binaries executed** (vs 14 in old telemetry)
- **91 .so files opened** (vs 39 in old telemetry)
- **462 samples** in std::function<bool> (hottest path)

### Processed
- Symbol rankings
- LMFDB classification (99 KB)
- Markov patterns (14 KB)
- Binary symbols (26 KB)

### Generated
- Telemetry logs (JSONL)
- Parquet probe data
- Canonical JSON formats

## 🎯 Current Focus

### Active Work
1. **Nix centralization** - 4/40 files migrated (10%)
2. **Perf instrumentation** - Macros working, need automation
3. **71 experiments** - Ready to run
4. **Bott[8] visualization** - Solver ready

### Next Steps
1. Wire up automatic perf in nix_canonical_builder
2. Implement perf_canonical_reader
3. Run 71 experiments
4. Export to MiniZinc
5. Visualize in 8D

## 🧙♂️ The Pattern

**11 → 23 → 47 → 71**

- **11**: Genesis (first prime > 10)
- **23**: Trinity (stability)
- **47**: Completeness (Gandalf the Grey)
- **71**: Return (Gandalf the White) ← WE ARE HERE

**71 = 7 × 10 + 1 = Return after mastery**

## 📝 Documentation Files

### Discovery
- `COMPLETE_71_DISCOVERY.md` - The seven streams
- `THE_71_OMEN.md` - Meme interpretation
- `BOTT_PERIODICITY_71.md` - Mathematical foundation

### Systems
- `NIX_BUILD_AUDIT.md` - 40 files to centralize
- `PERF_USAGE_AUDIT.md` - 11 scripts to migrate
- `PERF_READER_AUDIT.md` - Reader centralization

### Progress
- `NIX_CENTRALIZATION_PROGRESS.md` - 4/40 done
- `PERF_RECORDING_STATUS.md` - Current state

### Architecture
- `AI_OBSERVABILITY_ARCHITECTURE.md` - Overall design
- `UNIVERSAL_BINARY_UNDERSTANDING.md` - Binary analysis
- `HARMONIC_UNIFICATION_PLAN.md` - Unification strategy

## 🚀 Quick Start

### Run Perf Recording
```bash
sudo ./perf_nix_rust_beta.sh
```

### Analyze Perf Data
```bash
cargo run --bin rust_perf_decoder -- data/perf_rankings/*.perf.data
```

### Build with Canonical Nix
```rust
use nix_canonical_builder::nix_build_flake;
let result = nix_build_flake(".#hello")?;
```

### Use Perf Macros
```rust
use perf_macros::{perf_auto, perf_probe};

#[perf_auto]
#[perf_probe]
fn my_function() {
    // Automatic perf + telemetry + parquet
}
```

---

**Status**: Systems built, data captured, ready for 71 experiments
**Next**: Run 71 experiments, visualize in Bott[8]
**Pattern**: 🧙♂️ = 71 = The Return = ∞
