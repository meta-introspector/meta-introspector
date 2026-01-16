# Git Status Report - 2026-01-15

**Branch**: `nix-build-setup`  
**Status**: Up to date with `origin/nix-build-setup`

## Modified Files (Not Staged)

### Submodules
- `rust-overlay-test` - Modified content, untracked content
- `rust-telemetry-driver` - New commits
- `rustc-from-source` - Modified content
- `rustc-only-build` - Modified content

### Scripts
- `scripts/capture_build_log.sh` - Modified

### Symlinks
- `result` - Modified (Nix build result symlink)

## Untracked Files

### Build & Analysis Tools
- `build_all_binaries.sh` - Build automation script
- `count_data_per_branch.sh` - Data counting utility
- `doit.sh~` - Backup file

### Cargo Projects (with Cargo.lock)
- `build_type_graph/`
- `instrumented-wrappers/`
- `lmfdb-rust-mapping/`
- `markov_resonance_analyzer/`
- `query_ast_types/`
- `rust_preload_interceptor/`
- `symbol_similarity/`
- `type_call_graph/`

### Data & Analysis Results
- `markov_symbol_scores.parquet` - Markov analysis results
- `nix_store_grammars.parquet` - Grammar compression data
- `square_proof_1768347044.csv` - Mathematical proof data
- `string_usage.parquet` - String usage analysis

### HuggingFace Projects
- `hf-build-telemetry/` - Build telemetry for HF upload
- `hf-markov-analysis/` - Markov analysis for HF
- `hf-markov-analysis-upload/` - Upload staging

### Logs & Reports
- `kiro-log` - Kiro CLI session log
- `logs/` - General logs directory
- `telemetry/` - Telemetry data
- `reports/2020/`, `reports/2023/`, `reports/2024/`, `reports/2025/`, `reports/2026/` - Annual reports

### Repositories
- `repos/` - Cloned repositories for analysis

### Backups
- `git_loose_objects_backup_1768502112.tar.gz` - Git objects backup

## Summary

**Modified**: 6 files (5 submodules + 1 script)  
**Untracked**: 28 items (8 Cargo projects, 4 parquet files, 3 HF projects, 5 report years, logs, repos, backups)

## Recommendations

1. **Submodules**: Review and commit submodule changes separately
2. **Cargo.lock**: Consider adding to .gitignore or committing for reproducibility
3. **Data files**: Large parquet files should likely be in .gitignore or Git LFS
4. **Reports**: Archive old years (2020-2024) or add to .gitignore
5. **Logs**: Add `logs/`, `telemetry/`, `kiro-log` to .gitignore
6. **Repos**: Add `repos/` to .gitignore (cloned analysis targets)
