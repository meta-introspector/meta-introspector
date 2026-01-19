# Resource Schedule & Capacity Planning

**Date:** 2026-01-18  
**System:** 12th Gen Intel i9-12900KF (24 cores)

## Current Resource Status

### CPU
- **Total Cores:** 24 (16 physical, 2 threads/core)
- **Current Load:** 2.31 (9.8% utilization)
- **Available:** ~22 cores idle
- **Status:** ✅ Plenty of capacity

### Memory
- **Total:** 31 GiB
- **Used:** 1.4 GiB
- **Free:** 29 GiB
- **Status:** ✅ Excellent availability

### Disk
- **Total:** 7.3 TB
- **Used:** 5.5 TB (80%)
- **Free:** 1.5 TB
- **Status:** ⚠️ Monitor - 80% full

### Active Jobs
- **Background build:** PID 2009417 (idle/waiting)
- **Nix processes:** None active
- **Status:** ⚠️ Build may be stuck

## Resource Allocation Plan

### Phase 1: Data Collection (This Week)

#### Job 1: Complete Background Build
**Status:** Check if stuck
```bash
tail -f build_batch.log
kill -0 2009417 && echo "Running" || echo "Dead"
```

**Resources:**
- CPU: 1-4 cores (sequential builds)
- Memory: 2-4 GB per build
- Disk: ~100 MB per build log
- Time: 1-2 hours for 20 projects

**Action:** Restart if stuck

#### Job 2: Build All 111 Successful Projects
**Resources:**
- CPU: 8 cores (parallel builds)
- Memory: 16 GB (2 GB × 8 concurrent)
- Disk: ~11 GB (100 MB × 111)
- Time: 4-6 hours

**Schedule:** After Job 1 completes

#### Job 3: Convert to Parquet
**Resources:**
- CPU: 4 cores (Rust parallel processing)
- Memory: 4 GB
- Disk: ~500 MB (compressed Parquet)
- Time: 10-15 minutes

**Schedule:** After Job 2 completes

### Phase 2: Analysis & ML (Next Week)

#### Job 4: Feature Engineering
**Resources:**
- CPU: 8 cores (parallel processing)
- Memory: 8 GB
- Disk: ~1 GB (feature matrices)
- Time: 1-2 hours

#### Job 5: Model Training
**Resources:**
- CPU: 16 cores (ML training)
- Memory: 16 GB
- Disk: ~2 GB (model checkpoints)
- Time: 2-4 hours

### Phase 3: Production (Week 3+)

#### Job 6: Continuous Building
**Resources:**
- CPU: 12 cores (always-on)
- Memory: 24 GB
- Disk: ~10 GB/day (logs)
- Time: Continuous

#### Job 7: Model Inference
**Resources:**
- CPU: 4 cores (decision generation)
- Memory: 4 GB
- Disk: ~100 MB (decisions)
- Time: Continuous (every 1 hour)

## Capacity Constraints

### Disk Space ⚠️
**Current:** 1.5 TB free (80% used)

**Projected usage:**
- Build logs: ~11 GB (111 projects)
- Parquet files: ~500 MB
- Model training: ~2 GB
- Continuous logs: ~10 GB/day

**Action needed:**
- Clean old logs after Parquet export
- Archive to HuggingFace
- Set up log rotation

**Cleanup script:**
```bash
# After Parquet export
find /nix/store -name "*-build-log" -mtime +7 -delete
find /nix/store -name "*-with-logs" -mtime +7 -delete
```

### CPU (No Constraint) ✅
**Available:** 22 cores idle
**Max usage:** 16 cores (training)
**Headroom:** 6 cores for system

### Memory (No Constraint) ✅
**Available:** 29 GB free
**Max usage:** 24 GB (continuous building)
**Headroom:** 5 GB for system

## Optimized Schedule

