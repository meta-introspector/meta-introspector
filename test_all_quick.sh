#!/bin/bash
echo "🏗️  Testing all 71 builds..."
SUCCESS=0
FAIL=0
FAILED_LANGS=""

for dir in const_71_test/*/; do
    LANG=$(basename $dir)
    printf "%-20s " "$LANG"
    
    cd $dir
    if timeout 60 nix build 2>&1 | grep -q "error:"; then
        echo "❌"
        ((FAIL++))
        FAILED_LANGS="$FAILED_LANGS $LANG"
    else
        echo "✅"
        ((SUCCESS++))
    fi
    cd ../..
done

echo ""
echo "📊 Results: $SUCCESS ✅ / $FAIL ❌"
[ -n "$FAILED_LANGS" ] && echo "Failed:$FAILED_LANGS"
