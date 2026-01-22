# Use: nix run ./perf-recorder#perf-build -- .#target
# See: docs/perf/README.md for canonical patterns

#!/usr/bin/env bash
set -euo pipefail

LAYER="$1"
BINARY="$2"

echo "🔬 Validating Layer $LAYER: $BINARY"

# Create validation directory
VALIDATION_DIR="zos-validation/layer-$LAYER"
mkdir -p "$VALIDATION_DIR"

# 1. QEMU trace
echo "📊 Running QEMU trace..."
qemu-x86_64 -d exec,cpu,in_asm "$BINARY" > "$VALIDATION_DIR/qemu_trace.log" 2>&1 || true

# 2. Perf analysis
echo "⚡ Running perf analysis..."
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
perf report -i "$VALIDATION_DIR/perf.data" > "$VALIDATION_DIR/perf_report.txt" 2>/dev/null || true

# 3. Strace
echo "🔍 Running strace..."
strace -o "$VALIDATION_DIR/strace.log" -f -tt "$BINARY" 2>/dev/null || true

# 4. Goblin binary analysis
echo "🧙 Running goblin analysis..."
cargo run --bin binary_analysis "$BINARY" > "$VALIDATION_DIR/goblin.json" 2>/dev/null || true

# 5. Harmonic analysis
echo "🎵 Running harmonic analysis..."
cargo run --bin harmonic_filter "$BINARY" > "$VALIDATION_DIR/harmonic.json" 2>/dev/null || true

# 6. Compress to parquet
echo "💾 Compressing to parquet..."
cargo run --bin compress_validation "$VALIDATION_DIR" "$VALIDATION_DIR/validation.parquet"

# 7. Calculate score
SCORE=$(cargo run --bin calculate_score "$VALIDATION_DIR/validation.parquet")

echo "📈 Validation score: $SCORE"

if (( $(echo "$SCORE > 0.8" | bc -l) )); then
    echo "✅ Layer $LAYER PASSED"
    touch "$VALIDATION_DIR/PASSED"
    exit 0
else
    echo "❌ Layer $LAYER FAILED"
    touch "$VALIDATION_DIR/FAILED"
    exit 1
fi
