#!/bin/bash
# Extract instruction/function spectrum from perf data

PERF_FILE=$1
LANG=$2

echo "🔬 Fingerprinting $LANG"
echo "======================="

# Top 10 instruction pointers
echo ""
echo "📍 Top 10 Instruction Pointers:"
perf script -i "$PERF_FILE" 2>/dev/null | \
  awk '{print $4}' | \
  grep -v "^$" | \
  sort | uniq -c | sort -rn | head -10 | \
  awk '{printf "  %8d  %s\n", $1, $2}'

# Top 10 functions (symbols)
echo ""
echo "🎯 Top 10 Functions:"
perf report -i "$PERF_FILE" --stdio -n 2>/dev/null | \
  grep "%" | head -10 | \
  awk '{printf "  %6s  %s\n", $1, $NF}'

# Syscall distribution
echo ""
echo "🚪 Syscall Distribution:"
perf script -i "$PERF_FILE" 2>/dev/null | \
  grep -oP '(read|write|open|close|mmap|fork|exec|stat)' | \
  sort | uniq -c | sort -rn | head -5 | \
  awk '{printf "  %8d  %s\n", $1, $2}'

# Unique instruction count
echo ""
echo "📊 Spectrum Stats:"
unique_ips=$(perf script -i "$PERF_FILE" 2>/dev/null | awk '{print $4}' | grep -v "^$" | sort -u | wc -l)
total_samples=$(perf script -i "$PERF_FILE" 2>/dev/null | wc -l)
echo "  Unique IPs:     $unique_ips"
echo "  Total samples:  $total_samples"
echo "  Diversity:      $(echo "scale=2; $unique_ips / $total_samples * 100" | bc)%"

echo ""
