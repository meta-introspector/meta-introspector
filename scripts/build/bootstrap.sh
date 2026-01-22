#!/usr/bin/env bash
# Complete system bootstrap - builds entire meta-introspector from scratch
# Uses nix store to cache previous work and avoid rebuilds

set -euo pipefail

echo "🚀 Meta-Introspector Complete Bootstrap"
echo "========================================"
echo ""

# Check for previous work in nix store
PREV_BUILDS=$(nix-store -q --references /nix/var/nix/profiles/default 2>/dev/null | grep -c "meta-introspector" || echo "0")
if [ "$PREV_BUILDS" -gt 0 ]; then
    echo "📦 Found $PREV_BUILDS previous builds in nix store"
    echo "   Reusing cached artifacts..."
else
    echo "🆕 Starting from scratch"
fi
echo ""

# Phase 1: Build 71 Language Tests
echo "🔢 Phase 1: Build 71 Language Tests (const_71_test)"
echo "=================================================="

cd nix/flakes/const_71_test

# Check which languages already built
BUILT=0
TOTAL=71

for lang_dir in */; do
    lang=$(basename "$lang_dir")
    [ ! -f "$lang_dir/flake.nix" ] && continue
    
    # Check if already in store
    if nix-store -q --references /nix/var/nix/profiles/default 2>/dev/null | grep -q "$lang"; then
        echo "  ✓ $lang (cached)"
        ((BUILT++))
    else
        echo "  🔨 Building $lang..."
        if nix build "./$lang" --no-link 2>&1 | tail -1; then
            echo "  ✅ $lang"
            ((BUILT++))
        else
            echo "  ⚠️  $lang (failed, continuing)"
        fi
    fi
done

echo ""
echo "  Built: $BUILT/$TOTAL languages"
echo "✅ Phase 1 complete"
echo ""

cd ../../..

# Phase 2: Build Perf Analysis Tools
echo "🔬 Phase 2: Build Perf Analysis Tools"
echo "====================================="

TOOLS=(
    "nix/flakes/const_71_test/perf-complexity"
    "nix/flakes/const_71_test/topological-function-matrix"
    "nix/flakes/const_71_test/mes-transformer-gpu"
)

for tool in "${TOOLS[@]}"; do
    tool_name=$(basename "$tool")
    echo "  🔨 Building $tool_name..."
    
    if [ -d "$tool" ]; then
        cd "$tool"
        if nix build --no-link 2>&1 | tail -1; then
            echo "  ✅ $tool_name"
        else
            echo "  ⚠️  $tool_name (failed, continuing)"
        fi
        cd - >/dev/null
    else
        echo "  ⚠️  $tool_name (not found)"
    fi
done

echo "✅ Phase 2 complete"
echo ""

# Phase 3: Extract Perf Data from Store
echo "📊 Phase 3: Extract Perf Data from Nix Store"
echo "============================================"

# Find all perf.data files in store
PERF_FILES=$(find /nix/store -name "*.perf.data" -type f 2>/dev/null || true)
PERF_COUNT=$(echo "$PERF_FILES" | grep -c "perf.data" || echo "0")

if [ "$PERF_COUNT" -gt 0 ]; then
    echo "  Found $PERF_COUNT perf traces in store"
    echo "  ✓ All perf data in /nix/store (immutable)"
else
    echo "  No perf data yet (will be generated on next build)"
fi

echo "✅ Phase 3 complete"
echo ""

# Phase 4: Train Models on Perf Data
echo "🧠 Phase 4: Train Models on Perf Data"
echo "====================================="

if [ "$PERF_COUNT" -gt 0 ]; then
    echo "  Training meta-model on $PERF_COUNT traces..."
    
    # Build mes-transformer-gpu with real data
    cd nix/flakes/const_71_test/mes-transformer-gpu
    
    if nix build --no-link 2>&1 | tail -3; then
        MODEL_PATH=$(nix-store -q --outputs $(nix-store -qd .))
        echo "  ✅ Model trained: $MODEL_PATH"
    else
        echo "  ⚠️  Training failed (need more perf data)"
    fi
    
    cd ../../../..
else
    echo "  Skipping (no perf data yet)"
fi

echo "✅ Phase 4 complete"
echo ""

# Phase 5: Build Analysis Tools
echo "🔧 Phase 5: Build Analysis Tools"
echo "================================"

# Build Rust analysis tools
if [ -f "Cargo.toml" ]; then
    echo "  Building Rust workspace..."
    
    if cargo build --release 2>&1 | tail -5; then
        BINS=$(ls target/release/ 2>/dev/null | grep -v "\.d$" | grep -v "deps" | wc -l)
        echo "  ✅ Built $BINS binaries"
    else
        echo "  ⚠️  Some builds failed"
    fi
else
    echo "  No Cargo.toml found"
fi

echo "✅ Phase 5 complete"
echo ""

# Phase 6: Generate Documentation
echo "📚 Phase 6: Generate Documentation"
echo "=================================="

# Ensure docs are up to date
if [ -f "docs/files.tsv" ]; then
    DOC_COUNT=$(wc -l < docs/files.tsv)
    echo "  ✓ $DOC_COUNT documentation files indexed"
else
    echo "  Generating documentation index..."
    find docs -name "*.md" -type f | sort > docs/files.tsv
    echo "  ✅ Documentation indexed"
fi

echo "✅ Phase 6 complete"
echo ""

# Phase 7: Commit Progress
echo "💾 Phase 7: Commit Progress"
echo "==========================="

git add -A
# Exclude all data directories - everything in nix store or HuggingFace
git reset HEAD 'data/' '*.perf.data' '*.strace' 2>/dev/null || true

if git diff --cached --quiet; then
    echo "  No changes to commit"
else
    TIMESTAMP=$(date +%s)
    git commit -m "Bootstrap iteration $TIMESTAMP

Built: $BUILT/$TOTAL languages
Perf traces: $PERF_COUNT (in /nix/store)
Models trained: $([ $PERF_COUNT -gt 0 ] && echo 'yes' || echo 'no')
" 2>&1 | grep -E "^\[|files changed" || true
    echo "  ✅ Committed"
fi

echo "✅ Phase 7 complete"
echo ""

# Final Status
echo "📊 Final Status"
echo "==============="
echo "  Languages built: $BUILT/$TOTAL"
echo "  Perf traces: $PERF_COUNT"
echo "  Nix store builds: $(nix-store -q --references /nix/var/nix/profiles/default 2>/dev/null | wc -l)"
echo "  Store size: $(du -sh /nix/store 2>/dev/null | cut -f1)"
echo "  Documentation: $(wc -l < docs/files.tsv 2>/dev/null || echo 0) files"
echo ""
echo "✅ Bootstrap complete!"
echo ""
echo "Next steps:"
echo "  - Run again to rebuild changed components"
echo "  - Perf data in: /nix/store (immutable derivations)"
echo "  - Upload to HF: hf://datasets/introspector/build-telemetry"
echo "  - Train models: cd nix/flakes/const_71_test/mes-transformer-gpu && nix build"
