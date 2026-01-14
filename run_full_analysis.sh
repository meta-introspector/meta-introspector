#!/bin/bash
# Run Markov analysis on full 34,506 ELF file list with complete logging

INPUT_FILE="elf_files_filtered.txt"
OUTPUT_DIR="markov_results"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
LOG_FILE="${OUTPUT_DIR}/full_analysis_${TIMESTAMP}.log"
TIMING_FILE="${OUTPUT_DIR}/full_analysis_${TIMESTAMP}_timing.txt"
SUMMARY_FILE="${OUTPUT_DIR}/full_analysis_${TIMESTAMP}_summary.txt"

mkdir -p "$OUTPUT_DIR"

echo "🚀 Starting full Markov resonance analysis" | tee -a "$LOG_FILE"
echo "================================================" | tee -a "$LOG_FILE"
echo "📋 Input: $INPUT_FILE" | tee -a "$LOG_FILE"
wc -l "$INPUT_FILE" | tee -a "$LOG_FILE"
echo "💾 Memory budget: 20GB shared across 20 workers" | tee -a "$LOG_FILE"
echo "📁 Output directory: $OUTPUT_DIR" | tee -a "$LOG_FILE"
echo "📝 Log file: $LOG_FILE" | tee -a "$LOG_FILE"
echo "================================================" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

START_TIME=$(date +%s)
START_DATE=$(date '+%Y-%m-%d %H:%M:%S')
echo "⏰ Start time: $START_DATE" | tee -a "$LOG_FILE"
echo "" | tee -a "$LOG_FILE"

# Run analysis with full output capture
{ time cargo run --release -p markov_resonance_analyzer -- "$INPUT_FILE" 2>&1; } 2>&1 | tee -a "$LOG_FILE" > "$TIMING_FILE"
EXIT_CODE=${PIPESTATUS[0]}

END_TIME=$(date +%s)
END_DATE=$(date '+%Y-%m-%d %H:%M:%S')
DURATION=$((END_TIME - START_TIME))
MINUTES=$((DURATION / 60))
SECONDS=$((DURATION % 60))

echo "" | tee -a "$LOG_FILE"
echo "================================================" | tee -a "$LOG_FILE"
echo "⏰ End time: $END_DATE" | tee -a "$LOG_FILE"
echo "⏱️  Duration: ${MINUTES}m ${SECONDS}s (${DURATION}s total)" | tee -a "$LOG_FILE"
echo "🔢 Exit code: $EXIT_CODE" | tee -a "$LOG_FILE"
echo "================================================" | tee -a "$LOG_FILE"

# Generate summary
{
    echo "=== FULL MARKOV ANALYSIS SUMMARY ==="
    echo ""
    echo "Timestamp: $TIMESTAMP"
    echo "Start: $START_DATE"
    echo "End: $END_DATE"
    echo "Duration: ${MINUTES}m ${SECONDS}s"
    echo "Exit code: $EXIT_CODE"
    echo ""
    echo "Input file: $INPUT_FILE"
    wc -l "$INPUT_FILE"
    echo ""
    
    if [ $EXIT_CODE -eq 0 ]; then
        echo "✅ Analysis completed successfully"
        echo ""
        
        # Extract statistics from log
        echo "=== Statistics ==="
        grep "Total symbols extracted:" "$LOG_FILE" | tail -1
        grep "Total distributions:" "$LOG_FILE" | tail -1
        grep "Memory used:" "$LOG_FILE" | tail -1
        grep "Files skipped" "$LOG_FILE" | tail -1
        echo ""
        
        # Check output files
        echo "=== Output Files ==="
        if [ -f "markov_symbol_scores.json" ]; then
            SYMBOLS=$(jq 'length' markov_symbol_scores.json 2>/dev/null || echo "N/A")
            SIZE=$(ls -lh markov_symbol_scores.json | awk '{print $5}')
            echo "markov_symbol_scores.json: $SIZE ($SYMBOLS symbols)"
            mv markov_symbol_scores.json "${OUTPUT_DIR}/markov_symbols_full_${TIMESTAMP}.json"
        fi
        
        if [ -f "markov_global_matrix.json" ]; then
            DISTS=$(jq '.files | length' markov_global_matrix.json 2>/dev/null || echo "N/A")
            SIZE=$(ls -lh markov_global_matrix.json | awk '{print $5}')
            echo "markov_global_matrix.json: $SIZE ($DISTS distributions)"
            mv markov_global_matrix.json "${OUTPUT_DIR}/markov_matrix_full_${TIMESTAMP}.json"
        fi
        
        echo ""
        echo "=== Top 10 Symbols by Resonance ==="
        jq -r '[.[] | {name, file: .file | split("/")[-1], cell, score}] | sort_by(-.score) | .[0:10] | .[] | "\(.score | tostring | .[0:8])\t\(.name)\t\(.file)"' \
            "${OUTPUT_DIR}/markov_symbols_full_${TIMESTAMP}.json" 2>/dev/null || echo "Could not extract top symbols"
    else
        echo "❌ Analysis failed with exit code $EXIT_CODE"
        echo ""
        echo "=== Last 50 lines of log ==="
        tail -50 "$LOG_FILE"
    fi
    
    echo ""
    echo "=== Disk Usage ==="
    du -sh "$OUTPUT_DIR"
    
} > "$SUMMARY_FILE"

# Display summary
cat "$SUMMARY_FILE"

echo ""
echo "📁 All files saved to: $OUTPUT_DIR/"
echo "   - Log: $(basename $LOG_FILE)"
echo "   - Timing: $(basename $TIMING_FILE)"
echo "   - Summary: $(basename $SUMMARY_FILE)"

if [ $EXIT_CODE -eq 0 ]; then
    echo ""
    echo "✅ Full analysis complete!"
    exit 0
else
    echo ""
    echo "❌ Analysis failed - check logs for details"
    exit $EXIT_CODE
fi
