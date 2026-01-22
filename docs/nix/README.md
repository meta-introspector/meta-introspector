# Nix Documentation Index

Consolidated documentation for Nix builds, derivations, and reproducibility.

## Core Concepts

### Nix Store
- `/nix/store/`: Immutable content-addressed storage
- Each build = unique derivation hash
- Reproducible: same inputs → same outputs

### Derivations
- `.drv` files: Build instructions
- Outputs: `/nix/store/xxx-package/`
- Dependencies: Explicit in derivation

## Build System

### Flakes
Modern Nix with `flake.nix`:

```nix
{
  inputs = { nixpkgs.url = "github:NixOS/nixpkgs"; };
  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.default = /* ... */;
  };
}
```

### Building
```bash
nix build .#package
nix build github:user/repo#package
```

## Integration with Perf

See: `docs/nix/perf/` for detailed integration.

### Perf-Wrapped Builds
```nix
perf-wrapper.lib.wrapWithPerf {
  inherit pkgs;
  package = pkgs.myPackage;
}
```

Output includes perf data:
```
/nix/store/xxx-myPackage-with-perf/
├── bin/
└── perf/
    └── build.perf.data
```

### Training Derivations
Each build = training batch:

```nix
packages.default = pkgs.stdenv.mkDerivation {
  preBuild = ''
  '';
  
  postBuild = ''
    # Train model on perf data
    train-model --perf $out/perf/build.perf.data
  '';
};
```

## 71 Languages Test Suite

Location: `const_71_test/`

Each language has a flake that outputs "71":

```bash
cd const_71_test/rust && nix build --impure
./result/bin/const71
# Output: 71
```

### With Perf
```bash
nix build .#rust-with-perf
ls result/perf/build.perf.data
```

## Topological Build Order

Nix dependency graph = topological ordering:

```
mes → tcc → gcc → rust → haskell
```

Each node = layer in function matrix.

See: `const_71_test/topological-function-matrix/`

## Reproducibility

### Content Addressing
- Hash of inputs determines output path
- Same derivation = same `/nix/store/xxx` path
- Bit-for-bit reproducible

### Training Data
- Perf data stored in nix store
- Reproducible training: same derivation = same perf data
- Distributed training: merge results from multiple stores

## Tools

### nix_builder.sh
Queue and build projects with telemetry:

```bash
./nix_builder.sh queue /path/to/project
./nix_builder.sh watch
```

### Bootstrap
```bash
./bootstrap.sh
```


## References

- Main README: `README.md`
- Perf Integration: `docs/nix/perf/README.md`
- 71 Languages: `const_71_test/*/flake.nix`
- Training Derivations: `const_71_test/mes-transformer-gpu/flake.nix`
