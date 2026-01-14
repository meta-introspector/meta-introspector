# Markov Resonance Analysis - Complete Summary

## Overview
Comprehensive analysis of 34,506 ELF binaries from nix store using Markov resonance patterns, eigenvector centrality, and auto-labeling.

## Data Collection

### Input
- **34,506 ELF files** from `/nix/store`
- **26,383 valid binaries** (8,123 failed - scripts, non-ELF)
- **912,524 symbols** extracted

### Processing
- **48 workers** (2x 24 CPU cores)
- **20GB shared memory budget** (0.30GB used)
- **Parquet format**: 106MB (vs 211MB JSON, 50% smaller)

## Analysis Layers

### 1. File-Level Similarity Matrix
- **26383×26383 matrix** (5.2GB binary format)
- Cosine similarity of resonance vectors
- **Dominant eigenvalue**: 20,589.24 (converged in 10 iterations)

**Top files by centrality:**
- iptables modules (libxt_*.so, libip6t_*.so)
- glibc encodings (ISO8859-14, ISO-2022-JP)
- glibc vector math (libmvec.so.1)
- Serialization libs (libcbor.so)

### 2. Symbol-Level Similarity
- **500×500 symbol matrix** for top symbols
- **Dominant eigenvalue**: 269.81 (converged in 8 iterations)

**Top symbols by centrality:**
- Rust tracing_core callsite metadata (70-105 files)
- util-linux string parsing (ul_strtos32, ul_parse_size)
- SANE scanner library (sanei_usb_*, sanei_xml_*)
- md5_stream, string conversion utilities

### 3. Markov Name/Path Analysis
- **269K unique bigrams**, **1.2M trigrams** from symbol names
- **Top patterns**: `__z`, `e_e`, `cold__`, `z_n` (C++ mangling)
- **Path components**: nix/store/lib dominates (709K symbols)

### 4. Auto-Labeling System

**17 semantic labels:**
1. `shared_library` - 419,423 symbols (46%)
2. `rust_mangled` - 194,343 symbols (21%)
3. `other` - 115,397 symbols (13%)
4. `glibc` - 49,984 symbols (5%)
5. `elf_lifecycle` - 34,094 symbols (4%)
6. `rust_alloc` - 32,410 symbols (4%)
7. `rust_std` - 19,165 symbols (2%)
8. `rust_panic` - 16,394 symbols (2%)
9. `crypto` - 10,916 symbols (1%)
10. `sane_scanner` - 7,467 symbols (0.8%)
11. `plugin_system` - 4,076 symbols (0.4%)
12. `util_linux` - 3,870 symbols (0.4%)
13. `xml_parser` - 1,627 symbols
14. `rust_tracing` - 1,161 symbols
15. `string_util` - 831 symbols
16. `usb_driver` - 792 symbols
17. `stack_protection` - 574 symbols

### 5. Eigenvector Cluster Labels

**Top 50 eigenvector symbols by label:**
- **26%** SANE scanner library (sanei_usb_*, sanei_xml_*)
- **24%** other utilities (string conversion, path handling)
- **22%** util-linux (ul_parse_*, ul_str*)
- **12%** Rust tracing infrastructure
- **8%** Rust panic/alloc/mangled
- **4%** shared library utilities
- **2%** crypto (md5_stream)

## Key Insights

### Cross-Cutting Concerns
Eigenvector centrality reveals libraries that appear consistently across many binaries:
1. **SANE scanner library** - USB/XML handling for scanners
2. **util-linux utilities** - String parsing and conversion
3. **Rust tracing** - Instrumentation infrastructure
4. **Crypto functions** - md5_stream appears in 90 files

### Symbol Patterns
- **1,653 symbols** appear in 1000+ files
- `__stack_chk_fail` (1,653 files) - stack protection
- `_fini/_init` (1,653 files) - ELF constructors/destructors
- Rust std symbols cluster together (0.87-1.0 similarity)

### Binary Structure
- **iptables modules** have highest file-level centrality
- **glibc encodings** represent "typical" binary patterns
- **Rust binaries** create consistent symbol patterns

## Output Files

### Data Files
- `markov_symbol_scores.parquet` (106MB) - All symbols with scores
- `markov_symbol_scores.json` (211MB) - JSON format for compatibility
- `markov_similarity_matrix.bin` (5.2GB) - File similarity matrix
- `markov_file_index_mapping.json` - Index to filename mapping
- `failed_files_exclude.txt` - 8,123 non-ELF files to skip

### Analysis Results
- `markov_dominant_eigenvector.txt` - File-level eigenvector
- `symbol_eigenvector_results.txt` - Symbol-level eigenvector
- `symbol_similarity_results.txt` - Symbol pair similarities
- `markov_name_path_analysis.txt` - Markov patterns
- `eigenvector_label_mapping.txt` - Cluster labels

## Tools

### Analyzers
- `markov_resonance_analyzer` - Main analysis tool (48 workers, 20GB budget)
- `eigenvector_calculator` - File-level eigenvector computation
- `symbol_similarity` - Symbol-level analysis suite
  - `symbol_eigenvector` - Symbol eigenvector computation
  - `markov_labels` - Name/path Markov analysis
  - `label_mapper` - Eigenvector cluster labeling

### Performance
- **Full analysis**: ~6 minutes for 34,506 files
- **Parquet loading**: Instant (vs minutes for JSON)
- **Eigenvector convergence**: 8-10 iterations
- **Memory efficiency**: 0.30GB / 20GB budget used

## Interpretation

The analysis reveals a **hierarchical structure** in the nix store binaries:

1. **Foundation layer**: glibc, ELF lifecycle, stack protection (universal)
2. **Infrastructure layer**: Rust std/tracing, util-linux utilities (cross-cutting)
3. **Domain layer**: SANE scanners, iptables, crypto (specialized)
4. **Application layer**: Individual binaries using above layers

Eigenvector centrality identifies the **infrastructure layer** - components that:
- Appear in many binaries
- Have similar resonance patterns
- Represent reusable abstractions
- Form the "backbone" of the binary ecosystem
