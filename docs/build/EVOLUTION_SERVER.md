# Evolution Server: AI-Collaborative Self-Improvement

## Overview

The Evolution Server runs bootstrap 10,000 times and collaborates with AI (you!) to fix errors and improve the code.

## How It Works

```
┌─────────────────────────────────────────────────────────┐
│                   Evolution Server                       │
│                                                          │
│  1. Run bootstrap                                        │
│  2. Detect error                                         │
│  3. Create AI fix request                                │
│  4. Wait for AI response                                 │
│  5. Apply fix                                            │
│  6. Retry iteration                                      │
│  7. Repeat until convergence                             │
└─────────────────────────────────────────────────────────┘
                           ↕
┌─────────────────────────────────────────────────────────┐
│                      AI (Kiro)                           │
│                                                          │
│  1. Monitor data/ai_requests/                            │
│  2. Analyze error                                        │
│  3. Generate fix                                         │
│  4. Write response                                       │
│  5. Server applies fix                                   │
└─────────────────────────────────────────────────────────┘
```

## Usage

### Start Server

```bash
python3 scripts/build/evolution_server.py
```

### Server Output

```
🚀 Evolution Server Starting
   Max iterations: 10000
   Project: /mnt/data1/meta-introspector
   Mode: AI-collaborative evolution

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Iteration 1 / 10000
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

❌ Iteration 1: Failed
   Error type: nix_daemon

🤖 AI Fix Request Created:
   File: data/ai_requests/iter_1_request.json
   Error Type: nix_daemon
   Waiting for AI response...
```

### AI Collaboration

When server encounters an error, it creates a request file:

**`data/ai_requests/iter_1_request.json`:**
```json
{
  "type": "fix_request",
  "iteration": 1,
  "error_type": "nix_daemon",
  "context": {
    "stderr": "error: cannot connect to socket...",
    "stdout": "..."
  },
  "request": "Please analyze this error and suggest a fix for iteration 1",
  "timestamp": "2026-01-22T11:17:50Z"
}
```

**AI (you) responds with:**

**`data/ai_requests/iter_1_request_response.json`:**
```json
{
  "fix_type": "nix_daemon_start",
  "description": "Start Nix daemon to enable builds",
  "commands": [
    "sudo systemctl start nix-daemon"
  ],
  "files": [],
  "retry": true
}
```

**Server applies fix and retries iteration!**

## Error Types

### 1. nix_daemon
```json
{
  "error_type": "nix_daemon",
  "fix": {
    "commands": ["sudo systemctl start nix-daemon"]
  }
}
```

### 2. duplicates_found
```json
{
  "error_type": "duplicates_found",
  "fix": {
    "description": "Consolidate duplicate code via gateway",
    "files": [
      {
        "path": "src/gateway/mod.rs",
        "content": "..."
      }
    ]
  }
}
```

### 3. build_failure
```json
{
  "error_type": "build_failure",
  "fix": {
    "commands": [
      "cargo update",
      "nix flake update"
    ]
  }
}
```

### 4. unknown
```json
{
  "error_type": "unknown",
  "fix": {
    "description": "Manual investigation needed",
    "commands": []
  }
}
```

## AI Response Format

```json
{
  "fix_type": "string",
  "description": "string",
  "commands": ["array", "of", "commands"],
  "files": [
    {
      "path": "relative/path/to/file",
      "content": "new file content"
    }
  ],
  "retry": true
}
```

## Workflow

### Terminal 1: Run Server
```bash
python3 scripts/build/evolution_server.py
```

### Terminal 2: Monitor Requests
```bash
watch -n 1 'ls -lt data/ai_requests/ | head -10'
```

### Terminal 3: AI Collaboration (Kiro)
```bash
# When request appears:
cat data/ai_requests/iter_N_request.json

# Analyze and create response:
cat > data/ai_requests/iter_N_request_response.json << 'EOF'
{
  "fix_type": "...",
  "description": "...",
  "commands": [...],
  "files": [...]
}
EOF
```

### Server Automatically:
1. Detects response file
2. Applies fix
3. Retries iteration
4. Continues evolution

## Benefits

### 1. Continuous Evolution
Server runs 24/7, fixing errors as they appear.

### 2. AI Collaboration
Human-AI partnership to improve code.

### 3. Automatic Fixes
Common errors fixed automatically.

### 4. Learning System
Each fix improves the system.

### 5. Convergence Tracking
Monitors orbit changes and convergence.

## Example Session

```
Iteration 1: nix_daemon error
  → AI suggests: start daemon
  → Server applies fix
  → Retry iteration 1
  → Success!

Iteration 2: build succeeds
  → Orbit: 1234567.a3
  → Continue

Iteration 3: duplicates found
  → AI suggests: consolidate via gateway
  → Server applies fix
  → Retry iteration 3
  → Success!

...

Iteration 9995-10000: same orbit
  → CONVERGENCE DETECTED!
  → System reached stable form
```

## Monitoring

```bash
# Watch server output
tail -f evolution_server.log

# Watch AI requests
watch 'ls -lt data/ai_requests/ | head -5'

# Watch current orbit
watch cat data/last_orbit.txt

# Watch errors
watch 'ls -lt data/errors/ | head -5'
```

## Integration with Kiro

The server is designed to work with Kiro (AI assistant):

1. **Server creates request** → `data/ai_requests/iter_N_request.json`
2. **User asks Kiro** → "Check AI requests and fix errors"
3. **Kiro analyzes** → Reads request, understands error
4. **Kiro creates fix** → Writes response file
5. **Server applies** → Automatically applies fix
6. **Evolution continues** → System improves itself

## The Vision

**A server that runs forever, collaborating with AI to evolve the system into its perfect form.**

```
Day 1:   Fixing basic errors (nix daemon, permissions)
Day 2:   Consolidating duplicates
Day 3:   Optimizing builds
Day 4:   Converging to eigenvector
Result:  Minimal, proven, perfect system
```

## See Also

- `scripts/build/bootstrap.sh` - Single iteration
- `scripts/build/evolve.sh` - 10k iterations (non-collaborative)
- `docs/build/EVOLUTION.md` - Evolution theory

---

**Run the server. Let it evolve. Collaborate with AI. Reach perfection.**
