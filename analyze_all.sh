#!/bin/bash
# Analyze all existing perf data

echo "🔬 Analyzing all perf data..."

for perf_file in data/71_flakes_perf/*_build.perf.data; do
    [ -f "$perf_file" ] || continue
    
    # Extract language name
    basename=$(basename "$perf_file")
    lang=$(echo "$basename" | sed 's/_[0-9]*_build\.perf\.data//')
    
    output="data/71_results/${lang}_analysis.txt"
    
    # Skip if already analyzed recently
    if [ -f "$output" ] && [ "$output" -nt "$perf_file" ]; then
        continue
    fi
    
    echo -n "  $lang... "
    if ./target/release/harmonic_analyzer "$perf_file" > "$output" 2>&1; then
        # Extract Galois field
        galois=$(grep "GF(2^" "$output" | grep "100.000000%" | head -1 | grep -oP 'GF\(2\^\d+\)' || echo "N/A")
        echo "✅ $galois"
    else
        echo "❌"
    fi
done

echo ""
echo "✅ Analysis complete!"
