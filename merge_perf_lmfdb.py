#!/usr/bin/env python3
"""
Merge perf ranking with LMFDB symbol data to create prioritized wrapper list
"""

import json
import sys

def load_perf_ranking(perf_json):
    """Load perf ranking data"""
    with open(perf_json) as f:
        data = json.load(f)
    
    # Extract symbol names and counts
    symbols = {}
    for entry in data.get('top_symbols', []):
        symbol = entry['symbol']
        count = entry['count']
        # Clean up symbol name
        if symbol and not symbol.isdigit():
            symbols[symbol] = count
    
    return symbols

def extract_function_name(symbol):
    """Extract clean function name from mangled symbol"""
    # Remove common prefixes
    symbol = symbol.replace('__GI_', '').replace('__', '')
    
    # Extract function name before (
    if '(' in symbol:
        symbol = symbol.split('(')[0]
    
    # Extract function name before +
    if '+' in symbol:
        symbol = symbol.split('+')[0]
    
    return symbol.strip()

def main():
    if len(sys.argv) < 2:
        print("Usage: merge_perf_lmfdb.py <perf_ranking.json>")
        sys.exit(1)
    
    perf_file = sys.argv[1]
    
    print(f"📊 Loading perf ranking from {perf_file}")
    perf_symbols = load_perf_ranking(perf_file)
    
    print(f"✅ Found {len(perf_symbols)} unique symbols")
    print("\n🔥 Top 50 hottest symbols by perf count:\n")
    
    # Sort by count
    sorted_symbols = sorted(perf_symbols.items(), key=lambda x: x[1], reverse=True)
    
    # Output as JSON
    output = {
        "source": "perf_ranking",
        "total_symbols": len(sorted_symbols),
        "ranked_symbols": []
    }
    
    for i, (symbol, count) in enumerate(sorted_symbols[:200], 1):
        clean_name = extract_function_name(symbol)
        entry = {
            "rank": i,
            "symbol": symbol,
            "clean_name": clean_name,
            "perf_count": count,
            "priority": "high" if count > 10 else "medium" if count > 5 else "low"
        }
        output["ranked_symbols"].append(entry)
        
        if i <= 50:
            print(f"{i:3d}. {clean_name:40s} (count: {count:4d}) [{entry['priority']}]")
    
    # Save output
    output_file = perf_file.replace('.json', '_merged.json')
    with open(output_file, 'w') as f:
        json.dump(output, f, indent=2)
    
    print(f"\n💾 Saved merged ranking to: {output_file}")
    
    # Print summary
    high_priority = sum(1 for s in output["ranked_symbols"] if s["priority"] == "high")
    medium_priority = sum(1 for s in output["ranked_symbols"] if s["priority"] == "medium")
    low_priority = sum(1 for s in output["ranked_symbols"] if s["priority"] == "low")
    
    print(f"\n📈 Priority Summary:")
    print(f"   High priority (>10 calls):   {high_priority}")
    print(f"   Medium priority (5-10 calls): {medium_priority}")
    print(f"   Low priority (<5 calls):      {low_priority}")

if __name__ == "__main__":
    main()
