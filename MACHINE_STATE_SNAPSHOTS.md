# Machine State Snapshots in Nix Store

## Concept
Use Nix derivations to create **immutable, content-addressed snapshots** of machine state at hourly/block intervals.

## Architecture

### Snapshot Derivation
```nix
{
  description = "Machine state snapshot - hourly block";
  
  outputs = { self, nixpkgs }: {
    packages.x86_64-linux.default = pkgs.runCommand "machine-state-${timestamp}" {
      timestamp = "2026-01-18T10:00:00Z";
      block_number = 12345;
      machine_id = "i9-12900KF-meta-introspector";
    } ''
      mkdir -p $out
      
      # 1. File changes (git diff since last block)
      git diff --name-status HEAD~1 > $out/file_changes.txt
      git log -1 --stat > $out/git_log.txt
      
      # 2. System metrics
      cat > $out/system_metrics.json <<EOF
      {
        "timestamp": "$timestamp",
        "block": $block_number,
        "cpu_usage": $(top -bn1 | grep "Cpu(s)" | awk '{print 100 - $8}'),
        "memory_used": $(free | awk '/Mem:/ {print int($3/$2 * 100)}'),
        "disk_used": $(df /mnt/data1 | awk 'NR==2 {print $5}'),
        "load_average": "$(uptime | awk -F'load average:' '{print $2}')"
      }
      EOF
      
      # 3. Build activity (from logs)
      ls /nix/store/*-with-logs 2>/dev/null | wc -l > $out/build_count.txt
      
      # 4. Process snapshot
      ps aux > $out/processes.txt
      
      # 5. Network activity (if available)
      netstat -s > $out/network_stats.txt 2>/dev/null || true
      
      # 6. Convert to Parquet
      ${pkgs.python3}/bin/python3 << 'PYTHON'
      import json
      import pandas as pd
      
      # Load metrics
      with open('$out/system_metrics.json') as f:
        metrics = json.load(f)
      
      # Create DataFrame
      df = pd.DataFrame([metrics])
      
      # Save to Parquet
      df.to_parquet('$out/snapshot.parquet', compression='snappy')
      PYTHON
      
      # 7. Create NAR archive
      ${pkgs.nix}/bin/nix-store --export $out > $out/snapshot.nar
      
      # 8. Metadata
      cat > $out/metadata.json <<EOF
      {
        "snapshot_id": "$out",
        "timestamp": "$timestamp",
        "block": $block_number,
        "machine": "$machine_id",
        "git_commit": "$(git rev-parse HEAD)",
        "nix_version": "${pkgs.nix.version}",
        "store_path": "$out"
      }
      EOF
    '';
  };
}
```

## Hourly Snapshot Schedule

### Cron Job
```bash
# /etc/cron.hourly/nix-snapshot
#!/bin/bash
cd /mnt/data1/meta-introspector

# Calculate block number (hours since epoch)
BLOCK=$(($(date +%s) / 3600))
TIMESTAMP=$(date -Iseconds)

# Build snapshot
nix build .#machine-snapshot-$BLOCK

# Link to latest
ln -sf result snapshots/latest

# Export to Parquet
./build-logs-to-parquet/target/release/build-logs-to-parquet \
  /nix/store \
  snapshots/block-$BLOCK.parquet

# Push to HuggingFace
huggingface-cli upload \
  introspector/machine-snapshots \
  snapshots/block-$BLOCK.parquet \
  --repo-type dataset
```

## Data Structure

### Per-Block Parquet Schema
```
timestamp          datetime64[ns]
block_number       uint64
machine_id         string
git_commit         string
cpu_usage          float64
memory_used        float64
disk_used          float64
load_average       string
build_count        uint64
file_changes       uint64
processes_count    uint64
network_bytes_sent uint64
network_bytes_recv uint64
snapshot_path      string
```

### Time-Series Query
```sql
-- CPU usage over last 24 hours
SELECT timestamp, cpu_usage 
FROM 'snapshots/*.parquet'
WHERE timestamp > NOW() - INTERVAL '24 hours'
ORDER BY timestamp;

-- Builds per hour
SELECT block_number, build_count
FROM 'snapshots/*.parquet'
WHERE block_number > 12000
ORDER BY block_number;

-- Disk growth rate
SELECT 
  block_number,
  disk_used,
  disk_used - LAG(disk_used) OVER (ORDER BY block_number) as growth
FROM 'snapshots/*.parquet';
```

