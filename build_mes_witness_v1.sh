#!/bin/bash
# Build Mes with full telemetry capture (Witness v1)

set -e

WITNESS_DIR="/mnt/data1/meta-introspector/witnesses/mes-v1"
TIMESTAMP=$(date +%s)

mkdir -p "$WITNESS_DIR"

echo "🚀 Building GNU Mes with full telemetry (Witness v1)"
echo "📁 Output: $WITNESS_DIR"
echo ""

# Build with Nix
echo "🔨 Starting Nix build..."
nix-build /mnt/data1/meta-introspector/mes-witness-v1.nix \
  -o "$WITNESS_DIR/result" \
  2>&1 | tee "$WITNESS_DIR/nix_build_${TIMESTAMP}.log"

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Build successful!"
    echo ""
    
    # Copy telemetry to witness directory
    if [ -d "$WITNESS_DIR/result/telemetry" ]; then
        cp -r "$WITNESS_DIR/result/telemetry/"* "$WITNESS_DIR/"
        
        echo "📊 Telemetry captured:"
        ls -lh "$WITNESS_DIR/"*.{strace,log,txt,data} 2>/dev/null || true
        
        echo ""
        echo "📈 Syscall summary:"
        head -20 "$WITNESS_DIR/syscall_summary.txt" 2>/dev/null || true
        
        echo ""
        echo "🔍 Converting to parquet..."
        
        # Convert strace to parquet
        cargo run --release --bin strace_to_parquet -- \
          "$WITNESS_DIR/mes_bootstrap.strace" \
          "$WITNESS_DIR/mes_bootstrap_witness_v1.parquet"
        
        # Convert perf to parquet
        cargo run --release --bin perf_to_parquet -- \
          "$WITNESS_DIR/mes_bootstrap.perf.data" \
          "$WITNESS_DIR/mes_perf_witness_v1.parquet"
        
        echo ""
        echo "✅ Witness v1 complete!"
        echo "📊 Files:"
        ls -lh "$WITNESS_DIR/"*.parquet 2>/dev/null || true
        
    else
        echo "⚠️  No telemetry directory found"
    fi
else
    echo "❌ Build failed"
    exit 1
fi

echo ""
echo "🎯 Next steps:"
echo "  1. Analyze witness: cargo run --bin analyze_witness $WITNESS_DIR"
echo "  2. Classify with LMFDB: cargo run --bin classify_witness $WITNESS_DIR"
echo "  3. Push to HF: cargo run --bin push_witness_to_hf $WITNESS_DIR"
