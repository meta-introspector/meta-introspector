#!/usr/bin/env bash
set -euo pipefail

REPO_PATH="${1:-.}"
OUTPUT="${2:-discovered_urls.txt}"

cd "$REPO_PATH"

echo "🔍 Extracting all URLs from: $REPO_PATH"

# Extract from .gitmodules
git config --file .gitmodules --get-regexp url | awk '{print $2}' > "$OUTPUT.tmp" 2>/dev/null || true

# Extract from git remotes
git remote -v | awk '{print $2}' | sort -u >> "$OUTPUT.tmp" 2>/dev/null || true

# Extract from Cargo.toml files
find . -name "Cargo.toml" -exec grep -oP '(?<=git = ")[^"]+' {} \; >> "$OUTPUT.tmp" 2>/dev/null || true

# Extract from flake.nix files
find . -name "flake.nix" -exec grep -oP 'github:[^";\s]+|https://github\.com/[^";\s]+' {} \; >> "$OUTPUT.tmp" 2>/dev/null || true

# Extract from shell scripts
find . -name "*.sh" -exec grep -oP 'https://[^\s"]+\.git|git@[^\s"]+' {} \; >> "$OUTPUT.tmp" 2>/dev/null || true

# Normalize and deduplicate
sort -u "$OUTPUT.tmp" | grep -E '^(https?://|git@)' > "$OUTPUT"
rm "$OUTPUT.tmp"

COUNT=$(wc -l < "$OUTPUT")
echo "✅ Found $COUNT unique URLs"
echo "📄 Saved to: $OUTPUT"
