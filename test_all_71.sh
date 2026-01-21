#!/usr/bin/env bash
# Test that all 71 languages actually build and output "71"

set -e

RESULTS_FILE="test_results.txt"
TIMEOUT=10  # 10 second timeout per test

echo "🧪 Testing all 71 languages (${TIMEOUT}s timeout)..." > $RESULTS_FILE
echo "" >> $RESULTS_FILE

SUCCESS=0
FAIL=0
TIMEOUT_COUNT=0

for dir in const_71_test/*/; do
    LANG=$(basename $dir)
    echo -n "Testing $LANG... "
    
    cd $dir
    
    # Try to build and run with timeout
    if timeout $TIMEOUT nix build 2>/dev/null; then
        if OUTPUT=$(timeout $TIMEOUT nix run 2>/dev/null | head -1); then
            if echo "$OUTPUT" | grep -q "71"; then
                echo "✅ PASS: $OUTPUT"
                echo "✅ $LANG: $OUTPUT" >> ../../$RESULTS_FILE
                ((SUCCESS++))
            else
                echo "❌ FAIL: Wrong output: $OUTPUT"
                echo "❌ $LANG: Wrong output: $OUTPUT" >> ../../$RESULTS_FILE
                ((FAIL++))
            fi
        else
            echo "⏱️  TIMEOUT: Run exceeded ${TIMEOUT}s"
            echo "⏱️  $LANG: Timeout" >> ../../$RESULTS_FILE
            ((TIMEOUT_COUNT++))
        fi
    else
        echo "⏱️  TIMEOUT: Build exceeded ${TIMEOUT}s"
        echo "⏱️  $LANG: Build timeout" >> ../../$RESULTS_FILE
        ((TIMEOUT_COUNT++))
    fi
    
    cd ../..
done

echo ""
echo "📊 Results: $SUCCESS passed, $FAIL failed, $TIMEOUT_COUNT timeout"
echo "" >> $RESULTS_FILE
echo "📊 Results: $SUCCESS passed, $FAIL failed, $TIMEOUT_COUNT timeout" >> $RESULTS_FILE

cat $RESULTS_FILE