## Immutability Properties

### Content-Addressed
```
/nix/store/abc123-machine-state-2026-01-18T10:00:00Z/
├── snapshot.parquet      # Immutable data
├── snapshot.nar          # NAR archive
├── metadata.json         # Provenance
└── file_changes.txt      # Git diff
```

**Hash:** `abc123` = SHA256(all contents)
**Immutable:** Cannot modify without changing hash
**Reproducible:** Same inputs → same hash

### Git-Backed
```bash
# Each snapshot references git commit
git_commit: "e8f064fc"

# Can reconstruct state from:
git checkout e8f064fc
nix build .#machine-snapshot-12345
```

## Abuse of Nix (Intentional)

### Why This Works
1. **Content-addressed storage** - Deduplication
2. **Garbage collection** - Old snapshots auto-cleaned
3. **Reproducibility** - Can rebuild any snapshot
4. **Distribution** - NAR files shareable
5. **Caching** - Binary cache for snapshots

### What We're Storing (Not Traditional Nix)
- System metrics (CPU, memory, disk)
- Process snapshots
- Network stats
- Build activity
- File changes
- Parquet time-series data

**Nix becomes a distributed, immutable time-series database!**

## Integration with ML Pipeline

### Training Data
```python
# Load all snapshots
snapshots = pd.read_parquet('snapshots/*.parquet')

# Features for ML
features = snapshots[[
    'cpu_usage', 'memory_used', 'disk_used',
    'build_count', 'file_changes'
]]

# Predict future resource usage
model.fit(features, target='cpu_usage')
```

### Anomaly Detection
```sql
-- Find unusual CPU spikes
SELECT timestamp, cpu_usage
FROM 'snapshots/*.parquet'
WHERE cpu_usage > (
  SELECT AVG(cpu_usage) + 2 * STDDEV(cpu_usage)
  FROM 'snapshots/*.parquet'
);
```

## HuggingFace Dataset

### Structure
```
introspector/machine-snapshots/
├── 2026-01-18/
│   ├── block-12340.parquet
│   ├── block-12341.parquet
│   ├── block-12342.parquet
│   └── ...
├── 2026-01-19/
│   └── ...
└── metadata.json
```

### Query from HF
```python
from datasets import load_dataset

# Load time-series
ds = load_dataset('introspector/machine-snapshots')

# Query specific time range
df = ds['2026-01-18'].to_pandas()
df[df['timestamp'] > '2026-01-18T10:00:00']
```

## Benefits

### 1. Immutable Audit Trail
Every machine state change is recorded and cannot be altered.

### 2. Time-Travel Debugging
```bash
# What was the state at block 12340?
nix build .#machine-snapshot-12340
cat result/system_metrics.json
```

### 3. Distributed Replication
```bash
# Share snapshots via binary cache
nix copy --to s3://cache result
```

### 4. Automatic Deduplication
Identical snapshots share storage (content-addressed).

### 5. ML Training Corpus
Continuous stream of labeled data for resource prediction.

## Implementation

### 1. Create Snapshot Flake
```bash
cd /mnt/data1/meta-introspector
mkdir -p snapshots
cat > snapshots/flake.nix << 'EOF'
# Machine snapshot flake
EOF
```

### 2. Schedule Hourly Builds
```bash
# Add to crontab
0 * * * * cd /mnt/data1/meta-introspector && nix build .#snapshot
```

### 3. Export to Parquet
```bash
# After each snapshot
./build-logs-to-parquet/target/release/build-logs-to-parquet \
  result snapshots/block-$(date +%s).parquet
```

### 4. Push to HuggingFace
```bash
# Hourly upload
huggingface-cli upload introspector/machine-snapshots snapshots/
```

## The Vision

**Every hour, the machine:**
1. Creates immutable snapshot in /nix/store
2. Exports to Parquet
3. Pushes to HuggingFace
4. Becomes training data for ML models

**Result:**
- Complete audit trail of machine state
- Time-series database in Nix
- Distributed, immutable, reproducible
- ML training corpus grows automatically

**Nix store becomes a blockchain of machine state** - each block is a snapshot, content-addressed and immutable.

This is **abuse of Nix in the best possible way** - using its immutability guarantees for time-series data storage.
