#!/usr/bin/env bash
set -euo pipefail

echo "🔀 Splitting ZOS into layers"

# Create layer directories
for i in {0..6}; do
    mkdir -p "zos/layer$i"
done

# Layer 0: Constants and primitives
echo "📦 Layer 0: Constants (Genus 0)"
find . -name "*.rs" -type f | while read f; do
    if grep -q "^const.*: u64 = [0-9]" "$f" 2>/dev/null; then
        echo "  $f"
    fi
done | head -10 > zos/layer0/files.txt

# Layer 1: Simple types
echo "📦 Layer 1: Simple types"
find . -name "*.rs" -type f | while read f; do
    if grep -q "^type.*=" "$f" 2>/dev/null; then
        echo "  $f"
    fi
done | head -10 > zos/layer1/files.txt

# Layer 2: Structs
echo "📦 Layer 2: Structs"
find . -name "*.rs" -type f | while read f; do
    if grep -q "^struct " "$f" 2>/dev/null; then
        echo "  $f"
    fi
done | head -10 > zos/layer2/files.txt

# Layer 3: Functions
echo "📦 Layer 3: Functions"
find . -name "*.rs" -type f | while read f; do
    if grep -q "^fn " "$f" 2>/dev/null; then
        echo "  $f"
    fi
done | head -10 > zos/layer3/files.txt

# Layer 4: Recursive types
echo "📦 Layer 4: Recursive types (Genus 2)"
find . -name "*.rs" -type f | while read f; do
    if grep -q "Box<.*Self" "$f" 2>/dev/null; then
        echo "  $f"
    fi
done | head -10 > zos/layer4/files.txt

# Layer 5: Complex systems
echo "📦 Layer 5: Complex systems"
ls *.rs 2>/dev/null | head -10 > zos/layer5/files.txt || true

# Layer 6: Documentation and meta
echo "📦 Layer 6: Documentation"
ls zos/*.md 2>/dev/null > zos/layer6/files.txt || true

# Summary
echo ""
echo "✅ Layers created:"
for i in {0..6}; do
    count=$(wc -l < "zos/layer$i/files.txt" 2>/dev/null || echo 0)
    echo "  Layer $i: $count files"
done
