#!/usr/bin/env bash
# Quick test - just check if they can run (assume already built)

echo "🧪 Quick test of 71 languages..."
SUCCESS=0
FAIL=0

# Test languages that use writeShellScript (instant)
QUICK_LANGS="bash python ruby perl lua tcl zsh fish scheme nix_flake"

for lang in $QUICK_LANGS; do
    if [ -d "const_71_test/$lang" ]; then
        echo -n "$lang... "
        if timeout 5 nix run const_71_test/$lang 2>/dev/null | head -1 | grep -q "71"; then
            echo "✅"
            ((SUCCESS++))
        else
            echo "❌"
            ((FAIL++))
        fi
    fi
done

echo ""
echo "Quick test: $SUCCESS passed, $FAIL failed"
