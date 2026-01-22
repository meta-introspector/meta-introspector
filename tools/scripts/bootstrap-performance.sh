# Use: nix run ./perf-recorder#perf-build -- .#target
# See: docs/perf/README.md for canonical patterns

#!/usr/bin/env bash
set -euo pipefail

echo "🎭 ZOS Bootstrap Performance - Recording MES from seed"
echo "======================================================="

OUT="zos-results/bootstrap-performance"
mkdir -p "$OUT"

# Performance: Build MES from 357 bytes using Guix
echo "🎬 Act 1: The Performance (Guix building MES from seed)"
perf record -g -o "$OUT/performance.perf.data" -- \
strace -f -o "$OUT/performance.strace" -e trace=all -- \
  guix build --no-substitutes mes 2>&1 | tee "$OUT/performance.log"

echo ""
echo "🎧 Act 2: The Reception (analyzing the traces)"

# Reception 1: Perf analysis
echo "  📊 Analyzing perf data..."
perf report -i "$OUT/performance.perf.data" --stdio --no-children \
  > "$OUT/reception-perf.txt"

# Reception 2: Strace analysis  
echo "  🔍 Analyzing syscalls..."
grep -E "^[0-9]+ " "$OUT/performance.strace" | \
  awk '{print $2}' | sort | uniq -c | sort -rn \
  > "$OUT/reception-syscalls.txt"

# Reception 3: File access patterns
echo "  📁 Analyzing file access..."
grep -E "open|read|write|stat" "$OUT/performance.strace" | \
  grep -oE '"/[^"]*"' | sort | uniq -c | sort -rn \
  > "$OUT/reception-files.txt"

echo ""
echo "🔬 Act 3: The Introspection (tools analyzing tools)"

# Introspection 1: Use our own meta_discovery
if [ -f target/release/meta_discovery ]; then
  echo "  🪞 Running meta_discovery on traces..."
  ./target/release/meta_discovery "$OUT" > "$OUT/introspection-meta.json" 2>&1 || true
fi

# Introspection 2: Use our own oeis_recognizers
if [ -f target/release/oeis_recognizers ]; then
  echo "  🔢 Running OEIS recognizers on traces..."
  ./target/release/oeis_recognizers "$OUT" > "$OUT/introspection-oeis.json" 2>&1 || true
fi

# Introspection 3: Use our own fold_traces
if [ -f target/release/fold_traces ]; then
  echo "  📦 Folding all traces..."
  ./target/release/fold_traces "$OUT" > "$OUT/introspection-folded.parquet" 2>&1 || true
fi

echo ""
echo "📊 Final Report"
echo "==============="
echo "Performance recorded in: $OUT/performance.perf.data"
echo "Reception analyzed in: $OUT/reception-*.txt"
echo "Introspection results in: $OUT/introspection-*.{json,parquet}"
echo ""
echo "The system has observed itself bootstrapping."
echo "This is the first introspection."
