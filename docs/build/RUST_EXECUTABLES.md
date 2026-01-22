# Rust Executables Inventory

We have **240 Rust executables** in the root directory.

## Strategy: Convert to Nix Pure Builds

Each executable should become a nix derivation that:
1. **Reads**: Input files/data (from nix store)
2. **Writes**: Output files (to $out in nix store)
3. **Pure**: No side effects, reproducible

## Analysis Template

For each `main()`:
```
Name: <executable_name>
Reads: <input files/sources>
Writes: <output files/results>
Purpose: <what it does>
Nix Job: analysis/NNN_<name>/flake.nix
```

## Current Nix Jobs

Already converted:
- ✅ 001_keywords - Extract terms, emoji labels
- ✅ 002_primes - Prime arithmetization
- ✅ 003_harmonic_filter - Name/impl harmony
- ✅ 004_markov_model - Markov sequences

## Next Conversions

### High Priority (Analysis Tools)

1. **build_value_lattice.rs**
   - Reads: Source files
   - Writes: Value lattice graph
   - Purpose: Build complexity lattice
   - Nix: analysis/005_value_lattice/

2. **intrinsic_complexity.rs**
   - Reads: Source files
   - Writes: Complexity metrics
   - Purpose: Measure intrinsic complexity
   - Nix: analysis/006_intrinsic_complexity/

3. **eigenvector_word_model.rs**
   - Reads: Term frequencies
   - Writes: Word eigenvectors
   - Purpose: Semantic embeddings
   - Nix: analysis/007_eigenvectors/

4. **symbol_similarity/** (8 binaries)
   - moonshine, label_mapper, eigenvector, etc
   - Purpose: Symbol analysis
   - Nix: analysis/008_symbol_similarity/

### Medium Priority (Build Tools)

5. **build_order_pipeline.rs**
   - Reads: Dependency graph
   - Writes: Build order
   - Purpose: Topological sort
   - Nix: analysis/009_build_order/

6. **nix_cargo_interceptor.rs**
   - Reads: Cargo builds
   - Writes: Telemetry
   - Purpose: Intercept cargo
   - Nix: analysis/010_cargo_intercept/

### Low Priority (Experimental)

7. **research/experimental/** (50+ demos)
   - Already isolated
   - Keep as research
   - Don't convert to nix jobs

## Conversion Process

For each executable:

1. **Analyze I/O**:
   ```bash
   grep -E "File::open|read|write|stdin|stdout" <file>.rs
   ```

2. **Create Nix Job**:
   ```nix
   packages.default = pkgs.stdenv.mkDerivation {
     name = "<executable>";
     src = ../..;
     
     buildPhase = ''
       mkdir -p $out/{input,output}
       
       # Run executable
       cargo run --release --bin <executable> \
         --input $src \
         --output $out/output
     '';
   };
   ```

3. **Add to Bootstrap**:
   ```bash
   echo "N️⃣  <Executable>..."
   nix build ./analysis/NNN_<name> --no-link
   ```

## Batch Conversion Script

```bash
#!/usr/bin/env bash
# Convert all executables to nix jobs

for rs_file in *.rs; do
    name=$(basename "$rs_file" .rs)
    
    # Check if has main
    if grep -q "fn main(" "$rs_file"; then
        echo "Converting: $name"
        
        # Create nix job directory
        mkdir -p "analysis/NNN_$name"
        
        # Generate flake.nix
        cat > "analysis/NNN_$name/flake.nix" <<EOF
{
  description = "$name - converted to nix job";
  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.default = nixpkgs.legacyPackages.x86_64-linux.rustPlatform.buildRustPackage {
      name = "$name";
      src = ../..;
      cargoLock.lockFile = ../../Cargo.lock;
    };
  };
}
EOF
    fi
done
```

## Benefits

1. **Reproducible**: Same input = same output
2. **Cacheable**: Nix reuses results
3. **Parallel**: Independent jobs run simultaneously
4. **Queryable**: All outputs in /nix/store
5. **Documented**: Each job has clear I/O

## Current Status

- Total executables: 240
- Converted to nix: 4
- Remaining: 236
- Target: Convert top 20 analysis tools

## Next Steps

1. Analyze I/O for top 20 executables
2. Create nix jobs (005-024)
3. Add to bootstrap pipeline
4. Document each job's purpose
