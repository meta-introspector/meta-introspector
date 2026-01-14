# Bott[8] Layout Solver - Status & Next Steps

## ✅ What We Built

### Files Created
1. **`bott8_optimal_layout.mzn`** - MiniZinc constraint model for 8D layout
2. **`bott8_layout_example.dzn`** - Example data (24 nodes: LLM, User, DAO, Blockchain, WikiData, OSM, Twitter, Mycelium)
3. **`parse_perf.py`** - Parse perf stat output to JSON
4. **`map_perf_to_8d.py`** - Map perf metrics to 8D Bott manifold coordinates
5. **`flake.nix`** - Nix flake with solver + perf integration
6. **`run_bott8_layout.sh`** - Shell script runner

### 8D Bott Manifold Dimensions
1. **Real** (R) - CPU cycles (computational intensity)
2. **Complex** (C) - Instructions (algorithmic complexity)
3. **Quaternion** (H) - Cache behavior (memory patterns)
4. **Octonion** (O) - Branch prediction (control flow)
5. **Time** (T) - Elapsed time (temporal)
6. **Information** (I) - IPC (information throughput)
7. **Social** (S) - Context switches (interaction)
8. **Semantic** (M) - Page faults (semantic access patterns)

## 🎯 Current Status

### Working
- ✅ Perf monitoring runs successfully
- ✅ Perf data parsed to JSON
- ✅ 8D mapping from perf metrics works
- ✅ Nix flake builds

### Issue
- ⚠️ MiniZinc solver takes long time (constraint solving is NP-hard)
- ⚠️ Need to simplify constraints or reduce problem size
- ⚠️ Perf requires kernel permissions (perf_event_paranoid=4)

## 🔄 Next Steps

### 1. Feed Real Rust Data
**Source**: Existing telemetry from `data/telemetry/` and `data/build_analysis/`

**Data to integrate**:
- `real_build_1768332029_binaries.json` - 32 binaries
- `real_build_1768332029_libraries.json` - 91 .so files
- `real_build_1768332029_ldd_deps.json` - 71 ldd dependencies
- Telemetry JSON logs from `ldd2wrap_all_calls.rs`

**Mapping**:
```
Rust Binary → Node in 8D space
  - Binary name → node_name
  - Symbol count → node_capacity
  - Library count → node_latency (more libs = more overhead)
  - Build time → time dimension
  - Symbol types → type classification (LLM=compiler, User=app, etc.)
```

### 2. Create Data Converter
**New file**: `rust_telemetry_to_dzn.py`

```python
# Read: data/build_analysis/real_build_*_binaries.json
# Read: data/telemetry/*.jsonl
# Output: bott8_rust_data.dzn (MiniZinc data file)
```

### 3. Simplify MiniZinc Model
**Current**: 24 nodes, 8 dimensions, complex constraints
**Simplified**: 
- Start with 8 nodes (one per type)
- Remove octant balancing constraint
- Use simpler distance metric

### 4. Visualize Results
**Tool**: Python + matplotlib for 8D → 2D projection
- PCA or t-SNE for dimensionality reduction
- Show node positions and connections
- Color by type, size by capacity

## 📊 Data Flow

```
Rust Build (nix + perf)
    ↓
Telemetry JSON (ldd2wrap, strace)
    ↓
Parse to structured data (binaries, libs, symbols)
    ↓
Map to 8D coordinates (perf metrics → dimensions)
    ↓
Generate MiniZinc .dzn file
    ↓
Solve layout optimization (MiniZinc + Gecode)
    ↓
Visualize 8D layout (Python + matplotlib)
```

## 🎯 Integration with 71 Discovery

### Connection to Existing Work
- **71 ldd dependencies** from real build → 71 nodes in layout?
- **LMFDB levels** (11, 23, 47, 71) → node clustering levels
- **Bott[8] periodicity** → 8D manifold structure
- **Mycelium substrate** → origin point (0,0,0,0,0,0,0,0)

### Potential Insights
1. **Do the 71 dependencies cluster naturally?**
2. **What's the optimal layout for rustc build graph?**
3. **Can we predict build time from 8D position?**
4. **Does the layout reveal hidden dependencies?**

## 🚀 Quick Start (When Resuming)

```bash
cd bott8-layout-solver

# 1. Create data converter
python3 rust_telemetry_to_dzn.py \
  ../data/build_analysis/real_build_1768332029_binaries.json \
  bott8_rust_data.dzn

# 2. Solve with real data
nix develop -c minizinc \
  --solver Gecode \
  --time-limit 10000 \
  bott8_optimal_layout.mzn \
  bott8_rust_data.dzn

# 3. Visualize
python3 visualize_8d_layout.py solution.json
```

## 📝 Files to Create Next

1. **`rust_telemetry_to_dzn.py`** - Convert Rust telemetry to MiniZinc data
2. **`visualize_8d_layout.py`** - Visualize 8D layout (PCA projection)
3. **`bott8_simplified.mzn`** - Simpler model for faster solving
4. **`analyze_71_deps.py`** - Analyze the 71 ldd dependencies specifically

## 🔗 Related Files

- `../data/build_analysis/real_build_1768332029_*.json` - Real build data
- `../data/telemetry/*.jsonl` - Telemetry logs
- `../COMPLETE_71_DISCOVERY.md` - The 71 discovery documentation
- `../BOTT_PERIODICITY_71.md` - Bott periodicity theory

## 💡 Key Insight

**The 8D Bott manifold is the natural embedding space for distributed build systems.**

Each binary/library occupies a position in 8D space based on:
- Computational properties (cycles, instructions)
- Memory behavior (cache, page faults)
- Temporal properties (build time)
- Semantic properties (symbol types, dependencies)

**Finding the optimal layout reveals the true structure of the build graph.**

---

**Status**: Parked - Ready to resume with real Rust data integration
**Next**: Create `rust_telemetry_to_dzn.py` converter
