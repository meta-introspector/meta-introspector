#!/usr/bin/env bash
# Build WASM version with nix store in localStorage

echo "🦀 Building WASM with localStorage nix store..."

# Install wasm-pack if needed
command -v wasm-pack >/dev/null || cargo install wasm-pack

# Build for web
wasm-pack build --target web --out-dir pkg

echo "✅ Built! Open index.html in browser"
echo "📦 Nix store will use localStorage"
