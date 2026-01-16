# Repository Processing Work Queue System

## Architecture

### 1. Queue Manager
- **Input:** `recent_repos_3months.txt` (569 repos sorted by date)
- **Priority:** Most recent changes first
- **Status tracking:** pending, processing, completed, failed

### 2. Worker Process
- Runs fixed `complete_indexer.rs` on each repo
- Updates status in real-time
- Handles failures gracefully

### 3. Dashboard
- Shows queue progress
- Real-time status updates
- Repository processing results

## Implementation Plan

### Phase 1: Queue Structure
```rust
struct RepoJob {
    path: String,
    priority: u64,        // timestamp for ordering
    status: JobStatus,    // pending/processing/completed/failed
    started_at: Option<DateTime>,
    completed_at: Option<DateTime>,
    error: Option<String>,
}
```

### Phase 2: Worker
- Process jobs from queue in priority order
- Run indexer on each repository
- Update job status and results

### Phase 3: Dashboard
- Web interface showing queue status
- Progress bars and completion stats
- Real-time updates via WebSocket

## Files to Create
1. `repo_queue_manager.rs` - Queue management
2. `repo_worker.rs` - Job processor  
3. `queue_dashboard.rs` - Web dashboard
4. `queue_status.json` - Persistent state

Ready to implement?
