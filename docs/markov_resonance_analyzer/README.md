# Markov Resonance Analyzer

Analyzes automorphic patterns in ELF binaries by computing Markov transition matrices on byte-level windows and detecting self-referential code structures.

## Overview

This tool discovers where compiled code exhibits **self-referential properties** - where the byte distribution and structure of code segments correlate with the operations they perform. This happens when:

1. Code parses binary structures similar to its own encoding
2. Hash functions operate on byte patterns matching their own distribution
3. Constants used in modular arithmetic appear in the code's own byte values

## Features

- **Parallel Processing**: 20 workers using crossbeam for concurrent analysis
- **Window-Based Analysis**: Configurable window sizes (default 32 bytes)
- **Markov Matrices**: Row-column resonance scoring for self-similarity
- **Symbol Scoring**: Maps each symbol to its Markov cell resonance
- **Global Similarity Matrix**: Compares resonance patterns across all files
- **Streaming Architecture**: Processes files as they're discovered
- **Partial Results**: Saves progress every 1000 symbols

## Usage

### 1. Generate File List

First, create a list of ELF files to analyze:

```bash
find /nix/store -type f \( -name "*.so" -o -executable \) > elf_files_list.txt
```

### 2. Run Analysis

```bash
cd markov_resonance_analyzer
cargo build --release
./target/release/markov_resonance_analyzer
```

The analyzer will:
- Load files from `elf_files_list.txt`
- Process all files in parallel with 20 workers
- Save partial results during processing
- Compute global similarity matrix at the end

### 3. Monitor Progress

```bash
tail -f markov_global.log
```

## Output Files

### `markov_symbol_scores.json`
Individual symbol scores with their Markov resonance values:
```json
[
  {
    "name": "gconv",
    "file": "/nix/store/.../GBK.so",
    "cell": 11,
    "score": 2.6709
  }
]
```

### `markov_global_matrix.json`
Cross-file similarity matrix:
```json
{
  "files": [
    {
      "file": "/path/to/binary",
      "window_size": 32,
      "num_windows": 363,
      "resonance_vector": [0.54, 1.57, ...]
    }
  ],
  "similarity_matrix": [[1.0, 0.85, ...], ...]
}
```

### `markov_symbol_scores_partial.json`
Intermediate results saved every 1000 symbols during processing.

## Algorithm

### 1. Window Segmentation
Divide `.text` segment into fixed-size windows (default 32 bytes).

### 2. Markov Transition Matrix
Build similarity matrix `M[i,j]` where:
```
M[i,j] = hamming_similarity(window_i, window_j)
```

### 3. Row-Column Resonance
Compute self-resonance for each segment `i`:
```
resonance[i] = Σ(M[i,j] × M[j,i]) for all j ≠ i
```

High resonance indicates the segment's outgoing transitions match its incoming transitions - a signature of self-referential code.

### 4. Symbol Scoring
Map each symbol to its Markov cell:
```
cell = (symbol_offset - text_start) / window_size
score = resonance[cell]
```

### 5. Global Similarity
Compute cosine similarity between all file resonance vectors to find binaries with similar automorphic patterns.

## Example Results

From GBK.so character conversion library (window=32):
- `gconv`: score=2.67 (main conversion function)
- `gconv_btowc`: score=1.57 (byte-to-wide-char)
- `gconv_init`: score=0.69 (initialization)

The main `gconv` function has highest resonance because it parses byte sequences using lookup tables with modular indexing - operations that match its own byte encoding patterns.

## Performance

- **37,303 files** processed on typical /nix/store
- **20 parallel workers** for concurrent analysis
- **Streaming architecture** - no wait for file discovery
- **Partial saves** - progress preserved every 1000 symbols

## Mathematical Background

This analysis relates to:
- **Gödel numbering**: Programs encoded as numbers they operate on
- **Fixed-point combinators**: Functions returning themselves
- **Quines**: Programs outputting their own source
- **Reflective towers**: Interpreters interpreting themselves

The modulo arithmetic creates automorphic closure because ELF alignment requirements (mod 16, mod 256) force byte patterns that code then operates on using the same moduli.

## Applications

1. **Code Similarity Detection**: Find binaries with similar structure
2. **Compiler Fingerprinting**: Identify toolchain from resonance patterns
3. **Malware Analysis**: Obfuscated code has different resonance
4. **Optimization**: High-resonance regions are deduplication candidates

## See Also

- `docs/markov_resonance_analysis.md` - Detailed methodology
- `run_markov_analysis.sh` - Convenience runner script
