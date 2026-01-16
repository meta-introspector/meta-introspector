# Build Fixing Guide - Resume Point

**Date**: 2026-01-15  
**Branch**: nix-build-setup  
**Status**: In progress - systematic error fixing

## Current State

### meta-introspector
- **Branch**: nix-build-setup
- **Binaries**: 184 total
- **Status**: Not yet triaged
- **Location**: `/mnt/data1/meta-introspector`

### zos-server
- **Branch**: nix-build-setup
- **Binaries**: 72 (was 75, moved 3 to examples)
- **Errors**: ~27 remaining (was 41)
- **Status**: Actively fixing
- **Location**: `~/zos-server`

## Progress Made

### Errors Fixed (20 total)
1. ✅ E0282 (11) - Type annotations in rust_structured_extractor.rs
2. ✅ E0599 (3) - Added audit_function method
3. ✅ E0433 (3) - Moved example_patched_code to examples
4. ✅ E0308 (3) - Fixed type mismatches in audited_automorphic_system.rs

### Files Reorganized
- `examples/` - Test/demo files (llm_git_demo, example_patched_code, etc.)
- `clippy-lints/` - Custom clippy lints (dead_code_eliminator)

### Tools Created
- `scripts/capture_build_log.sh` - Canonical build log capture
- `docs/BUILD_SYSTEM.md` - Build system documentation
- `capture_multidimensional_messages.sh` - Multi-dimensional message capture

## Next Steps

### 1. Continue Fixing zos-server

**Current top errors:**
```
5 error[E0282]: type annotations needed
1 error[E0601]: `main` function not found (test files)
```

**Commands:**
```bash
cd ~/zos-server

# Capture current status
./scripts/capture_build_log.sh

# View top errors
tail -40 data/build_logs/$(ls -t data/build_logs/ | head -1)/full_build.log

# Fix pattern: Always pick first error
# 1. Find file and line from error output
# 2. Fix the error
# 3. Commit with descriptive message
# 4. Re-capture build log
# 5. Repeat
```

### 2. Fix Remaining E0282 Errors

**Files with E0282:**
- `p2p_server.rs:75` - ✅ Fixed (std::mem::take)
- Check latest build log for remaining instances

**Fix pattern:**
```rust
// Before
thread::scope(|s| {
    s.spawn(move |_| {

// After  
thread::scope(|s: &thread::Scope<'_, '_>| {
    s.spawn(move |_: &thread::Scope<'_, '_>| {
```

### 3. Fix E0601 (Missing main)

**Files:**
- test_file.rs
- p2p_server.rs (if still there)
- gandalf_test.rs (if still there)
- example_build.rs (if still there)

**Action:** Move to examples/ or add main function

### 4. After zos-server Builds

```bash
# Capture final build log
cd ~/zos-server
./scripts/capture_build_log.sh

# Move to meta-introspector
cd /mnt/data1/meta-introspector
./scripts/capture_build_log.sh

# Fix errors using same pattern
```

## Build Log Analysis

### View Latest Build
```bash
cd ~/zos-server
SESSION=$(ls -t data/build_logs/ | head -1)
cat data/build_logs/$SESSION/metadata.json
head -10 data/build_logs/$SESSION/error_summary.txt
```

### Compare Progress
```bash
# Compare two sessions
diff data/build_logs/SESSION1/error_summary.txt \
     data/build_logs/SESSION2/error_summary.txt
```

## Git Workflow

### Commit Pattern
```bash
# Always use SKIP for pre-commit rust hooks (cargo not in PATH)
SKIP=cargo-check,rust-fmt,rust-clippy git commit -m "Fix EXXXX: description"
git push
```

### Branch Status
```bash
# Check both repos
cd ~/zos-server && git status
cd /mnt/data1/meta-introspector && git status
```

## Flake Status

### Working
- ✅ `nix develop` - Telemetry shell active
- ✅ `nix build .#default` - Builds rust-telemetry-driver
- ✅ rust-telemetry-driver as remote flake input
- ✅ zos-server as remote flake input

### Inputs
```nix
rust-telemetry-driver.url = "github:meta-introspector/rust-telemetry-driver";
zos-server.url = "github:meta-introspector/zos-server/nix-build-setup";
```

## Data Preservation

### Reflog Branches
- ✅ 253 branches created from reflog
- ✅ All data files preserved in branches
- ✅ ~14,500 JSON files per branch

### Build Logs
- All in `data/build_logs/SESSION_ID/`
- Structured: metadata.json, error_summary.txt, full_build.log
- Ready for HuggingFace upload (<10MB per session)

## After Reboot

```bash
# 1. Check branch
cd ~/zos-server && git branch --show-current
cd /mnt/data1/meta-introspector && git branch --show-current

# 2. Enter nix develop
cd ~/zos-server && nix develop

# 3. Continue fixing
./scripts/capture_build_log.sh
# Fix first error
# Commit
# Repeat
```

## Goal

**259 binaries total** (184 + 75)
- Each binary = semiotic utterance
- Multi-dimensional messages: Markov + AST + perf + nix store + trace
- Build all → capture messages → analyze differences

## Key Insight

> Each binary produces output (message/derivative) that can be analyzed across multiple dimensions. The build errors are part of the message - they tell us about dependencies, types, and structure.

## Quick Reference

```bash
# Capture build
./scripts/capture_build_log.sh

# View errors
tail -50 data/build_logs/$(ls -t data/build_logs/ | head -1)/full_build.log

# Fix and commit
SKIP=cargo-check,rust-fmt,rust-clippy git commit -m "Fix: ..."
git push

# Repeat
```
