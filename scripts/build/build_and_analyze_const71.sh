#!/bin/bash
# Build and analyze const x = 71 across Rust, GCC, LLVM

echo "🔬 Building const x = 71 across 8 compilers with perf tracing"
echo "================================================================"

OUTPUT="const_71_analysis"
mkdir -p "$OUTPUT"

# Build each with perf tracing
for compiler in rust gcc llvm python node ocaml haskell lean4; do
    echo ""
    echo "🔨 Building with $compiler..."
    
    cd const_71_test/$compiler
    
    # Trace the build
    perf record -e cycles,instructions -o "../../$OUTPUT/build_${compiler}.data" \
        nix build --no-link 2>&1 | tail -3
    
    # Get the result path
    RESULT=$(nix build --print-out-paths 2>/dev/null)
    
    if [ -n "$RESULT" ]; then
        echo "   ✅ Built: $RESULT"
        
        # Copy binary
        find "$RESULT" -type f -executable -exec cp {} "../../$OUTPUT/binary_${compiler}" \;
        
        # Get perf trace
        cd ../..
        perf script -i "$OUTPUT/build_${compiler}.data" > "$OUTPUT/build_trace_${compiler}.txt" 2>/dev/null
        
        # Analyze binary
        if [ -f "$OUTPUT/binary_${compiler}" ]; then
            echo "   📊 Analyzing binary..."
            
            # Disassemble
            objdump -d "$OUTPUT/binary_${compiler}" > "$OUTPUT/disasm_${compiler}.txt"
            
            # Find const 71 (0x47)
            grep -n "0x47\|\\$0x47\|#71" "$OUTPUT/disasm_${compiler}.txt" | head -20 > "$OUTPUT/const_refs_${compiler}.txt"
            
            # Extract symbols
            nm "$OUTPUT/binary_${compiler}" > "$OUTPUT/symbols_${compiler}.txt"
            
            # Get ELF info
            readelf -h "$OUTPUT/binary_${compiler}" > "$OUTPUT/elf_header_${compiler}.txt"
            
            # Extract .text section for Markov analysis
            objcopy -O binary -j .text "$OUTPUT/binary_${compiler}" "$OUTPUT/text_${compiler}.bin" 2>/dev/null
            
            echo "   ✅ Analysis complete"
        fi
    else
        echo "   ❌ Build failed"
        cd ../..
    fi
done

echo ""
echo "================================================================"
echo "📊 Comparison Results"
echo "================================================================"

for compiler in rust gcc llvm python node ocaml haskell lean4; do
    if [ -f "$OUTPUT/binary_${compiler}" ]; then
        SIZE=$(stat -c%s "$OUTPUT/binary_${compiler}")
        CONST_REFS=$(wc -l < "$OUTPUT/const_refs_${compiler}.txt" 2>/dev/null || echo 0)
        SYMBOLS=$(wc -l < "$OUTPUT/symbols_${compiler}.txt" 2>/dev/null || echo 0)
        
        echo ""
        echo "$compiler:"
        echo "  Binary size: $SIZE bytes"
        echo "  Const 71 references: $CONST_REFS"
        echo "  Total symbols: $SYMBOLS"
        
        # Show first const reference
        if [ -s "$OUTPUT/const_refs_${compiler}.txt" ]; then
            echo "  First reference:"
            head -1 "$OUTPUT/const_refs_${compiler}.txt" | sed 's/^/    /'
        fi
    fi
done

echo ""
echo "================================================================"
echo "🌙 Running Markov Resonance Analysis"
echo "================================================================"

# Create file list for Markov analyzer
ls -1 "$OUTPUT"/binary_* > "$OUTPUT/binaries.txt"

echo "Binary list:"
cat "$OUTPUT/binaries.txt"

echo ""
echo "Running analyzer..."
cargo run --release -p markov_resonance_analyzer -- "$OUTPUT/binaries.txt" 2>&1 | tail -30

echo ""
echo "================================================================"
echo "✅ Analysis Complete"
echo "================================================================"
echo "Results in: $OUTPUT/"
echo ""
echo "Key files:"
echo "  - binary_* : Compiled binaries"
echo "  - disasm_* : Disassembly"
echo "  - const_refs_* : References to const 71"
echo "  - markov_symbol_scores.* : Resonance analysis"
