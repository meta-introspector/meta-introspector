# Git Sources - Centralized Repository Management

Centralized git repository management system using canonical symlinks (no git submodules).

## Overview

Similar to how we centralized nix flakes and perf data, `git-sources` provides a unified registry of all git repositories with canonical symlink access.

## Features

- **Canonical naming**: Normalized repo names (lowercase, hyphenated)
- **Symlink access**: All repos accessible via `data/git-sources/<canonical-name>`
- **Metadata tracking**: URL, branch, last commit, last updated timestamp
- **Batch ingestion**: Process list.txt with multiple paths
- **Auto-discovery**: Scan directories for git repos
- **JSON registry**: Structured data in `data/git-sources-registry.json`

## Usage

### Ingest from list.txt
```bash
./target/release/git-sources ingest list.txt
```

### Scan directory
```bash
./target/release/git-sources scan /path/to/repos
```

### Register single repo
```bash
./target/release/git-sources register my-repo /path/to/repo
```

### List all repos
```bash
./target/release/git-sources list
```

## Registry Structure

```json
{
  "sources": {
    "hf-dataset-validator-rust": {
      "name": "hf-dataset-validator-rust",
      "url": "https://github.com/meta-introspector/hugging-face-dataset-validator-rust",
      "branch": "lattice-system-2025-01-15",
      "checkout_path": "/mnt/data1/nix/time/2025/08/07/hf-dataset-validator-rust",
      "canonical_link": "data/git-sources/hf-dataset-validator-rust",
      "last_commit": "327d4b34...",
      "last_updated": "2026-01-15T05:08:00Z"
    }
  }
}
```

## Current Registry (11 repos)

1. **hf-dataset-validator-rust** - Parquet lattice system (just pushed!)
2. **amazon-q-developer-cli** - AWS Q CLI
3. **solfunmeme-index** - Solana meme index
4. **solfunmeme-metameme** - Metameme system
5. **hf-rust-dataset** - Rust dataset on HF
6. **bootstrap** - Bootstrap system
7. **ragit** - RAG + Git integration
8. **emigo** - Emacs + Go
9. **rust-analyser-hf-dataset** - Rust analyzer dataset
10. **dataset-viewer** - HuggingFace dataset viewer
11. **meta-introspector** - This repo!

## Symlink Access

All repos accessible via canonical symlinks:
```bash
ls -la data/git-sources/
# hf-dataset-validator-rust -> /mnt/data1/nix/time/2025/08/07/hf-dataset-validator-rust
# meta-introspector -> /mnt/data1/meta-introspector
# ...
```

## Integration with Other Systems

### Nix Flakes
- Can reference repos via symlinks: `data/git-sources/hf-dataset-validator-rust`
- Consistent paths across builds

### Perf Data
- Correlate perf captures with specific repo commits
- Track performance across repo versions

### Telemetry
- Link telemetry data to exact repo state
- Reproducible analysis with commit hashes

## list.txt Format

```
# Individual repos
/mnt/data1/nix/time/2025/08/07/hf-dataset-validator-rust

# Directories to scan (will find all .git repos inside)
/mnt/data1/nix/time/2025/08/07
/mnt/data1/meta-introspector

# Comments and blank lines ignored
```

## Why Not Git Submodules?

- **Flexibility**: Point to existing checkouts, no duplication
- **Independence**: Repos can be on different branches/commits
- **Simplicity**: Just symlinks + JSON registry
- **Metadata**: Track URLs, branches, commits, timestamps
- **Batch operations**: Ingest many repos at once

## Future Enhancements

- [ ] Update all repos with `git-sources update`
- [ ] Query by URL, branch, or commit
- [ ] Export to various formats (TOML, YAML)
- [ ] Integration with cargo workspace
- [ ] Automatic dependency detection
- [ ] Commit history tracking
