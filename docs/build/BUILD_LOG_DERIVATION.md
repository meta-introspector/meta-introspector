# Build Log Derivation System

## Vision
Every build (success OR failure) creates a log derivation in /nix/store with full semantic metadata. Share as structured Parquet dataset on HuggingFace.

## The Problem with Current System
```bash
# Build fails → logs in /tmp (ephemeral)
nix build → error → /tmp/nix-build-xyz.log → LOST

# Build succeeds → no metadata
nix build → /nix/store/abc-project → NO CONTEXT
```

## The Solution: Log Derivation

Every build creates TWO derivations:
1. **Project derivation** (if successful)
2. **Log derivation** (ALWAYS, even on failure)

```nix
{
  outputs = { self, nixpkgs }: {
    # Always create log derivation FIRST
    packages.x86_64-linux.build-log = pkgs.runCommand "project-build-log" {
      # Metadata inputs
      gitCommit = self.rev or "dirty";
      gitBranch = self.ref or "unknown";
      buildTime = builtins.currentTime;
    } ''
      mkdir -p $out
      
      # 1. Source upstream package state
      cat > $out/1-upstream.json <<EOF
      {
        "package": "project-name",
        "upstream_url": "https://github.com/upstream/project",
        "upstream_version": "v1.2.3",
        "upstream_commit": "abc123",
        "last_sync": "2026-01-18",
        "tracking_branch": "main"
      }
      EOF
      
      # 2. Current branch and fork state
      cat > $out/2-fork-state.json <<EOF
      {
        "fork_url": "https://github.com/meta-introspector/project",
        "branch": "$gitBranch",
        "commit": "$gitCommit",
        "commits_ahead": 5,
        "commits_behind": 2,
        "open_issues": 3,
        "open_prs": 1,
        "known_vulns": [],
        "pending_patches": ["fix-build.patch", "update-deps.patch"]
      }
      EOF
      
      # 3. Build state
      cat > $out/3-build-state.json <<EOF
      {
        "status": "unknown",
        "build_time": $buildTime,
        "system": "${builtins.currentSystem}",
        "nix_version": "${builtins.nixVersion}",
        "attempt": 1,
        "previous_attempts": []
      }
      EOF
      
      # 4. Collected information
      cat > $out/4-collected-info.json <<EOF
      {
        "analysis_phases": {
          "source_archive": "pending",
          "ngrams": "pending",
          "markov": "pending",
          "embeddings": "pending",
          "cargo_build": "pending",
          "rust_ir": "pending",
          "binary_analysis": "pending"
        },
        "dependencies": [],
        "file_count": 0,
        "loc": 0,
        "languages": []
      }
      EOF
      
      # 5. Missing information
      cat > $out/5-missing.json <<EOF
      {
        "missing": [
          "upstream vulnerability scan",
          "dependency graph",
          "test coverage",
          "documentation coverage",
          "license compliance"
        ],
        "blockers": [],
        "next_steps": [
          "Run phase 1-4 analysis",
          "Collect dependencies",
          "Scan for vulnerabilities"
        ]
      }
      EOF
      
      # Build log placeholder
      echo "Build not started" > $out/build.log
      
      # Summary
      cat > $out/summary.json <<EOF
      {
        "project": "project-name",
        "git_commit": "$gitCommit",
        "build_time": $buildTime,
        "log_derivation": "$out",
        "status": "initialized"
      }
      EOF
    '';
    
    # Then try actual build (wrapped to capture logs)
    packages.x86_64-linux.default = pkgs.runCommand "project-with-logs" {
      buildLog = self.packages.x86_64-linux.build-log;
    } ''
      # Try to build
      set +e
      ${self.packages.x86_64-linux.actual-build} 2>&1 | tee build.log
      BUILD_STATUS=$?
      set -e
      
      # Update log derivation
      cp -r $buildLog $out
      
      # Update build state
      cat > $out/3-build-state.json <<EOF
      {
        "status": $([ $BUILD_STATUS -eq 0 ] && echo "\"success\"" || echo "\"failed\""),
        "exit_code": $BUILD_STATUS,
        "build_log": "$out/build.log"
      }
      EOF
      
      # If success, link result
      if [ $BUILD_STATUS -eq 0 ]; then
        ln -s ${self.packages.x86_64-linux.actual-build} $out/result
      fi
      
      # Always save logs
      cp build.log $out/
    '';
  };
}
```

## Universal Build Wrapper Template

