#!/bin/bash
# Build additional languages (Python, Node, OCaml, Haskell, Lean4)

echo "🔨 Building const x = 71 in additional languages"
echo "================================================"

OUTPUT="const_71_analysis"
mkdir -p "$OUTPUT"

for lang in python node ocaml haskell lean4; do
    echo ""
    echo "Building $lang..."
    
    cd const_71_test/$lang
    
    nix build --print-out-paths 2>&1 | tail -3
    RESULT=$(nix build --print-out-paths 2>/dev/null)
    
    if [ -n "$RESULT" ]; then
        echo "  ✅ $RESULT"
        
        # Copy binary
        find "$RESULT" -type f -executable -exec cp {} "../../$OUTPUT/binary_${lang}" \;
        
        cd ../..
        
        # Quick analysis
        if [ -f "$OUTPUT/binary_${lang}" ]; then
            SIZE=$(stat -c%s "$OUTPUT/binary_${lang}")
            echo "  📊 Size: $SIZE bytes"
            
            # Check for const 71
            strings "$OUTPUT/binary_${lang}" | grep -i "71\|x = " | head -3
        fi
    else
        echo "  ❌ Build failed"
        cd ../..
    fi
done

echo ""
echo "================================================"
echo "✅ Additional languages built"
ls -lh "$OUTPUT"/binary_{python,node,ocaml,haskell,lean4} 2>/dev/null
