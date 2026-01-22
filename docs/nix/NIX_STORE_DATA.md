# Nix Store Data Management

## Problem
`data/` directory is mutable and can be corrupted. All analysis results should be in `/nix/store/` for immutability and reproducibility.

## Solution

### 1. Store perf data in derivations

```nix
{
  packages.rust-71-perf = pkgs.stdenv.mkDerivation {
    name = "rust-71-perf";
    src = ./const_71_test/rust;
    nativeBuildInputs = [ pkgs.perf ];
    buildPhase = ''
    '';
    installPhase = ''
      mkdir -p $out
      cp perf.data $out/rust_build.perf.data
    '';
  };
}
```

### 2. Store analysis results in derivations

```nix
{
  packages.rust-71-analysis = pkgs.stdenv.mkDerivation {
    name = "rust-71-analysis";
    src = self.packages.rust-71-perf;
    nativeBuildInputs = [ harmonic_analyzer ];
    buildPhase = ''
      harmonic_analyzer $src/rust_build.perf.data > analysis.txt
    '';
    installPhase = ''
      mkdir -p $out
      cp analysis.txt $out/rust_analysis.txt
    '';
  };
}
```

### 3. Store lattice in derivation

```nix
{
  packages.complexity-lattice = pkgs.stdenv.mkDerivation {
    name = "complexity-lattice";
    src = ./.;
    buildInputs = [ 
      self.packages.rust-71-analysis
      self.packages.python-71-analysis
      # ... all 71 analyses
    ];
    nativeBuildInputs = [ lattice_builder ];
    buildPhase = ''
      # Collect all analyses
      mkdir -p analyses
      ${lib.concatMapStringsSep "\n" (lang: ''
        cp ${self.packages."${lang}-71-analysis"}/${lang}_analysis.txt analyses/
      '') all_languages}
      
      # Build lattice
      lattice_builder analyses > lattice.json
    '';
    installPhase = ''
      mkdir -p $out
      cp lattice.json $out/
      cp lattice.dot $out/
    '';
  };
}
```

## Benefits

1. **Immutable**: Once built, data cannot be corrupted
2. **Reproducible**: Same inputs = same outputs
3. **Cacheable**: Nix cache can share results
4. **Traceable**: Full dependency graph
5. **Garbage collected**: Old results cleaned automatically

## Migration

```bash
# Old way (mutable)
./target/release/mkbootstrap
ls data/71_results/

# New way (immutable)
nix build .#complexity-lattice
ls result/
```

## Implementation

All mkbootstrap! workflows should generate nix derivations, not write to `data/`.

```rust
// Instead of:
fs::write("data/71_results/rust_analysis.txt", analysis)?;

// Generate nix derivation:
generate_nix_derivation("rust-71-analysis", inputs, build_script)?;
```
