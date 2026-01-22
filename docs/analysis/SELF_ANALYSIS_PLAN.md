# Binary Classification & Self-Analysis Plan

## Summary

**Total binaries:** 537
- Analysis: 27
- Telemetry: 25  
- Build: 59
- Git: 37
- Query: 3
- Test: 53
- Unknown: 333

## Plan: Run Each Binary on Our Own Codebase

### Phase 1: Analysis Binaries (27)

**Input: Git repo** → **Output: JSON/stdout**
```bash
# Run on meta-introspector itself
cargo run --bin analyze_cargo_deps -- /mnt/data1/meta-introspector > analysis/cargo_deps.json
cargo run --bin analyze_workspaces -- /mnt/data1/meta-introspector > analysis/workspaces.json
cargo run --bin cascading-repo-analyzer -- /mnt/data1/meta-introspector > analysis/cascading.json
```

**Input: Parquet** → **Output: stdout**
```bash
# Run on our own telemetry data
cargo run --bin analyze_char_transitions -- nix_build_logs.parquet
cargo run --bin analyze_transitions -- markov_symbol_scores.parquet
```

### Phase 2: Telemetry Binaries (25)

**Input: Perf data** → **Output: Parquet**
```bash
# Record our own build, then analyze it
scripts/perf/record.sh
cargo run --bin perf2parquet -- bootstrap perf_build_*.data
cargo run --bin query-parquet -- bootstrap_perf.parquet "SELECT * FROM bootstrap_perf LIMIT 10"
```

### Phase 3: Git Binaries (37)

**Input: Git repo** → **Output: JSON**
```bash
# Analyze our own git history
cargo run --bin all_commits_collector -- /mnt/data1/meta-introspector
cargo run --bin git_activity_tracker -- /mnt/data1/meta-introspector
cargo run --bin analyze_repo_ownership -- /mnt/data1/meta-introspector
```

### Phase 4: Build Binaries (59)

**Input: Source** → **Output: Artifacts**
```bash
# Build our own code with instrumentation
cargo run --bin mkbootstrap
cargo run --bin lattice_builder
cargo run --bin build_value_lattice
```

### Phase 5: Query Binaries (3)

**Input: Parquet** → **Output: Results**
```bash
# Query our own data
cargo run --bin query-parquet -- nix_build_logs.parquet "SELECT COUNT(*) FROM nix_build_logs"
```

## Nix Integration

Each binary becomes a nix derivation:

```nix
{
  inputs.perf-lib.url = "github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix";
  
  outputs = { perf-lib, ... }: {
    packages.analyze-self = perf-lib.lib.perfBuild {
      name = "analyze-self";
      buildCommand = ''
        # Run all analysis binaries on our own codebase
        cargo run --bin analyze_cargo_deps -- . > $out/cargo_deps.json
        cargo run --bin analyze_workspaces -- . > $out/workspaces.json
        cargo run --bin analyze_transitions -- *.parquet > $out/transitions.txt
      '';
    };
  };
}
```

## Expected Outputs

All stored in `/nix/store`:

```
/nix/store/xxx-analyze-self/
├── cargo_deps.json
├── workspaces.json
├── transitions.txt
├── git_activity.json
├── perf_analysis.parquet
└── build_telemetry.parquet
```

## Self-Analysis Loop

```
Source Code
  ↓
[Build with telemetry]
  ↓
Perf data + Parquet
  ↓
[Analysis binaries]
  ↓
JSON reports
  ↓
[Feed back to build]
  ↓
Improved code
```

## Implementation

Create `analysis/007_self_analysis/flake.nix`:

```nix
{
  description = "Run all analysis binaries on meta-introspector itself";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    perf-lib.url = "github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix";
  };
  
  outputs = { self, nixpkgs, perf-lib }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        name = "self-analysis";
        src = ../..;
        cargoLock.lockFile = ../../Cargo.lock;
        
        buildPhase = ''
          mkdir -p $out/analysis
          
          # Analysis binaries
          cargo run --release --bin analyze_cargo_deps -- . > $out/analysis/cargo_deps.json
          cargo run --release --bin analyze_workspaces -- . > $out/analysis/workspaces.json
          
          # Git binaries
          cargo run --release --bin all_commits_collector -- . > $out/analysis/commits.json
          
          # Telemetry binaries (if perf data exists)
          if ls *.perf.data 2>/dev/null; then
            cargo run --release --bin perf2parquet -- self *.perf.data
            mv self_perf.parquet $out/analysis/
          fi
        '';
      };
    };
}
```

## Next Steps

1. Create `analysis/007_self_analysis/flake.nix`
2. Add to bootstrap pipeline
3. Run: `nix build ./analysis/007_self_analysis`
4. Results in `/nix/store/xxx-self-analysis/analysis/`
5. Use results to improve code

## Success Criteria

- [ ] All 27 analysis binaries run on our codebase
- [ ] All 25 telemetry binaries process our perf data
- [ ] All 37 git binaries analyze our repo
- [ ] Results stored in /nix/store
- [ ] Self-analysis loop established
- [ ] Insights fed back into development

---

**This is true meta-introspection: using our tools on ourselves!**
