#!/bin/bash
# Queue all 71 jobs for the build server

echo "📋 Queueing all 71 Multiverse jobs..."
echo ""

# Level 0: Mes Bootstrap (already exists)
echo "✅ Level 0: Mes Bootstrap (mes-bootstrap-proof/)"
./nix_builder.sh queue $(pwd)/mes-bootstrap-proof

# Level 1: Toolchain Bootstrap (building now)
echo "✅ Level 1: Toolchain Bootstrap (toolchain-bootstrap/)"
# Already queued

# Level 2: Language Execution (already exists)
echo "✅ Level 2: Language Execution (perf_actual/)"
./nix_builder.sh queue $(pwd)/perf_actual

# Level 3: All 71 const tests
echo "📦 Level 3: Queueing all 71 language tests..."
for lang in const_71_test/*/; do
    ./nix_builder.sh queue $(pwd)/$lang
done

# Level 4: Mes-in-Languages (new)
echo "🔑 Level 4: Mes as key-value store..."
if [ -d mes-in-languages ]; then
    ./nix_builder.sh queue $(pwd)/mes-in-languages
fi

# Level 5: Feature Transport (new)
echo "🔀 Level 5: Feature transport system..."
if [ -d feature-transport ]; then
    ./nix_builder.sh queue $(pwd)/feature-transport
fi

echo ""
echo "✅ All jobs queued!"
./nix_builder.sh status
