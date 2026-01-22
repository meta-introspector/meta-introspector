# Gemini Monitoring, Rate Limiting, and Sandboxing System

## Overview

Complete system for monitoring Gemini usage, enforcing rate limits, capturing perf traces, and running in a secure sandbox.

## Components

### 1. Rate Limit Tracker
```bash
gemini-rate-tracker
```

Tracks daily API usage:
- Records every request with timestamp
- Warns at 1000 requests (approaching limit)
- Blocks at 1500 requests (daily limit)
- Stores in `~/.gemini-rate-limits.json`

### 2. Perf Tracing
```bash
gemini-traced -p "your prompt" --output-format json
```

Wraps Gemini with perf recording:
- Records all syscalls
- Captures scheduler events
- Generates call graphs
- Saves traces to `~/.gemini-traces/`
- Extracts metrics (syscalls, duration)

### 3. Sandboxed Execution
```bash
gemini-sandbox -p "your prompt"
```

Runs Gemini in secure sandbox:
- Limited to 2GB memory
- Limited to 50% CPU
- Provides tools: jq, curl, git
- Restricted filesystem access
- Network allowed (for API calls)

### 4. Bug Fix Scheduler
```bash
# Add bug to queue
gemini-bug-scheduler add "Fix rate limit handling"

# Process next bug
gemini-bug-scheduler process

# List queue
gemini-bug-scheduler list
```

Schedules Gemini to fix its own bugs:
- Maintains bug queue
- Processes bugs one at a time
- Respects rate limits
- Saves fixes

### 5. Trace Analyzer
```bash
analyze-gemini-traces
```

Analyzes all collected traces:
- Total requests
- Total syscalls
- Average syscalls per request
- Aggregated statistics

## Usage

### Build System
```bash
nix build ./nix/gemini-monitored.nix#default
```

### Run Sandboxed Gemini
```bash
nix run ./nix/gemini-monitored.nix#default -- \
  -p "Analyze this error and suggest a fix" \
  --output-format json \
  --model gemini-2.5-flash
```

### Schedule Bug Fix
```bash
nix run ./nix/gemini-monitored.nix#schedule-bug -- \
  add "Improve rate limit handling"

nix run ./nix/gemini-monitored.nix#schedule-bug -- process
```

### Analyze Traces
```bash
nix run ./nix/gemini-monitored.nix#analyze
```

## Integration with Evolution Server

Update evolution server to use monitored Gemini:

```python
def call_gemini_triage(self, request, request_file):
    """Call Gemini via monitored sandbox"""
    prompt = f"Fix this error: {request['error_type']}"
    
    result = subprocess.run([
        "nix", "run",
        "./nix/gemini-monitored.nix#default",
        "--",
        "-p", prompt,
        "--output-format", "json",
        "--model", "gemini-2.5-flash"
    ], capture_output=True, text=True, timeout=60)
    
    # Trace automatically saved to ~/.gemini-traces/
    # Rate limit automatically checked
    # Sandbox automatically enforced
```

## Data Collection

All traces stored in dataset:
```
~/.gemini-traces/
├── gemini_20260122_120000.perf.data
├── gemini_20260122_120000.trace.txt
├── gemini_20260122_120000.meta.json
├── gemini_20260122_120100.perf.data
└── ...
```

Metadata includes:
- Timestamp
- Syscall count
- Duration
- Arguments
- Rate limit count

## Rate Limit Management

```json
{
  "requests": [
    {"timestamp": "2026-01-22T12:00:00Z"},
    {"timestamp": "2026-01-22T12:01:00Z"}
  ],
  "daily_count": 2,
  "last_reset": "2026-01-22T00:00:00Z"
}
```

Automatic handling:
- Warns at 1000 requests
- Blocks at 1500 requests
- Resets daily
- Queues requests when limited

## Sandbox Permissions

```json
{
  "allowed_commands": ["jq", "curl", "git"],
  "allowed_paths": ["/tmp/sandbox-XXXXX"],
  "network": true,
  "max_memory": "2G",
  "max_cpu": "50%"
}
```

Gemini can:
- Parse JSON (jq)
- Make HTTP requests (curl)
- Read git repos (git)
- Write to sandbox directory
- Access network for API calls

Cannot:
- Access filesystem outside sandbox
- Use more than 2GB memory
- Use more than 50% CPU
- Run arbitrary commands

## Bug Fix Workflow

1. **Detect bug** in Gemini usage
2. **Add to queue**: `gemini-bug-scheduler add "bug description"`
3. **Process automatically**: Scheduler calls Gemini to fix itself
4. **Apply fix**: Gemini provides commands/files
5. **Verify**: Test fix with perf trace
6. **Collect data**: Add trace to dataset

## Analysis Pipeline

```
Gemini Request
    ↓
Rate Limit Check
    ↓
Perf Recording Start
    ↓
Sandbox Execution
    ↓
Perf Recording Stop
    ↓
Trace Analysis
    ↓
Dataset Collection
    ↓
Aggregate Statistics
```

## Benefits

1. **Rate Limit Protection** - Never exceed API limits
2. **Complete Tracing** - Every syscall recorded
3. **Secure Execution** - Sandboxed with resource limits
4. **Self-Improvement** - Gemini fixes its own bugs
5. **Data Collection** - All traces for analysis
6. **Reproducibility** - Nix ensures consistency

## Current Status

✅ Rate limit tracking
✅ Perf tracing wrapper
✅ Sandbox with tools
✅ Bug fix scheduler
✅ Trace analyzer
✅ Nix integration
🚧 Integration with evolution server
🚧 First trace collection

## Next Steps

1. Build monitored Gemini system
2. Update evolution server to use it
3. Collect first traces
4. Analyze syscall patterns
5. Schedule bug fixes
6. Iterate and improve

---

**Gemini wrapped in Nix with perf, rate limits, and sandbox!**
