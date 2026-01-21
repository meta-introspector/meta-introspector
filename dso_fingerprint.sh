#!/bin/bash
# Create DSO (library) fingerprint for each language

PERF_FILE=$1
LANG=$2

echo "🔬 DSO Fingerprint: $LANG"
echo "========================="
echo ""

./target/release/symbol_resolver "$PERF_FILE" 2>/dev/null | \
  grep "^|" | tail -n +3 | \
  awk '{print $5}' | \
  sort | uniq -c | sort -rn | \
  awk '{printf "%8d  %s\n", $1, $2}'

echo ""
