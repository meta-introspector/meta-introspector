#!/usr/bin/env bash
set -e

OUTPUT_LOG="qemu-plugin-build.log"

echo "Starting QEMU plugin Nix build in background..."
echo "Output will be captured to: $OUTPUT_LOG"

# Run in background, capture all output
nix build .#qemu-plugin --show-trace > "$OUTPUT_LOG" 2>&1 &
BUILD_PID=$!

echo "Build PID: $BUILD_PID"
echo "Monitor with: tail -f $OUTPUT_LOG"
echo "Check status: ps -p $BUILD_PID"

# Wait for completion
wait $BUILD_PID
EXIT_CODE=$?

if [ $EXIT_CODE -eq 0 ]; then
    echo "✅ Build succeeded!"
    ls -lh result/lib/*.so 2>/dev/null || echo "Library location:"
    readlink -f result
else
    echo "❌ Build failed with exit code: $EXIT_CODE"
    echo "Last 50 lines of output:"
    tail -50 "$OUTPUT_LOG"
fi

exit $EXIT_CODE
