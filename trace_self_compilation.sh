#!/usr/bin/env bash
set -e

echo "🔄 Self-Compilation Trace: QEMU Plugin compiling itself"
echo ""

# Build the plugin first
echo "📦 Step 1: Building QEMU plugin..."
cd qemu-plugin
cargo build --release 2>&1 | tail -5
cd ..

PLUGIN_PATH="qemu-plugin/target/release/libqemu_reachability_plugin.so"

if [ ! -f "$PLUGIN_PATH" ]; then
    echo "❌ Plugin not found at $PLUGIN_PATH"
    exit 1
fi

echo "✅ Plugin built: $PLUGIN_PATH"
echo ""

# Create output directory
mkdir -p self_trace_output

# Trace the plugin compiling itself
echo "🔍 Step 2: Tracing plugin compilation under QEMU..."
echo "   Input: qemu-plugin/src/lib.rs"
echo "   Output: self_trace_output/libqemu_reachability_plugin.so"
echo "   Trace: self_trace_output/self_trace.parquet"
echo ""

cd qemu-plugin

# Run rustc under QEMU with our plugin
qemu-x86_64 \
  -plugin "../$PLUGIN_PATH,output=../self_trace_output/self_trace.txt,parquet=../self_trace_output/self_trace.parquet" \
  $(which rustc) \
  --crate-type cdylib \
  --edition 2021 \
  -o ../self_trace_output/libqemu_reachability_plugin_traced.so \
  src/lib.rs \
  2>&1 | tee ../self_trace_output/qemu_output.log

cd ..

echo ""
echo "✅ Step 3: Trace complete!"
echo ""

# Check outputs
if [ -f "self_trace_output/self_trace.parquet" ]; then
    SIZE=$(stat -f%z "self_trace_output/self_trace.parquet" 2>/dev/null || stat -c%s "self_trace_output/self_trace.parquet")
    echo "📊 Parquet trace: $SIZE bytes"
fi

if [ -f "self_trace_output/self_trace.txt" ]; then
    LINES=$(wc -l < self_trace_output/self_trace.txt)
    echo "📄 Text report: $LINES lines"
fi

if [ -f "self_trace_output/libqemu_reachability_plugin_traced.so" ]; then
    SIZE=$(stat -f%z "self_trace_output/libqemu_reachability_plugin_traced.so" 2>/dev/null || stat -c%s "self_trace_output/libqemu_reachability_plugin_traced.so")
    echo "🔧 Compiled plugin: $SIZE bytes"
fi

echo ""
echo "📈 Step 4: Quick analysis..."

# Show summary from text report
if [ -f "self_trace_output/self_trace.txt" ]; then
    echo ""
    echo "=== Reachability Summary ==="
    grep -E "Tracked|Total records" self_trace_output/self_trace.txt || true
fi

echo ""
echo "✅ Self-compilation trace complete!"
echo ""
echo "Output files:"
echo "  • self_trace_output/self_trace.parquet - Reachability data"
echo "  • self_trace_output/self_trace.txt - Human-readable report"
echo "  • self_trace_output/libqemu_reachability_plugin_traced.so - Compiled plugin"
echo "  • self_trace_output/qemu_output.log - QEMU output"
echo ""
echo "Next steps:"
echo "  • Analyze parquet: python -c 'import pyarrow.parquet as pq; print(pq.read_table(\"self_trace_output/self_trace.parquet\"))'"
echo "  • View report: cat self_trace_output/self_trace.txt"
echo "  • Compare binaries: diff <(xxd qemu-plugin/target/release/libqemu_reachability_plugin.so) <(xxd self_trace_output/libqemu_reachability_plugin_traced.so)"
