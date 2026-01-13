#!/bin/bash

# Analysis Runner for Core 3 Repositories
# Applies both Value Lattice and Markov analysis

set -e

REPOS=(
    "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build"
    "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs"
    "/home/mdupont/zos-server"
)

echo "🔬 CORE REPOSITORY ANALYSIS RUNNER"
echo "=================================="

# Build analysis tools
echo "📦 Building analysis tools..."
cd /home/mdupont/zombie_driver2
cargo build --release --bin value_lattice_indexer

echo "✅ Analysis tools built"

# Run analysis on each repo
for repo in "${REPOS[@]}"; do
    if [ -d "$repo" ]; then
        echo ""
        echo "🎯 Analyzing: $repo"
        echo "----------------------------------------"
        
        # Value Lattice Analysis
        echo "📊 Running Value Lattice analysis..."
        cd "$repo"
        /home/mdupont/zombie_driver2/target/release/value_lattice_indexer
        
        echo "✅ Analysis complete for $repo"
    else
        echo "❌ Repository not found: $repo"
    fi
done

echo ""
echo "🎉 All analyses complete!"
echo "Results saved to respective output directories"
