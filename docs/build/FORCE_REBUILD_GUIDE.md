# Force Nix Rebuild Strategies

## Problem
Nix uses cached builds from `/nix/store`. We're recording file copying, not actual compilation.

## Solutions

### 1. Disable Binary Cache
```bash
# Force local builds only
nix build --option substitute false .#package

# Or set in config
nix build --no-substitute .#package
```

### 2. Add Timestamp to Force Rebuild
```nix
# In flake.nix
{
  packages.default = pkgs.stdenv.mkDerivation {
    name = "const71-${builtins.toString builtins.currentTime}";  # Forces new derivation
    # ... rest
  };
}
```

### 3. Use --rebuild Flag
```bash
nix build --rebuild .#package
```

### 4. Clear Local Cache
```bash
# Remove specific package
nix-store --delete /nix/store/*const71*

# Or garbage collect
nix-collect-garbage -d
```

### 5. Build from Source (Best for our use case)
```nix
{
  packages.default = pkgs.stdenv.mkDerivation {
    name = "const71";
    
    # Don't use pre-built binaries
    dontUseCmakeConfigure = true;
    
    # Force compilation
    buildPhase = ''
      # Compile from scratch
      gcc -o const71 const71.c
    '';
  };
}
```

### 6. Impure Build (Nuclear Option)
```bash
# Allow impure evaluation
nix build --impure --expr '
  with import <nixpkgs> {};
  stdenv.mkDerivation {
    name = "const71-${toString (builtins.currentTime)}";
    buildCommand = "gcc -o $out/const71 ${./const71.c}";
  }
'
```

## For Our 71 Languages

### Strategy: Add timestamp to each flake
```nix
# const_71_test/rust/flake.nix
{
  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.default = 
      let timestamp = builtins.toString builtins.currentTime;
      in pkgs.rustPlatform.buildRustPackage {
        name = "const71-rust-${timestamp}";
        # This forces a new derivation each time
      };
  };
}
```

### Rebuild Script
```bash
#!/usr/bin/env bash
# force_rebuild.sh

LANG=$1
PERF_DIR="data/real_compilation_perf"

cd const_71_test/$LANG

# Update timestamp in flake
sed -i "s/timestamp = [0-9]*/timestamp = $(date +%s)/" flake.nix

# Build without substitutes
  nix build --no-substitute --rebuild

# Run the result
  ./result/bin/*
```

## For Mes Bootstrap

The mes-perf-recorder needs to actually **execute** the bootstrap, not just copy it:

```nix
# mes-perf-recorder/flake.nix
{
  packages.default = pkgs.writeShellScript "run-mes-bootstrap" ''
    # Don't use cached bootstrap
    # Actually run Mes interpreter
    ${pkgs.mes}/bin/mes --version
    
    # Build TinyCC from source using Mes
    # (This is the hard part - need actual bootstrap chain)
  '';
}
```

## Verification

After rebuild, check perf data:
```bash
perf report -i real_compile.perf.data --stdio | head -20

# Should see:
# - rustc (not just sha256sum)
# - gcc/clang (actual compiler)
# - ld (linker)
# NOT just: sha256sum, cp, dd
```

## Next Steps

1. Update all 71 flakes with timestamp
2. Create force_rebuild.sh script
3. Run on each language
4. Verify perf shows actual compilation
5. Compare Galois numbers of real compilation vs cached builds

---

**Key Insight**: We need to see the **compiler** in the perf data, not just file operations!
