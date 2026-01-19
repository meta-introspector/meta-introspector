#!/bin/bash
# Build senator plugin as SO and WASM

set -e

echo "🔨 Building Senator Plugin..."

cd senator_plugin

# Build as shared object
echo "📦 Building SO..."
cargo build --release --lib
cp target/release/libsenator_plugin.so ../senator_plugin.so

# Generate SO hash
SO_HASH=$(sha256sum ../senator_plugin.so | awk '{print $1}')
echo "✅ SO hash: $SO_HASH"

# Build as WASM
echo "🌐 Building WASM..."
wasm-pack build --target web --release

# Generate WASM hash
WASM_HASH=$(sha256sum pkg/senator_plugin_bg.wasm | awk '{print $1}')
echo "✅ WASM hash: $WASM_HASH"

# Create manifest
cat > ../senator_plugin_manifest.json <<EOF
{
  "name": "senator_plugin",
  "version": "0.1.0",
  "so_hash": "$SO_HASH",
  "wasm_hash": "$WASM_HASH",
  "build_timestamp": $(date +%s),
  "signature": ""
}
EOF

echo "📝 Manifest created: senator_plugin_manifest.json"
echo "✅ Build complete!"
