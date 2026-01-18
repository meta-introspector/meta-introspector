#!/usr/bin/env python3
"""Classify Nix build failures"""

import json
import re
from pathlib import Path
from collections import Counter, defaultdict

class NixFailureClassifier:
    def __init__(self, logs_dir):
        self.logs_dir = Path(logs_dir)
        self.classifications = defaultdict(list)
        
    def classify_log(self, log_file):
        """Classify a single build log"""
        with open(log_file) as f:
            content = f.read()
        
        project = log_file.stem.rsplit('_', 2)[0]
        
        # Check for success
        if 'all checks passed!' in content:
            return project, 'success', None
        
        # Classify failures
        if 'does not provide attribute' in content:
            if 'packages.x86_64-linux.default' in content:
                return project, 'missing-default', 'No packages.default attribute'
        
        if 'error: cannot build derivation' in content:
            return project, 'cannot-build', 'Derivation build failed'
        
        if 'error: flake' in content and 'does not provide' in content:
            return project, 'flake-url', 'Invalid flake URL or reference'
        
        if 'assertion failed' in content:
            match = re.search(r'assertion failed: (.+)', content)
            reason = match.group(1) if match else 'Unknown assertion'
            return project, 'assertion', reason
        
        if 'error: attribute' in content and 'missing' in content:
            return project, 'missing-attr', 'Missing attribute'
        
        if 'error: hash mismatch' in content:
            return project, 'hash-mismatch', 'Hash mismatch in fetchurl/fetchgit'
        
        if 'error: builder for' in content and 'failed' in content:
            return project, 'builder-failed', 'Builder script failed'
        
        if 'error: getting status of' in content:
            return project, 'file-not-found', 'File or directory not found'
        
        # Generic error
        error_match = re.search(r'error: (.+)', content)
        if error_match:
            error_msg = error_match.group(1)[:100]
            return project, 'other', error_msg
        
        return project, 'unknown', 'Unknown failure'
    
    def classify_all(self):
        """Classify all build logs"""
        results = {
            'total': 0,
            'success': 0,
            'failed': 0,
            'by_category': Counter(),
            'projects': defaultdict(dict)
        }
        
        for log_file in self.logs_dir.glob('*.log'):
            results['total'] += 1
            
            project, category, reason = self.classify_log(log_file)
            
            if category == 'success':
                results['success'] += 1
            else:
                results['failed'] += 1
                results['by_category'][category] += 1
            
            results['projects'][project] = {
                'category': category,
                'reason': reason,
                'log': str(log_file)
            }
        
        return results
    
    def generate_report(self, results):
        """Generate markdown report"""
        report = []
        report.append("# Nix Build Failure Classification\n")
        
        # Summary
        report.append("## Summary\n")
        report.append(f"- **Total builds**: {results['total']}")
        report.append(f"- **Successful**: {results['success']} ({results['success']/results['total']*100:.1f}%)")
        report.append(f"- **Failed**: {results['failed']} ({results['failed']/results['total']*100:.1f}%)\n")
        
        # By category
        report.append("## Failures by Category\n")
        for category, count in results['by_category'].most_common():
            pct = count / results['failed'] * 100
            report.append(f"- **{category}**: {count} ({pct:.1f}%)")
        report.append("")
        
        # Detailed breakdown
        report.append("## Detailed Breakdown\n")
        
        by_category = defaultdict(list)
        for project, data in results['projects'].items():
            if data['category'] != 'success':
                by_category[data['category']].append((project, data['reason']))
        
        for category in sorted(by_category.keys()):
            report.append(f"### {category} ({len(by_category[category])} projects)\n")
            for project, reason in sorted(by_category[category])[:10]:
                report.append(f"- **{project}**: {reason}")
            if len(by_category[category]) > 10:
                report.append(f"- ... and {len(by_category[category]) - 10} more")
            report.append("")
        
        # Recommendations
        report.append("## Recommendations\n")
        
        if 'missing-default' in results['by_category']:
            count = results['by_category']['missing-default']
            report.append(f"### Fix missing-default ({count} projects)")
            report.append("Add `packages.${{system}}.default` to flake.nix:")
            report.append("```nix")
            report.append("packages.${system}.default = pkgs.hello;")
            report.append("```\n")
        
        if 'cannot-build' in results['by_category']:
            count = results['by_category']['cannot-build']
            report.append(f"### Fix cannot-build ({count} projects)")
            report.append("Check derivation dependencies and build inputs.\n")
        
        if 'hash-mismatch' in results['by_category']:
            count = results['by_category']['hash-mismatch']
            report.append(f"### Fix hash-mismatch ({count} projects)")
            report.append("Update hashes with: `nix flake update`\n")
        
        return '\n'.join(report)

# Run classification
classifier = NixFailureClassifier('/home/mdupont/.local/share/nix-builder/logs')
results = classifier.classify_all()
report = classifier.generate_report(results)

# Save report
with open('NIX_BUILD_FAILURES.md', 'w') as f:
    f.write(report)

# Save JSON
with open('nix_build_failures.json', 'w') as f:
    json.dump(results, f, indent=2, default=str)

print(report)
