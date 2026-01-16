# Data Files HuggingFace Upload Plan

**Date**: 2026-01-15  
**Strategy**: Upload all data files <10MB to HuggingFace datasets

## Files Ready for Upload (<10MB)

### Root Directory Data Files

**Total**: 95 files ready for direct upload

#### Large Files (>1MB, <10MB)
- `lmfdb_self_analysis.json` - 3.3M
- `rustc_complete_grammar.json` - 2.6M
- `nix_store_grammars.parquet` - 1.5M

#### Medium Files (100KB-1MB)
- `rustc_bootstrap_rustc_bootstrap_1768321651.json` - 805K
- `enum_distribution_analysis.json` - 594K
- `rust_build_compressed.json` - 521K
- `chrome_profiler.json` - 284K
- `build_order_analysis.json` - 263K
- `type_function_graph.json` - 123K
- `string_usage.parquet` - 107K

#### Small Files (<100KB)
- 85 additional JSON/CSV files (all <100KB)

### Directories Ready for Upload

#### reports/ - 1016K (1MB)
```
reports/2020/
reports/2023/
reports/2024/
reports/2025/
reports/2026/
```
Generated monthly reports by user/year/month.

#### logs/ - 16K
Runtime logs from various tools.

#### telemetry/ - 28K
Build telemetry data.

#### repos/ - 4K
Repository metadata.

## Files Requiring Splitting (>10MB)

### markov_symbol_scores.parquet - 106M
**Action Required**: Split into chunks <10MB

**Recommended Approach**:
```bash
# Split parquet by row groups
python3 << 'EOF'
import pyarrow.parquet as pq
table = pq.read_table('markov_symbol_scores.parquet')
rows_per_chunk = len(table) // 12  # ~12 chunks of ~9MB each
for i in range(0, len(table), rows_per_chunk):
    chunk = table.slice(i, rows_per_chunk)
    pq.write_table(chunk, f'markov_symbol_scores_part_{i//rows_per_chunk:03d}.parquet')
EOF
```

## Large Directories (Already Staged for HF)

### hf-build-telemetry/ - 5.4M
Already prepared for upload.

### hf-build-telemetry-upload/ - 14M
Needs chunking or compression.

### hf-markov-analysis/ - 2.2G
**Too large** - needs partitioning (use metis-partition-markov.rs).

### hf-markov-analysis-upload/ - 163M
Already partitioned, ready for upload.

### hf-git-activity/ - 616M
**Too large** - needs monthly/yearly partitioning.

## HuggingFace Dataset Structure

### Proposed Datasets

#### 1. introspector/meta-analysis-results
**Size**: ~10MB  
**Contents**: Root directory small files (<1MB)
```
markov_analysis/
  ├── complex_type_instance_markov.json
  ├── datatype_markov_*.json
  └── enum_distribution_analysis.json
compression/
  ├── aggregated_compression_results.json
  ├── rust_build_compressed.json
  └── rustc_intercept_compression.json
type_analysis/
  ├── type_function_graph.json
  ├── struct_composition_analysis.json
  └── build_order_analysis.json
```

#### 2. introspector/large-analysis-artifacts
**Size**: ~8MB  
**Contents**: Large JSON/parquet files (1-10MB)
```
lmfdb_self_analysis.json
rustc_complete_grammar.json
nix_store_grammars.parquet
rustc_bootstrap_rustc_bootstrap_1768321651.json
```

#### 3. introspector/monthly-reports
**Size**: ~1MB  
**Contents**: reports/ directory
```
2020/
2023/
2024/
2025/
2026/
```

#### 4. introspector/markov-symbol-scores
**Size**: ~106MB (split into 12 parts)  
**Contents**: Partitioned markov_symbol_scores.parquet
```
markov_symbol_scores_part_000.parquet
markov_symbol_scores_part_001.parquet
...
markov_symbol_scores_part_011.parquet
README.md (how to merge)
```

#### 5. introspector/markov-analysis-partitioned
**Size**: ~163MB  
**Contents**: hf-markov-analysis-upload/
```
partition_0000.json
partition_0001.json
...
```

## Upload Commands

### Create datasets
```bash
# Install huggingface_hub
pip install huggingface_hub

# Login
huggingface-cli login

# Create and upload
python3 << 'EOF'
from huggingface_hub import HfApi
api = HfApi()

# Create datasets
datasets = [
    "introspector/meta-analysis-results",
    "introspector/large-analysis-artifacts",
    "introspector/monthly-reports",
    "introspector/markov-symbol-scores",
    "introspector/markov-analysis-partitioned"
]

for dataset in datasets:
    api.create_repo(repo_id=dataset, repo_type="dataset", exist_ok=True)
EOF
```

### Upload files
```bash
# Upload small files
huggingface-cli upload introspector/meta-analysis-results . --include="*.json" --exclude="*_part_*.json" --exclude="lmfdb_*.json" --exclude="rustc_complete_*.json"

# Upload large artifacts
huggingface-cli upload introspector/large-analysis-artifacts lmfdb_self_analysis.json
huggingface-cli upload introspector/large-analysis-artifacts rustc_complete_grammar.json
huggingface-cli upload introspector/large-analysis-artifacts nix_store_grammars.parquet

# Upload reports
huggingface-cli upload introspector/monthly-reports reports/

# Upload partitioned markov (after splitting)
huggingface-cli upload introspector/markov-symbol-scores markov_symbol_scores_part_*.parquet

# Upload markov analysis
huggingface-cli upload introspector/markov-analysis-partitioned hf-markov-analysis-upload/
```

## Gitignore Strategy

Keep these untracked (will be on HF):
```gitignore
# Data files (on HuggingFace)
*.parquet
*_part_*.json
markov_symbol_scores*.parquet
reports/
logs/
telemetry/
hf-*/
```

Keep these tracked (small configs):
```
# Small configs and metadata
rust_build_config.json
selected_repos_*.json
compression_batch.json
```

## Next Steps

1. **Split large parquet file**
   ```bash
   cargo run --bin split-parquet -- markov_symbol_scores.parquet --max-size 9M
   ```

2. **Create HF datasets**
   ```bash
   python3 create_hf_datasets.py
   ```

3. **Upload in batches**
   ```bash
   ./upload_to_hf.sh
   ```

4. **Update .gitignore**
   ```bash
   git add .gitignore
   git commit -m "Ignore data files uploaded to HuggingFace"
   ```

5. **Document in README**
   - Add HF dataset links
   - Add download instructions
   - Add merge instructions for partitioned files

## File Count Summary

- **Ready for upload**: 95 files (<10MB each)
- **Needs splitting**: 1 file (markov_symbol_scores.parquet - 106M)
- **Already partitioned**: hf-markov-analysis-upload/ (163M)
- **Total data to upload**: ~280MB across 5 datasets
