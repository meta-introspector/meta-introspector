#!/bin/bash
# Queue all 71 language tests

echo "📋 Queueing 71 language builds..."

for dir in const_71_test/*/; do
    if [ -f "$dir/flake.nix" ]; then
        lang=$(basename "$dir")
        echo "  Queueing: $lang"
        ./nix_builder.sh queue "$(pwd)/$dir"
    fi
done

echo ""
echo "✅ All builds queued!"
./nix_builder.sh status
