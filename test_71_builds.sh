#!/bin/bash
# Test that all 71 can build (quick check)

echo "🏗️  Testing 71 builds (quick mode)..."
SUCCESS=0
FAIL=0

for dir in const_71_test/*/; do
    LANG=$(basename $dir)
    printf "%-20s " "$LANG"
    
    cd $dir
    if timeout 30 nix build 2>&1 | tail -1 | grep -q "error:"; then
        echo "❌ Build failed"
        ((FAIL++))
    else
        echo "✅ Built"
        ((SUCCESS++))
    fi
    cd ../..
done

echo ""
echo "📊 Results: $SUCCESS built, $FAIL failed"
