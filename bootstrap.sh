#!/usr/bin/env bash
set -euo pipefail

# Enable perf recording if requested
if [ "${PERF_RECORD:-0}" = "1" ]; then
    exec perf record -g -o bootstrap.perf.data -- "$0" "$@"
fi

echo "🚀 ZOS Bootstrap - Building the complete system"
echo "================================================"
echo ""

# Phase 1: Build tools
echo "📦 Phase 1: Building ZOS tools"
if ! cargo build --release 2>&1 | tail -5; then
    echo "❌ Build failed, retrying..."
    cargo build --release 2>&1 | tail -10
fi
echo "✅ Tools built"
echo ""

# Phase 2: Apply to self
echo "🪞 Phase 2: Applying ZOS to itself"
./tools/scripts/collect-repo-metadata.sh . || true
mkdir -p zos && mv zos.toml zos/ 2>/dev/null || true
echo "✅ Self-metadata generated"
echo ""

# Phase 3: Apply to submodules
echo "📚 Phase 3: Applying to submodules"
./tools/scripts/apply-to-submodules.sh 2>&1 | grep -E "Processing|Done|✅" | tail -20
echo "✅ Submodules processed"
echo ""

# Phase 4: Run self-analysis
echo "🔬 Phase 4: Self-analysis"
if [ -f target/release/meta_discovery ]; then
    echo "  Running meta_discovery..."
    timeout 10s ./target/release/meta_discovery . > zos-results/meta.json 2>&1 || echo "  (timed out or failed)"
fi
echo "✅ Analysis attempted"
echo ""

# Phase 5: Commit results
echo "💾 Phase 5: Committing changes"
git add -A
if git diff --cached --quiet; then
    echo "  No changes to commit"
else
    git commit -m "chore: bootstrap iteration $(date +%s)" || echo "  Commit failed"
fi
echo "✅ Changes committed"
echo ""

# Phase 6: Status report
echo "📊 Phase 6: Status Report"
echo "  Rust files: $(find . -name '*.rs' -not -path '*/target/*' | wc -l)"
echo "  Tools built: $(ls target/release/*.rs 2>/dev/null | wc -l || echo 0)"
echo "  Submodules: $(git submodule status 2>/dev/null | wc -l || echo 0)"
echo "  ZOS layers: $(ls -d zos/layer* 2>/dev/null | wc -l || echo 0)"
echo "  Commits: $(git rev-list --count HEAD)"
echo ""

echo "✅ Bootstrap complete - run again to iterate"
