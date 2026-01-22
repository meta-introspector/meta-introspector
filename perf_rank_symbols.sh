# Use: nix run ./perf-recorder#perf-build -- .#target
# See: docs/perf/README.md for canonical patterns

#!/bin/bash
# Rank symbols by actual runtime usage using perf

set -e

PROJECT=${1:-"hello-world"}
DURATION=${2:-10}
OUTPUT_DIR="data/perf_rankings"

mkdir -p "$OUTPUT_DIR"

echo "🔬 Perf-based symbol ranking for: $PROJECT"
echo "Duration: ${DURATION}s"

# Create test project if needed
if [ ! -d "/tmp/$PROJECT" ]; then
    echo "📦 Creating test project..."
    cd /tmp
    cargo new "$PROJECT" --bin
fi

cd "/tmp/$PROJECT"

# Build with debug symbols
echo "🔨 Building with debug symbols..."
cargo build --release

BINARY="target/release/$PROJECT"

if [ ! -f "$BINARY" ]; then
    echo "❌ Binary not found: $BINARY"
    exit 1
fi

# Record perf data
echo "📊 Recording perf data for ${DURATION}s..."
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
PID=$!

sleep "$DURATION"
kill -INT $PID 2>/dev/null || true
wait $PID 2>/dev/null || true

# Generate symbol report
echo "📈 Generating symbol frequency report..."
perf report -i perf.data --stdio -n --percent-limit 0.01 > "${OUTPUT_DIR}/${PROJECT}_perf_report.txt"

# Extract top symbols with counts
echo "🎯 Extracting top symbols..."
perf script -i perf.data | \
    awk '{print $5}' | \
    grep -v '^$' | \
    sort | uniq -c | sort -rn > "${OUTPUT_DIR}/${PROJECT}_symbol_counts.txt"

# Parse into JSON
echo "💾 Creating JSON ranking..."
cat > "${OUTPUT_DIR}/${PROJECT}_perf_ranking.json" << 'EOF'
{
  "project": "PROJECT_NAME",
  "duration_seconds": DURATION_SEC,
  "timestamp": "TIMESTAMP",
  "top_symbols": [
EOF

head -100 "${OUTPUT_DIR}/${PROJECT}_symbol_counts.txt" | \
    awk '{print "    {\"count\": " $1 ", \"symbol\": \"" $2 "\"},"}' | \
    sed '$ s/,$//' >> "${OUTPUT_DIR}/${PROJECT}_perf_ranking.json"

cat >> "${OUTPUT_DIR}/${PROJECT}_perf_ranking.json" << 'EOF'
  ]
}
EOF

# Replace placeholders
sed -i "s/PROJECT_NAME/$PROJECT/g" "${OUTPUT_DIR}/${PROJECT}_perf_ranking.json"
sed -i "s/DURATION_SEC/$DURATION/g" "${OUTPUT_DIR}/${PROJECT}_perf_ranking.json"
sed -i "s/TIMESTAMP/$(date -Iseconds)/g" "${OUTPUT_DIR}/${PROJECT}_perf_ranking.json"

echo ""
echo "✅ Results saved to:"
echo "   - ${OUTPUT_DIR}/${PROJECT}_perf_report.txt"
echo "   - ${OUTPUT_DIR}/${PROJECT}_symbol_counts.txt"
echo "   - ${OUTPUT_DIR}/${PROJECT}_perf_ranking.json"
echo ""
echo "🔥 Top 10 hottest symbols:"
head -10 "${OUTPUT_DIR}/${PROJECT}_symbol_counts.txt"
