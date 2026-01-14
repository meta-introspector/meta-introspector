#!/bin/bash
# Capture perf data from Nix Rust beta build

set -e

OUTPUT_DIR="/mnt/data1/meta-introspector/data/perf_rankings"
TIMESTAMP=$(date +%s)
SESSION="nix_rust_beta_${TIMESTAMP}"

mkdir -p "$OUTPUT_DIR"

echo "🔬 Perf capture for Nix Rust beta build"
echo "Session: $SESSION"

cd /mnt/data1/meta-introspector/rust-overlay-test

# Start perf recording in background
echo "📊 Starting perf record..."
sudo perf record -F 99 -g --call-graph dwarf -a -o "${OUTPUT_DIR}/${SESSION}.perf.data" &
PERF_PID=$!

echo "Perf PID: $PERF_PID"
sleep 2

# Run the Nix build
echo "🔨 Starting Nix Rust beta build..."
time nix build .#rustNightlyProfiling

echo "✅ Build complete"

# Stop perf
echo "🛑 Stopping perf..."
sudo kill -INT $PERF_PID
wait $PERF_PID 2>/dev/null || true

# Generate reports
echo "📈 Generating symbol reports..."

# Full report with demangling
sudo perf report -i "${OUTPUT_DIR}/${SESSION}.perf.data" --stdio -n --percent-limit 0.1 --demangle \
    > "${OUTPUT_DIR}/${SESSION}_report.txt"

# Symbol counts
sudo perf script -i "${OUTPUT_DIR}/${SESSION}.perf.data" | \
    awk '{print $5}' | \
    grep -v '^$' | \
    sort | uniq -c | sort -rn \
    > "${OUTPUT_DIR}/${SESSION}_symbol_counts.txt"

# Top 200 as JSON
echo "{" > "${OUTPUT_DIR}/${SESSION}_ranking.json"
echo "  \"session\": \"$SESSION\"," >> "${OUTPUT_DIR}/${SESSION}_ranking.json"
echo "  \"timestamp\": $(date +%s)," >> "${OUTPUT_DIR}/${SESSION}_ranking.json"
echo "  \"build_type\": \"nix_rust_beta\"," >> "${OUTPUT_DIR}/${SESSION}_ranking.json"
echo "  \"top_symbols\": [" >> "${OUTPUT_DIR}/${SESSION}_ranking.json"

head -200 "${OUTPUT_DIR}/${SESSION}_symbol_counts.txt" | \
    awk '{print "    {\"count\": " $1 ", \"symbol\": \"" $2 "\"}"}' | \
    sed '$ ! s/$/,/' >> "${OUTPUT_DIR}/${SESSION}_ranking.json"

echo "  ]" >> "${OUTPUT_DIR}/${SESSION}_ranking.json"
echo "}" >> "${OUTPUT_DIR}/${SESSION}_ranking.json"

echo ""
echo "✅ Results saved:"
echo "   - ${OUTPUT_DIR}/${SESSION}.perf.data"
echo "   - ${OUTPUT_DIR}/${SESSION}_report.txt"
echo "   - ${OUTPUT_DIR}/${SESSION}_symbol_counts.txt"
echo "   - ${OUTPUT_DIR}/${SESSION}_ranking.json"
echo ""
echo "🔥 Top 30 hottest symbols:"
head -30 "${OUTPUT_DIR}/${SESSION}_symbol_counts.txt"
