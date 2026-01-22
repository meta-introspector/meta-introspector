# Nix Build Configuration

## Production Builds

```bash
# Build production tools (excludes demos)
nix build .#tools

# Build specific tool
nix build .#analyze-and-prove

# Enter dev environment
nix develop
```

## Archived Demos (Quarantined)

⚠️ **WARNING**: Demos contain fake data and incomplete implementations

```bash
# Build archived demos for analysis only
nix build .#demos

# Output location
./result/bin/archived-demos/

# These binaries are NOT production-ready
```

## Package Structure

### `.#tools` (Production)
- ✅ Production-ready binaries
- ✅ Full error handling
- ✅ Real data sources
- ✅ Tested and maintained

Includes:
- `reach_tracer`
- `source2test`
- `harmonic_filter`
- `homotopy_classifier`
- `fake_detector`
- `fake_replacer`
- And more...

### `.#demos` (Archived)
- ❌ NOT production-ready
- ❌ Contains fake data
- ❌ Incomplete implementations
- ⚠️ For analysis only

Includes:
- `demo_compression_*`
- `demo_content_*`
- `demo_p2p_*`
- And 45 more...

## Build Flags

Production builds use:
```nix
cargoBuildFlags = [
  "--bins"
  "--exclude-bin" "archived_demos"
];
```

This ensures demos are never included in production builds.

## Usage

### Production
```bash
# Build and run production tool
nix build .#tools
./result/bin/reach_tracer input.rs
```

### Analysis (Demos)
```bash
# Build demos for binary analysis
nix build .#demos

# Analyze demo binaries
./result/bin/archived-demos/demo_compression_study
# WARNING: May panic on fake data!
```

## CI/CD

Production CI should only build `.#tools`:
```yaml
- name: Build production
  run: nix build .#tools
  
- name: Test production
  run: nix build .#tools && ./result/bin/fake_detector src/
```

Demos can be built separately for analysis:
```yaml
- name: Build demos (analysis only)
  run: nix build .#demos
  continue-on-error: true  # May fail due to fake data
```

## Safety

1. **Production builds** exclude all demos automatically
2. **Demo builds** are isolated in separate package
3. **Warnings** are displayed when building demos
4. **README** files warn about fake data

## Verification

```bash
# Verify production build has no demos
nix build .#tools
ls result/bin/ | grep demo
# Should return nothing

# Verify demos are quarantined
nix build .#demos
ls result/bin/archived-demos/ | grep demo
# Should list demo binaries
```
