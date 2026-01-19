#!/usr/bin/env python3
"""List all successfully built packages with their store paths"""

import json
import subprocess
from pathlib import Path

log_dir = Path.home() / ".local/share/nix-builder"
successful = []

for log_file in log_dir.glob("*.log"):
    content = log_file.read_text()
    
    # Check if build succeeded
    if "error:" in content.lower() or "failed" in content.lower():
        continue
    
    # Extract project name from log filename
    project = log_file.stem
    
    # Try to find store path in log
    store_paths = []
    for line in content.split('\n'):
        if '/nix/store/' in line and line.strip().startswith('/nix/store/'):
            store_paths.append(line.strip())
    
    if store_paths:
        successful.append({
            'project': project,
            'store_paths': store_paths[:5],  # First 5 paths
            'log': str(log_file)
        })

print(f"Found {len(successful)} successful builds with store paths\n")

# Group by type
binaries = []
libraries = []
other = []

for pkg in successful:
    name = pkg['project']
    if any(x in name for x in ['bin', 'tool', 'cmd', 'cli']):
        binaries.append(pkg)
    elif any(x in name for x in ['lib', 'crate', 'package']):
        libraries.append(pkg)
    else:
        other.append(pkg)

print(f"Binaries: {len(binaries)}")
print(f"Libraries: {len(libraries)}")
print(f"Other: {len(other)}")
print(f"\nTotal packages: {len(successful)}")

# Save to JSON
output = {
    'total': len(successful),
    'binaries': binaries,
    'libraries': libraries,
    'other': other
}

with open('successful_packages.json', 'w') as f:
    json.dump(output, f, indent=2)

print("\nSaved to: successful_packages.json")
