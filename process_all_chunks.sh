#!/bin/bash
# Process all chunks sequentially

CHUNK_DIR="elf_chunks"
OUTPUT_DIR="markov_results"

mkdir -p "$OUTPUT_DIR"

echo "🚀 Starting chunked Markov analysis"
echo "=================================="

for chunk_file in "$CHUNK_DIR"/chunk_*.txt; do
    chunk_name=$(basename "$chunk_file" .txt)
    echo ""
    echo "📦 Processing $chunk_name..."
    
    cargo run --release -p markov_resonance_analyzer -- "$chunk_file" > "$OUTPUT_DIR/${chunk_name}.log" 2>&1
    EXIT_CODE=$?
    
    if [ $EXIT_CODE -eq 0 ]; then
        echo "✅ $chunk_name completed successfully"
        # Move results to output dir
        mv markov_symbols_chunk_*.json "$OUTPUT_DIR/" 2>/dev/null
        mv markov_distributions_chunk_*.json "$OUTPUT_DIR/" 2>/dev/null
    else
        echo "❌ $chunk_name failed with exit code $EXIT_CODE"
        echo "   Check $OUTPUT_DIR/${chunk_name}.log for details"
    fi
done

echo ""
echo "🎉 All chunks processed!"
echo "📊 Results in: $OUTPUT_DIR/"
ls -lh "$OUTPUT_DIR"/*.json 2>/dev/null | wc -l
echo " JSON files created"
