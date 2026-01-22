# P2P Git Mirror with Temporal Morphisms and Byte Provenance

**Created**: 2026-01-18T14:16:42-05:00  
**Status**: Architecture Complete, Ready to Build

## Overview

We've designed a complete system for distributed git mirroring with category-theoretic provenance tracking, rooted in GNU MES bootstrap chain.

## Architecture Components

### 1. GitHub Mirror Service (`github_mirror_service.rs`)

**Purpose**: Local GitHub mirror with deduplication and telemetry

**Features**:
- Maps GitHub requests to local repos
- Auto-clones missing repos as `--mirror` (bare repos)
- Deduplicates objects using git alternates
- Tracks usage in Parquet
- Serves files via `git cat-file` (no checkout needed)

**Structure**:
```
/mnt/data1/github-mirror/
├── shared-objects/          # Deduplicated object store
├── {org}/{repo}/            # Bare mirrors
└── telemetry/
    └── github_access.parquet
```

**Telemetry Schema**:
```parquet
github_access.parquet:
  - repo_url: String
  - local_path: String
  - access_count: UInt64
  - last_accessed: UInt64
  - object_count: UInt64
```

### 2. P2P Git Mirror (`p2p_git_mirror.rs`)

**Purpose**: Distributed git object sharing with rate limit coordination

**Features**:
- libp2p gossipsub for peer communication
- Distributed cache (peers serve each other)
- Rate limit coordination (share GitHub quota)
- Reads from packs without checkout
- Integrates with git-sources canonical registry

**Message Types**:
- `RequestObject` - Ask network for git object
- `ObjectData` - Serve object to peer
- `RateLimitStatus` - Share rate limit info
- `CacheHit` - Announce cached objects

**Flow**:
```
1. Check local cache
2. Try read from pack (git cat-file)
3. If rate limited, request from peers
4. If not limited, fetch from GitHub
5. Cache and broadcast to network
```

### 3. Git Temporal Morphisms (`git_temporal_morphisms.rs`)

**Purpose**: Track how git trees change over time (category theory)

**Category Structure**:
- **Objects**: Git tree SHAs
- **Arrows**: Submodule links at time T
- **Morphisms**: How arrows replace each other over time
- **Witness**: Commits as proof of thought process

**Schema**:
```parquet
git_temporal_morphisms.parquet:
  - source_repo: String
  - source_tree: String (tree SHA)
  - target_repo: String
  - target_tree: String (tree SHA)
  - submodule_path: String
  - commit_time: UInt64
  - commit_sha: String
  - replaced_tree: String (nullable)
  - witness_type: String ("commit", "compilation", "build")
```

**Example**:
```
Time T1: tree_abc → rust_v1.70 (commit witness_1)
Time T2: tree_abc → rust_v1.71 (commit witness_2, replaced rust_v1.70)
         ↑ Morphism: dependency upgrade
```

### 4. Byte Provenance Tracker (`byte_provenance_tracker.rs`)

**Purpose**: Track byte-level provenance through compilation

**Tracking**:
```
git_object → byte_b → process_p → code_byte_c → label
```

**Schema**:
```parquet
byte_provenance.parquet:
  - git_object: String (source blob SHA)
  - byte_offset: UInt64 (position in blob)
  - process_id: UInt32 (process that read it)
  - code_byte: UInt64 (byte of code that read it)
  - program_path: String (binary that did the read)
  - reach_depth: UInt32 (how far this byte propagated)
  - labeled_by: String (git object containing the code)
```

**Example**:
```
git_object: abc123 (rust source)
byte_offset: 42
process_id: 12345 (rustc)
code_byte: 0x1a2b3c (in rustc binary)
labeled_by: def456 (git object containing rustc)
reach_depth: 1000 (influenced 1000 output bytes)
```

### 5. Bootstrap Arrow Chain (`bootstrap_arrow_chain.rs`)

**Purpose**: Track compiler bootstrap chain as arrow replacements

