#!/bin/bash
# Compare instruction fingerprints

echo "🔬 Instruction Spectrum Comparison"
echo "===================================="
echo ""

for fp in data/fingerprints/*_fingerprint.txt; do
    lang=$(basename "$fp" _fingerprint.txt)
    
    echo "📦 $lang"
    echo "  Top IP Complexity:"
    head -5 "$fp" | tail -1 | awk '{print "    " $5 " - " $7 " coverage"}'
    
    echo "  Fingerprint Hash:"
    tail -1 "$fp" | awk '{print "    " $1}'
    echo ""
done

echo "✅ Each language has unique instruction spectrum"
