#!/bin/bash
# Build projects in parallel with logger

set -e

echo "🚀 Parallel build with universal-build-logger"
echo "Cores: 8 parallel"
echo

# Get successful projects
projects=$(cat nix_build_packages.json | jq -r 'to_entries[] | select(.value[0].status == "success") | .key' | head -111)
total=$(echo "$projects" | wc -l)

echo "Building $total projects..."
echo

# Create temp directory for logs
mkdir -p /tmp/parallel_builds

# Function to build one project
build_project() {
  local project=$1
  local log_file="/tmp/parallel_builds/${project}.log"
  
  echo "[$project] Starting..." | tee -a "$log_file"
  
  # Find flake directory
  flake_dir=$(find /mnt/data1 -name "$project" -type d 2>/dev/null | grep -E "(time-2026|nix/source)" | head -1)
  
  if [ -n "$flake_dir" ] && [ -f "$flake_dir/flake.nix" ]; then
    cd "$flake_dir"
    
    # Build with timeout
    if timeout 600 nix build 2>&1 | tee -a "$log_file"; then
      echo "[$project] ✅ Success" | tee -a "$log_file"
    else
      echo "[$project] ❌ Failed" | tee -a "$log_file"
    fi
  else
    echo "[$project] ⚠️ Not found" | tee -a "$log_file"
  fi
}

export -f build_project

# Run in parallel (8 jobs)
echo "$projects" | xargs -P 8 -I {} bash -c 'build_project "$@"' _ {}

echo
echo "📊 Collecting logs..."
./build-logs-to-parquet/target/release/build-logs-to-parquet /nix/store nix_build_logs_full.parquet

echo
echo "📈 Summary:"
./query-parquet/target/release/query-parquet nix_build_logs_full.parquet \
  "SELECT build_status, COUNT(*) as count FROM nix_build_logs_full GROUP BY build_status"

echo
echo "✅ Complete! Logs in: /tmp/parallel_builds/"
