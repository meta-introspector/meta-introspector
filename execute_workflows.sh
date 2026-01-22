#!/bin/bash
# Execute workflows - Record perf data for all 71 languages
# 
# This is a thin interface to the nix derivation that does the actual work.
# Perf data is stored in /nix/store for reproducibility.

set -e

echo "🔬 Recording perf data for 71 languages..."
echo "==========================================="
echo ""

# Build the derivation that records perf for all languages
# This stores all perf data in /nix/store
nix build ./nix/flakes/const_71_test#perf-all --print-build-logs

# Create convenient symlink
ln -sf result perf-data

echo ""
echo "✅ Complete!"
echo ""
echo "Perf data stored in: perf-data/perf/"
echo ""
echo "Analyze individual languages:"
echo "  perf report -i perf-data/perf/rust.perf.data"
echo "  perf report -i perf-data/perf/python.perf.data"
echo ""
echo "List all:"
echo "  ls -lh perf-data/perf/"
