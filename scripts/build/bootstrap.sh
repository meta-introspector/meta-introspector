#!/usr/bin/env bash
# Complete system bootstrap via central build system
# All analysis scheduled as nix jobs

set -euo pipefail

echo "🚀 Meta-Introspector Bootstrap"
echo "==============================="
echo ""
echo "Central Build System - 7 Jobs:"
echo "  1. languages (71 languages)"
echo "  2. build-graph (first ordering)"
echo "  3. perf-analysis (perf traces)"
echo "  4. topological-matrix (function matrix)"
echo "  5. harmonic-analysis (harmonics)"
echo "  6. model-training (NN models)"
echo "  7. complete (all jobs)"
echo ""

cd "$(dirname "$0")/../.."

# Build complete system
nix build ./nix#default --print-build-logs

echo ""
echo "✅ Bootstrap complete!"
echo ""
echo "📊 Results in: result/"
echo "  languages/   - 71 language outputs"
echo "  graphs/      - Build graph (first ordering)"
echo "  analysis/    - Perf analysis"
echo "  matrix/      - Topological function matrix"
echo "  harmonics/   - Harmonic analysis"
echo "  models/      - Trained models"
echo ""
echo "Query:"
echo "  cat result/.meta-introspector/metadata.json"
echo "  cat result/graphs/build-order.txt"
echo "  ls result/models/"
