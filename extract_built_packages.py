#!/usr/bin/env python3
"""Extract successfully built packages from nix-builder logs"""

import re
from pathlib import Path
from collections import defaultdict

log_dir = Path.home() / ".local/share/nix-builder/logs"
results = defaultdict(list)

for log_file in sorted(log_dir.glob("*.log")):
    content = log_file.read_text()
    
    # Extract project name (remove timestamp)
    name = re.sub(r'_\d{8}_\d{6}\.log$', '', log_file.name)
    
    # Check for success indicators
    has_error = 'error:' in content
    has_store_path = '/nix/store/' in content
    
    # Extract store paths
    store_paths = re.findall(r'/nix/store/[a-z0-9]+-[^\s\'"]+', content)
    
    status = 'failed' if has_error else 'success'
    
    results[name].append({
        'status': status,
        'store_paths': list(set(store_paths))[:10],
        'log': str(log_file),
        'has_derivation': any('drv' in p for p in store_paths)
    })

# Summarize
success = [k for k, v in results.items() if v[0]['status'] == 'success']
failed = [k for k, v in results.items() if v[0]['status'] == 'failed']

print(f"Total projects: {len(results)}")
print(f"Success: {len(success)} ({len(success)*100//len(results)}%)")
print(f"Failed: {len(failed)} ({len(failed)*100//len(results)}%)")
print()

# Show successful packages with binaries
print("=== SUCCESSFUL PACKAGES WITH STORE PATHS ===")
for name in sorted(success)[:20]:
    paths = results[name][0]['store_paths']
    if paths:
        print(f"\n{name}:")
        for p in paths[:3]:
            print(f"  {p}")

# Save full results
import json
with open('nix_build_packages.json', 'w') as f:
    json.dump(dict(results), f, indent=2)

print(f"\n\nFull results: nix_build_packages.json")
