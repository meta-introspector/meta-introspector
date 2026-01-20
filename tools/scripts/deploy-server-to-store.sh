#!/bin/bash
# Deploy ZOS server to content-addressed store

set -e

STORE_ROOT="/nix/store"
BUILD_DIR="$HOME/zos-server"

echo "📦 Building ZOS server..."
cd "$BUILD_DIR"
cargo build --release

# Calculate hash of entire build output
BINARY="$BUILD_DIR/target/release/zos_server"
WWW_DIR="$BUILD_DIR/www"

# Create temporary directory with all runtime dependencies
TEMP_ROOT=$(mktemp -d)
mkdir -p "$TEMP_ROOT/bin"
mkdir -p "$TEMP_ROOT/www"

cp "$BINARY" "$TEMP_ROOT/bin/"
cp -r "$WWW_DIR"/* "$TEMP_ROOT/www/" 2>/dev/null || true

# Calculate hash of entire store path
HASH=$(find "$TEMP_ROOT" -type f -exec sha256sum {} \; | sort | sha256sum | awk '{print $1}' | cut -c1-32)
STORE_PATH="$STORE_ROOT/$HASH-zos-server"

echo "📁 Creating store path: $STORE_PATH"
sudo mkdir -p "$STORE_PATH"
sudo cp -r "$TEMP_ROOT"/* "$STORE_PATH/"
sudo chmod -R 755 "$STORE_PATH"

# Create metadata
sudo tee "$STORE_PATH/metadata.json" > /dev/null << EOF
{
  "name": "zos-server",
  "hash": "$HASH",
  "version": "$(git rev-parse --short HEAD)",
  "built": "$(date -Iseconds)",
  "binary": "bin/zos_server",
  "www": "www"
}
EOF

# Create symlink to latest
sudo ln -sfn "$STORE_PATH" "$STORE_ROOT/zos-server-latest"

echo "✅ Deployed to: $STORE_PATH"
echo "🔗 Latest: $STORE_ROOT/zos-server-latest"
echo "📋 Metadata:"
cat "$STORE_PATH/metadata.json"

# Create launcher script
cat > "$BUILD_DIR/run-from-store.sh" << 'LAUNCHER'
#!/bin/bash
STORE_PATH=$(readlink -f /nix/store/zos-server-latest)
HASH=$(basename "$STORE_PATH" | cut -d'-' -f1)
echo "🚀 Starting ZOS server from store: $HASH"
cd "$STORE_PATH"
exec ./bin/zos_server serve
LAUNCHER

chmod +x "$BUILD_DIR/run-from-store.sh"
echo "🎯 Run with: $BUILD_DIR/run-from-store.sh"

rm -rf "$TEMP_ROOT"
