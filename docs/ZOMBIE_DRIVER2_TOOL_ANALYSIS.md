# Zombie Driver2 Tool Analysis

## Tools Using librustc_driver.so (101 files)

### Binary Analysis Tools
These tools analyze the 2.8GB rustc_driver.so binary directly:

#### **address_learning_system.rs**
- **Purpose**: Learn function addresses from rustc_driver.so
- **Method**: Uses goblin ELF parser to extract symbol addresses
- **Output**: Address mappings for function hooking

#### **call_graph_exporter.rs** 
- **Purpose**: Export function call graphs from compiled rustc
- **Method**: Analyzes ELF symbols and generates call relationship data
- **Output**: JSON call graphs to `/home/mdupont/nix/vendor/rust/cargo2nix/submodules/mycelial-usage-data/new-batch/call_graph`

#### **bytecode_decoder_discovery.rs**
- **Purpose**: Discover bytecode patterns in rustc_driver.so
- **Method**: Binary analysis to find instruction patterns
- **Output**: Bytecode instruction mappings

### Frequency Analysis Tools
These analyze code patterns for next-char/word prediction:

#### **frequency_analysis.rs**
- **Purpose**: Rust AST frequency analysis for prediction models
- **Input**: `rust_eigenmatrix.json` 
- **Method**: Statistical analysis of AST node frequencies
- **Output**: Character/word frequency data for ML models

#### **ngram_monster_phi.rs**
- **Purpose**: N-gram analysis with phi coefficients
- **Method**: Mathematical analysis using Monster Group theory
- **Output**: N-gram patterns for next-word prediction

### Compiler Integration Tools
These hook into the actual compilation process:

#### **live_rustc_caller.rs**
- **Purpose**: Direct integration with rustc compiler functions
- **Method**: Dynamic loading of rustc_driver.so functions
- **Output**: Real-time compilation data capture

#### **cargo_hijack_system.rs**
- **Purpose**: Hook into cargo build system
- **Method**: Intercept cargo compilation calls
- **Output**: Build process analysis data

## Working vs Broken Analysis

### ✅ **Working Tools** (Direct Binary Analysis)
These tools work because they directly analyze the existing 2.8GB rustc_driver.so:
- `address_learning_system.rs` - Reads ELF symbols
- `call_graph_exporter.rs` - Exports function relationships  
- `bytecode_decoder_discovery.rs` - Analyzes binary patterns
- `frequency_analysis.rs` - Processes existing eigenmatrix data

### ❌ **Potentially Broken** (Build Dependencies)
These tools may have issues because they depend on build processes:
- `syn_moonshine/build.rs` - Tries to copy/instrument rustc_driver.so
- `build_syn_so.sh` - Depends on syn_moonshine build
- `analyze_syn_ast.sh` - Depends on syn.so being built

## Key Insight
The **analysis tools are already working** - they directly process the 2.8GB rustc_driver.so that exists. The issue is only with the **build system** that tries to create syn.so.

## Recommendation
1. **Use existing working tools first**: `frequency_analysis.rs`, `call_graph_exporter.rs`
2. **Skip syn.so build for now** - focus on tools that work with existing binaries
3. **Test direct analysis** on our 954K Rust files using working tools

## File Categories Summary

| Category | Count | Status | Purpose |
|----------|-------|--------|---------|
| Binary Analyzers | 30+ | ✅ Working | Analyze rustc_driver.so directly |
| Frequency Tools | 15+ | ✅ Working | Text/AST frequency analysis |
| Export Tools | 20+ | ✅ Working | Data export (HuggingFace, Parquet) |
| Build Tools | 10+ | ❌ Issues | Build syn.so and instrumentation |
| Integration Tools | 25+ | ❓ Unknown | Runtime compiler hooks |

**Bottom Line**: We have 65+ working analysis tools ready to use on our 954K Rust files without needing to fix the build system first.
