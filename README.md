# Meta-Introspector Project Documentation

This repository contains the source code and documentation for the Meta-Introspector project. Our goal is to consolidate and provide comprehensive, easily accessible documentation for all aspects of the project.

## Recent Compilation Fixes (2026-01-17)

Successfully resolved all blocking compilation errors! The codebase now builds cleanly with `cargo build`.

### Key Issues Resolved:

1. **Missing Macro Crates**: Created separate `bootstrap-macros/` and `telemetry-macros/` crates for procedural macros
2. **Import Path Corrections**: Fixed `crate::rand_shim` → `libnix::rand_shim` and other module path issues
3. **Struct Field Additions**: Added missing fields (`godel_number`, `node_id`, `telemetry_enabled`) to various structs
4. **Stub Type Definitions**: Created stub types for missing dependencies (`NixBuildRequest`, `NixBuildResult`, etc.)
5. **Axum Handler Fixes**: Corrected handler signatures and return types for axum 0.7 compatibility
6. **Type Annotations**: Added explicit type annotations for empty vectors and ambiguous types
7. **Syntax Errors**: Fixed mismatched braces, extra closing delimiters, and format string argument counts

### Build Status

✅ **Compilation**: All binaries now compile successfully  
⚠️ **Warnings**: Some unused variables and imports remain (non-blocking)

### Building with Nix

The project includes a comprehensive Nix flake for reproducible builds:

```bash
# Build the minimal server
nix build .#minimal-build-server

# Build all 220 binaries
nix build .#meta-introspector-binaries

# Enter development shell with telemetry
nix develop

# Build specific packages
nix build .#telemetry-driver
nix build .#zos
```

**Note**: `linuxPackages.perf` is disabled by default. To enable perf support, uncomment the line in `flake.nix`.

## Documentation Status

We are currently in the process of setting up a centralized documentation portal.

### General Project Documentation (Markdown & Text Files)

All general project documentation, including architectural overviews, research findings, deployment notes, and guides (originally in various Markdown and text files), has been moved into the `docs/` directory.

A static documentation website is being set up using **MkDocs** and the `mkdocs-material` theme. This website will be published to GitHub Pages, providing a searchable and navigable interface for our non-code-specific documentation.

### Rust Code API Documentation (Rustdoc)

✅ **Status**: Compilation errors resolved! You can now generate Rust API documentation.

**Generate Documentation:**

```bash
cargo doc --no-deps --open
```

This will build documentation for all crates and open it in your browser.

## How to Access Documentation

*   **Static Site (MkDocs)**: Once deployed to GitHub Pages, the general project documentation will be accessible via: `https://meta-introspector.github.io/`
*   **Rustdoc (Local)**: After the compilation errors are resolved, you can generate `rustdoc` locally by running `cargo doc`. The output will be in `target/doc/`.

## Contributing to Documentation

Please add new documentation files to the `docs/` directory. For Rust code, ensure you use `///` and `//!` doc comments following Rust's documentation conventions.
