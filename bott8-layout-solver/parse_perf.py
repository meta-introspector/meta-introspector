#!/usr/bin/env python3
"""Parse perf stat output to JSON"""

import sys
import json
import re

def parse_perf_stat(perf_file):
    """Parse perf stat output file to structured JSON"""
    
    metrics = {}
    
    with open(perf_file, 'r') as f:
        for line in f:
            line = line.strip()
            
            # Match lines like: "1,234,567 cycles"
            match = re.match(r'^\s*([\d,]+)\s+(\S+)', line)
            if match:
                value_str = match.group(1).replace(',', '')
                metric_name = match.group(2)
                
                try:
                    value = int(value_str)
                    metrics[metric_name] = value
                except ValueError:
                    continue
            
            # Match lines with time: "1.234567 seconds time elapsed"
            match = re.match(r'^\s*([\d.]+)\s+seconds\s+(\S+)', line)
            if match:
                value = float(match.group(1))
                metric_name = match.group(2)
                metrics[f"seconds_{metric_name}"] = value
    
    return metrics

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: parse_perf.py <perf_report.txt> <output.json>")
        sys.exit(1)
    
    perf_file = sys.argv[1]
    output_file = sys.argv[2]
    
    metrics = parse_perf_stat(perf_file)
    
    with open(output_file, 'w') as f:
        json.dump(metrics, f, indent=2)
    
    print(f"✅ Parsed {len(metrics)} metrics")
