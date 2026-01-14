#!/bin/bash
# Run Markov resonance analysis on all ELF binaries in /nix/store

set -e

cd /mnt/data1/meta-introspector

echo "🔬 Starting Markov Resonance Analysis"
echo "======================================"
echo ""

# Build if needed
if [ ! -f target/release/text_segment_distribution ]; then
    echo "Building analyzer..."
    cargo build --release
fi

# Run analysis
echo "Analyzing ELF binaries with 20 parallel workers..."
echo "This will take ~60-90 seconds for 500 files"
echo ""

./target/release/text_segment_distribution 2>&1 | tee markov_resonance_output.log

# Check results
if [ -f markov_symbol_scores.json ]; then
    SYMBOL_COUNT=$(jq length markov_symbol_scores.json)
    echo ""
    echo "✅ Analysis complete!"
    echo "   Symbols extracted: $SYMBOL_COUNT"
    echo "   Results saved to: markov_symbol_scores.json"
    echo ""
    
    # Show top 10 symbols
    echo "Top 10 symbols by resonance:"
    jq -r 'sort_by(-.score) | .[:10] | .[] | "\(.name) - score: \(.score) - \(.file | split("/")[-1])"' markov_symbol_scores.json
else
    echo "⚠️  Analysis incomplete or failed"
    echo "   Check markov_resonance_output.log for details"
fi
