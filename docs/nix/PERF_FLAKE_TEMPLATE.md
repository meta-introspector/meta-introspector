# Standard Flake Template

All perf recording flakes should follow this pattern:

## Template

```nix
{
  description = "My specific use case";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    perf-lib.url = "github:meta-introspector/meta-introspector?dir=nix";
  };
  
  outputs = { self, nixpkgs, perf-lib }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = perf-lib.lib.perfBuild {
        inherit pkgs;
        name = "my-build";
        buildCommand = "nix build .#target";  # Only this is custom
      };
    };
}
```

## What to Include

**ONLY include:**
- Your specific `buildCommand`
- Any custom inputs
- Custom output processing

**DO NOT include:**
- `perf record` command (use `perf-lib`)
- Boilerplate setup
- Standard flags (-F 99, -g, etc)

## Examples

### Simple Build Recording
```nix
{
  inputs.perf-lib.url = "github:meta-introspector/meta-introspector?dir=nix";
  
  outputs = { perf-lib, nixpkgs, ... }: {
    packages.x86_64-linux.default = perf-lib.lib.perfBuild {
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      name = "rust-build";
      buildCommand = "cargo build --release";
    };
  };
}
```

### With Overlay
```nix
{
  inputs.perf-lib.url = "github:meta-introspector/meta-introspector?dir=nix";
  
  outputs = { perf-lib, nixpkgs, ... }: {
    packages.x86_64-linux.default = 
      let pkgs = import nixpkgs { 
        overlays = [ perf-lib.overlays.perf ];
      };
      in pkgs.withPerf pkgs.hello;  # Adds perf recording to hello
  };
}
```

### Multiple Builds
```nix
{
  inputs.perf-lib.url = "github:meta-introspector/meta-introspector?dir=nix";
  
  outputs = { perf-lib, nixpkgs, ... }:
    let
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      record = name: cmd: perf-lib.lib.perfBuild {
        inherit pkgs name;
        buildCommand = cmd;
      };
    in {
      packages.x86_64-linux = {
        rust = record "rust" "cargo build";
        python = record "python" "python setup.py build";
        gcc = record "gcc" "make";
      };
    };
}
```

## Migration Guide

### Before (Boilerplate)
```nix
{
  outputs = { nixpkgs, ... }: {
    packages.x86_64-linux.default = pkgs.stdenv.mkDerivation {
      name = "my-build";
      buildPhase = ''
        mkdir -p $out/perf
        perf record -o $out/perf/build.perf.data -F 99 -g -- cargo build
      '';
    };
  };
}
```

### After (Template)
```nix
{
  inputs.perf-lib.url = "github:meta-introspector/meta-introspector?dir=nix";
  outputs = { perf-lib, nixpkgs, ... }: {
    packages.x86_64-linux.default = perf-lib.lib.perfBuild {
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      name = "my-build";
      buildCommand = "cargo build";
    };
  };
}
```

**Reduction:** 15 lines → 7 lines, no boilerplate

## Scripts

Scripts should use the library functions:

```bash
#!/bin/bash
# Use standard perf library via github
nix run github:meta-introspector/meta-introspector?dir=nix#perf-build -- .#target
```

## See Also

- `nix/perf-lib.nix` - Standard library (canonical implementation)
- `docs/perf/README.md` - Usage guide
- `perf-recorder/` - Interactive tool (uses perf-lib)
