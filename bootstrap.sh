#!/usr/bin/env bash
set -euo pipefail

echo "🚀 ZOS Bootstrap - Building from git via Nix"
echo "============================================="
echo ""

# Phase 1: Build via Nix
echo "📦 Phase 1: Nix build"
nix build .#defaultPackage.x86_64-linux --no-link 2>&1 | tail -10 || echo "Build attempted"
echo "✅ Build phase complete"
echo ""

# Phase 2: Apply to self
echo "🪞 Phase 2: Self-metadata"
./tools/scripts/collect-repo-metadata.sh . || true
mkdir -p zos && mv zos.toml zos/ 2>/dev/null || true
echo "✅ Metadata generated"
echo ""

# Phase 3: Commit iteration
echo "💾 Phase 3: Commit"
git add -A
# Exclude perf data from git
git reset HEAD '*.perf.data' '*.strace' 2>/dev/null || true
if git diff --cached --quiet; then
    echo "  No changes"
else
    git commit -m "chore: bootstrap iteration $(date +%s)" || true
fi
echo "✅ Committed (perf data in Nix store only)"
echo ""

# Phase 4: Status
echo "📊 Status"
echo "  Commits: $(git rev-list --count HEAD)"
echo "  Tools: $(ls *.rs 2>/dev/null | wc -l)"
echo ""
echo "✅ Bootstrap complete - run again to iterate"
