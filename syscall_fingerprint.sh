#!/bin/bash
PERF_FILE=$1
LANG=$2

echo "🔬 Syscall Fingerprint: $LANG"
echo "============================="
echo ""

perf script -i "$PERF_FILE" 2>/dev/null | \
  grep -oP '(sys_|__x64_sys_)\w+' | \
  sed 's/__x64_sys_//' | sed 's/sys_//' | \
  sort | uniq -c | sort -rn | head -15 | \
  awk '{printf "%8d  %s\n", $1, $2}'

echo ""
