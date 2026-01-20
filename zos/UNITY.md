# Unity Control System

Central flake that all repos include for unified management.

## Architecture

```
meta-introspector/meta-introspector (v1 branch)
└── zos/unity/flake.nix  ← Central control point

All other repos:
└── self/flake.nix  ← Includes unity as input
```

## Unity Flake Location

```
github:meta-introspector/meta-introspector/v1?dir=zos/unity
```

## What Unity Provides

- **Standard build functions**: `mkPackage` for Rust/generic builds
- **Common tools**: git, nix, rustc, cargo
- **Dev shell**: Consistent environment across all repos
- **Version control**: Unity v1, v2, etc. for coordinated updates

## Usage in Repos

Every repo's `self/flake.nix` includes:

```nix
inputs = {
  unity.url = "github:meta-introspector/meta-introspector/v1?dir=zos/unity";
  # ...
};

outputs = { unity, ... }: {
  packages.default = unity.lib.mkPackage { ... };
  devShells.default = unity.devShells.default;
};
```

## Benefits

- **Single source of truth**: Update unity → all repos updated
- **Consistent builds**: Same build logic everywhere
- **Version pinning**: Lock to unity v1, upgrade to v2 when ready
- **Centralized control**: Manage thousands of repos from one flake

## Workflow

1. **Create v1 branch** in meta-introspector
2. **Inject unity** into all repos via mass-inject-zos.sh
3. **Update unity** to add features/tools
4. **All repos** automatically get updates on next build

## Example: Adding a Tool

```nix
# In zos/unity/flake.nix
unityTools = pkgs.buildEnv {
  paths = [
    # Add new tool here
    pkgs.my-new-tool
  ];
};
```

All repos now have `my-new-tool` available.
