#!/usr/bin/env bash
# Wrap nix build to add metadata headers to all derivations
# This allows us to find and query all our builds in the store

set -euo pipefail

# Generate metadata
METADATA=$(cat <<EOF
{
  "meta-introspector": {
    "version": "1.0",
    "timestamp": "$(date -Iseconds)",
    "commit": "$(git rev-parse HEAD 2>/dev/null || echo 'unknown')",
    "branch": "$(git branch --show-current 2>/dev/null || echo 'unknown')",
    "builder": "bootstrap",
    "hostname": "$(hostname)",
    "build_id": "$(date +%s)-$$"
  }
}
EOF
)

# Create wrapper flake that adds metadata
WRAPPER_DIR=$(mktemp -d)
trap "rm -rf $WRAPPER_DIR" EXIT

cat > "$WRAPPER_DIR/flake.nix" <<'EOF'
{
  description = "Meta-introspector build with metadata";
  
  inputs.target.url = "path:${TARGET_FLAKE}";
  
  outputs = { self, target }: {
    packages = target.packages // {
      default = target.packages.default.overrideAttrs (old: {
        # Add metadata to derivation
        passthru = (old.passthru or {}) // {
          meta-introspector-metadata = builtins.fromJSON ''${METADATA}'';
        };
        
        # Store metadata in output
        postInstall = (old.postInstall or "") + ''
          mkdir -p $out/.meta-introspector
          cat > $out/.meta-introspector/metadata.json <<'METADATA'
          ${METADATA}
          METADATA
        '';
      });
    };
  };
}
EOF

# Export for flake
export TARGET_FLAKE="$1"
export METADATA

# Build with wrapper
cd "$WRAPPER_DIR"
nix build --no-link "${@:2}"

# Show where metadata is stored
RESULT=$(nix-store -qR $(nix-store -qd .) | grep "meta-introspector" | head -1)
if [ -n "$RESULT" ]; then
    echo ""
    echo "✅ Build complete with metadata"
    echo "   Store path: $RESULT"
    echo "   Metadata: $RESULT/.meta-introspector/metadata.json"
fi
