#!/usr/bin/env bash
# Quick verification - test each language outputs 71, stop on first error

echo "🧪 Testing 71 languages (5s timeout each)..."
echo ""

SUCCESS=0

for dir in const_71_test/*/; do
    LANG=$(basename $dir)
    printf "%-20s " "$LANG"
    
    START=$(date +%s.%N)
    cd $dir
    
    OUTPUT=$(timeout 5 nix run 2>&1)
    EXIT_CODE=$?
    END=$(date +%s.%N)
    TIME=$(echo "$END - $START" | bc)
    
    FIRST_LINE=$(echo "$OUTPUT" | head -1)
    
    if [ $EXIT_CODE -eq 124 ]; then
        echo "⏱️  TIMEOUT"
        echo "ERROR: $LANG timed out after 5s"
        echo "Output: $OUTPUT"
        exit 1
    elif echo "$FIRST_LINE" | grep -q "71"; then
        printf "✅ %s (%.2fs)\n" "$FIRST_LINE" "$TIME"
        ((SUCCESS++))
    else
        echo "❌ FAILED"
        echo "ERROR: $LANG did not output 71"
        echo "Output: $OUTPUT"
        exit 1
    fi
    
    cd ../..
done

echo ""
echo "📊 All $SUCCESS tests passed!"
