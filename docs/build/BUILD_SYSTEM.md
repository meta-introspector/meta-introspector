# Meta-Introspector Build System

## Components

### 1. Nix Flake Integration
- **librustc**: `github:meta-introspector/librustc` - Spun-off rustc library loading
- **zos-server**: `github:meta-introspector/zos-server` - Cross-platform server
- **rust-telemetry-driver**: `github:meta-introspector/rust-telemetry-driver` - Build telemetry

### 2. Library Protocol
- `lib_protocol.rs` - Self-registration protocol for .so libraries
- `zos_loader.rs` - Generic loader that calls `zos_register()` and `zos_call()`
- `librustc_plugin.rs` - librustc self-registers capabilities (compile, parse, analyze)

### 3. Build Targets
```bash
nix build .#meta-introspector-binaries  # All 200+ analysis binaries
nix build .#zos-server                   # ZOS server
nix build .#rust-telemetry-driver        # Telemetry driver
nix build .#librustc                     # librustc plugin
```

### 4. GitHub Actions
- `.github/workflows/build.yml` - Builds all components on push
- Uses meta-introspector org actions only
- Caches nix builds

### 5. Bootstrap Process
```bash
./bootstrap.sh
```

**What it does:**
1. Builds all binaries via nix
2. Starts zos-server
3. Uses server to build itself (self-hosting)
4. Generates build report

### 6. Self-Hosted CLI
The system is designed as a monolithic CLI that:
- Loads .so libraries dynamically via protocol
- Discovers capabilities from libraries (no hardcoded rustc details in zos-server)
- Can load bld (https://github.com/meta-introspector/bld) as build plugin
- Self-builds using its own server

## Next Steps
1. Fix remaining compilation errors in individual binaries
2. Test bootstrap process
3. Integrate bld as build plugin
4. Add reporting dashboard
