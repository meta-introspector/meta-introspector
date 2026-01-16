# Flake Update: Use rust-telemetry-driver as Remote Input

**Date**: 2026-01-15  
**Status**: Committed, pending flake.lock update

## Changes Made

### flake.nix

**Added rust-telemetry-driver as flake input:**
```nix
inputs = {
  nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  rust-overlay.url = "github:oxalica/rust-overlay";
  flake-utils.url = "github:numtide/flake-utils";
  rust-telemetry-driver.url = "github:meta-introspector/rust-telemetry-driver";
  rust-telemetry-driver.inputs.nixpkgs.follows = "nixpkgs";
};
```

**Updated outputs signature:**
```nix
outputs = { self, nixpkgs, rust-overlay, flake-utils, rust-telemetry-driver }:
```

**Removed local build, use remote package:**
```nix
# Before: Built from local submodule
rust-telemetry-driver = pkgs.rustPlatform.buildRustPackage {
  pname = "rust-telemetry-driver";
  version = "0.1.0";
  src = ./rust-telemetry-driver;
  cargoLock.lockFile = ./rust-telemetry-driver/Cargo.lock;
  nativeBuildInputs = with pkgs; [ pkg-config ];
};

# After: Use from flake input
TELEMETRY_DRIVER="${rust-telemetry-driver.packages.${system}.default}/bin/rust-telemetry-driver"
```

## Benefits

1. **No submodule Cargo.lock issues** - Nix won't try to track files in submodules
2. **Cleaner separation** - rust-telemetry-driver is its own flake
3. **Easier updates** - `nix flake lock --update-input rust-telemetry-driver`
4. **Cacheable** - Binary cache can be shared across projects

## Next Steps

### Update flake.lock (when GitHub API available)

```bash
# Wait for GitHub API rate limit to reset, then:
nix flake lock --update-input rust-telemetry-driver

# Or update all inputs:
nix flake update
```

### Alternative: Manual flake.lock entry

If GitHub API continues to have issues, manually add to `flake.lock`:

```json
{
  "nodes": {
    "rust-telemetry-driver": {
      "inputs": {
        "nixpkgs": ["nixpkgs"]
      },
      "locked": {
        "lastModified": 1234567890,
        "narHash": "sha256-...",
        "owner": "meta-introspector",
        "repo": "rust-telemetry-driver",
        "rev": "...",
        "type": "github"
      },
      "original": {
        "owner": "meta-introspector",
        "repo": "rust-telemetry-driver",
        "type": "github"
      }
    }
  }
}
```

### Workaround: Use local path temporarily

If needed, can temporarily use local path:

```nix
rust-telemetry-driver.url = "path:./rust-telemetry-driver";
```

Then switch back to GitHub URL once rate limits reset.

## Testing

Once flake.lock is updated:

```bash
# Test the development shell
nix develop

# Should see telemetry shell active
# Build a test project to verify telemetry capture
```

## Error Fixed

**Before**: 
```
error: Path 'rust-telemetry-driver/Cargo.lock' in the repository 
"/mnt/data1/meta-introspector" is not tracked by Git.
```

**After**: 
No longer tries to access submodule files, uses remote flake instead.
