# Canonical Dataset Index

## Published HuggingFace Datasets

### introspector/git-activity
- **URL**: https://huggingface.co/datasets/introspector/git-activity
- **Size**: 564 MB
- **Content**: 402,816 commits from 9,001 authors across 53 repos
- **Structure**: `activity/{platform}/{user}/{year}/{month}/activity.json`
- **Status**: ✅ Published
- **Use**: Git activity analysis, contributor metrics, commit patterns

## Local Datasets (Need HF Upload)

### 1. data-markov-analysis ⚠️ LARGE
- **Path**: `data-markov-analysis/` (submodule)
- **Size**: 17 GB
- **Purpose**: Markov chain analysis of code patterns
- **Recommended HF**: `introspector/markov-analysis` (split into chunks or use Git LFS)
- **Action**: Analyze structure, consider chunking by date/type

### 2. data-eigenvectors
- **Path**: `data-eigenvectors/` (submodule)
- **Size**: 436 KB
- **Purpose**: Eigenvector analysis of code structures
- **Recommended HF**: Add to `introspector/git-activity` as subdirectory
- **Action**: Small enough to include in existing dataset

### 3. data-moonshine
- **Path**: `data-moonshine/` (submodule)
- **Size**: 292 KB
- **Purpose**: Moonshine project data (codecs, fingerprints)
- **Recommended HF**: `introspector/moonshine`
- **Action**: Create dedicated dataset

### 4. data-blockchain
- **Path**: `data-blockchain/` (submodule)
- **Size**: 360 KB
- **Purpose**: Blockchain-related analysis data (blocks, contracts)
- **Recommended HF**: `introspector/blockchain-analysis`
- **Action**: Create dedicated dataset

### 5. data-telemetry
- **Path**: `data-telemetry/` (submodule)
- **Size**: 5.7 MB
- **Purpose**: Build telemetry from nix/cargo builds (JSONL format)
- **Recommended HF**: `introspector/build-telemetry`
- **Action**: Create dedicated dataset with structured telemetry

### 6. data-const71
- **Path**: `data-const71/` (submodule)
- **Size**: 224 KB
- **Purpose**: Const71 build analysis (binaries, analysis)
- **Recommended HF**: Add to `introspector/build-telemetry`
- **Action**: Merge with telemetry dataset

### 7. data/perf_sessions
- **Path**: `data/perf_sessions/`
- **Purpose**: Perf capture from cascading repo analyzer
- **Recommended HF**: Add to `introspector/git-activity` as subdirectory
- **Action**: Upload as `perf_sessions/` in git-activity dataset

### 8. data/71_flakes_perf
- **Path**: `data/71_flakes_perf/`
- **Purpose**: Performance analysis of 71 nix flakes
- **Recommended HF**: `introspector/nix-flakes-perf`
- **Action**: Create new dataset with syscall/event analysis

### 9. data/build_analysis
- **Path**: `data/build_analysis/`
- **Purpose**: Real build analysis with strace (32 binaries, 92 libs)
- **Recommended HF**: `introspector/build-analysis`
- **Action**: Create new dataset with structured build data

## External Datasets to Index

### h4/* datasets
- **URL**: https://huggingface.co/h4/datasets
- **Action**: Query HF API to list all h4 datasets
- **Purpose**: Identify relevant datasets for meta-introspection

## Next Steps

1. **Initialize all submodules**: `git submodule update --init --recursive`
2. **Analyze each submodule**: Check size, content, structure
3. **Create HF datasets**: For each data-* submodule
4. **Upload to HuggingFace**: Using `huggingface-cli` or git
5. **Update .gitmodules**: Point to HF dataset URLs instead of local paths
6. **Document usage**: Add README.md to each dataset

## Dataset Organization Strategy

### Option A: Separate Datasets (Recommended)
- Each data type gets its own HF dataset
- Easier to version and manage
- Better discoverability

### Option B: Monorepo Dataset
- All data in `introspector/meta-introspector-data`
- Subdirectories for each type
- Single version control

### Option C: Hybrid
- Core datasets separate (git-activity, build-analysis)
- Related datasets grouped (markov-analysis + eigenvectors)
