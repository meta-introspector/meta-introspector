# Consolidation: Nix Store Perf Data Usage

## Current State

Files using nix store for perf data:

### Documentation
- `BOOTSTRAP.md` - Documents perf data in store
- `MEMO_NIX_STORE.md` - Policy: never use find
- `README.md` - Quick start mentions perf

### Scripts
- `bootstrap.sh` - References perf data (but doesn't create derivations yet)
- `capture_multidimensional_messages.sh` - Uses perf
- `tools/scripts/record-cargo2nix.sh` - Attempts to create derivation (has syntax error)

### Tools
- `conformity_test.rs` - Looks for perf data in store
- `extract_orbits.rs` - Reads perf data

## Problems

1. **No actual derivations created**: Perf data not in /nix/store yet
2. **Syntax errors**: `record-cargo2nix.sh` has Nix expression errors
3. **Using find**: Some scripts violate policy
4. **No flake inputs**: Not using proper flake input pattern

## Correct Pattern (from policy)

```nix
{
  inputs = {
    perf-data.url = "github:meta-introspector/meta-introspector?dir=results";
  };
  
  outputs = { self, perf-data, ... }: {
    analyze = analyze-orbits perf-data;
  };
}
```

## Action Plan

### 1. Fix `record-cargo2nix.sh`
Remove the broken Nix expression, just record perf data locally.

### 2. Create proper derivation
```nix
packages.perf-data = stdenv.mkDerivation {
  name = "bootstrap-perf-data";
  src = ./.;
  buildPhase = ''
  '';
};
```

### 3. Reference via flake input
```nix
inputs.bootstrap-perf.url = "path:./result-perf";
packages.analysis = analyze-orbits bootstrap-perf;
```

### 4. Store references in git
```json
{
  "timestamp": "2026-01-20T19:36:00-05:00",
  "nix_store_path": "/nix/store/abc-perf-data",
  "hf_dataset": "hf://datasets/introspector/build-telemetry"
}
```

## Files to Update

1. `tools/scripts/record-cargo2nix.sh` - Remove broken Nix expr
2. `flake.nix` - Add perf-data derivation
3. `bootstrap.sh` - Use derivation, not local files
4. `conformity_test.rs` - Take flake input, not find
5. `extract_orbits.rs` - Take flake input, not find

## References

- `MEMO_NIX_STORE.md` - Policy
- `zos/CROSS_MODAL_RESONANCE.md` - Usage pattern
- `/mnt/data1/nix/source/.../nix_path_purity.md` - External policy
