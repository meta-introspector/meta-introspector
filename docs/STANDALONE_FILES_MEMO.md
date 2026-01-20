# Memo: Standalone Temporary Files Problem

**Date**: 2026-01-19  
**Subject**: Proliferation of Standalone Temporary Files Across Project  
**Status**: 🔴 Critical Technical Debt

## Problem Statement

The meta-introspector project suffers from widespread proliferation of standalone temporary files, scripts, and binaries that create maintenance burden and organizational chaos.

## Evidence

### 1. Standalone Rust Scripts (Root Directory)
```
/mnt/data1/meta-introspector/
├── error_store.rs
├── all_commits_collector.rs
├── universal_llm_proxy.rs
├── nix_cargo_interceptor.rs
├── git_pack_scanner.rs
├── markov_chain_miner.rs
├── telemetry_hook_test_driver.rs
├── push_to_hf.rs
├── meme_swarm.rs
├── singularity_example.rs
├── type_complexity.rs
├── build_order_pipeline.rs
└── ... (50+ more standalone .rs files)
```

### 2. Temporary Cache Files
- `rpc_cache2/temp_*.json` - Solana RPC request/response temps
- `temp_*.txt` - Error logs scattered everywhere
- Build artifacts in random locations

### 3. Duplicate Functionality
- Multiple JSON parsers (Rust, Lean, Python)
- Multiple RPC clients
- Multiple Parquet converters being written

## Impact

### Development Velocity
- **Build Confusion**: `cargo build` doesn't know which binary to build
- **Dependency Hell**: Each standalone file may have different dependencies
- **No Reusability**: Code duplication across standalone files

### Maintenance Burden
- **Discovery Problem**: Hard to find what tools exist
- **Version Skew**: Same functionality implemented differently
- **Documentation Rot**: No single source of truth

### Storage Waste
- Temporary files never cleaned up
- Multiple copies of same data
- Git bloat from committed temps

## Root Causes

1. **Rapid Prototyping Culture**: Quick scripts for immediate needs
2. **No Cleanup Policy**: Temps become permanent
3. **Missing Build System**: No unified way to organize tools
4. **Lack of Modules**: Everything at root level

## Proposed Solutions

### Immediate (This Week)

1. **Create `tools/` Directory Structure**
```
tools/
├── solana/
│   ├── rpc_client/
│   ├── parquet_converter/
│   └── cache_manager/
├── nix/
│   ├── builder/
│   └── archive/
└── data/
    ├── markov/
    └── telemetry/
```

2. **Unified Cargo Workspace**
```toml
[workspace]
members = [
    "tools/solana/*",
    "tools/nix/*",
    "tools/data/*"
]
```

3. **Temp File Policy**
- All temps go to `$PROJECT_ROOT/.tmp/`
- Add `.tmp/` to `.gitignore`
- Cleanup script: `make clean-temps`

### Medium Term (This Month)

4. **Consolidate Duplicate Code**
- Single JSON parser library
- Single RPC client with caching
- Single Parquet writer

5. **Build System Integration**
- Nix flake for all tools
- `nix run .#tool-name` for any tool
- Reproducible builds

6. **Documentation**
- `TOOLS.md` - Catalog of all tools
- Each tool has README
- Usage examples

### Long Term (This Quarter)

7. **Service Architecture**
- Convert standalone scripts to services
- Unified API layer
- Proper state management

8. **Data Management**
- Centralized cache directory
- Automatic cleanup policies
- Compression for old data

## Migration Plan

### Phase 1: Audit (1 day)
```bash
# Find all standalone files
find . -maxdepth 1 -name "*.rs" > standalone_rust.txt
find . -name "temp_*" > temp_files.txt
```

### Phase 2: Categorize (1 day)
- Group by functionality
- Identify duplicates
- Mark for deletion/migration

### Phase 3: Migrate (1 week)
- Move to `tools/` structure
- Update imports
- Test builds

### Phase 4: Cleanup (1 day)
- Delete temps
- Remove duplicates
- Update documentation

## Success Metrics

- ✅ Zero standalone .rs files in root
- ✅ All temps in `.tmp/` directory
- ✅ Single `cargo build` works
- ✅ `nix flake check` passes
- ✅ Documentation complete

## Immediate Action Items

1. [ ] Create `tools/` directory structure
2. [ ] Move `json_to_parquet.rs` to `tools/solana/parquet_converter/`
3. [ ] Create workspace Cargo.toml
4. [ ] Add `.tmp/` to .gitignore
5. [ ] Write cleanup script

## Related Issues

- SOLFUNMEME dataset needs proper Parquet conversion
- Nix builds failing due to missing dependencies
- Git repo size bloating from temps

## Conclusion

The standalone file problem is solvable but requires discipline. We need:
1. **Structure** - Proper directory organization
2. **Policy** - Clear rules for temps
3. **Tooling** - Automated cleanup
4. **Culture** - Stop creating standalone files

**Recommendation**: Implement Phase 1-2 immediately before continuing SOLFUNMEME work.

---

**Author**: Kiro AI Assistant  
**Reviewed**: Pending  
**Priority**: High
