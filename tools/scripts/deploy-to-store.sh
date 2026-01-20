#!/bin/bash
# Deploy WASM plugins to content-addressed store

set -e

STORE_DIR="/home/mdupont/zos-server/www/store"
mkdir -p "$STORE_DIR"

echo "📦 Building and deploying plugins to content-addressed store..."

# Build plugin
cd /mnt/data1/nix/time/2025/06/01/solfunmeme-dioxus/plugins/solana-p2p
cargo build --target wasm32-unknown-unknown --release

# Generate bindings
cd /mnt/data1/nix/time/2025/06/01/solfunmeme-dioxus
~/.cargo/bin/wasm-bindgen target-wasm-solfunmeme/wasm32-unknown-unknown/release/solana_p2p_plugin.wasm \
    --out-dir /tmp/wasm-build --target web --no-typescript

# Calculate hash and create store path
WASM_FILE="/tmp/wasm-build/solana_p2p_plugin_bg.wasm"
HASH=$(sha256sum "$WASM_FILE" | awk '{print $1}' | cut -c1-32)
STORE_PATH="$STORE_DIR/$HASH-solana-p2p-plugin"

mkdir -p "$STORE_PATH"
cp /tmp/wasm-build/* "$STORE_PATH/"

echo "✅ Deployed to: $STORE_PATH"
echo "$HASH" > "$STORE_DIR/latest-solana-p2p"

# Create manifest
cat > "$STORE_PATH/manifest.json" << EOF
{
  "name": "solana-p2p-plugin",
  "hash": "$HASH",
  "files": {
    "wasm": "solana_p2p_plugin_bg.wasm",
    "js": "solana_p2p_plugin.js"
  },
  "version": "$(cd /mnt/data1/nix/time/2025/06/01/solfunmeme-dioxus && git rev-parse --short HEAD)"
}
EOF

echo "📋 Manifest:"
cat "$STORE_PATH/manifest.json"
echo ""
echo "🔗 Load URL: /store/$HASH-solana-p2p-plugin/solana_p2p_plugin.js"
