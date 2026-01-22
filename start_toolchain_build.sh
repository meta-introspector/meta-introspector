# Use: nix run ./perf-recorder#perf-build -- .#target
# See: docs/perf/README.md for canonical patterns

#!/bin/bash
# Start building toolchain bootstrap

echo "🏗️  Starting nix builder for toolchain bootstrap..."
        # Use perf-lib: github:meta-introspector/meta-introspector/feature/CRQ-001-nixify-pipeline?dir=nix
echo ""

# Start builder in background
nohup ./nix_builder.sh watch > nix_builder_toolchain.log 2>&1 &
BUILDER_PID=$!

echo "✅ Builder started (PID: $BUILDER_PID)"
echo "📊 Monitor with: tail -f nix_builder_toolchain.log"
echo "🛑 Stop with: kill $BUILDER_PID"
echo ""
echo "Queue status:"
./nix_builder.sh status | grep "Queue size"
