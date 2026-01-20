# Self-Building Flake System

Every repo gets a `self/flake.nix` that builds the repo from its GitHub URL and branch.

## Structure

```
repo/
├── zos/
│   └── zos.toml          # Metadata
└── self/
    └── flake.nix         # Self-building flake
```

## Standard Build Location

All repos build to a standard Nix store path:

```bash
# Build any repo
nix build github:OWNER/REPO/BRANCH#default

# Output: /nix/store/...-REPO
```

## Features

- **Hermetic**: Builds from GitHub URL, not local checkout
- **Reproducible**: Same inputs = same output path
- **Metadata-aware**: Reads zos/zos.toml for version info
- **Auto-detect**: Rust (Cargo.toml) or generic build
- **Standard location**: All builds in /nix/store

## Usage

### Generate for Single Repo

```bash
./tools/scripts/generate-self-flake.sh /path/to/repo
```

### Build from Self Flake

```bash
cd /path/to/repo
nix build ./self#default
ls -l result/
```

### Build from GitHub

```bash
nix build github:meta-introspector/meta-introspector/main#default
```

## Integration with NixOps

```bash
# Atomic operation: inject + build + verify
nixops-run inject-and-build /path/to/repo

# This will:
# 1. Generate self/flake.nix
# 2. Build in pure Nix environment
# 3. Store result in /nix/store
# 4. Record build metadata
```

## Benefits

- **Consistency**: Every repo builds the same way
- **Traceability**: Build path encodes all inputs
- **Caching**: Nix binary cache for all builds
- **Isolation**: No dependency on local state
- **Scalability**: Works across thousands of repos
