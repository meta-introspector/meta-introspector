#!/usr/bin/env bash
set -euo pipefail

TEMPLATE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../template-repo" && pwd)"
TARGET_DIR="${1:-.}"

if [ ! -d "$TEMPLATE_DIR" ]; then
    echo "Error: Template directory not found at $TEMPLATE_DIR"
    exit 1
fi

echo "📦 Injecting standard Nix infrastructure into $TARGET_DIR"

# Copy flake.nix if not exists
if [ ! -f "$TARGET_DIR/flake.nix" ]; then
    cp "$TEMPLATE_DIR/flake.nix" "$TARGET_DIR/" 2>/dev/null || echo "No flake.nix in template"
fi

# Copy prelude if not exists
if [ -d "$TEMPLATE_DIR/prelude" ] && [ ! -d "$TARGET_DIR/prelude" ]; then
    cp -r "$TEMPLATE_DIR/prelude" "$TARGET_DIR/"
fi

# Copy Cargo.toml template if Rust project
if [ -f "$TARGET_DIR/Cargo.toml" ] && [ -f "$TEMPLATE_DIR/Cargo.toml" ]; then
    echo "✓ Rust project detected"
fi

echo "✅ Infrastructure injected"
