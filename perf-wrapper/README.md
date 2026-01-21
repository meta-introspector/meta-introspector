# Perf Wrapper

Wraps any Nix derivation to record perf data during build.

## Usage

### Wrap a single package

```nix
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    perf-wrapper.url = "path:./perf-wrapper";
  };

  outputs = { self, nixpkgs, perf-wrapper }:
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs { inherit system; };
    in {
      packages.${system} = {
        # Original package
        my-tool = pkgs.rustPlatform.buildRustPackage { ... };
        
        # Wrapped with perf
        my-tool-with-perf = perf-wrapper.lib.wrapWithPerf {
          inherit pkgs;
          drv = self.packages.${system}.my-tool;
        };
      };
    };
}
```

### Use overlay to wrap all packages

```nix
{
  outputs = { self, nixpkgs, perf-wrapper }:
    let
      pkgs = import nixpkgs {
        system = "x86_64-linux";
        overlays = [ perf-wrapper.overlays.default ];
      };
    in {
      packages.x86_64-linux = {
        # Automatically wrapped
        rustc-with-perf = pkgs.withPerf "rustc";
        cargo-with-perf = pkgs.withPerf "cargo";
      };
    };
}
```

## Output Structure

```
/nix/store/abc-package-with-perf/
├── bin/              # Original outputs
├── lib/
└── perf/
    ├── build.perf.data
    └── metadata.json
```

## Analyze Results

```bash
# Build with perf
nix build .#my-tool-with-perf

# Extract perf data
cp result/perf/build.perf.data ./

# Analyze
perf report -i build.perf.data

# Or use our tools
nix build .#analyze-orbits ./result
```

## Integration with ZOS

```nix
{
  inputs = {
    zos.url = "github:meta-introspector/meta-introspector";
    perf-wrapper.url = "github:meta-introspector/meta-introspector?dir=perf-wrapper";
  };

  outputs = { self, zos, perf-wrapper }:
    let
      # Wrap ZOS build with perf
      zos-with-perf = perf-wrapper.lib.wrapWithPerf {
        pkgs = zos.pkgs;
        drv = zos.packages.x86_64-linux.default;
      };
      
      # Analyze the perf data
      analysis = zos.packages.x86_64-linux.analyze-orbits zos-with-perf;
    in {
      packages.x86_64-linux = {
        inherit zos-with-perf analysis;
      };
    };
}
```

## Benefits

1. **Reproducible**: Perf data stored in /nix/store
2. **Content-addressed**: Same build → same perf data
3. **Composable**: Wrap any derivation
4. **Traceable**: Metadata included
5. **Policy-compliant**: No `find`, uses flake inputs
