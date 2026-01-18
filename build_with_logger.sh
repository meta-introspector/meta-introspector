#!/bin/bash
# Build projects with universal-build-logger

echo "🔨 Building projects with logger"
echo

# Get list of successful projects
projects=$(cat nix_build_packages.json | jq -r 'to_entries[] | select(.value[0].status == "success") | .key' | head -20)

echo "Building 20 successful projects..."
echo

for project in $projects; do
  echo "📦 $project"
  
  # Find the flake directory
  flake_dir=$(find /mnt/data1 -name "$project" -type d 2>/dev/null | grep -E "(time-2026|nix/source)" | head -1)
  
  if [ -n "$flake_dir" ] && [ -f "$flake_dir/flake.nix" ]; then
    echo "  Found: $flake_dir"
    
    # Build it (already has logger if it's a recent build)
    cd "$flake_dir"
    timeout 60 nix build 2>&1 | tail -5
    
    echo "  ✅ Built"
  else
    echo "  ⚠️  Flake not found"
  fi
  echo
done

echo
echo "📊 Collecting all logs..."
./build-logs-to-parquet/target/release/build-logs-to-parquet /nix/store nix_build_logs_batch.parquet

echo
echo "📈 Summary:"
./query-parquet/target/release/query-parquet nix_build_logs_batch.parquet \
  "SELECT build_status, COUNT(*) as count FROM nix_build_logs_batch GROUP BY build_status"
