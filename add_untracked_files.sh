#!/bin/bash
# Add all untracked Rust files and documentation

set -e

echo "🔍 Finding untracked files..."

# Find untracked Rust files
UNTRACKED_RS=$(comm -23 <(find . -maxdepth 1 -name "*.rs" -type f | sort) <(git ls-files "*.rs" | sort))
COUNT_RS=$(echo "$UNTRACKED_RS" | wc -l)

echo "📊 Found $COUNT_RS untracked Rust files"

# Add new documentation
echo "📝 Adding new documentation..."
git add docs/CANONICAL_DATA_SYSTEM.md
git add docs/FILE_IO_INVENTORY.md
git add docs/UNIFIED_INDEX_SYSTEM.md

# Add HuggingFace pusher
echo "🤗 Adding HuggingFace pusher..."
git add push_to_hf.rs

# Add all untracked Rust files
echo "📦 Adding $COUNT_RS Rust files..."
echo "$UNTRACKED_RS" | while read -r file; do
    if [ -n "$file" ]; then
        echo "  + $file"
        git add "$file"
    fi
done

# Show status
echo ""
echo "✅ Files staged for commit:"
git status --short | head -20
echo ""
echo "Total staged: $(git diff --cached --name-only | wc -l) files"

echo ""
echo "📝 Ready to commit with:"
echo "   git commit -m 'Add 280 untracked Rust files and documentation'"
