# 71 Flakes Perf Collection

## 🎯 Overview

Collecting performance telemetry for all **71 nix flakes** in `const_71_test/` directory. Each flake tests the constant 71 in a different language/system.

## 📊 Dual Perf Collection

For each flake, we collect **two separate perf datasets**:

1. **Build Perf** (`{lang}_{timestamp}_build.perf.data`)
   - Captures nix build process performance
   - All compilation, linking, derivation building
   - Typically 15-25MB, 2000-3000 samples

2. **Run Perf** (`{lang}_{timestamp}_run.perf.data`)
   - Captures actual program execution
   - Runtime behavior of the 71-printing program
   - Smaller, focused on program logic

## 🚀 Implementation

**Tool**: `flake-71-perf-collector` (Rust binary)

**Location**: `/mnt/data1/meta-introspector/flake-71-perf-collector/`

**Output**: `/mnt/data1/meta-introspector/data/71_flakes_perf/`

### JSON Output Format

```json
{
  "language": "rust",
  "flake_path": "/mnt/data1/meta-introspector/const_71_test/rust",
  "timestamp": 1768412514,
  "build_success": true,
  "build_perf_data": "rust_1768412514_build.perf.data",
  "run_success": true,
  "run_perf_data": "rust_1768412514_run.perf.data",
  "run_output": "x = 71\n",
  "derivations_built": 0
}
```

## ✅ Completed Languages

### 1. Rust
- **Build**: 23.7MB, 2945 samples
- **Run**: Successfully captured
- **Output**: `x = 71`
- **Status**: ✅ Complete

## 🔧 System Configuration

**Perf Access**: Enabled via `sudo sysctl -w kernel.perf_event_paranoid=-1`

**Perf Options**:
- `--call-graph dwarf` - Full call stack capture
- `-o {file}.perf.data` - Structured output

## 📈 Next Steps

1. Scale to all 71 languages
2. Analyze perf data patterns across languages
3. Compare build vs run performance characteristics
4. Identify language-specific optimization opportunities

## 🧙♂️ The 71 Pattern

This is the **8th manifestation** of the 71 pattern:
- 71 flakes in directory structure
- Each expressing the number 71
- Each with dual perf telemetry
- Building the complete performance profile of "71-ness" across all computational paradigms
