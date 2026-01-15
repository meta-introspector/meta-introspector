#!/bin/bash
# Build all binaries and document errors

set -e

OUTPUT_DIR="build_errors"
mkdir -p "$OUTPUT_DIR"

echo "🔨 Building all 184 binaries..."
echo "Errors will be saved to $OUTPUT_DIR/"

# Get all binary names
BINARIES=$(grep 'name = ' Cargo.toml | grep -A1 '\[\[bin\]\]' | grep 'name = ' | cut -d'"' -f2)

SUCCESS=0
FAILED=0

for bin in $BINARIES; do
    echo -n "Building $bin... "
    if cargo build --bin "$bin" 2>"$OUTPUT_DIR/${bin}_error.log" >/dev/null; then
        echo "✅"
        rm "$OUTPUT_DIR/${bin}_error.log"
        SUCCESS=$((SUCCESS + 1))
    else
        echo "❌"
        FAILED=$((FAILED + 1))
    fi
done

echo ""
echo "📊 Results:"
echo "  Success: $SUCCESS"
echo "  Failed:  $FAILED"
echo ""
echo "Error logs in: $OUTPUT_DIR/"

# Generate summary
echo "# Build Error Summary" > "$OUTPUT_DIR/SUMMARY.md"
echo "" >> "$OUTPUT_DIR/SUMMARY.md"
echo "Total: $((SUCCESS + FAILED))" >> "$OUTPUT_DIR/SUMMARY.md"
echo "Success: $SUCCESS" >> "$OUTPUT_DIR/SUMMARY.md"
echo "Failed: $FAILED" >> "$OUTPUT_DIR/SUMMARY.md"
echo "" >> "$OUTPUT_DIR/SUMMARY.md"
echo "## Failed Binaries" >> "$OUTPUT_DIR/SUMMARY.md"
for err in "$OUTPUT_DIR"/*_error.log; do
    if [ -f "$err" ]; then
        bin=$(basename "$err" _error.log)
        echo "- $bin" >> "$OUTPUT_DIR/SUMMARY.md"
    fi
done
