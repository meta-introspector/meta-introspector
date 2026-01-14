#!/bin/bash
# Run Markov analysis on all 35 chunks with shared 20GB memory budget

OUTPUT_DIR="markov_results"
CHUNK_DIR="elf_chunks"

mkdir -p "$OUTPUT_DIR"

echo "🚀 Starting batch Markov analysis with 20GB shared memory budget"
echo "📊 Processing 35 chunks (~34,500 files total)"
echo "================================================"

START_TIME=$(date +%s)

for chunk_file in "$CHUNK_DIR"/chunk_*.txt; do
    chunk_name=$(basename "$chunk_file" .txt)
    echo ""
    echo "📦 Processing $chunk_name..."
    
    cargo run --release -p markov_resonance_analyzer -- "$chunk_file" > "$OUTPUT_DIR/${chunk_name}.log" 2>&1
    EXIT_CODE=$?
    
    if [ $EXIT_CODE -eq 0 ]; then
        echo "✅ $chunk_name completed"
        # Move results with chunk prefix
        mv markov_symbol_scores.json "$OUTPUT_DIR/markov_symbols_${chunk_name}.json" 2>/dev/null
        mv markov_global_matrix.json "$OUTPUT_DIR/markov_matrix_${chunk_name}.json" 2>/dev/null
        
        # Extract summary stats
        SYMBOLS=$(jq 'length' "$OUTPUT_DIR/markov_symbols_${chunk_name}.json")
        DISTS=$(jq '.files | length' "$OUTPUT_DIR/markov_matrix_${chunk_name}.json")
        MEM=$(grep "Memory used:" "$OUTPUT_DIR/${chunk_name}.log" | tail -1)
        echo "   📈 $SYMBOLS symbols, $DISTS distributions"
        echo "   💾 $MEM"
    else
        echo "❌ $chunk_name failed with exit code $EXIT_CODE"
    fi
done

END_TIME=$(date +%s)
DURATION=$((END_TIME - START_TIME))

echo ""
echo "================================================"
echo "✅ Batch processing complete in ${DURATION}s"
echo "📁 Results in: $OUTPUT_DIR/"
echo ""
echo "📊 Summary statistics:"
jq -s 'add | length' "$OUTPUT_DIR"/markov_symbols_chunk_*.json 2>/dev/null | xargs -I {} echo "   Total symbols: {}"
ls -1 "$OUTPUT_DIR"/markov_matrix_chunk_*.json 2>/dev/null | wc -l | xargs -I {} echo "   Chunks completed: {}/35"
du -sh "$OUTPUT_DIR" | awk '{print "   Total size: " $1}'
