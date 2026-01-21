#!/usr/bin/env bash
# Test force rebuild on one language

set -e

LANG=${1:-rust}
PERF_DIR="data/real_compilation_perf"
mkdir -p $PERF_DIR

echo "🔨 Force rebuilding $LANG from source..."

cd const_71_test/$LANG

# Build without cache
echo "📊 Recording compilation..."
perf record -F 99 -g -o ../../$PERF_DIR/${LANG}_forced.perf.data \
  nix build --no-substitute --rebuild 2>&1 | tee ../../$PERF_DIR/${LANG}_build.log

echo "✅ Done! Check perf data:"
echo "  perf report -i $PERF_DIR/${LANG}_forced.perf.data"
