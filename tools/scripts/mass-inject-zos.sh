#!/usr/bin/env bash
set -euo pipefail

REPOS_FILE="${1:-data/master_url_list.txt}"
BRANCH_NAME="${2:-zos-metadata}"
MIRROR_BASE="${3:-/mnt/data1/git}"

if [ ! -f "$REPOS_FILE" ]; then
    echo "Error: Repos file not found: $REPOS_FILE"
    exit 1
fi

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INJECT_SCRIPT="$SCRIPT_DIR/inject-zos-metadata.sh"

echo "🚀 Mass injection of zos metadata into all repos"
echo "📋 Reading from: $REPOS_FILE"
echo "🌿 Branch: $BRANCH_NAME"
echo ""

TOTAL=$(wc -l < "$REPOS_FILE")
COUNT=0

while IFS= read -r url; do
    COUNT=$((COUNT + 1))
    
    # Convert URL to local path
    REPO_NAME=$(basename "$url" .git)
    REPO_PATH="$MIRROR_BASE/$REPO_NAME"
    
    if [ ! -d "$REPO_PATH/.git" ]; then
        echo "[$COUNT/$TOTAL] ⏭️  Skipping (not cloned): $REPO_NAME"
        continue
    fi
    
    echo "[$COUNT/$TOTAL] 📦 Processing: $REPO_NAME"
    
    (
        cd "$REPO_PATH"
        bash "$INJECT_SCRIPT" "$REPO_PATH" "$BRANCH_NAME" 2>&1 | sed 's/^/  /'
    ) || echo "  ❌ Failed"
    
done < "$REPOS_FILE"

echo ""
echo "✅ Mass injection complete: $COUNT repos processed"
