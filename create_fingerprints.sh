#!/bin/bash
# Create instruction fingerprints for all languages

mkdir -p data/fingerprints

for perf in data/71_flakes_perf/{rust,agda,coq,bash,python,ruby}_*.perf.data; do
    [ -f "$perf" ] || continue
    
    lang=$(basename "$perf" | sed 's/_.*\.perf\.data//')
    output="data/fingerprints/${lang}_fingerprint.txt"
    
    echo "🔬 Fingerprinting $lang..."
    ./target/release/ip_galois "$perf" > "$output" 2>&1
    
    # Add hash
    echo "" >> "$output"
    echo "🔐 Instruction Fingerprint:" >> "$output"
    ./target/release/ip_galois "$perf" 2>/dev/null | \
        grep "^|" | sha256sum | awk '{print "  " $1}' >> "$output"
done

echo ""
echo "✅ Fingerprints created in data/fingerprints/"
ls -lh data/fingerprints/
