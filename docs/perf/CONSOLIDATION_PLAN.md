# Perf Record Consolidation Plan

## Current State

**368 occurrences** of "perf record" across **99 files**

### Top Offenders
1. `execute_workflows.sh` - 142 occurrences (generated script, repetitive)
2. `docs/perf/PERF_USAGE_AUDIT.md` - 14 occurrences
3. `docs/nix/perf/README.md` - 9 occurrences
4. Various other docs and scripts - 203 occurrences

## Consolidation Strategy

### 1. Create Canonical Perf Recording Function

**Location:** `scripts/perf/record-build.sh`

```bash
#!/bin/bash
# Canonical perf recording wrapper

record_build() {
    local name="$1"
    local output_dir="${2:-data/perf}"
    local command="${3:-nix build}"
    
    mkdir -p "$output_dir"
    perf record \
        -o "$output_dir/${name}.perf.data" \
        -F 99 \
        -g \
        $command
}

# Usage:
#   record_build "rust_build" "data/71_flakes_perf" "nix build"
```

### 2. Replace execute_workflows.sh

**Current:** 142 repetitive `perf record` commands

**Replace with:**
```bash
#!/bin/bash
source scripts/perf/record-build.sh

for lang in agda asm bash bazel brainfuck chisel cirq cmake coq datalog ...; do
    echo "Building $lang..."
    cd "const_71_test/$lang"
    record_build "${lang}_build" "../../data/71_flakes_perf"
    record_build "${lang}_rebuild" "../../data/71_flakes_perf" "nix build --rebuild --no-substitute"
    cd ../..
done
```

**Savings:** 142 → ~10 lines

### 3. Consolidate Documentation

#### Create Single Perf Guide
**Location:** `docs/perf/PERF_GUIDE.md`

Merge content from:
- `docs/perf/PERF_USAGE_AUDIT.md` (14 occurrences)
- `docs/nix/perf/README.md` (9 occurrences)
- `docs/perf/PERF_RECORDING_STATUS.md` (4 occurrences)
- `docs/build/BOOTSTRAP_RECORDING_PLAN.md` (4 occurrences)

**Structure:**
```markdown
# Perf Recording Guide

## Quick Start
record_build "my_build" "output/dir" "command"

## Standard Patterns
- Nix builds
- Rust compilation
- Bootstrap recording

## Analysis
- perf report
- perf script
- Galois field analysis

## Integration
- Bootstrap pipeline
- Nix flakes
- Telemetry collection
```

**Savings:** 31 → 1 file

### 4. Update Code References

#### Rust Code
- `perf-macros/src/lib.rs` (4 occurrences)
- `perf_canonical_recorder.rs` (4 occurrences)
- `src/workflow.rs` (3 occurrences)

**Action:** Point to canonical script in comments

#### Nix Flakes
- `nix/flakes/const_71_test/meta-perf/flake.nix` (6 occurrences)
- `perf-recorder/flake.nix` (4 occurrences)
- `nix/script-complexity.nix` (4 occurrences)

**Action:** Use canonical function via `callPackage`

### 5. Remove Duplicates

#### Duplicate Scripts
- `record_bootstrap_simple.sh` (3 occurrences)
- `tools/scripts/complete-bootstrap-performance.sh` (5 occurrences)

**Action:** Consolidate into `scripts/perf/record-bootstrap.sh`

#### Duplicate Docs
- `docs/misc/UNIVERSAL_SEMANTIC_LABELER.md` (4 occurrences)
- `docs/misc/GPU_SELF_SAMPLING.md` (4 occurrences)
- `docs/lore/poem.md` (7 occurrences - keep for historical value)

**Action:** Reference canonical guide, remove duplicate examples

## Implementation Plan

### Phase 1: Create Canonical Function
1. Create `scripts/perf/record-build.sh`
2. Add to bootstrap
3. Test with single build

### Phase 2: Replace execute_workflows.sh
1. Rewrite using canonical function
2. Test all 71 languages
3. Verify perf data output

### Phase 3: Consolidate Documentation
1. Create `docs/perf/PERF_GUIDE.md`
2. Merge 4 perf docs
3. Update references

### Phase 4: Update Code
1. Update Rust code comments
2. Update Nix flakes
3. Remove duplicate scripts

### Phase 5: Cleanup
1. Remove old docs
2. Update index
3. Verify all references

## Expected Results

**Before:**
- 368 occurrences
- 99 files
- Scattered documentation
- Repetitive code

**After:**
- ~50 occurrences (canonical + references)
- ~20 files (consolidated)
- Single perf guide
- DRY code

**Reduction:** 86% fewer occurrences, 80% fewer files

## Testing

```bash
# Test canonical function
source scripts/perf/record-build.sh
record_build "test" "data/test" "echo hello"

# Test execute_workflows.sh
./execute_workflows.sh

# Verify perf data
ls -lh data/71_flakes_perf/*.perf.data
```

## Rollback

All changes in feature branch:
```bash
git checkout feature/CRQ-001-nixify-pipeline
# Make changes
git commit -m "refactor: Consolidate perf record usage"

# If issues:
git revert HEAD
```

## Success Criteria

- [ ] Canonical function created
- [ ] execute_workflows.sh reduced to <20 lines
- [ ] Single perf guide document
- [ ] All tests pass
- [ ] Perf data still collected correctly
- [ ] 80%+ reduction in duplicates

---

**Ready to implement after nix reinstall completes.**
