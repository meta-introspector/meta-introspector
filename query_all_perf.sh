#!/bin/bash
echo "🔬 Language Instruction Spectrum Comparison"
echo "==========================================="
echo ""

for parquet in *_perf.parquet; do
    [ -f "$parquet" ] || continue
    lang=$(basename "$parquet" _perf.parquet)
    
    ./query-parquet/target/release/query-parquet "$parquet" \
        "SELECT '$lang' as language, COUNT(*) as samples, COUNT(DISTINCT ip) as unique_ips, 
         ROUND(COUNT(DISTINCT ip) * 100.0 / COUNT(*), 1) as diversity 
         FROM ${lang}_perf" 2>&1 | grep -A 5 "^|" | tail -2
done
