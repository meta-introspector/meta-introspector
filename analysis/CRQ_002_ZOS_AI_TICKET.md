# CRQ-002: ZOS AI-Ticket Migration

**Status**: Planned  
**Priority**: High  
**Branch**: feature/CRQ-002-zos-ai-ticket  
**Dependencies**: CRQ-001 (Nixify Pipeline)

## Objective

Migrate ai-ticket system from Python to Rust, integrate with ZOS, deploy on GitHub Actions with ZK proofs.

## Background

AI-Ticket is a human-powered AI-Ops system for handling the "last mile" of AI-generated code. It creates tickets for human review and execution.

Current: Python-based, Docker deployment  
Target: Rust-based, Nix deployment, ZOS integration, GitHub Actions

## Scope

### In Scope
- Nixify existing Python ai-ticket
- Lift Python → Rust via perf traces
- Integrate with ZOS gateway system
- Add ZK proofs for all operations
- Deploy on GitHub Actions
- Reward system integration

### Out of Scope
- Changing core ticket workflow
- Modifying reward economics
- Altering GitHub API integration

## Technical Approach

### 1. Nixify (Phase 1)
```nix
# zos/ai-ticket/flake.nix
{
  description = "AI-Ticket in Nix";
  
  outputs = { nixpkgs, ... }: {
    packages.x86_64-linux.default = pkgs.python3Packages.buildPythonApplication {
      pname = "ai-ticket";
      src = /mnt/data1/2023/09/24/ai-ticket;
      # ... dependencies
    };
  };
}
```

### 2. Lift to Rust (Phase 2)
```bash
# For each Python file
python3 scripts/build/lift_python.py zos/ai-ticket/python/ticket_manager.py
# → script2test → test2perf → perf2prompt → Gemini → Rust
```

### 3. ZOS Integration (Phase 3)
```rust
// src/ticket.rs
use crate::gateway;

pub fn create_ticket(task: &str) -> Result<String, String> {
    // Create ticket with ZK proof
    let proof = gateway::gateway().git().create_issue(task)?;
    Ok(proof)
}
```

### 4. GitHub Actions (Phase 4)
```yaml
# .github/workflows/zos-ticket.yml
on:
  issues:
    types: [opened]

jobs:
  create-ticket:
    runs-on: ubuntu-latest
    steps:
      - uses: cachix/install-nix-action@v22
      - run: nix run .#zos-ai-ticket -- create "${{ github.event.issue.body }}"
```

## Gemini Task Schedule

### Task 1: Nixify ai-ticket
**File**: `data/gemini_tasks/task_001_nixify_ai_ticket.json`
```json
{
  "id": 1,
  "title": "Nixify ai-ticket Python project",
  "priority": "high",
  "prompt": "Create Nix flake for /mnt/data1/2023/09/24/ai-ticket...",
  "status": "pending"
}
```

### Task 2: Lift ticket_manager.py
**File**: `data/gemini_tasks/task_002_lift_ticket_manager.json`

### Task 3: Lift proxy_server.py
**File**: `data/gemini_tasks/task_003_lift_proxy_server.json`

### Task 4: Lift autogpt_plugin.py
**File**: `data/gemini_tasks/task_004_lift_autogpt.json`

### Task 5: Lift rewards.py
**File**: `data/gemini_tasks/task_005_lift_rewards.json`

### Task 6: ZOS Integration
**File**: `data/gemini_tasks/task_006_zos_integration.json`

### Task 7: GitHub Actions
**File**: `data/gemini_tasks/task_007_github_actions.json`

## Success Metrics

- [ ] Nix build succeeds
- [ ] All Python replaced with Rust
- [ ] GitHub Action works
- [ ] ZK proofs generated
- [ ] Performance improved
- [ ] Type safety proven

## Timeline

- Week 1: Nixify
- Week 2-3: Rust rewrites (7 components)
- Week 4: ZOS integration
- Week 5: GitHub Actions deployment

## Dependencies

- CRQ-001: Nixify pipeline (completed)
- Gemini API access
- GitHub Actions access
- Nix daemon running

## Risks

- Gemini rate limits (mitigated: rate tracker)
- Complex Python dependencies (mitigated: Nix)
- GitHub API changes (mitigated: versioned API)

## Deliverables

1. `zos/ai-ticket/flake.nix` - Nix build
2. `zos/ai-ticket/src/*.rs` - Rust implementation
3. `.github/workflows/zos-ticket.yml` - GitHub Action
4. `docs/zos/AI_TICKET.md` - Documentation
5. ZK proofs for all operations

---

**CRQ-002: Migrate ai-ticket to ZOS with Rust and GitHub Actions**
