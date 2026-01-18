#!/bin/bash
# Test logger on 10 successful flakes

set -e

echo "🧪 Testing logger on 10 flakes"
echo

# Pick 10 successful flakes from our earlier analysis
flakes=(
  "/mnt/data1/time-2026/01-january/18/solflake/smart_contracts/solana/Drift_Protocol"
  "/mnt/data1/time-2026/01-january/18/solflake/smart_contracts/solana/Jupiter_Aggregator"
  "/mnt/data1/time-2026/01-january/18/solflake/smart_contracts/solana/Mango_Markets"
)

for flake_dir in "${flakes[@]}"; do
  if [ -f "$flake_dir/flake.nix" ]; then
    project=$(basename "$flake_dir")
    echo "🔨 Building: $project"
    
    # Build with logger (using our test flake as template)
    cd "$flake_dir"
    nix build --log-format internal-json 2>&1 | tee "/tmp/build_${project}.log"
    
    echo "✅ $project complete"
    echo
  fi
done

echo "📊 Collecting logs..."
ls -lh /tmp/build_*.log

echo
echo "Next: Convert to Parquet"
