#!/bin/bash
# Build all binaries and collect errors

echo "🔨 Building all binaries via minimal-build-server..."
echo ""

# Get list of all binaries
BINS=$(grep -A1 '^\[\[bin\]\]' Cargo.toml | grep '^name = ' | cut -d'"' -f2)

SUCCESS=0
FAILED=0
ERRORS_FILE="build_errors.json"

echo "[" > $ERRORS_FILE

for bin in $BINS; do
    echo -n "Building $bin... "
    
    RESULT=$(curl -s -X POST http://127.0.0.1:3000/compile \
        -H "Content-Type: application/json" \
        -d "{\"target\":\"$bin\"}")
    
    if echo "$RESULT" | jq -e '.success == true' > /dev/null 2>&1; then
        echo "✅"
        ((SUCCESS++))
    else
        echo "❌"
        ((FAILED++))
        
        # Extract errors
        echo "$RESULT" | jq -c "{bin: \"$bin\", errors: .errors, output: .output}" >> $ERRORS_FILE
        echo "," >> $ERRORS_FILE
    fi
done

# Close JSON array
echo "]" >> $ERRORS_FILE

echo ""
echo "📊 Results:"
echo "  ✅ Success: $SUCCESS"
echo "  ❌ Failed: $FAILED"
echo ""
echo "📝 Errors saved to: $ERRORS_FILE"