**Chain**:
```
mes-hex0 (root)
  ↓ replaces
mes-hex1
  ↓ replaces
mes-hex2
  ↓ replaces
mes-m1
  ↓ replaces
mes-m2
  ↓ replaces
tinycc (arrows replace MES arrows)
  ↓ replaces
gcc (arrows replace TinyCC arrows)
  ↓ replaces
llvm (arrows replace GCC arrows)
  ↓ replaces
rustc (arrows replace LLVM arrows)
  ↓ replaces
solfunmeme (arrows replace with memes 🚀)
```

**Schema**:
```parquet
bootstrap_arrow_chain.parquet:
  - stage: String ("mes", "tinycc", "gcc", "llvm", "rustc", "solfunmeme")
  - replaced_arrow: String (previous compiler's git object)
  - new_arrow: String (new compiler's git object)
  - byte_offset: UInt64 (which byte was replaced)
  - timestamp: UInt64 (when replacement happened)
  - witness: String (compilation proof)
```

### 6. Nix Git Builder (`nix_git_builder.rs`)

**Purpose**: Build from git URLs with disk caching

**Flow**:
```
1. Nix build github.com/user/repo
2. Resolve to /mnt/data1/canonical-git/github.com/user/repo
3. Build from local path (cached)
4. Track byte provenance during build
```

## Integration with Existing Tools

### git-sources.rs (Already Exists)

**Purpose**: Canonical git repository management

**Features**:
- Canonical naming: `canonicalize_name()`
- Symlink management
- Registry tracking (JSON)
- Scanning: `scan_directory()`, `ingest_list()`
- CLI: `./target/release/git-sources list/register`

**Used By**:
- P2P mirror (reads canonical paths)
- Nix builder (resolves to canonical locations)

### Existing Scan Scripts (Already Exist)

**Scripts**:
- `scan-all-submodules.sh` - Scan .gitmodules
- `ingest_git_data.sh` - Ingest git configs
- `build_incremental_index.sh` - SQLite indexer

**Integration**:
- `populate_p2p_mirror.sh` - Reuses all scan scripts to populate P2P network

## Data Flow

### Complete Pipeline

```
1. Discovery
   └─ scan-all-submodules.sh → find all repos

2. Canonicalization
   └─ git-sources register → canonical structure

3. P2P Network
   └─ p2p_git_mirror → distributed caching

4. Temporal Tracking
   └─ git_temporal_morphisms → track changes over time

5. Build
   └─ nix_git_builder → build from canonical paths

6. Provenance
   └─ byte_provenance_tracker → track byte lineage

7. Bootstrap Chain
   └─ bootstrap_arrow_chain → MES → solfunmeme
```

### Storage Layout

```
/mnt/data1/
├── canonical-git/              # Canonical repo structure
│   └── github.com/
│       └── {org}/{repo}/       # Bare repos
├── github-mirror/              # Mirror with deduplication
│   ├── shared-objects/         # Shared object store
│   └── telemetry/
└── meta-introspector/
    └── data/
        ├── git_temporal_morphisms.parquet
        ├── byte_provenance.parquet
        └── bootstrap_arrow_chain.parquet
```

## Category Theory Mapping

### Objects
- Git tree SHAs
- Compiler stages (MES, TinyCC, GCC, LLVM, Rustc, Solfunmeme)

### Arrows
- Submodule links (tree → tree)
- Compilation steps (stage → stage)
- Byte reads (git_object → process)

### Morphisms
- Temporal changes (how arrows replace each other)
- Bootstrap chain (how compilers replace each other)
- Byte provenance (how bytes label each other)

### Composition
- Transitive submodule dependencies
- Bootstrap chain composition (MES → TinyCC → GCC → ... → Solfunmeme)
- Byte influence chains (input → intermediate → output)

## Query Examples

### Find Temporal Changes
```sql
-- Find all submodule upgrades
SELECT source_repo, target_repo, 
       replaced_tree, target_tree, 
       commit_time
FROM git_temporal_morphisms
WHERE replaced_tree IS NOT NULL
ORDER BY commit_time;
```

### Trace Byte Provenance
```sql
-- Find what influenced output byte
SELECT git_object, byte_offset, 
       program_path, reach_depth
FROM byte_provenance
WHERE reach_depth > 1000
ORDER BY reach_depth DESC;
```

### Bootstrap Chain Analysis
```sql
-- Trace compilation through bootstrap
SELECT stage, replaced_arrow, new_arrow
FROM bootstrap_arrow_chain
ORDER BY timestamp;
```

