#!/usr/bin/env bash
set -euo pipefail

echo "🔧 Applying ZOS submodule policy to all submodules"

# Get list of submodules
SUBMODULES=$(git config --file .gitmodules --get-regexp path | awk '{print $2}')

for submodule in $SUBMODULES; do
    echo ""
    echo "📦 Processing: $submodule"
    
    # Skip if not initialized
    if [ ! -d "$submodule/.git" ]; then
        echo "  ⏭️  Skipping (not initialized)"
        continue
    fi
    
    cd "$submodule"
    
    # 1. Generate zos metadata
    echo "  📊 Generating metadata..."
    bash ../tools/scripts/collect-repo-metadata.sh . || true
    mkdir -p zos
    mv zos.toml zos/ 2>/dev/null || true
    
    # 2. Generate self/flake.nix
    echo "  ❄️  Generating self-building flake..."
    bash ../tools/scripts/generate-self-flake.sh . || true
    
    # 3. Inject template
    echo "  📋 Injecting template..."
    bash ../tools/scripts/inject-template.sh . || true
    
    # 4. Commit changes
    if git diff --quiet && git diff --cached --quiet; then
        echo "  ✓ No changes"
    else
        git add zos/ self/ 2>/dev/null || true
        git commit -m "feat: inject ZOS metadata and self-building flake" || echo "  ⚠️  Commit failed"
    fi
    
    cd - > /dev/null
    
    echo "  ✅ Done"
done

echo ""
echo "✅ All submodules processed"
