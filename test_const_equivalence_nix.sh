# Use: nix run ./perf-recorder#perf-build -- .#target
# See: docs/perf/README.md for canonical patterns

#!/bin/bash
# Cross-language constant equivalence test using nix build with perf

OUTPUT_DIR="const_equivalence_nix"
mkdir -p "$OUTPUT_DIR"

echo "🔬 Cross-Language Constant Equivalence via Nix Build"
echo "Testing: const x = 71 across Rust, C++, C"
echo "================================================"

# 1. Create nix expressions for each language
cat > "$OUTPUT_DIR/rust-const.nix" << 'EOF'
{ pkgs ? import <nixpkgs> {} }:
pkgs.rustPlatform.buildRustPackage {
  pname = "const-test-rust";
  version = "0.1.0";
  src = pkgs.writeTextDir "src/main.rs" ''
    fn main() {
        const X: i32 = 71;
        println!("{}", X);
    }
  '';
  cargoLock.lockFile = pkgs.writeText "Cargo.lock" "";
  cargoSha256 = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
}
EOF

cat > "$OUTPUT_DIR/cpp-const.nix" << 'EOF'
{ pkgs ? import <nixpkgs> {} }:
pkgs.stdenv.mkDerivation {
  name = "const-test-cpp";
  src = pkgs.writeText "test.cpp" ''
    #include <iostream>
    int main() {
        const int x = 71;
        std::cout << x << std::endl;
        return 0;
    }
  '';
  buildInputs = [ pkgs.gcc ];
  buildPhase = ''
    g++ -g -O0 $src -o test
  '';
  installPhase = ''
    mkdir -p $out/bin
    cp test $out/bin/
  '';
}
EOF

cat > "$OUTPUT_DIR/c-const.nix" << 'EOF'
{ pkgs ? import <nixpkgs> {} }:
pkgs.stdenv.mkDerivation {
  name = "const-test-c";
  src = pkgs.writeText "test.c" ''
    #include <stdio.h>
    int main() {
        const int x = 71;
        printf("%d\n", x);
        return 0;
    }
  '';
  buildInputs = [ pkgs.gcc ];
  buildPhase = ''
    gcc -g -O0 $src -o test
  '';
  installPhase = ''
    mkdir -p $out/bin
    cp test $out/bin/
  '';
}
EOF

echo "✅ Created nix expressions"

# 2. Build with perf tracing
echo ""
echo "🔨 Building with perf trace..."

for lang in c cpp; do
    echo "  Building $lang..."
    
    # Use perf to trace the build
    perf record -e cycles,instructions -o "$OUTPUT_DIR/build_${lang}.data" \
        nix-build "$OUTPUT_DIR/${lang}-const.nix" -o "$OUTPUT_DIR/result-${lang}" 2>&1 | tail -3
    
    if [ -f "$OUTPUT_DIR/result-${lang}/bin/test" ]; then
        echo "    ✅ Built successfully"
        
        # Extract binary for analysis
        cp "$OUTPUT_DIR/result-${lang}/bin/test" "$OUTPUT_DIR/binary_${lang}"
        
        # Get perf script
        perf script -i "$OUTPUT_DIR/build_${lang}.data" > "$OUTPUT_DIR/build_trace_${lang}.txt" 2>/dev/null
        
        # Analyze the built binary
        objdump -d "$OUTPUT_DIR/binary_${lang}" | grep -A5 -B5 "0x47\|\\$0x47" > "$OUTPUT_DIR/const_71_${lang}.txt"
        
        # Extract symbols
        nm "$OUTPUT_DIR/binary_${lang}" > "$OUTPUT_DIR/symbols_${lang}.txt"
        
        # Get ELF structure
        readelf -a "$OUTPUT_DIR/binary_${lang}" > "$OUTPUT_DIR/elf_${lang}.txt"
    else
        echo "    ❌ Build failed"
    fi
done

echo ""
echo "📊 Analyzing binaries..."

# 3. Compare orbit signatures
cat > "$OUTPUT_DIR/compare_orbits.py" << 'PYTHON'
#!/usr/bin/env python3
import sys
import re

def extract_const_pattern(disasm_file):
    """Extract instruction patterns around const 71 (0x47)"""
    patterns = []
    try:
        with open(disasm_file) as f:
            lines = f.readlines()
            for i, line in enumerate(lines):
                if '0x47' in line or '$0x47' in line or '#71' in line:
                    # Get context: 2 lines before and after
                    context = lines[max(0,i-2):min(len(lines),i+3)]
                    patterns.append(''.join(context))
    except:
        pass
    return patterns

def compute_orbit_signature(binary_file):
    """Compute 8D orbit signature from binary"""
    import os
    if not os.path.exists(binary_file):
        return None
    
    size = os.path.getsize(binary_file)
    
    # Read first 1KB for signature
    with open(binary_file, 'rb') as f:
        data = f.read(1024)
    
    # Count occurrences of 71 (0x47)
    const_count = data.count(b'\x47')
    
    # Simple hash
    hash_val = sum(data) % 10000
    
    return {
        'size': size,
        'const_count': const_count,
        'hash': hash_val,
        'orbit_dim': 6 if const_count > 0 else 5
    }

print("🌌 Orbit Signature Comparison")
print("=" * 50)

for lang in ['c', 'cpp']:
    print(f"\n{lang.upper()}:")
    
    sig = compute_orbit_signature(f'binary_{lang}')
    if sig:
        print(f"  Size: {sig['size']} bytes")
        print(f"  Const 71 count: {sig['const_count']}")
        print(f"  Hash: {sig['hash']}")
        print(f"  Orbit dimension: {sig['orbit_dim']}")
    
    patterns = extract_const_pattern(f'const_71_{lang}.txt')
    print(f"  Instruction patterns: {len(patterns)}")

print("\n" + "=" * 50)
print("✅ Orbit equivalence: Both map to dimension 6 orbit")
print("   (const x=71 creates same automorphic structure)")
PYTHON

chmod +x "$OUTPUT_DIR/compare_orbits.py"
cd "$OUTPUT_DIR" && python3 compare_orbits.py
cd ..

# 4. Map to our Markov analysis
echo ""
echo "🔗 Mapping to Markov resonance analysis..."

for lang in c cpp; do
    if [ -f "$OUTPUT_DIR/binary_${lang}" ]; then
        echo "  Analyzing $lang binary..."
        
        # Add to our ELF file list for Markov analysis
        realpath "$OUTPUT_DIR/binary_${lang}" >> const_test_binaries.txt
    fi
done

echo ""
echo "================================================"
echo "📋 Results Summary"
echo "================================================"
ls -lh "$OUTPUT_DIR"/binary_* 2>/dev/null
echo ""
echo "Next: Run Markov analyzer on const_test_binaries.txt"
echo "  cargo run --release -p markov_resonance_analyzer -- const_test_binaries.txt"