```nix
# universal-build-logger.nix
{ pkgs, self, project }:

let
  # Collect upstream info
  upstreamInfo = pkgs.runCommand "upstream-info" {} ''
    mkdir -p $out
    
    # Query GitHub API for upstream
    ${pkgs.curl}/bin/curl -s https://api.github.com/repos/upstream/project > $out/upstream.json
    
    # Check for vulnerabilities
    ${pkgs.curl}/bin/curl -s https://api.osv.dev/v1/query -d '{"package":{"name":"project"}}' > $out/vulns.json
    
    # Check for pending patches
    ${pkgs.git}/bin/git log --oneline upstream/main..HEAD > $out/pending-patches.txt || echo "none" > $out/pending-patches.txt
  '';
  
  # Collect fork state
  forkState = pkgs.runCommand "fork-state" {} ''
    mkdir -p $out
    
    # Current branch info
    echo '{"branch":"${self.ref or "unknown"}","commit":"${self.rev or "dirty"}"}' > $out/fork.json
    
    # Open issues/PRs
    ${pkgs.curl}/bin/curl -s https://api.github.com/repos/meta-introspector/project/issues > $out/issues.json
  '';
  
  # Run analysis phases
  analysisInfo = pkgs.runCommand "analysis-info" {} ''
    mkdir -p $out
    
    # Phase 1: Source archive
    ${pkgs.tree}/bin/tree ${self} > $out/file-tree.txt
    find ${self} -type f | wc -l > $out/file-count.txt
    
    # Phase 2-4: Placeholder for now
    echo '{"status":"pending"}' > $out/ngrams.json
    echo '{"status":"pending"}' > $out/markov.json
    echo '{"status":"pending"}' > $out/embeddings.json
  '';

in pkgs.runCommand "${project.name}-build-log" {
  inherit upstreamInfo forkState analysisInfo;
  buildTime = builtins.currentTime;
  gitCommit = self.rev or "dirty";
} ''
  mkdir -p $out
  
  # 1. Upstream state
  cp -r $upstreamInfo $out/1-upstream
  
  # 2. Fork state
  cp -r $forkState $out/2-fork-state
  
  # 3. Build state (will be updated)
  cat > $out/3-build-state.json <<EOF
  {
    "status": "building",
    "start_time": $buildTime,
    "system": "${builtins.currentSystem}",
    "nix_version": "${builtins.nixVersion}"
  }
  EOF
  
  # 4. Collected info
  cp -r $analysisInfo $out/4-collected-info
  
  # 5. Missing info
  cat > $out/5-missing.json <<EOF
  {
    "missing": [
      "full dependency graph",
      "test results",
      "benchmark results",
      "security scan"
    ]
  }
  EOF
  
  # Try to build project
  set +e
  ${project} 2>&1 | tee $out/build.log
  BUILD_EXIT=$?
  set -e
  
  # Update build state
  cat > $out/3-build-state.json <<EOF
  {
    "status": $([ $BUILD_EXIT -eq 0 ] && echo "\"success\"" || echo "\"failed\""),
    "exit_code": $BUILD_EXIT,
    "end_time": $(date +%s),
    "duration": $(($(date +%s) - $buildTime)),
    "log_path": "$out/build.log"
  }
  EOF
  
  # If success, link result
  if [ $BUILD_EXIT -eq 0 ]; then
    ln -s ${project} $out/result
  fi
  
  # Create summary
  cat > $out/summary.json <<EOF
  {
    "project": "${project.name}",
    "git_commit": "$gitCommit",
    "build_status": $([ $BUILD_EXIT -eq 0 ] && echo "\"success\"" || echo "\"failed\""),
    "log_derivation": "$out",
    "upstream": "$upstreamInfo",
    "fork_state": "$forkState",
    "analysis": "$analysisInfo"
  }
  EOF
''
```

## Apply to All 500 Projects

```bash
#!/bin/bash
# wrap_with_logger.sh

for flake in $(find /mnt/data1/nix/source -name "flake.nix"); do
  dir=$(dirname "$flake")
  project=$(basename "$dir")
  
  echo "Adding build logger to: $project"
  
  # Add build-logger input
  sed -i '/inputs = {/a\    build-logger.url = "github:meta-introspector/build-logger";' "$flake"
  
  # Wrap packages
  sed -i 's/packages\.\(.*\)\.default = \(.*\);/packages.\1.default = build-logger.lib.wrap { inherit pkgs self; project = \2; };/' "$flake"
done
```

## Result: Every Build Creates Log Derivation

```bash
# Build succeeds
nix build → /nix/store/abc-project-build-log/
├── 1-upstream/
│   ├── upstream.json
│   └── vulns.json
├── 2-fork-state/
│   ├── fork.json
│   └── issues.json
├── 3-build-state.json
├── 4-collected-info/
│   ├── file-tree.txt
│   └── ngrams.json
├── 5-missing.json
├── build.log
├── summary.json
└── result → /nix/store/xyz-project

# Build fails
nix build → /nix/store/def-project-build-log/
├── 1-upstream/
├── 2-fork-state/
├── 3-build-state.json  # status: "failed", exit_code: 1
├── 4-collected-info/
├── 5-missing.json
├── build.log           # Full error output
└── summary.json        # No result link
```

