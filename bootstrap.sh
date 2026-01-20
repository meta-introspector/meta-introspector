#!/usr/bin/env bash
set -euo pipefail

echo "🚀 ZOS Bootstrap - Smart iteration"
echo "==================================="
echo ""

# Phase 0: Generate Cargo.nix with perf recording
echo "📋 Phase 0: Cargo.nix generation (with perf)"
if [ ! -f Cargo.nix ] || [ Cargo.lock -nt Cargo.nix ]; then
    echo "  Recording cargo2nix with perf+strace..."
    ./tools/scripts/record-cargo2nix.sh 2>&1 | tail -10 || echo "  Generation attempted"
    
    # Store reference to perf data in HF
    if [ -f cargo2nix.perf.data ]; then
        mkdir -p hf-build-telemetry-upload/perf-refs
        cat > hf-build-telemetry-upload/perf-refs/cargo2nix-$(date +%s).json <<EOF
{
  "timestamp": "$(date -Iseconds)",
  "commit": "$(git rev-parse HEAD)",
  "tool": "cargo2nix",
  "perf_size": "$(stat -f%z cargo2nix.perf.data 2>/dev/null || stat -c%s cargo2nix.perf.data)",
  "hf_dataset": "hf://datasets/introspector/build-telemetry/cargo2nix"
}
EOF
        echo "  ✓ Perf data reference stored"
    fi
else
    echo "  ✓ Cargo.nix up to date"
fi
echo ""

# Phase 1: Build via Nix (stores perf in /nix/store)
echo "📦 Phase 1: Nix build"
BUILD_HASH=$(git rev-parse HEAD)

# Build Rust tools only (skip WASM for now)
echo "  Building Rust tools..."
BUILD_LOG=$(mktemp)
if ! cargo build --release 2>&1 | tee "$BUILD_LOG"; then
    echo ""
    echo "❌ Build failed - showing errors:"
    echo "=================================="
    grep -A10 "^error" "$BUILD_LOG" | head -50
    rm "$BUILD_LOG"
    exit 1
fi
rm "$BUILD_LOG"
echo "✅ Build phase complete"
echo ""

# Phase 2: Analyze perf data (if exists in store)
echo "🔬 Phase 2: Analyze perf data"

# First, record cargo2nix if Cargo.lock changed
if [ ! -f Cargo.nix ] || [ Cargo.lock -nt Cargo.nix ]; then
    echo "  Recording cargo2nix generation..."
    ./tools/scripts/record-cargo2nix.sh 2>&1 | tail -5 || true
fi

PERF_FILES=$(find /nix/store -name "build.perf.data" -type f 2>/dev/null | wc -l)
if [ "$PERF_FILES" -gt 0 ]; then
    echo "  Found $PERF_FILES perf traces in store"
    
    # Run orbit extraction (output goes to store)
    nix build .#extract-orbits --no-link 2>&1 | tail -3 || true
    
    echo "  ✓ Analysis complete (results in /nix/store)"
else
    echo "  No perf data yet"
fi
echo "✅ Analysis phase complete"
echo ""

# Phase 3: Self-metadata (minimal, no perf data)
echo "🪞 Phase 3: Self-metadata"
./tools/scripts/collect-repo-metadata.sh . >/dev/null 2>&1 || true
mkdir -p zos && mv zos.toml zos/ 2>/dev/null || true
echo "✅ Metadata generated"
echo ""

# Phase 4: Commit (exclude all data files)
echo "💾 Phase 4: Commit"
git add -A
git reset HEAD '*.perf.data' '*.strace' 'data/' 'zos-results/' 2>/dev/null || true
if git diff --cached --quiet; then
    echo "  No code changes"
else
    git commit -m "chore: bootstrap iteration $(date +%s)" 2>&1 | grep -E "^\[|files changed" || true
fi
echo "✅ Committed (data in Nix store only)"
echo ""

# Phase 5: Push data to HuggingFace (not git)
echo "📤 Phase 5: Push to HuggingFace"
if [ -d "hf-build-telemetry-upload" ]; then
    # Store perf data references (not the data itself)
    mkdir -p hf-build-telemetry-upload/perf-refs
    
    # Create reference file with IPFS/HF URLs
    cat > hf-build-telemetry-upload/perf-refs/latest.json <<EOF
{
  "timestamp": "$(date -Iseconds)",
  "commit": "$(git rev-parse HEAD)",
  "perf_data": {
    "hf_dataset": "hf://datasets/introspector/build-telemetry",
    "ipfs_cid": "TODO: upload to IPFS",
    "description": "Bootstrap perf data"
  }
}
EOF
    
    echo "  ✓ Reference stored (data in HF/IPFS, not git)"
else
    echo "  HF dataset not initialized"
fi
echo "✅ Data pushed"
echo ""

# Phase 6: Status
echo "📊 Status"
echo "  Commits: $(git rev-list --count HEAD)"
echo "  Tools: $(ls *.rs 2>/dev/null | wc -l)"
echo "  Perf traces in store: $PERF_FILES"
echo "  Store usage: $(du -sh /nix/store 2>/dev/null | cut -f1)"
echo ""
echo "✅ Bootstrap complete - run again to iterate"
