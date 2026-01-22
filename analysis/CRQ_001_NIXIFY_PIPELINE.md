# CRQ-001: Nixify Meta-Introspector Analysis Pipeline

## Problem Statement

The meta-introspector project has 236 Rust executables in the root directory that need to be converted to pure, reproducible Nix builds. Currently, these executables are built ad-hoc without:
- Clear input/output specifications
- Reproducible build environments
- Integration with the analysis pipeline
- Governance documentation

## Proposed Solution

Systematically convert the top 20 priority executables (identified by meta-analysis) to Nix flakes with:
1. Pure builds (no side effects)
2. Declared inputs from nix store
3. Outputs to $out
4. CRQ documentation for each
5. Integration with bootstrap pipeline

## Scope

**In Scope:**
- Top 20 executables from meta-analysis priority list
- CRQ document for each conversion
- Nix flake for each executable
- Bootstrap integration
- Documentation updates

**Out of Scope:**
- Remaining 216 executables (future CRQs)
- Modification of original .rs files
- Changes to core analysis tools (001-005)

## Technical Details

### Current State
```
meta-introspector/
├── *.rs (240 executables in root)
├── analysis/
│   ├── 001_keywords/
│   ├── 002_primes/
│   ├── 003_harmonic_filter/
│   ├── 004_markov_model/
│   ├── 005_meta_analysis/
│   └── 006_crq_integration/
└── scripts/build/bootstrap.sh
```

### Target State
```
meta-introspector/
├── *.rs (original files preserved)
├── analysis/
│   ├── 001-006/ (existing)
│   ├── 007_<executable1>/
│   │   ├── flake.nix
│   │   └── CRQ_007.md
│   ├── 008_<executable2>/
│   │   ├── flake.nix
│   │   └── CRQ_008.md
│   └── ... (through 026)
└── scripts/build/bootstrap.sh (updated)
```

### Architecture

**Nix Flake Template:**
```nix
{
  description = "<executable> - nixified";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    meta-introspector.url = "path:../..";
  };
  
  outputs = { self, nixpkgs, meta-introspector }: {
    packages.x86_64-linux.default = pkgs.rustPlatform.buildRustPackage {
      name = "<executable>";
      src = meta-introspector;
      cargoLock.lockFile = ../../Cargo.lock;
      
      buildPhase = ''
        cargo build --release --bin <executable>
      '';
      
      installPhase = ''
        mkdir -p $out/{bin,analysis}
        cp target/release/<executable> $out/bin/
        # Run analysis, output to $out/analysis/
      '';
    };
  };
}
```

**CRQ Template:**
Each conversion gets a CRQ following ai-ml-zk-ops format (see analysis/006_crq_integration/merged/CRQ_TEMPLATE.md).

## Implementation Plan

### Phase 1: Setup (This CRQ)
1. Create feature branch: `feature/CRQ-001-nixify-pipeline`
2. Create CRQ document structure
3. Run meta-analysis to get priority list
4. Document branching workflow

### Phase 2: Convert Top 5 (CRQ-002 through CRQ-006)
5. Generate CRQs for executables 7-11
6. Create nix flakes
7. Test builds
8. Update bootstrap

### Phase 3: Convert Next 15 (CRQ-007 through CRQ-021)
9. Generate CRQs for executables 12-26
10. Create nix flakes
11. Test builds
12. Update bootstrap

### Phase 4: Integration (CRQ-022)
13. Merge all branches
14. Update documentation
15. Verify complete pipeline

## Testing Plan

### Unit Tests
```bash
# Test each flake builds
nix build ./analysis/007_<name>
nix build ./analysis/008_<name>
# ... etc

# Verify outputs
ls -la result/analysis/
```

### Integration Tests
```bash
# Test bootstrap runs all
./bootstrap

# Verify all results in nix store
ls -la /nix/store/*-<executable>/
```

### Validation
- [ ] All 20 executables build successfully
- [ ] All outputs in /nix/store
- [ ] Bootstrap completes without errors
- [ ] All CRQs documented
- [ ] Git history clean with CRQ references

## Rollback Plan

1. Checkout main branch
2. Original .rs files unchanged
3. Remove analysis/007-026/ directories
4. Revert bootstrap.sh changes

## Dependencies

- Existing analysis jobs (001-006) ✅
- Meta-analysis priority list (from 005) ⏳
- ai-ml-zk-ops CRQ system (integrated in 006) ✅
- Nix build system ✅

## Success Criteria

- [ ] 20 executables converted to nix flakes
- [ ] 20 CRQs documented
- [ ] Bootstrap runs all 26 jobs (001-026)
- [ ] All results queryable in /nix/store
- [ ] Documentation updated
- [ ] Merged to main with clean history

## References

- Meta-Introspector Guide: `docs/META_INTROSPECTOR_GUIDE.md`
- CRQ Integration: `analysis/006_crq_integration/`
- Bootstrap: `scripts/build/bootstrap.sh`
- ai-ml-zk-ops CRQ Guide: `github:meta-introspector/ai-ml-zk-ops/e3551db`

## Timeline

- Week 1: Phase 1 (Setup)
- Week 2: Phase 2 (Top 5)
- Week 3: Phase 3 (Next 15)
- Week 4: Phase 4 (Integration)

## Commit References

- Initial CRQ: [this commit]
- Implementation: [to be added]
- Completion: [to be added]

---

**Status:** 🚧 In Progress  
**Created:** 2026-01-22  
**Branch:** `feature/CRQ-001-nixify-pipeline`  
**Author:** Meta-Introspector Team