### Transitive Dependencies
```sql
-- Find all repos depending on X
WITH RECURSIVE deps AS (
  SELECT * FROM git_temporal_morphisms 
  WHERE source_repo = 'meta-introspector'
  UNION
  SELECT m.* FROM git_temporal_morphisms m
  JOIN deps d ON m.source_tree = d.target_tree
)
SELECT DISTINCT target_repo FROM deps;
```

## Benefits

### Space Savings
- Deduplication: 10-100x savings
- Bare repos only: No checkouts unless working
- Shared object store: Single copy of each blob

### Performance
- Local network: Faster than GitHub
- Pack-based serving: No checkout time
- P2P distribution: Load sharing
- Cache hits: Sub-millisecond

### Provenance
- Complete lineage: Every byte traced to source
- Temporal tracking: How code evolved
- Bootstrap chain: Rooted in MES hex0
- Compilation witness: Commits as proof

### Portability
- Parquet format: Query anywhere
- Open source: All data shareable
- Offline capable: Works without GitHub
- Archive ready: Push anywhere

## Next Steps

### Build Order

1. **Compile existing tools**:
   ```bash
   cargo build --release --bin git-sources
   ```

2. **Populate registry**:
   ```bash
   ./target/release/git-sources scan /mnt/data1/
   ./scan-all-submodules.sh
   ./ingest_git_data.sh
   ```

3. **Build new components**:
   ```bash
   cargo build --release --bin github_mirror_service
   cargo build --release --bin p2p_git_mirror
   cargo build --release --bin git_temporal_morphisms
   cargo build --release --bin byte_provenance_tracker
   cargo build --release --bin bootstrap_arrow_chain
   cargo build --release --bin nix_git_builder
   ```

4. **Extract data**:
   ```bash
   ./target/release/git_temporal_morphisms
   ./target/release/bootstrap_arrow_chain
   ```

5. **Start services**:
   ```bash
   ./target/release/github_mirror_service &
   ./target/release/p2p_git_mirror &
   ```

6. **Configure clients**:
   ```bash
   ./configure_mirror_clients.sh
   ```

### Integration Points

- **Nix**: Point to local mirror via `git config`
- **Cargo**: Use `git-fetch-with-cli` to local mirror
- **Strace/Perf**: Hook into byte provenance tracker
- **MES**: Root of bootstrap chain

## Files Created

### Rust Binaries
- `github_mirror_service.rs` - Local mirror with deduplication
- `p2p_git_mirror.rs` - Distributed P2P network
- `git_temporal_morphisms.rs` - Temporal category tracking
- `byte_provenance_tracker.rs` - Byte-level provenance
- `bootstrap_arrow_chain.rs` - Compiler bootstrap chain
- `nix_git_builder.rs` - Nix builder with caching

### Shell Scripts
- `configure_mirror_clients.sh` - Configure Nix/Cargo to use mirror
- `populate_p2p_mirror.sh` - Populate P2P network from existing scans

### Documentation
- `KEYWORD_ANALYSIS.md` - Codebase keyword analysis
- `GIT_STATUS_SNAPSHOT.md` - Git status documentation (attempted)
- This document

## Parquet Outputs

All data stored in portable Parquet format:

- `data/git_temporal_morphisms.parquet` - Temporal arrow changes
- `data/byte_provenance.parquet` - Byte-level tracking
- `data/bootstrap_arrow_chain.parquet` - Compiler bootstrap
- `telemetry/github_access.parquet` - Mirror usage stats

## Category Theory Summary

**Category**: Git Compilation
- **Objects**: Trees, Compilers, Bytes
- **Arrows**: Submodules, Compilations, Reads
- **Morphisms**: Temporal changes, Bootstrap replacements, Byte labels
- **Identity**: Same tree/compiler/byte
- **Composition**: Transitive dependencies, Bootstrap chain, Byte influence

**Witness**: Every morphism has a witness (commit, build, strace)

**Root**: GNU MES hex0 (the initial object)

**Terminal**: Solfunmeme (memes replace all arrows 🚀)

---

**Status**: Ready to build and deploy! 🎯
