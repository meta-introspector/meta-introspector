#!/bin/bash
# Resolve top IPs to function names

PERF_FILE=$1
LANG=$2

echo "🎯 Top Functions for $LANG"
echo "=========================="
echo ""

# Get top 20 user-space functions with symbols
perf report -i "$PERF_FILE" --stdio -n --sort symbol 2>/dev/null | \
  grep -E "^\s+[0-9]+\.[0-9]+%" | \
  grep -v "\[kernel\]" | \
  head -20 | \
  awk '{
    pct=$1; 
    for(i=2;i<=NF;i++) if($i !~ /^[0-9]+$/) {
      for(j=i;j<=NF;j++) printf "%s ", $j;
      break;
    }
    printf " (%s)\n", pct;
  }'

echo ""
