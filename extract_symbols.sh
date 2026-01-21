#!/bin/bash
PERF_FILE=$1
LANG=$2

echo "🎯 Top User Functions: $LANG"
echo "============================"
echo ""

perf script -i "$PERF_FILE" 2>/dev/null | \
  grep -v "\[kernel\." | \
  grep -E "^\s+[0-9a-f]+" | \
  awk '{print $2}' | \
  grep -v "^$" | \
  sort | uniq -c | sort -rn | head -20 | \
  awk '{printf "%8d  %s\n", $1, $2}'

echo ""
