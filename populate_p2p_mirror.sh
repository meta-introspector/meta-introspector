#!/bin/bash
# Populate P2P mirror with all known repos (reusing existing scan code)

set -e

echo "🔍 Populating P2P mirror with all known repositories..."

# 1. Use existing git-sources registry
echo "📋 Step 1: Loading from git-sources registry..."
if [ -f ./target/release/git-sources ]; then
    ./target/release/git-sources list | grep "Path:" | awk '{print $2}' > /tmp/repos_from_registry.txt
    echo "  Found $(wc -l < /tmp/repos_from_registry.txt) repos in registry"
fi

# 2. Use existing scan-all-submodules.sh
echo "📋 Step 2: Running scan-all-submodules.sh..."
./scan-all-submodules.sh 2>/dev/null | grep "→" | awk '{print $2}' > /tmp/repos_from_submodules.txt || true
echo "  Found $(wc -l < /tmp/repos_from_submodules.txt) submodules"

# 3. Use existing ingest_git_data.sh logic
echo "📋 Step 3: Running ingest_git_data.sh..."
./ingest_git_data.sh 2>/dev/null | grep "Remote:" | awk '{print $3}' > /tmp/repos_from_ingest.txt || true
echo "  Found $(wc -l < /tmp/repos_from_ingest.txt) remotes"

# 4. Merge all
cat /tmp/repos_from_*.txt 2>/dev/null | sort -u > ~/nix/index/all_repo_urls.txt
echo "✅ Total: $(wc -l < ~/nix/index/all_repo_urls.txt) unique repos"
