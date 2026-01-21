#!/bin/bash
# Analyze actual CPU instructions used

PERF_FILE=$1
LANG=$2

echo "🔬 Instruction Spectrum: $LANG"
echo "=============================="
echo ""

# Disassemble top functions and count instruction types
perf annotate -i "$PERF_FILE" --stdio 2>/dev/null | \
  grep -E "^\s+[0-9a-f]+:" | \
  awk '{print $3}' | \
  grep -v "^$" | \
  sort | uniq -c | sort -rn | head -30 | \
  awk '{printf "%8d  %s\n", $1, $2}'

echo ""
echo "📊 Instruction Categories:"

# Count by instruction type
perf annotate -i "$PERF_FILE" --stdio 2>/dev/null | \
  grep -E "^\s+[0-9a-f]+:" | \
  awk '{print $3}' | \
  sed 's/[0-9]//g' | \
  sort | uniq -c | sort -rn | head -15 | \
  awk '{printf "%8d  %s\n", $1, $2}'

echo ""
