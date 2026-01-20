#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INJECT_SCRIPT="$SCRIPT_DIR/inject-template.sh"

echo "🔄 Applying template to all tool projects..."

for project in tools/p2p-block-collector tools/so-plugins/* tools/wasm-plugins/*; do
    if [ -d "$project" ] && [ -f "$project/Cargo.toml" ]; then
        echo "Processing: $project"
        "$INJECT_SCRIPT" "$project"
    fi
done

echo "✅ All projects updated"
