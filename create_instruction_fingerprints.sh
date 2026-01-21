#!/bin/bash
# Create instruction spectrum fingerprints

mkdir -p data/instruction_fingerprints

for lang in rust agda; do
    case $lang in
        rust) perf=/nix/store/zfw4f0c5nh0f0j2mfaqdxip4cf4mbhbc-rust-actual-perf/rust_actual.perf.data ;;
        agda) perf=/nix/store/fac4cgf0qj2fnqg5r0jzl8r02pp7bkmw-agda-actual-perf/agda_actual.perf.data ;;
    esac
    
    output="data/instruction_fingerprints/${lang}_instruction_spectrum.txt"
    
    echo "🔬 Analyzing $lang instruction spectrum..."
    ./target/release/ip_spectrum "$perf" > "$output" 2>&1
    
    # Add function names
    echo "" >> "$output"
    echo "🎯 Top Functions:" >> "$output"
    perf report -i "$perf" --stdio 2>/dev/null | \
        grep -E "^\s+[0-9]+\.[0-9]+%" | \
        grep -v kernel | head -10 | \
        sed 's/^/  /' >> "$output"
    
    # Create hash
    echo "" >> "$output"
    echo "🔐 Instruction Fingerprint Hash:" >> "$output"
    ./target/release/ip_spectrum "$perf" 2>/dev/null | \
        sha256sum | awk '{print "  " $1}' >> "$output"
done

echo ""
echo "✅ Instruction fingerprints created"
ls -lh data/instruction_fingerprints/
