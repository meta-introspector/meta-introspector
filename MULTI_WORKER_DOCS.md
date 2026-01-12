# Multi-Worker Repository Processing System

## Current Status
✅ **20 workers** processing 569 repositories in parallel
✅ **Structured output** - Individual JSON files per repository
✅ **Background processing** with logging
✅ **State persistence** and progress tracking

## Architecture

### Worker System
- **20 parallel workers** processing repositories by recency
- **Git analysis**: status, branch, remotes, commits (with `--ignore-submodules`)
- **Real-time progress** tracking and state persistence

### Output Structure
```
data/processed/repo_results/
├── {repo_name}.json     # Individual repository analysis
├── {repo_name}.json     # Git status, branch, remotes, commits
└── ...                  # 569 total files expected
```

### State Management
- **Queue state**: `data/raw/queue_status.json`
- **Worker logs**: `worker.log`
- **Progress tracking**: Real-time completion percentage

## Next Phase: Focused Processing Queue

### Target Criteria
1. **Untracked files** in repositories
2. **Out-of-date repositories** (behind remote)
3. **Fork repositories** needing sync
4. **Focus**: Changes from last 3 weeks by user `mdupont`

### Implementation Plan

#### Phase 1: Analysis Queue
```rust
struct FocusedJob {
    repo_path: String,
    job_type: JobType,        // Untracked, OutOfDate, Fork
    priority: u64,            // Based on user activity
    user_changes: Vec<String>, // Files changed by mdupont
}

enum JobType {
    UnTrackedFiles,
    OutOfDateRepo,
    ForkSync,
}
```

#### Phase 2: User Activity Filter
- Parse git logs for `mdupont` commits in last 3 weeks
- Identify repositories with user changes
- Prioritize by recent user activity

#### Phase 3: Specialized Workers
- **Untracked worker**: `git status --porcelain` → process untracked files
- **Sync worker**: `git fetch` → check if behind remote
- **Fork worker**: Check upstream status and sync needs

## Files to Create
1. `focused_queue_builder.rs` - Build focused job queue from results
2. `user_activity_analyzer.rs` - Find mdupont changes in last 3 weeks
3. `focused_worker.rs` - Process untracked/outdated/fork jobs
4. `sync_dashboard.rs` - Monitor focused processing

Ready to implement focused processing queue?
