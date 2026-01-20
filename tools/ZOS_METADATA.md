# ZOS Metadata System

Mathematical description and classification system for all repositories.

## Structure

Each repo gets a `zos/` directory containing:

- `zos.toml` - Compact mathematical description of the repo
  - Git metadata (commits, branches, contributors)
  - File statistics (counts by language)
  - Classification (primary language, entropy, signatures)
  - Timeline (first/last commits)

## Usage

### Single Repo

```bash
# Collect metadata only
./tools/scripts/collect-repo-metadata.sh /path/to/repo

# Inject zos metadata + create branch
./tools/scripts/inject-zos-metadata.sh /path/to/repo zos-metadata
```

### Mass Injection (All Repos)

```bash
# Process all repos from master_url_list.txt
./tools/scripts/mass-inject-zos.sh data/master_url_list.txt zos-metadata

# This will:
# 1. Create zos-metadata branch in each repo
# 2. Add zos/zos.toml with metadata
# 3. Inject standard Nix infrastructure
# 4. Commit without disturbing main branch
```

## ZOS TOML Schema

```toml
[repo]
remote = "https://github.com/..."
branch = "main"
commit = "abc123..."
commit_count = 1234

[stats]
total_files = 100
rust_files = 50
nix_files = 10
primary_language = "rust"
contributors = 5

[timeline]
first_commit = "2020-01-01 00:00:00"
last_commit = "2026-01-20 09:00:00"

[classification]
file_entropy = 0.85
symbol_count = 5000
markov_signature = "hash..."
```

## Benefits

- Non-invasive: separate branch, doesn't touch main
- Standardized: same structure across all repos
- Queryable: TOML format for easy parsing
- Scalable: works across thousands of repos
- Mathematical: enables similarity analysis
