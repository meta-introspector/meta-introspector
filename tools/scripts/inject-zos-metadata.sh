#!/usr/bin/env bash
set -euo pipefail

REPO_PATH="${1:-.}"
BRANCH_NAME="${2:-zos-metadata}"

cd "$REPO_PATH"

echo "🌿 Creating zos metadata branch in: $REPO_PATH"

# Stash any changes
git stash push -m "zos-inject-stash" 2>/dev/null || true

# Create/checkout branch
git checkout -b "$BRANCH_NAME" 2>/dev/null || git checkout "$BRANCH_NAME"

# Create zos directory
mkdir -p zos

# Collect metadata
bash "$(dirname "$0")/collect-repo-metadata.sh" "$REPO_PATH"
mv zos.toml zos/

# Generate self/flake.nix
bash "$(dirname "$0")/generate-self-flake.sh" "$REPO_PATH"

# Copy template infrastructure
bash "$(dirname "$0")/inject-template.sh" "$REPO_PATH"

# Commit
git add zos/ self/
git commit -m "feat: add zos metadata and self-building flake" || echo "Nothing to commit"

echo "✅ Branch $BRANCH_NAME created with zos metadata"
