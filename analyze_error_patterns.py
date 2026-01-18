#!/usr/bin/env python3
"""Analyze error patterns in Nix build failures"""

import json
import re
from collections import Counter

with open('nix_build_failures.json') as f:
    data = json.load(f)

# Categorize "other" errors more specifically
error_patterns = {
    'undefined-variable': [],
    'path-not-in-git': [],
    'cannot-find-flake': [],
    'duplicate-attribute': [],
    'file-not-found': [],
    'build-failed': [],
    'flake-attribute-not-supported': [],
    'coercion-error': [],
}

for project, info in data['projects'].items():
    if info['category'] != 'other':
        continue
    
    reason = info['reason']
    
    if 'undefined variable' in reason:
        var = re.search(r"undefined variable '(\w+)'", reason)
        var_name = var.group(1) if var else 'unknown'
        error_patterns['undefined-variable'].append((project, var_name))
    
    elif 'Path' in reason and 'does not exist in Git repository' in reason:
        path = re.search(r"Path '([^']+)'", reason)
        path_name = path.group(1) if path else 'unknown'
        error_patterns['path-not-in-git'].append((project, path_name))
    
    elif 'cannot find flake' in reason:
        flake = re.search(r"cannot find flake '([^']+)'", reason)
        flake_name = flake.group(1) if flake else 'unknown'
        error_patterns['cannot-find-flake'].append((project, flake_name))
    
    elif 'already defined' in reason:
        attr = re.search(r"attribute '([^']+)'", reason)
        attr_name = attr.group(1) if attr else 'unknown'
        error_patterns['duplicate-attribute'].append((project, attr_name))
    
    elif 'Could not open file' in reason or 'No such file' in reason:
        error_patterns['file-not-found'].append((project, reason[:80]))
    
    elif 'failed to run custom build' in reason or 'build command' in reason:
        error_patterns['build-failed'].append((project, reason[:80]))
    
    elif 'attribute' in reason and 'is not supported' in reason:
        error_patterns['flake-attribute-not-supported'].append((project, reason[:80]))
    
    elif 'cannot coerce' in reason:
        error_patterns['coercion-error'].append((project, reason[:80]))

# Print report
print("# Detailed Error Pattern Analysis\n")

for pattern, errors in error_patterns.items():
    if not errors:
        continue
    
    print(f"## {pattern.replace('-', ' ').title()} ({len(errors)} projects)\n")
    
    if pattern == 'undefined-variable':
        var_counts = Counter(var for _, var in errors)
        print("Most common undefined variables:")
        for var, count in var_counts.most_common(5):
            print(f"- `{var}`: {count} projects")
        print("\nProjects:")
        for proj, var in errors[:5]:
            print(f"- {proj}: undefined `{var}`")
    
    elif pattern == 'path-not-in-git':
        print("Missing paths:")
        for proj, path in errors[:5]:
            print(f"- {proj}: `{path}`")
    
    elif pattern == 'cannot-find-flake':
        flake_counts = Counter(flake for _, flake in errors)
        print("Missing flakes:")
        for flake, count in flake_counts.most_common(5):
            print(f"- `{flake}`: {count} projects")
    
    else:
        for proj, reason in errors[:5]:
            print(f"- {proj}: {reason}")
    
    if len(errors) > 5:
        print(f"- ... and {len(errors) - 5} more")
    
    print()

# Summary
print("## Summary\n")
total_other = sum(len(errors) for errors in error_patterns.values())
print(f"Total 'other' errors analyzed: {total_other}")
print("\nTop issues:")
for pattern, errors in sorted(error_patterns.items(), key=lambda x: len(x[1]), reverse=True):
    if errors:
        print(f"- {pattern}: {len(errors)}")
