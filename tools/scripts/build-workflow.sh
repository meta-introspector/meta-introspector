#!/bin/bash
# Automated workflow: commit → push → mirror → flake update → build

set -e

REPO_PATH="/mnt/data1/nix/time/2025/06/01/solfunmeme-dioxus"
MIRROR_PATH="/mnt/data1/git/github.com/meta-introspector/solfunmeme-dioxus"
FLAKE_PATH="/mnt/data1/meta-introspector/nix/solfunmeme-dioxus.nix"
GITCONFIG="/mnt/data1/meta-introspector/nix/gitconfig-local-mirrors"

echo "🔄 Automated Build Workflow"
echo "============================"
echo ""

# Step 1: Commit changes
echo "📝 Step 1: Committing changes..."
cd "$REPO_PATH"
if [[ -n $(git status -s) ]]; then
    git add -A
    git commit -m "Auto-commit: $(date -I)" || echo "Nothing to commit"
fi

# Step 2: Push to GitHub
echo "📤 Step 2: Pushing to GitHub..."
git push origin feature/block-collector

# Step 3: Update local mirror
echo "🔄 Step 3: Updating local mirror..."
cd "$MIRROR_PATH"
git fetch --all
echo "✅ Mirror updated"

# Step 4: Flake update (with local mirror redirect)
echo "🔄 Step 4: Updating flake..."
export GIT_CONFIG_GLOBAL="$GITCONFIG"
cd "$(dirname $FLAKE_PATH)"
nix flake update solfunmeme-dioxus 2>&1 | tail -5

# Step 5: Build
echo "🔨 Step 5: Building with Nix..."
nix-build "$FLAKE_PATH" 2>&1 | tail -10

echo ""
echo "✅ Build complete!"
echo "📦 Result: $(readlink -f result)"
