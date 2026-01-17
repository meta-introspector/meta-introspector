# Cross-Compilation Setup

## Quick Start

```bash
# Enter cross-compilation environment
nix-shell shell-cross.nix

# Build all targets
./cross-compile.sh

# Or build specific target
nix build --impure -f cross-compile.sh
```

## Targets

- **Windows**: x86_64-pc-windows-gnu (MinGW)
- **macOS Intel**: x86_64-apple-darwin
- **macOS ARM**: aarch64-apple-darwin

## Files

- `shell-cross.nix` - Nix shell with cross-compilation tools
- `cross-compile.sh` - Build script for all targets

## Usage

### Windows Cross-Compilation

```bash
nix build --impure --expr '
  let pkgs = import <nixpkgs> {
    crossSystem = { config = "x86_64-w64-mingw32"; };
  };
  in pkgs.rustPlatform.buildRustPackage {
    pname = "minimal-build-server";
    version = "0.1.0";
    src = ./.;
    cargoLock.lockFile = ./Cargo.lock;
  }
'
```

### Test Windows Binary

```bash
wine64 result-windows/bin/minimal-build-server.exe
```

## Requirements

- Nix with flakes enabled
- Wine (for testing Windows binaries)
- Darwin SDK (for macOS cross-compilation)
