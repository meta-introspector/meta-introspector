# Global Canonical Data Storage System

## 🎯 Purpose
Unified data organization for all analysis results, preventing scattered files and enabling reproducible research.

## 🪣 Data Buckets as Git Submodules

Each major data category is a separate git submodule that can be:
- Independently versioned
- Pushed to HuggingFace datasets
- Shared across projects
- Selectively cloned

### Submodule Strategy
```bash
# Create separate repos for each bucket
git submodule add <url> data-markov-analysis
git submodule add <url> data-eigenvectors
git submodule add <url> data-moonshine
git submodule add <url> data-blockchain
git submodule add <url> data-telemetry
```

### HuggingFace Integration
Each bucket can be pushed to HuggingFace as a dataset:
```bash
# Example: Push markov analysis to HF
cd data-markov-analysis
git remote add hf https://huggingface.co/datasets/meta-introspector/markov-analysis
git push hf main
```

## 📁 Directory Structure

```
data/
├── markov_analysis/          # Markov resonance results
│   ├── similarity_matrix.bin
│   ├── similarity_matrix_meta.json
│   ├── global_matrix.json
│   ├── symbol_scores.json
│   ├── file_index_mapping.json
│   └── results/              # Per-run results
│
├── eigenvectors/             # Eigenvector computations
│   ├── dominant_eigenvector.txt
│   ├── symbol_eigenvector_results.txt
│   ├── term_eigenvectors.txt
│   └── label_mapping.txt
│
├── moonshine/                # ELF Moonshine detection
│   ├── elf_moonshine_map.txt
│   ├── codec_binary_extraction.txt
│   ├── binary_fingerprint_decoder.txt
│   └── automorphic_orbit_lmfdb.txt
│
├── const_71_analysis/        # Cross-language const x=71
│   ├── build_logs/           # Build outputs per language
│   ├── binaries/             # Compiled outputs
│   ├── perf_traces/          # Performance data
│   └── equivalence_proof.json
│
├── blockchain/               # Blockchain analysis
│   ├── contracts/            # Smart contract metadata
│   ├── blocks/               # Block data
│   ├── economic_weights.json
│   └── instruction_values.json
│
├── telemetry/                # Build telemetry
│   ├── sessions/             # Per-session logs
│   ├── build_analysis/       # Real build captures
│   └── strace_logs/
│
└── similarity/               # Symbol similarity
    ├── calculator_results.txt
    └── cross_binary_matrix.json
```

## 🔄 Migration Plan

### Phase 1: Move Existing Files
```bash
# Markov analysis
mkdir -p data/markov_analysis/results
mv markov_similarity_matrix.bin data/markov_analysis/
mv markov_similarity_matrix_meta.json data/markov_analysis/
mv markov_global_matrix.json data/markov_analysis/
mv markov_symbol_scores*.json data/markov_analysis/
mv markov_file_index_mapping.json data/markov_analysis/
mv markov_results/* data/markov_analysis/results/ 2>/dev/null || true

# Eigenvectors
mkdir -p data/eigenvectors
mv markov_dominant_eigenvector.txt data/eigenvectors/
mv symbol_eigenvector_results.txt data/eigenvectors/
mv term_eigenvectors.txt data/eigenvectors/
mv eigenvector_label_mapping.txt data/eigenvectors/

# Moonshine
mkdir -p data/moonshine
mv elf_moonshine_map.txt data/moonshine/
mv codec_binary_extraction.txt data/moonshine/
mv binary_fingerprint_decoder.txt data/moonshine/
mv automorphic_orbit_lmfdb.txt data/moonshine/

# Const 71
mkdir -p data/const_71_analysis/{build_logs,binaries,perf_traces}
mv const_71_analysis/* data/const_71_analysis/ 2>/dev/null || true
mv const_equivalence_nix/* data/const_71_analysis/ 2>/dev/null || true

# Blockchain
mkdir -p data/blockchain/{contracts,blocks}
mv top_contracts/*.json data/blockchain/contracts/
mv blockchain_blocks/*.json data/blockchain/blocks/

# Similarity
mkdir -p data/similarity
mv symbol_similarity_results.txt data/similarity/
```

### Phase 2: Update Scripts
All analysis scripts should write to `data/` subdirectories with:
- Timestamped filenames
- Session IDs for grouping
- JSON metadata for each run

### Phase 3: Add .gitignore
```
# Large binary data
data/markov_analysis/*.bin
data/telemetry/strace_logs/*.log

# Keep structure and metadata
!data/**/*.json
!data/**/*.txt
!data/**/README.md
```

## 📊 Data Retention Policy

- **Keep**: All JSON metadata, summaries, analysis results
- **Archive**: Large binary matrices (>100MB) to compressed format
- **Clean**: Temporary logs older than 7 days
- **Backup**: Critical results to git LFS or external storage

## 🔍 Query Interface

Future: Create `data_query.rs` tool to:
- Search across all analysis results
- Join data from multiple sources
- Export subsets for specific research questions
- Generate reports from canonical data

## 🎯 Benefits

1. **Reproducibility** - All inputs/outputs in known locations
2. **Discoverability** - Clear hierarchy, easy to find data
3. **Collaboration** - Standard paths for sharing results
4. **Automation** - Scripts know where to read/write
5. **Version Control** - Selective git tracking of important data
