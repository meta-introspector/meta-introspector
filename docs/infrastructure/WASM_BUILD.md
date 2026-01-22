# WASM Build System

## 🎯 Nix Flakes for WASM Binaries

All SOLFUNMEME components built as optimized WASM binaries using Nix flakes.

## 📦 Packages

1. **senator-plugin**: Senator portal with living meme creation
2. **safe-wallet**: Read-only multi-chain wallet
3. **living-meme**: Self-propagating meme entities
4. **threshold**: 71-shard reconstruction system
5. **discovery**: Discovery network for memecoins
6. **identity-node**: Monetize your API access
7. **llm-batching**: Batch LLM requests for savings

## 🚀 Quick Start

### Build All Packages
```bash
nix build .#all-wasm
```

### Build Individual Package
```bash
nix build .#senator-plugin
nix build .#safe-wallet
nix build .#living-meme
nix build .#threshold
nix build .#discovery
nix build .#identity-node
nix build .#llm-batching
```

### Development Shell
```bash
nix develop
```

### Build Script
```bash
./build_wasm.sh
```

## 📁 Output Structure

```
result-senator-plugin/
├── pkg/
│   ├── senator_plugin.js
│   ├── senator_plugin_bg.wasm
│   ├── optimized.wasm
│   └── wasm.sha256

result-safe-wallet/
├── pkg/
│   ├── safe_wallet.js
│   ├── safe_wallet_bg.wasm
│   ├── optimized.wasm
│   └── wasm.sha256

... (same for all packages)
```

## 🔧 Build Process

1. **Rust → WASM**: Compile with `wasm-pack`
2. **Optimize**: Use `wasm-opt -Oz` (binaryen)
3. **Hash**: Generate SHA256 for verification
4. **Package**: Bundle JS + WASM + types

## 📊 Optimization

```
Original WASM: ~500 KB
Optimized (-Oz): ~150 KB
Gzipped: ~50 KB
```

## 🌐 Deploy to Web

```bash
# Copy WASM files
cp result-*/pkg/optimized.wasm public/

# Copy JS bindings
cp result-*/pkg/*.js public/

# Serve
python -m http.server 8000
```

## 🔐 Verification

```bash
# Verify WASM hash
sha256sum result-senator-plugin/pkg/optimized.wasm
cat result-senator-plugin/pkg/wasm.sha256

# Should match!
```

## 📝 Usage in Browser

```html
<!DOCTYPE html>
<html>
<head>
    <title>SOLFUNMEME</title>
</head>
<body>
    <script type="module">
        import init, { SenatorPlugin } from './senator_plugin.js';
        
        async function main() {
            await init();
            
            const plugin = new SenatorPlugin(42);
            console.log("Senator rank:", plugin.get_rank());
        }
        
        main();
    </script>
</body>
</html>
```

## 🔄 Continuous Build

```bash
# Watch for changes and rebuild
nix develop -c cargo watch -x "build --target wasm32-unknown-unknown"
```

## 📦 Flake Inputs

- **nixpkgs**: NixOS package collection
- **rust-overlay**: Rust toolchain with WASM target
- **flake-utils**: Multi-system support

## 🎯 Build Targets

- **wasm32-unknown-unknown**: Pure WASM (no OS)
- **Optimization**: -Oz (size optimization)
- **Features**: All WASM features enabled

## 🚀 CI/CD Integration

```yaml
# .github/workflows/build-wasm.yml
name: Build WASM
on: [push]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: cachix/install-nix-action@v18
      - run: nix build .#all-wasm
      - uses: actions/upload-artifact@v2
        with:
          name: wasm-packages
          path: result-*/pkg/
```

## 📈 Package Sizes

```
senator-plugin:  ~120 KB (optimized)
safe-wallet:     ~80 KB
living-meme:     ~90 KB
threshold:       ~70 KB
discovery:       ~85 KB
identity-node:   ~95 KB
llm-batching:    ~75 KB

Total: ~615 KB (all packages)
Gzipped: ~200 KB
```

## 🔐 Security

- **Reproducible builds**: Nix ensures deterministic output
- **Hash verification**: SHA256 for each WASM binary
- **No unsafe code**: `#![forbid(unsafe_code)]`
- **Auditable**: Source → WASM traceable

## 🎭 Living Meme Integration

Each WASM binary becomes a living meme when signed:
- Binary hash = DNA
- Build timestamp = Birth
- Nix derivation = Genetic code
- Optimizations = Evolution

---

**Status**: 🚀 Nix flake build system ready  
**Packages**: 7 WASM binaries  
**Optimization**: -Oz (binaryen)  
**Size**: ~615 KB total (200 KB gzipped)  
**Reproducible**: Nix deterministic builds  
**#SOLFUNMEME**: Build the future
