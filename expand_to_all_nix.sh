#!/bin/bash
# Expand LMFDB classification to ALL /nix/store

set -e

OUTPUT_DIR="data/nix_lmfdb_analysis"
mkdir -p "$OUTPUT_DIR"

echo "🔬 LMFDB Analysis of ALL /nix/store"
echo "📊 Finding all .so files..."

# Find ALL .so files in /nix/store
find /nix/store -type f -name "*.so*" 2>/dev/null | \
    head -1000 > "$OUTPUT_DIR/all_libraries.txt"

TOTAL=$(wc -l < "$OUTPUT_DIR/all_libraries.txt")
echo "✅ Found $TOTAL libraries"

# Process in batches of 100
BATCH_SIZE=100
BATCH_NUM=0

split -l $BATCH_SIZE "$OUTPUT_DIR/all_libraries.txt" "$OUTPUT_DIR/batch_"

for batch_file in "$OUTPUT_DIR"/batch_*; do
    BATCH_NUM=$((BATCH_NUM + 1))
    echo ""
    echo "📦 Processing batch $BATCH_NUM..."
    
    # Create JSON for this batch
    echo '{"libraries": [' > "$OUTPUT_DIR/batch_${BATCH_NUM}.json"
    cat "$batch_file" | while read path; do
        echo "  \"$path\","
    done | sed '$ s/,$//' >> "$OUTPUT_DIR/batch_${BATCH_NUM}.json"
    echo ']}' >> "$OUTPUT_DIR/batch_${BATCH_NUM}.json"
    
    # Run classifier on this batch
    timeout 300 ./target/debug/lmfdb_function_composer \
        "$OUTPUT_DIR/batch_${BATCH_NUM}.json" \
        "$OUTPUT_DIR/batch_${BATCH_NUM}_results.json" \
        2>&1 | tail -20
    
    rm "$batch_file"
done

echo ""
echo "✅ Processed $BATCH_NUM batches"
echo "📊 Merging results..."

# Merge all results
echo '{"batches": [' > "$OUTPUT_DIR/merged_results.json"
for result in "$OUTPUT_DIR"/batch_*_results.json; do
    cat "$result"
    echo ","
done | sed '$ s/,$//' >> "$OUTPUT_DIR/merged_results.json"
echo ']}' >> "$OUTPUT_DIR/merged_results.json"

echo "💾 Saved to: $OUTPUT_DIR/merged_results.json"
