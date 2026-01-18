#!/bin/bash
# Collect logs from existing successful builds

echo "📊 Collecting existing build logs from /nix/store"
echo

# Find all build log derivations
logs=$(find /nix/store -maxdepth 1 -name "*-with-logs" -o -name "*-build-log" 2>/dev/null)
count=$(echo "$logs" | grep -v '^$' | wc -l)

echo "Found $count existing log derivations"
echo

if [ $count -gt 0 ]; then
  echo "Sample logs:"
  echo "$logs" | head -5
  echo
  
  echo "Converting to Parquet..."
  ./build-logs-to-parquet/target/release/build-logs-to-parquet /nix/store nix_build_logs_all.parquet
  
  echo
  echo "Querying results..."
  ./query-parquet/target/release/query-parquet nix_build_logs_all.parquet \
    "SELECT project, build_status, exit_code FROM nix_build_logs_all"
else
  echo "No logs found yet. Need to build with logger first."
  echo
  echo "Building hello-test as example..."
  cd universal-build-logger
  nix build .#test
  
  echo
  echo "Now collecting..."
  ./build-logs-to-parquet/target/release/build-logs-to-parquet /nix/store nix_build_logs_all.parquet
fi
