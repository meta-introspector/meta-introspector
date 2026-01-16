#!/usr/bin/env bash
# Build all binaries using the server with hot reload

SERVER_URL="http://127.0.0.1:3000"

echo "🔥 Building all binaries with hot reload..."

# Get all binary names
BINS=$(grep '^\[\[bin\]\]' Cargo.toml -A 1 | grep 'name = ' | cut -d'"' -f2)

TOTAL=0
SUCCESS=0
FAILED=0

for bin in $BINS; do
    TOTAL=$((TOTAL + 1))
    echo -n "[$TOTAL] Building $bin... "
    
    RESULT=$(curl -s -X POST $SERVER_URL/hot-build \
        -H "Content-Type: application/json" \
        -d "{\"target\":\"$bin\"}")
    
    if echo "$RESULT" | jq -e '.success' > /dev/null 2>&1; then
        echo "✅"
        SUCCESS=$((SUCCESS + 1))
    else
        echo "❌"
        FAILED=$((FAILED + 1))
        
        # Try to auto-fix
        echo "  Attempting auto-fix..."
        curl -s -X POST $SERVER_URL/fix-all > /dev/null
    fi
done

echo ""
echo "📊 Results:"
echo "  Total: $TOTAL"
echo "  Success: $SUCCESS"
echo "  Failed: $FAILED"

# Get final error summary
curl -s $SERVER_URL/errors | jq '{total_errors, by_type: .by_type | to_entries | sort_by(-.value) | .[0:5] | from_entries}'
