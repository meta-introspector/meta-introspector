#!/usr/bin/env bash
# Build all WASM packages with Nix

set -e

echo "🚀 Building SOLFUNMEME WASM Packages"
echo ""

# Build individual packages
echo "📦 Building senator-plugin..."
nix build .#senator-plugin -o result-senator-plugin

echo "📦 Building safe-wallet..."
nix build .#safe-wallet -o result-safe-wallet

echo "📦 Building living-meme..."
nix build .#living-meme -o result-living-meme

echo "📦 Building threshold..."
nix build .#threshold -o result-threshold

echo "📦 Building discovery..."
nix build .#discovery -o result-discovery

echo "📦 Building identity-node..."
nix build .#identity-node -o result-identity-node

echo "📦 Building llm-batching..."
nix build .#llm-batching -o result-llm-batching

echo ""
echo "✅ All WASM packages built!"
echo ""
echo "📊 Package sizes:"
for result in result-*; do
    if [ -d "$result/pkg" ]; then
        size=$(du -sh "$result/pkg/optimized.wasm" | cut -f1)
        hash=$(cat "$result/pkg/wasm.sha256" | cut -d' ' -f1 | cut -c1-16)
        echo "  $result: $size (hash: $hash...)"
    fi
done

echo ""
echo "🌐 Deploy to web:"
echo "  cp result-*/pkg/*.wasm public/"
echo "  cp result-*/pkg/*.js public/"
