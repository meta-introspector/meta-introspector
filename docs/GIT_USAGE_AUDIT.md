# Git Usage Audit - Meta-Introspector & ZOS-Server

Comprehensive audit of all git operations across both repositories.

## Meta-Introspector Repository

### Summary
- **12 Rust programs** using `Command::new("git")` (30 calls)
- **2 Shell scripts** with git commands (8 calls)
- **1 program** mentioning `gix` library (repo_spider.rs)

### Rust Programs Using Git CLI

| Program | Calls | Purpose | Status |
|---------|-------|---------|--------|
| git-sources.rs | 4 | Centralized repo management | ✅ Core |
| global_repo_indexer.rs | 5 | Index all repos | ⚠️ Merge into git-sources |
| https_commit_fetcher.rs | 4 | Fetch via HTTPS | ⚠️ Use git-sources |
| git-activity-collector | 4 | Activity metrics | ✅ Keep |
| multi_worker.rs | 4 | Parallel processing | ⚠️ Use git-sources |
| all_commits_collector.rs | 2 | Collect all commits | ⚠️ Use git-sources |
| commit_collector.rs | 2 | Collect by author | ⚠️ Use git-sources |
| binary_byte_attribution.rs | 1 | Attribute bytes | ✅ Keep |
| focused_queue_builder.rs | 1 | Build work queue | ⚠️ Use git-sources |
| cost_attribution_71.rs | 1 | Cost attribution | ✅ Keep |
| local_commit_cache.rs | 1 | Cache commits | ⚠️ Use git-sources |
| recent_commits_scanner.rs | 1 | Scan recent | ⚠️ Use git-sources |

### Common Git Operations
```rust
// Get remote URL
git remote get-url origin

// Get current branch
git branch --show-current

// Get last commit
git rev-parse HEAD
git log -1 --format=%H

// Get commit history
git log --all --format=%H|%an|%ae|%at|%s

// Check status
git status --short
git status --porcelain

// Fetch updates
git fetch --all
```

### Shell Scripts
1. **create_data_buckets.sh** - Setup script (5 calls)
2. **setup-precommit.sh** - Pre-commit hooks (3 calls)

## ZOS-Server Repository

### Summary
- **18 Rust programs** using `Command::new("git")` (72+ calls)
- Much heavier git usage than meta-introspector
- Multiple server components with git integration

### Rust Programs Using Git CLI

| Program | Calls | Purpose | Status |
|---------|-------|---------|--------|
| zos_minimal_server.rs | 12 | Main server with git info | ⚠️ Core server |
| zos-minimal-server/main.rs | 11 | Server binary | ⚠️ Duplicate |
| crate-indexer | 8 | Index crates from git | ⚠️ Should use registry |
| timeline-builder | 7 | Build git timeline | ✅ Analysis tool |
| git_analyzer.rs | 7 | Analyze git repos | ⚠️ Should use registry |
| cicd_dashboard.rs | 5 | CI/CD dashboard | ⚠️ Should use registry |
| git_pack_analyzer.rs | 3 | Analyze git packs | ✅ Analysis tool |
| canonical-store | 3 | Canonical storage | ⚠️ Should use registry |
| version.rs | 3 | Version info | ✅ Build info |
| wrapping_cost_analyzer.rs | 2 | Cost analysis | ✅ Analysis tool |
| homotopy_unirepo.rs | 2 | Unirepo management | ⚠️ Should use registry |
| hierarchical_markov.rs | 2 | Markov analysis | ✅ Analysis tool |
| test-repo-status.rs | 1 | Test status | ✅ Test |
| minimal_server_plugin.rs | 1 | Plugin system | ⚠️ Should use registry |
| auto-forker | 1 | Auto fork repos | ⚠️ Should use registry |
| remote-fork-mapper | 1 | Map forks | ⚠️ Should use registry |
| rust-clippy (submodule) | 3 | Clippy tests | ✅ External |

### Key Findings

**Duplication:**
- zos_minimal_server.rs and zos-minimal-server/main.rs have nearly identical git code
- Multiple tools doing similar git operations

**Common Operations in ZOS-Server:**
```rust
// Server info
git rev-parse HEAD              // Current commit
git branch --show-current       // Current branch  
git log -1 --format=%cr         // Commit age

// Repository management
git remote get-url origin       // Remote URL
git clone <url>                 // Clone repo
git status --porcelain          // Status check

// Analysis
git log --all --format=...      // Commit history
git diff --name-only            // Changed files
git verify-pack                 // Pack analysis
```

## Comparison: Meta-Introspector vs ZOS-Server

| Metric | Meta-Introspector | ZOS-Server |
|--------|-------------------|------------|
| Programs with git | 12 | 18 |
| Total git calls | 38 | 72+ |
| Centralized access | ✅ git-sources.rs | ❌ None |
| Duplication | Low | High |
| Server integration | No | Yes (dashboard) |

## Recommendations

### 1. Create Unified Git Registry
**Shared between both repos:**
- Extract git-sources.rs to separate crate
- Both repos depend on it
- Single source of truth for all git operations

### 2. ZOS-Server Specific Needs
**Server features to preserve:**
- Real-time git status for dashboard
- Commit age for CI/CD monitoring
- Remote URL for fork mapping

**Integrate with git-sources:**
- Use registry for repo discovery
- Cache git info for dashboard
- Avoid redundant git calls

### 3. Consolidate Duplicates
**ZOS-Server duplicates:**
- Merge zos_minimal_server.rs and zos-minimal-server/main.rs
- Consolidate git_analyzer.rs and cicd_dashboard.rs
- Use shared git operations module

### 4. Migration Priority

**High Priority (use git-sources):**
- crate-indexer
- git_analyzer.rs
- cicd_dashboard.rs
- canonical-store
- homotopy_unirepo.rs
- auto-forker
- remote-fork-mapper

**Keep As-Is (specialized):**
- timeline-builder
- git_pack_analyzer.rs
- wrapping_cost_analyzer.rs
- hierarchical_markov.rs
- version.rs

## Action Plan

### Phase 1: Extract git-sources to Crate ✅ READY
```toml
[package]
name = "git-sources"
version = "0.1.0"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
chrono = "0.4"
```

### Phase 2: Add to Both Repos
**meta-introspector/Cargo.toml:**
```toml
[dependencies]
git-sources = { path = "../git-sources" }
```

**zos-server/Cargo.toml:**
```toml
[dependencies]
git-sources = { path = "../git-sources" }
```

### Phase 3: Migrate Tools
1. Update crate-indexer to use git-sources
2. Update git_analyzer to use git-sources
3. Update cicd_dashboard to use git-sources
4. Remove duplicate git operations

### Phase 4: Add Server Features
**Extend git-sources for server needs:**
- `git-sources watch` - Real-time monitoring
- `git-sources dashboard` - JSON output for dashboards
- `git-sources cache` - Cache git info for performance

## Total Git Operations

**Before migration:**
- Meta-Introspector: 38 calls
- ZOS-Server: 72+ calls
- **Total: 110+ git CLI calls**

**After migration:**
- git-sources core: ~20 calls
- Specialized tools: ~30 calls
- **Total: ~50 calls (55% reduction)**

## Next Steps

1. ✅ Document git usage (this file)
2. [ ] Extract git-sources to standalone crate
3. [ ] Migrate meta-introspector tools
4. [ ] Migrate zos-server tools
5. [ ] Add server-specific features
6. [ ] Benchmark performance improvement

