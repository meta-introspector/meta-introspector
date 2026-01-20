# MEMO: Never Use `find` on /nix/store

## Reference

Based on: `/mnt/data1/nix/source/github/meta-introspector/streamofrandom/2025/10/10/docs/sop/nix_path_purity.md`

See also: https://github.com/meta-introspector/crq-binstore

## The Problem

**WRONG:**
```bash
find /nix/store -name "build.perf.data"
PERF_FILES=$(find /nix/store -name "*.perf.data" | wc -l)
```

This violates Nix path purity:
- Impure (non-deterministic results)
- Breaks reproducibility
- Bypasses dependency tracking
- Violates external dependency policy

## The Right Way: Flake Inputs

**CORRECT:**
```nix
{
  inputs = {
    # Reference specific derivation by GitHub URL
    perf-data.url = "github:meta-introspector/meta-introspector?ref=main&dir=results";
    
    # Or use output from another build
    crq-binstore.url = "github:meta-introspector/crq-binstore";
  };
  
  outputs = { self, perf-data, crq-binstore, ... }: {
    # Use as explicit input
    analyze = analyze-orbits perf-data;
  };
}
```

## Pattern: Data Flow Through Derivations

```nix
# Step 1: Build with perf recording
build-with-perf = stdenv.mkDerivation {
  buildPhase = ''
    perf record -o $out/perf/build.perf.data -- make
  '';
};

# Step 2: Analyze takes build as explicit input
analyze = perfData: stdenv.mkDerivation {
  buildPhase = ''
    extract_orbits ${perfData}/perf/build.perf.data > $out/orbits.txt
  '';
};

# Step 3: Use it
packages.analysis = analyze build-with-perf;
```

## Policy

All external dependencies **must be referenced exclusively by their `github:meta-introspector` URLs**.

Format:
```
github:meta-introspector/<repo>?ref=<branch>&dir=<path>
```

**Prohibited:**
- `path:./some/path`
- `self + "/some/path"`
- `find /nix/store`
- Any local path references

## Benefits

1. **Reproducibility**: Same inputs → same outputs
2. **Content-addressed**: Nix store resolution works correctly
3. **Policy compliance**: Centralized, auditable sources
4. **Clear dependency graph**: Explicit inputs

## Action Items

- [x] Remove all `find /nix/store` from code
- [ ] Use flake inputs for all data dependencies
- [ ] Reference via `github:meta-introspector` URLs
- [ ] Make each analysis step a pure derivation

## References

- SOP: nix_path_purity.md
- crq-binstore: https://github.com/meta-introspector/crq-binstore
- time-2025: https://github.com/meta-introspector/time-2025
