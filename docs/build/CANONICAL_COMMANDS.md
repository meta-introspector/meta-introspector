# Canonical Build Commands

## Problem

- `nix build`: 437 occurrences
- `cargo build`: 233 occurrences  
- `nix flake update`: 19 occurrences
- `git commit`: 121 occurrences
- `git add`: 102 occurrences

**Total: 912 duplicate command invocations**

## Solution

Single canonical wrapper for each command.

## Canonical Implementations

### Shell Scripts

```bash
# Nix build
scripts/build/nix.sh <args>

# Cargo build
scripts/build/cargo.sh <args>

# Flake update
scripts/build/flake.sh <args>

# Git add
scripts/git/add.sh <files>

# Git commit
scripts/git/commit.sh -m "message"
```

### Rust Library

```rust
use crate::build;
use crate::git;

// Nix build
build::nix_build(&[".#default"])?;

// Cargo build
build::cargo_build(&["--release"])?;

// Git operations
git::add(&["file.rs"])?;
git::commit("feat: add feature")?;
```

### Nix Library

```nix
{
  inputs.build-lib.url = "github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix";
  
  outputs = { build-lib, ... }: {
    buildPhase = build-lib.lib.nixBuild {
      pkgs = nixpkgs.legacyPackages.x86_64-linux;
      target = ".#default";
    };
  };
}
```

## Migration

### Before
```bash
nix build .#default
cargo build --release
git add file.rs
git commit -m "message"
```

### After
```bash
scripts/build/nix.sh .#default
scripts/build/cargo.sh --release
scripts/git/add.sh file.rs
scripts/git/commit.sh -m "message"
```

## Benefits

1. **Single source of truth** - One place to update
2. **Instrumentation** - Add telemetry to all builds
3. **Policy enforcement** - Add checks before commands
4. **Consistent behavior** - Same flags everywhere

## Future Enhancements

Wrappers can add:
- Automatic perf recording
- Build telemetry collection
- Pre-commit hooks
- Error handling
- Retry logic
- Caching

## See Also

- `scripts/perf/record.sh` - Canonical perf recording
- `src/perf/mod.rs` - Perf Rust library
- `nix/perf-lib.nix` - Perf Nix library
