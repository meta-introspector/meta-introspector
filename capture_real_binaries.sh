#!/bin/bash
# 🔥 CAPTURE ALL BINARIES: Get complete list of binaries from nix build

echo "🔥 CAPTURING ALL BINARIES FROM NIX BUILD"
echo "========================================"

# Run nix build with comprehensive strace to capture all processes
echo "🔍 Running nix build with full process tracing..."
strace -f -e trace=execve -o /tmp/full_nix_execve.log \
    nix build ./rustc-only-build --rebuild 2>/dev/null

echo "📊 EXTRACTING EXECUTED BINARIES:"
echo "================================"

# Extract all unique binaries that were executed
grep "execve(" /tmp/full_nix_execve.log | \
    sed 's/.*execve("\([^"]*\)".*/\1/' | \
    sort | uniq > /tmp/nix_executed_binaries.txt

BINARY_COUNT=$(wc -l < /tmp/nix_executed_binaries.txt)
echo "📋 Total unique binaries executed: $BINARY_COUNT"

echo ""
echo "🔧 EXECUTED BINARIES:"
cat /tmp/nix_executed_binaries.txt

echo ""
echo "💾 CREATING UPDATED FRONTRUN RESULTS:"
echo "===================================="

# Create new frontrun results with actual executed binaries
TIMESTAMP=$(date +%s)
OUTPUT_FILE="frontrun_results_real_${TIMESTAMP}.json"

echo "{" > "$OUTPUT_FILE"
echo "  \"session_id\": \"real_${TIMESTAMP}\"," >> "$OUTPUT_FILE"
echo "  \"timestamp\": $TIMESTAMP," >> "$OUTPUT_FILE"
echo "  \"intercepted_binaries\": [" >> "$OUTPUT_FILE"

# Add each binary as JSON entry
FIRST=true
while IFS= read -r binary; do
    if [ "$FIRST" = true ]; then
        FIRST=false
    else
        echo "," >> "$OUTPUT_FILE"
    fi
    echo "    \"$binary\"" >> "$OUTPUT_FILE"
done < /tmp/nix_executed_binaries.txt

echo "" >> "$OUTPUT_FILE"
echo "  ]" >> "$OUTPUT_FILE"
echo "}" >> "$OUTPUT_FILE"

echo "✅ Created: $OUTPUT_FILE"
echo "📊 Contains $BINARY_COUNT real binaries from nix build"

echo ""
echo "🎯 NEXT STEPS:"
echo "============="
echo "1. Use this file with ldd2wrap_all_calls.rs"
echo "2. This will give accurate library counts matching strace"
echo "3. Telemetry will then reflect the real build process"
