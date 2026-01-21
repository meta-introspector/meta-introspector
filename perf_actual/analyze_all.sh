#!/bin/bash
echo "🔬 Instruction Spectrum Comparison"
echo "==================================="
echo ""
printf "%-10s %10s %10s %10s %10s\n" "Language" "Total IPs" "Unique" "Diversity" "User%"
echo "----------------------------------------------------------------"

for perf in result/*.perf.data; do
    lang=$(basename $perf _actual.perf.data)
    
    output=$(../../target/release/ip_spectrum "$perf" 2>&1)
    
    total=$(echo "$output" | grep "Total IPs:" | awk '{print $4}')
    unique=$(echo "$output" | grep "Unique IPs:" | awk '{print $4}')
    user=$(echo "$output" | grep "User space:" | awk '{print $3}')
    
    if [ -n "$total" ] && [ "$total" -gt 0 ]; then
        diversity=$(echo "scale=1; $unique * 100 / $total" | bc)
        printf "%-10s %10s %10s %9s%% %9s\n" "$lang" "$total" "$unique" "$diversity" "$user"
    fi
done