## Export to Parquet for HuggingFace

```python
#!/usr/bin/env python3
"""Convert all build logs to Parquet dataset"""

import json
import pandas as pd
from pathlib import Path

# Find all build log derivations
store = Path("/nix/store")
build_logs = list(store.glob("*-build-log"))

records = []
for log_dir in build_logs:
    # Read all JSON files
    summary = json.loads((log_dir / "summary.json").read_text())
    upstream = json.loads((log_dir / "1-upstream/upstream.json").read_text())
    fork = json.loads((log_dir / "2-fork-state/fork.json").read_text())
    build = json.loads((log_dir / "3-build-state.json").read_text())
    
    # Read build log
    build_log = (log_dir / "build.log").read_text()
    
    records.append({
        'project': summary['project'],
        'git_commit': summary['git_commit'],
        'build_status': build['status'],
        'exit_code': build.get('exit_code', 0),
        'duration': build.get('duration', 0),
        'upstream_url': upstream.get('html_url', ''),
        'upstream_version': upstream.get('default_branch', ''),
        'fork_branch': fork['branch'],
        'fork_commit': fork['commit'],
        'build_log': build_log,
        'log_derivation': str(log_dir),
        'nix_version': build['nix_version'],
        'system': build['system']
    })

# Create DataFrame
df = pd.DataFrame(records)

# Save to Parquet
df.to_parquet('nix_build_logs.parquet', compression='zstd')

print(f"Exported {len(records)} build logs to nix_build_logs.parquet")
print(f"Size: {Path('nix_build_logs.parquet').stat().st_size / 1024 / 1024:.2f} MB")
```

## Push to HuggingFace

```bash
#!/bin/bash
# push_build_logs.sh

# Convert all logs to Parquet
python3 export_build_logs_to_parquet.py

# Upload to HuggingFace
huggingface-cli upload \
  introspector/nix-build-logs \
  nix_build_logs.parquet \
  --repo-type dataset

echo "Build logs available at: https://huggingface.co/datasets/introspector/nix-build-logs"
```

## Schema: nix_build_logs.parquet

```
project              string      Project name
git_commit           string      Git commit hash
build_status         string      success/failed
exit_code            int64       Build exit code
duration             int64       Build duration (seconds)
upstream_url         string      Upstream repository URL
upstream_version     string      Upstream version/branch
fork_branch          string      Our branch
fork_commit          string      Our commit
build_log            string      Full build output
log_derivation       string      /nix/store path to logs
nix_version          string      Nix version used
system               string      Build system (x86_64-linux)
build_time           timestamp   When built
vulns_count          int64       Known vulnerabilities
open_issues          int64       Open issues
pending_patches      int64       Pending patches
```

## Benefits

### 1. No More Lost Logs
- All logs in /nix/store (permanent)
- Failures captured same as successes
- Can share logs via IPFS/HF

### 2. Full Semantic Context
- Know upstream state
- Know fork state
- Know what's missing
- Know next steps

### 3. Quality Dataset
- Structured Parquet format
- Queryable with DuckDB/SQL
- Grows over time
- Shareable on HuggingFace

### 4. Reproducible Debugging
- Every failure has full context
- Can rebuild with same inputs
- Can trace to upstream issues

## Query Examples

```sql
-- Find all failed builds
SELECT project, exit_code, build_log 
FROM 'nix_build_logs.parquet' 
WHERE build_status = 'failed';

-- Find builds with known vulnerabilities
SELECT project, vulns_count, upstream_url
FROM 'nix_build_logs.parquet'
WHERE vulns_count > 0;

-- Find builds that need patches
SELECT project, pending_patches, fork_branch
FROM 'nix_build_logs.parquet'
WHERE pending_patches > 0;

-- Average build time by status
SELECT build_status, AVG(duration) as avg_duration
FROM 'nix_build_logs.parquet'
GROUP BY build_status;
```

## Implementation Roadmap

1. ✅ Create this spec
2. ⚠️ Create universal-build-logger flake
3. ⚠️ Test on 1 project (Drift_Protocol)
4. ⚠️ Apply to all 500 projects
5. ⚠️ Rebuild all (creates log derivations)
6. ⚠️ Export to Parquet
7. ⚠️ Push to HuggingFace
8. ⚠️ Query and analyze

## Result

**Every build becomes a structured data point in a growing quality dataset.**

- 500 projects × multiple builds = thousands of data points
- Each with full semantic context
- Shareable, queryable, reproducible
- Gets better over time as we add more analysis phases
