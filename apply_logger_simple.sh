#!/bin/bash
# apply_logger_simple.sh - Simple wrapper application

set -e

echo "🔧 Applying universal-build-logger to all flakes"

# Just count and report for now
flakes=$(find /mnt/data1/nix/source -name "flake.nix" -type f 2>/dev/null)
total=$(echo "$flakes" | wc -l)

echo "Found $total flakes"
echo
echo "Sample flakes:"
echo "$flakes" | head -10
echo
echo "To apply logger, we need to:"
echo "1. Add build-logger input to each flake"
echo "2. Wrap packages.*.default with logger"
echo "3. Rebuild all flakes"
echo
echo "This will take significant time. Recommend:"
echo "- Test on 10 flakes first"
echo "- Then batch process in parallel"
echo
echo "Next: Create test batch script"
