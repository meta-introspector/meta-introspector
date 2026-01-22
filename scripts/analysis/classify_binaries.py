#!/usr/bin/env python3
"""
Classify all Rust binaries by input/output types
Plan to run each on our own codebase
"""

import re
import json
from pathlib import Path
from collections import defaultdict

def classify_binary(rs_file):
    """Analyze a Rust binary's I/O patterns"""
    try:
        content = rs_file.read_text()
    except:
        return None
    
    if 'fn main(' not in content:
        return None
    
    classification = {
        'name': rs_file.stem,
        'path': str(rs_file),
        'inputs': [],
        'outputs': [],
        'category': 'unknown'
    }
    
    # Detect input types
    if 'std::env::args' in content or 'clap::' in content:
        classification['inputs'].append('cli_args')
    if 'std::fs::read' in content or 'File::open' in content:
        classification['inputs'].append('files')
    if 'stdin' in content:
        classification['inputs'].append('stdin')
    if '.parquet' in content:
        classification['inputs'].append('parquet')
    if 'perf.data' in content or 'perf_event' in content:
        classification['inputs'].append('perf_data')
    if 'git' in content.lower():
        classification['inputs'].append('git_repo')
    
    # Detect output types
    if 'std::fs::write' in content or 'File::create' in content:
        classification['outputs'].append('files')
    if 'println!' in content or 'stdout' in content:
        classification['outputs'].append('stdout')
    if '.parquet' in content and 'write' in content:
        classification['outputs'].append('parquet')
    if 'serde_json' in content:
        classification['outputs'].append('json')
    
    # Categorize by purpose
    name_lower = classification['name'].lower()
    if 'perf' in name_lower or 'telemetry' in name_lower:
        classification['category'] = 'telemetry'
    elif 'analyze' in name_lower or 'scanner' in name_lower:
        classification['category'] = 'analysis'
    elif 'build' in name_lower or 'compile' in name_lower:
        classification['category'] = 'build'
    elif 'git' in name_lower or 'repo' in name_lower:
        classification['category'] = 'git'
    elif 'query' in name_lower or 'search' in name_lower:
        classification['category'] = 'query'
    elif 'test' in name_lower or 'demo' in name_lower:
        classification['category'] = 'test'
    
    return classification

def main():
    # Find all Rust binaries
    binaries = []
    for rs_file in Path('.').rglob('*.rs'):
        if 'target/' in str(rs_file) or '.git/' in str(rs_file):
            continue
        
        classification = classify_binary(rs_file)
        if classification:
            binaries.append(classification)
    
    # Group by category
    by_category = defaultdict(list)
    for binary in binaries:
        by_category[binary['category']].append(binary)
    
    # Output classification
    output = {
        'total': len(binaries),
        'by_category': {cat: len(bins) for cat, bins in by_category.items()},
        'categories': {}
    }
    
    for category, bins in sorted(by_category.items()):
        output['categories'][category] = [
            {
                'name': b['name'],
                'path': b['path'],
                'inputs': b['inputs'],
                'outputs': b['outputs']
            }
            for b in sorted(bins, key=lambda x: x['name'])[:10]  # Top 10 per category
        ]
    
    # Save classification
    with open('binary_classification.json', 'w') as f:
        json.dump(output, f, indent=2)
    
    print(f"✅ Classified {len(binaries)} binaries")
    print(f"\nBy category:")
    for cat, count in sorted(output['by_category'].items(), key=lambda x: -x[1]):
        print(f"  {cat}: {count}")
    
    print(f"\nSaved to: binary_classification.json")

if __name__ == '__main__':
    main()
