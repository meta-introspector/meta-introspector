#!/bin/bash
# Test LMFDB Harmonic Filters

cd /mnt/data1/meta-introspector

echo "🎵 LMFDB Harmonic Filter Tests"
echo "=============================="
echo ""

echo "📊 Baseline: Top 10% by complexity (default)"
cargo clean -q
cargo build --lib 2>&1 | grep -E "(🎵|🔬|Total)" | head -5
echo ""

echo "🔤 Filter: String functions only"
cargo clean -q
LMFDB_HARMONIC_FILTER=strings LMFDB_FILTER_PERCENT=100 cargo build --lib 2>&1 | grep -E "(🎵|🔬|Total)" | head -5
echo ""

echo "💾 Filter: Memory functions only"
cargo clean -q
LMFDB_HARMONIC_FILTER=memory LMFDB_FILTER_PERCENT=100 cargo build --lib 2>&1 | grep -E "(🎵|🔬|Total)" | head -5
echo ""

echo "🔐 Filter: Crypto functions only"
cargo clean -q
LMFDB_HARMONIC_FILTER=crypto LMFDB_FILTER_PERCENT=100 cargo build --lib 2>&1 | grep -E "(🎵|🔬|Total)" | head -5
echo ""

echo "📝 Filter: Constants only (AST complexity < 2)"
cargo clean -q
LMFDB_HARMONIC_FILTER=constants LMFDB_FILTER_PERCENT=100 cargo build --lib 2>&1 | grep -E "(🎵|🔬|Total)" | head -5
echo ""

echo "✅ Harmonic filters documented:"
echo "   LMFDB_HARMONIC_FILTER=strings   - String functions"
echo "   LMFDB_HARMONIC_FILTER=memory    - Memory management"
echo "   LMFDB_HARMONIC_FILTER=io        - I/O operations"
echo "   LMFDB_HARMONIC_FILTER=crypto    - Cryptographic functions"
echo "   LMFDB_HARMONIC_FILTER=constants - Constants (low complexity)"
echo "   LMFDB_HARMONIC_FILTER=simple    - Simple functions (conductor < 4000)"
echo "   LMFDB_HARMONIC_FILTER=complex   - Complex functions (conductor > 6000)"
echo "   LMFDB_FILTER_PERCENT=N          - Take top N% (default 10)"
