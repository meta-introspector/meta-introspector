#!/usr/bin/env python3
"""
Setup perf probes from LMFDB Parquet catalog
Instruments top-conductor functions automatically
"""

import pyarrow.parquet as pq
import subprocess
import sys

def load_top_functions(catalog_path, n=100, filter_type=None):
    """Load top N functions by conductor from Parquet"""
    print(f"📊 Loading {catalog_path}...")
    
    table = pq.read_table(catalog_path)
    df = table.to_pandas()
    
    print(f"✅ Loaded {len(df)} functions")
    
    # Filter by signature if specified
    if filter_type:
        if filter_type == 'memory':
            df = df[df['function_name'].str.contains('alloc|malloc|free', case=False, na=False)]
        elif filter_type == 'io':
            df = df[df['function_name'].str.contains('read|write|open|close', case=False, na=False)]
        elif filter_type == 'crypto':
            df = df[df['function_name'].str.contains('crypt|hash|sha|aes', case=False, na=False)]
        
        print(f"🔬 Filtered to {len(df)} {filter_type} functions")
    
    # Sort by conductor and take top N
    top = df.nlargest(n, 'conductor')
    
    return top

def add_perf_probe(binary, function, conductor):
    """Add a perf probe for a function"""
    try:
        # Try to add probe
        cmd = ['sudo', 'perf', 'probe', '-x', f'/nix/store/*/{binary}', function]
        result = subprocess.run(cmd, capture_output=True, text=True, timeout=5)
        
        if result.returncode == 0:
            print(f"  ✅ {function} (conductor: {conductor})")
            return True
        else:
            print(f"  ⚠️  {function} - {result.stderr.strip()[:50]}")
            return False
    except Exception as e:
        print(f"  ❌ {function} - {e}")
        return False

def main():
    catalog = 'data/nix_lmfdb_analysis/functions_all.parquet'
    
    filter_type = sys.argv[1] if len(sys.argv) > 1 else None
    n = int(sys.argv[2]) if len(sys.argv) > 2 else 50
    
    print(f"🔬 Setting up perf probes from LMFDB catalog")
    if filter_type:
        print(f"🎵 Filter: {filter_type}")
    print(f"🎯 Top {n} functions by conductor\n")
    
    # Load functions
    top_functions = load_top_functions(catalog, n, filter_type)
    
    print(f"\n🎯 Adding {len(top_functions)} perf probes...\n")
    
    success = 0
    failed = 0
    
    for _, row in top_functions.iterrows():
        if add_perf_probe(row['binary'], row['function_name'], row['conductor']):
            success += 1
        else:
            failed += 1
    
    print(f"\n✅ Added {success} probes")
    print(f"⚠️  Failed {failed} probes")
    
    # Show active probes
    print("\n📋 Active probes:")
    subprocess.run(['sudo', 'perf', 'probe', '-l'])
    
    print("\n🚀 Ready to record!")
    print("Run: sudo perf record -e 'probe_*' -a -- <your-command>")
    print("Then: sudo perf script > trace.txt")

if __name__ == '__main__':
    main()
