#!/usr/bin/env python3
"""Analyze project ownership from git remotes"""

import subprocess
import json
from pathlib import Path
from collections import Counter

def get_git_remote(project_path):
    """Get git remote URL for a project"""
    try:
        result = subprocess.run(
            ['git', '-C', project_path, 'remote', 'get-url', 'origin'],
            capture_output=True,
            text=True,
            timeout=2
        )
        if result.returncode == 0:
            return result.stdout.strip()
    except:
        pass
    return None

def parse_owner(remote_url):
    """Extract owner from git remote URL"""
    if not remote_url:
        return "local"
    
    # github.com/owner/repo
    if 'github.com' in remote_url:
        parts = remote_url.split('github.com')[-1].strip('/:').split('/')
        if len(parts) >= 2:
            return parts[0]
    
    # gitlab.com/owner/repo
    if 'gitlab.com' in remote_url:
        parts = remote_url.split('gitlab.com')[-1].strip('/:').split('/')
        if len(parts) >= 2:
            return parts[0]
    
    return "unknown"

# Load failed projects
with open('nix_build_failures.json') as f:
    data = json.load(f)

failed_projects = [
    name for name, info in data['projects'].items()
    if info['category'] != 'success'
]

print(f"Analyzing {len(failed_projects)} failed projects...\n")

# Analyze ownership
ownership = {}
for project in failed_projects:
    # Try to find the project directory
    paths = [
        f"/mnt/data1/nix/source/github/meta-introspector/streamofrandom/2025/{project}",
        f"/mnt/data1/nix/source/github/meta-introspector/streamofrandom/2025/flakes/{project}",
        f"/mnt/data1/nix/source/github/meta-introspector/streamofrandom/2025/10/{project}",
        f"/mnt/data1/meta-introspector/{project}",
    ]
    
    remote = None
    for path in paths:
        if Path(path).exists():
            remote = get_git_remote(path)
            if remote:
                break
    
    owner = parse_owner(remote)
    ownership[project] = {
        'owner': owner,
        'remote': remote or 'local'
    }

# Count by owner
owner_counts = Counter(info['owner'] for info in ownership.values())

print("## Ownership Summary\n")
print("| Owner | Count | % |")
print("|-------|-------|---|")
for owner, count in owner_counts.most_common():
    pct = count / len(failed_projects) * 100
    print(f"| {owner} | {count} | {pct:.1f}% |")

print(f"\n**Total:** {len(failed_projects)} failed projects\n")

# Show meta-introspector projects
meta_projects = [
    name for name, info in ownership.items()
    if info['owner'] == 'meta-introspector'
]

if meta_projects:
    print(f"\n## meta-introspector Projects ({len(meta_projects)})\n")
    for project in sorted(meta_projects)[:20]:
        print(f"- {project}")
    if len(meta_projects) > 20:
        print(f"- ... and {len(meta_projects) - 20} more")

# Show local projects
local_projects = [
    name for name, info in ownership.items()
    if info['owner'] == 'local'
]

if local_projects:
    print(f"\n## Local Projects (No Git Remote) ({len(local_projects)})\n")
    print("These are experimental flakes in streamofrandom/")
    print(f"Total: {len(local_projects)} projects")
    print("\nSample:")
    for project in sorted(local_projects)[:10]:
        print(f"- {project}")

