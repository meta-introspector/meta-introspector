#!/bin/bash
# Queue all 500 repos for Nix building

cd /mnt/data1/meta-introspector

echo "🔄 Queueing all repos with Nix flakes..."
echo ""

count=0
jq -r '.repos[] | select(.path != null) | .path' data/repo_registry.json | while read -r repo; do
    if [ -d "$repo" ] && [ -f "$repo/flake.nix" ]; then
        ./nix_builder.sh queue "$repo" 2>&1 | grep "Added to queue"
        ((count++))
    fi
done

echo ""
echo "✅ Queued repos with flakes"
echo ""
./nix_builder.sh status
