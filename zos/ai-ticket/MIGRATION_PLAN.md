# AI-Ticket → ZOS Migration Plan

## Objective
Migrate ai-ticket from Python to Rust, integrate with ZOS, deploy on GitHub Actions.

## Phase 1: Nixify (Week 1)

### Tasks
- [ ] Create `zos/ai-ticket/flake.nix`
- [ ] Package existing Python ai-ticket in Nix
- [ ] Create development shell
- [ ] Add to ZOS system

### Deliverables
- Working Nix build of Python ai-ticket
- `nix run .#ai-ticket` works
- Development environment ready

## Phase 2: Rust Rewrite (Week 2-3)

### Components to Rewrite

1. **Ticket Management** (Priority 1)
   - `ticket_manager.py` → `src/ticket.rs`
   - Create/list/claim/submit tickets
   - GitHub API integration

2. **Proxy Server** (Priority 2)
   - `proxy_server.py` → `src/proxy.rs`
   - HTTP server (axum)
   - Ticket creation on requests

3. **AutoGPT Integration** (Priority 3)
   - `autogpt_plugin.py` → `src/autogpt.rs`
   - Request assistance action
   - Ticket polling

4. **Reward System** (Priority 4)
   - `rewards.py` → `src/rewards.rs`
   - Credit tracking
   - Payment processing

### Lifting Pipeline
For each Python file:
```bash
python3 scripts/build/lift_python.py zos/ai-ticket/<file>.py
# → Generates perf traces
# → Creates Rust rewrite prompt
# → Gemini generates Rust code
# → Compile and test
```

## Phase 3: ZOS Integration (Week 4)

### Tasks
- [ ] Add ZK proof generation for ticket operations
- [ ] Integrate with ZOS gateway system
- [ ] Add impure derivations for GitHub API calls
- [ ] Create ZOS server deployment

### ZOS Features
- Every ticket operation → ZK proof
- GitHub API calls → Proven impure derivations
- Ticket state → Stored in /nix/store
- Rewards → On-chain (Solana/Ethereum)

## Phase 4: GitHub Actions (Week 5)

### Tasks
- [ ] Create `.github/workflows/zos-ticket.yml`
- [ ] Trigger on issue creation
- [ ] Auto-create tickets
- [ ] Post results as comments

### Workflow
```
Issue Created → ZOS Action → Create Ticket → Assign → Execute → Post Result
```

## File Structure

```
zos/ai-ticket/
├── flake.nix                 # Nix build
├── Cargo.toml                # Rust project
├── src/
│   ├── main.rs              # Entry point
│   ├── ticket.rs            # Ticket management
│   ├── proxy.rs             # HTTP server
│   ├── autogpt.rs           # AutoGPT integration
│   ├── rewards.rs           # Reward system
│   └── gateway.rs           # ZOS gateway integration
├── python/                   # Original Python (reference)
│   └── ...
└── .github/
    └── workflows/
        └── zos-ticket.yml   # GitHub Action
```

## Gemini Tasks

### Task 1: Nixify
```
Create Nix flake for ai-ticket Python project.
Include: development shell, build derivation, GitHub API dependencies.
```

### Task 2: Lift ticket_manager.py
```
Lift ticket_manager.py to Rust using perf traces.
Requirements: GitHub API (octocrab), JSON (serde), async (tokio).
Prove: Same ticket operations, same API calls.
```

### Task 3: Lift proxy_server.py
```
Lift proxy_server.py to Rust HTTP server.
Requirements: axum, tower, serde_json.
Prove: Same endpoints, same behavior.
```

### Task 4: ZOS Integration
```
Add ZOS gateway integration to Rust ai-ticket.
Requirements: ZK proofs for all operations, impure derivations for GitHub API.
```

### Task 5: GitHub Action
```
Create GitHub Action workflow for ZOS ai-ticket.
Trigger: issue creation, Deploy: Nix, Execute: Rust binary.
```

## Success Criteria

- [ ] All Python code replaced with Rust
- [ ] Nix build works
- [ ] GitHub Action deploys and runs
- [ ] ZK proofs generated for all operations
- [ ] Tickets created/managed via GitHub issues
- [ ] Rewards tracked and paid
- [ ] Performance improved (Rust vs Python)
- [ ] Type safety proven

## Timeline

- Week 1: Nixify
- Week 2-3: Rust rewrite (via Gemini lifting)
- Week 4: ZOS integration
- Week 5: GitHub Actions deployment

## Branch

`feature/CRQ-002-zos-ai-ticket`
