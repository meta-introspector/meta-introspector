#!/bin/bash
# Build all P2P git mirror components with Nix

set -e

echo "🏗️  Building P2P Git Mirror with Nix..."

# Build all components
nix build .#all --print-build-logs

echo ""
echo "✅ Build complete!"
echo ""
echo "Binaries available in:"
ls -lh result/bin/

echo ""
echo "🔬 Running binary similarity search..."
cargo build --release --bin binary_similarity_search
./target/release/binary_similarity_search

echo ""
echo "📊 Results:"
echo "  - Binaries: result/bin/"
echo "  - Similarities: data/binary_similarities.parquet"
