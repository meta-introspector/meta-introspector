# Perf Record Cleanup Plan

## Goal

## Existing Canonical Patterns (DO NOT RECREATE)

   - Examples: `mes-bootstrap-proof/flake.nix`, `nix/script-complexity.nix`
   
2. **Interactive Tool** - `perf-recorder/flake.nix`
   - Usage: `nix run ./perf-recorder#perf-build -- .#target`

3. **Script Pattern** - `scripts/build/build_and_analyze_const71.sh`
   - For one-off analysis

## Cleanup Actions

### 1. execute_workflows.sh (142 occurrences) - SIMPLIFY


**Action:** Keep as interface, call nix derivations

**New execute_workflows.sh:**
```bash
#!/bin/bash


# Build the derivation that records perf for all languages
nix build ./nix/flakes/const_71_test#perf-all

# Link results
ln -sf result perf-data

echo "✅ Perf data in: perf-data/perf/"
echo "Analyze: perf report -i perf-data/perf/rust.perf.data"
```

**Backend:** `nix/flakes/const_71_test/flake.nix` add:
```nix
packages.perf-all = stdenv.mkDerivation {
  name = "const71-perf-data";
  buildPhase = ''
    mkdir -p $out/perf
    ${lib.concatMapStringsSep "\n" (lang: ''
      cd const_71_test/${lang}
      cd ../..
    '') languages}
  '';
};
```

**Result:** 142 lines → 10 lines (interface) + nix derivation (backend)

### 2. Duplicate Docs (31 occurrences in 4 files) - MERGE

**Files to merge:**
- `docs/perf/PERF_USAGE_AUDIT.md` (14)
- `docs/nix/perf/README.md` (9)
- `docs/perf/PERF_RECORDING_STATUS.md` (4)
- `docs/build/BOOTSTRAP_RECORDING_PLAN.md` (4)

**Action:** 
1. Keep `docs/perf/README.md` (already exists)
2. Add section "Canonical Patterns" with 3 examples
3. Delete the 4 duplicate files
4. Update references

**Content:**
```markdown
# Perf Recording

## Canonical Patterns

### 1. Nix Derivation (Immutable)
See: mes-bootstrap-proof/flake.nix

### 2. Interactive
See: perf-recorder/
Usage: nix run ./perf-recorder#perf-build -- .#target

### 3. Script
See: scripts/build/build_and_analyze_const71.sh
```

**Result:** 4 files → 1 file

### 3. Duplicate Scripts (8 occurrences) - DELETE

**Files to delete:**
- `record_bootstrap_simple.sh` (3) - use perf-recorder instead
- `tools/scripts/complete-bootstrap-performance.sh` (5) - use nix derivation

**Action:** Delete both, update any references to use perf-recorder or nix derivation

**Result:** 2 files deleted

### 4. Code Comments (15 occurrences) - UPDATE

**Files:**
- `perf-macros/src/lib.rs` (4)
- `perf_canonical_recorder.rs` (4)
- `src/workflow.rs` (3)
- Others (4)

**Action:** Update comments to reference canonical patterns:
```rust
// Record perf data in nix derivation:
// 
// Or use perf-recorder interactively:
//   nix run ./perf-recorder#perf-build -- .#target
//
// See: docs/perf/README.md
```

**Result:** 15 comments updated

### 5. Nix Flakes (14 occurrences) - KEEP BUT DOCUMENT

**Files:**
- `nix/flakes/const_71_test/meta-perf/flake.nix` (6)
- `perf-recorder/flake.nix` (4)
- `nix/script-complexity.nix` (4)

**Action:** These are the canonical examples! Add comment at top:
```nix
# CANONICAL PATTERN: Store perf data in $out
# This is the reference implementation.
# See: docs/perf/README.md
```

**Result:** 3 files kept, marked as canonical

### 6. Misc Docs (remaining ~163 occurrences) - UPDATE REFERENCES

**Action:** Search and replace in all remaining files:
- Replace with: "See canonical patterns in docs/perf/README.md"
- Keep only 1-2 line references, not full examples

## Implementation Steps

1. **Create nix derivation for const71** (replaces execute_workflows.sh)
2. **Merge 4 docs into README.md** with canonical patterns section
3. **Delete duplicate scripts** (2 files)
4. **Update code comments** (15 files)
5. **Mark canonical flakes** (3 files)
6. **Update remaining docs** (search/replace)
7. **Delete execute_workflows.sh**
8. **Update any references** to deleted files

## Expected Results

**Before:**
- 368 occurrences
- 99 files
- Scattered patterns
- Duplicated examples

**After:**
- ~30 occurrences (canonical + references)
- ~25 files (70% reduction)
- 3 canonical patterns clearly documented
- All others reference these

**Reduction:** 92% fewer occurrences, 75% fewer files

## Testing

```bash
# Test nix derivation
nix build .#perf-all

# Verify perf data in store
ls -la /nix/store/*-const71-perf-data/perf/

# Test perf-recorder still works
nix run ./perf-recorder#perf-build -- .#default

# Verify docs reference canonical patterns
```

## Rollback

All changes in feature branch, can revert if needed.

## Success Criteria

- [ ] execute_workflows.sh deleted
- [ ] 4 duplicate docs merged into 1
- [ ] 2 duplicate scripts deleted
- [ ] Canonical patterns clearly documented
- [ ] All perf data stored in /nix/store
- [ ] <50 total occurrences remaining
- [ ] All occurrences either canonical or references