### Today (Sunday)
```
10:00 - Check background build status
10:15 - Restart if needed
11:00 - Job 1 complete (20 projects)
11:15 - Start Job 2 (111 projects, 8 parallel)
17:00 - Job 2 complete
17:15 - Job 3: Convert to Parquet
17:30 - Push to HuggingFace
18:00 - Day 1 complete
```

### Monday
```
09:00 - Job 4: Feature engineering
11:00 - Job 4 complete
11:15 - Job 5: Model training (5 models)
15:00 - Job 5 complete
15:15 - Validate models
16:00 - Week 1 complete
```

### Week 2
```
Mon-Fri: Refine models, build decision engine
Weekend: Deploy continuous system
```

### Week 3+
```
Continuous operation:
- 12 cores: Building (Job 6)
- 4 cores: Inference (Job 7)
- 8 cores: Available for other work
```

## Resource Monitoring

### Alerts
```bash
# Disk space
if [ $(df /mnt/data1 | awk 'NR==2 {print $5}' | sed 's/%//') -gt 90 ]; then
  echo "⚠️ Disk >90% full"
fi

# Memory
if [ $(free | awk '/Mem:/ {print int($3/$2 * 100)}') -gt 80 ]; then
  echo "⚠️ Memory >80% used"
fi

# CPU load
if [ $(uptime | awk -F'load average:' '{print $2}' | awk '{print int($1)}') -gt 20 ]; then
  echo "⚠️ Load >20"
fi
```

### Dashboard
```bash
watch -n 5 '
echo "=== RESOURCE DASHBOARD ==="
echo "CPU: $(top -bn1 | grep "Cpu(s)" | awk "{print 100 - \$8}") %"
echo "Mem: $(free | awk "/Mem:/ {print int(\$3/\$2 * 100)}") %"
echo "Disk: $(df /mnt/data1 | awk "NR==2 {print \$5}")"
echo "Load: $(uptime | awk -F"load average:" "{print \$2}")"
echo ""
echo "Active builds: $(ps aux | grep "nix build" | grep -v grep | wc -l)"
echo "Logs collected: $(find /nix/store -name "*-with-logs" | wc -l)"
'
```

## Optimization Opportunities

### 1. Parallel Builds
**Current:** Sequential (1 core)
**Optimized:** 8 parallel (8 cores)
**Speedup:** 8x faster

### 2. Incremental Parquet
**Current:** Convert all at end
**Optimized:** Stream to Parquet during build
**Benefit:** Real-time analysis

### 3. Distributed Building
**Current:** Single machine
**Future:** Multiple machines
**Benefit:** Linear scaling

### 4. GPU Acceleration
**Current:** CPU-only ML training
**Future:** GPU training
**Benefit:** 10-100x faster training

## Risk Mitigation

### Disk Full
**Risk:** Builds fail if disk fills
**Mitigation:** 
- Monitor at 85%
- Auto-cleanup old logs
- Alert at 90%

### Memory Exhaustion
**Risk:** OOM kills processes
**Mitigation:**
- Limit concurrent builds to 8
- Monitor memory per build
- Swap available (2 GB)

### Build Hangs
**Risk:** Stuck builds block queue
**Mitigation:**
- Timeout after 60 minutes
- Kill and retry
- Log for analysis

## Next Actions

### Immediate
1. ✅ Check background build status
2. ⚠️ Restart if stuck
3. ⚠️ Monitor completion

### Today
1. Complete 20-project build
2. Start 111-project build (8 parallel)
3. Convert to Parquet
4. Push to HuggingFace

### This Week
1. Feature engineering
2. Model training
3. Validation
4. Documentation

## Summary

**Resources:** ✅ Excellent capacity (22 cores, 29 GB RAM)  
**Constraint:** ⚠️ Disk space (80% full, need cleanup)  
**Status:** ⚠️ Background build may be stuck  
**Action:** Check and restart build process  
**Timeline:** 6-8 hours to complete Phase 1
