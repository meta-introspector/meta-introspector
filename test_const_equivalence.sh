#!/bin/bash
# Cross-language constant equivalence test with perf tracing

OUTPUT_DIR="const_equivalence_test"
mkdir -p "$OUTPUT_DIR"

echo "🔬 Cross-Language Constant Equivalence Test"
echo "Testing: const x = 71 across Rust, C++, C"
echo "================================================"

# 1. Create test programs
cat > "$OUTPUT_DIR/test.rs" << 'EOF'
fn main() {
    const X: i32 = 71;
    println!("{}", X);
}
EOF

cat > "$OUTPUT_DIR/test.cpp" << 'EOF'
#include <iostream>
int main() {
    const int x = 71;
    std::cout << x << std::endl;
    return 0;
}
EOF

cat > "$OUTPUT_DIR/test.c" << 'EOF'
#include <stdio.h>
int main() {
    const int x = 71;
    printf("%d\n", x);
    return 0;
}
EOF

echo "✅ Created test programs"

# 2. Compile with debug info
echo ""
echo "🔨 Compiling..."
rustc -g -C opt-level=0 "$OUTPUT_DIR/test.rs" -o "$OUTPUT_DIR/test_rust" 2>&1 | head -5
g++ -g -O0 "$OUTPUT_DIR/test.cpp" -o "$OUTPUT_DIR/test_cpp" 2>&1 | head -5
gcc -g -O0 "$OUTPUT_DIR/test.c" -o "$OUTPUT_DIR/test_c" 2>&1 | head -5

echo "✅ Compiled all versions"

# 3. Run with perf trace
echo ""
echo "📊 Running perf traces..."

for lang in rust cpp c; do
    echo "  Tracing $lang..."
    perf record -e cycles,instructions -o "$OUTPUT_DIR/perf_${lang}.data" \
        "$OUTPUT_DIR/test_${lang}" > /dev/null 2>&1
    
    perf script -i "$OUTPUT_DIR/perf_${lang}.data" > "$OUTPUT_DIR/perf_${lang}.trace" 2>/dev/null
    
    # Extract instruction addresses
    grep -oP '^\s+[0-9a-f]+' "$OUTPUT_DIR/perf_${lang}.trace" | \
        head -100 > "$OUTPUT_DIR/addrs_${lang}.txt"
done

echo "✅ Captured perf traces"

# 4. Extract binary sections
echo ""
echo "🔍 Extracting binary sections..."

for lang in rust cpp c; do
    objdump -d "$OUTPUT_DIR/test_${lang}" > "$OUTPUT_DIR/disasm_${lang}.txt"
    
    # Find the constant 71 (0x47) in binary
    grep -n "0x47\|\\$0x47\|#71" "$OUTPUT_DIR/disasm_${lang}.txt" | \
        head -20 > "$OUTPUT_DIR/const_refs_${lang}.txt"
    
    # Extract .rodata section
    objdump -s -j .rodata "$OUTPUT_DIR/test_${lang}" 2>/dev/null | \
        grep "47" > "$OUTPUT_DIR/rodata_${lang}.txt"
done

echo "✅ Extracted binary sections"

# 5. Analyze with our tools
echo ""
echo "🌙 Analyzing with orbit classifier..."

# Create analysis script
cat > "$OUTPUT_DIR/analyze_orbits.sh" << 'ANALYSIS'
#!/bin/bash
echo "Orbit Analysis Results:"
echo ""

for lang in rust cpp c; do
    echo "=== $lang ==="
    
    # Count instructions
    INST_COUNT=$(wc -l < "addrs_${lang}.txt")
    echo "  Instructions traced: $INST_COUNT"
    
    # Find const references
    CONST_REFS=$(wc -l < "const_refs_${lang}.txt")
    echo "  Constant 71 references: $CONST_REFS"
    
    # Check if in rodata
    if [ -s "rodata_${lang}.txt" ]; then
        echo "  Found in .rodata: YES"
    else
        echo "  Found in .rodata: NO (optimized to immediate)"
    fi
    
    echo ""
done
ANALYSIS

chmod +x "$OUTPUT_DIR/analyze_orbits.sh"
cd "$OUTPUT_DIR" && ./analyze_orbits.sh
cd ..

# 6. Summary
echo ""
echo "================================================"
echo "📋 Summary"
echo "================================================"

echo ""
echo "Files created:"
ls -lh "$OUTPUT_DIR"/*.{rs,cpp,c,data,trace,txt} 2>/dev/null | awk '{print "  " $9 " (" $5 ")"}'

echo ""
echo "Next steps:"
echo "1. Run: cargo run --release -p symbol_similarity --bin orbit_equivalence"
echo "2. This will classify the binaries by orbit"
echo "3. Prove const x=71 maps to same orbit across languages"
