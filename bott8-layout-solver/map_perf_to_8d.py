#!/usr/bin/env python3
"""Map perf metrics to 8D Bott manifold coordinates"""

import sys
import json
import math

def normalize(value, min_val, max_val):
    """Normalize value to [-100, 100] range"""
    if max_val == min_val:
        return 0
    normalized = ((value - min_val) / (max_val - min_val)) * 200 - 100
    return int(normalized)

def map_perf_to_8d(metrics):
    """Map perf metrics to 8D Bott manifold coordinates
    
    Dimensions:
    1. Real (R) - CPU cycles (computational intensity)
    2. Complex (C) - Instructions (algorithmic complexity)
    3. Quaternion (H) - Cache behavior (memory patterns)
    4. Octonion (O) - Branch prediction (control flow)
    5. Time (T) - Elapsed time (temporal)
    6. Information (I) - IPC (information throughput)
    7. Social (S) - Context switches (interaction)
    8. Semantic (M) - Page faults (semantic access patterns)
    """
    
    # Extract metrics with defaults
    cycles = metrics.get('cycles', 0)
    instructions = metrics.get('instructions', 0)
    cache_refs = metrics.get('cache-references', 0)
    cache_misses = metrics.get('cache-misses', 0)
    branches = metrics.get('branches', 0)
    branch_misses = metrics.get('branch-misses', 0)
    cpu_clock = metrics.get('cpu-clock', 0)
    task_clock = metrics.get('task-clock', 0)
    page_faults = metrics.get('page-faults', 0)
    context_switches = metrics.get('context-switches', 0)
    
    # Calculate derived metrics
    ipc = instructions / cycles if cycles > 0 else 0
    cache_miss_rate = cache_misses / cache_refs if cache_refs > 0 else 0
    branch_miss_rate = branch_misses / branches if branches > 0 else 0
    
    # Map to 8D coordinates (normalized to [-100, 100])
    # Using log scale for large values
    
    coords = {
        "Real": int(math.log10(cycles + 1) * 10) if cycles > 0 else 0,
        "Complex": int(math.log10(instructions + 1) * 10) if instructions > 0 else 0,
        "Quaternion": int(cache_miss_rate * 100) - 50,
        "Octonion": int(branch_miss_rate * 100) - 50,
        "Time": int(math.log10(cpu_clock + 1) * 10) if cpu_clock > 0 else 0,
        "Information": int(ipc * 50) if ipc < 2 else 100,
        "Social": int(math.log10(context_switches + 1) * 20) if context_switches > 0 else 0,
        "Semantic": int(math.log10(page_faults + 1) * 20) if page_faults > 0 else 0
    }
    
    # Clamp to [-100, 100]
    for key in coords:
        coords[key] = max(-100, min(100, coords[key]))
    
    return {
        "coordinates_8d": coords,
        "raw_metrics": metrics,
        "derived_metrics": {
            "ipc": ipc,
            "cache_miss_rate": cache_miss_rate,
            "branch_miss_rate": branch_miss_rate
        }
    }

if __name__ == "__main__":
    if len(sys.argv) != 3:
        print("Usage: map_perf_to_8d.py <perf.json> <output_8d.json>")
        sys.exit(1)
    
    perf_json = sys.argv[1]
    output_json = sys.argv[2]
    
    with open(perf_json, 'r') as f:
        metrics = json.load(f)
    
    result = map_perf_to_8d(metrics)
    
    with open(output_json, 'w') as f:
        json.dump(result, f, indent=2)
    
    print(f"✅ Mapped to 8D coordinates:")
    for dim, val in result["coordinates_8d"].items():
        print(f"  {dim:12s}: {val:4d}")
