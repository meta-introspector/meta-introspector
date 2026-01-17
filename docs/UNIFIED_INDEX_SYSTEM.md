# Unified Index System

**Purpose**: Query across 3M files, repos, projects, and datasets using canonical data store

## Current Indexes

### 1. File Index (3M files)
- **Source**: `git_file_mapper.rs`
- **Output**: `FILE_GIT_MAPPING.csv` (1.2GB)
- **Schema**: file_path, git_repo, commit, branch, remote, url
- **Status**: Needs Parquet conversion

### 2. Repository Index
- **Source**: `global_repo_indexer.rs`
- **Output**: `data/raw/global_repo_index.json`
- **Schema**: path, name, remote_url, is_fork, is_local, branch, status, last_commit
- **Status**: Needs Parquet conversion

### 3. Dataset Index
- **Source**: `dataset-indexer.rs`
- **Output**: TBD
- **Schema**: HF datasets + local datasets + untracked
- **Status**: Needs Parquet conversion

### 4. Project Metadata
- **Source**: Various analyzers
- **Output**: Scattered JSON files
- **Status**: Needs consolidation

## Unified Schema Design

### Master Index (Parquet)

```
data/
├── indexes/
│   ├── files.parquet           # 3M files
│   ├── repos.parquet           # Git repositories
│   ├── datasets.parquet        # HF + local datasets
│   ├── projects.parquet        # Project metadata
│   └── symbols.parquet         # Code symbols
└── registry.json               # DataRegistry
```

### Query Examples

```rust
// Find all Rust files in a specific repo
SELECT file_path FROM files 
WHERE git_repo = 'meta-introspector' 
AND file_path LIKE '%.rs'

// Find repos with most files
SELECT git_repo, COUNT(*) as file_count 
FROM files 
GROUP BY git_repo 
ORDER BY file_count DESC

// Find datasets for a project
SELECT d.* FROM datasets d
JOIN projects p ON d.project_id = p.id
WHERE p.name = 'meta-introspector'
```

## Migration Plan

### Phase 1: Convert to Parquet
1. `git_file_mapper.rs` → `data/indexes/files.parquet`
2. `global_repo_indexer.rs` → `data/indexes/repos.parquet`
3. `dataset-indexer.rs` → `data/indexes/datasets.parquet`

### Phase 2: Link Indexes
- Add foreign keys (repo_id, project_id)
- Create join tables
- Build query interface

### Phase 3: Query System
- DuckDB integration for SQL queries
- Polars for DataFrame operations
- REST API for remote queries
