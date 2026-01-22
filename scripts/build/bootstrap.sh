#!/usr/bin/env bash
# Complete system bootstrap - builds entire meta-introspector from scratch
# Creates result/ symlink with all build outputs and metadata

set -euo pipefail

echo "🚀 Meta-Introspector Complete Bootstrap"
echo "========================================"
echo ""

# Build everything as a single derivation
echo "📦 Building complete system..."
echo ""

cd nix/flakes/const_71_test

# Build all 71 languages + tools as one derivation
nix build --print-build-logs

# Result symlink now contains:
# - All 71 language outputs
# - All perf data
# - All metadata
# - Build logs

echo ""
echo "✅ Bootstrap complete!"
echo ""
echo "📊 Results:"
echo "  Output: ./result/"
echo "  Metadata: ./result/.meta-introspector/metadata.json"
echo "  Perf data: ./result/perf/*.perf.data"
echo "  Build logs: ./result/logs/"
echo ""
echo "Query builds:"
echo "  ls -la result/"
echo "  cat result/.meta-introspector/metadata.json"
echo "  ls result/perf/"
