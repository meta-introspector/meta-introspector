# ZOS AI-Ticket

**Status**: 🚧 Migration in Progress  
**CRQ**: CRQ-002  
**Branch**: feature/CRQ-002-zos-ai-ticket

## Overview

Migrating ai-ticket from Python to Rust with ZOS integration and GitHub Actions deployment.

## Progress

### Phase 1: Nixify ⏳
- [ ] Task 1: Create flake.nix for Python ai-ticket
- [ ] Test: `nix run .#ai-ticket`

### Phase 2: Rust Rewrite ⏳
- [ ] Task 2: Lift ticket_manager.py → src/ticket.rs
- [ ] Task 3: Lift proxy_server.py → src/proxy.rs
- [ ] Task 4: Lift autogpt_plugin.py → src/autogpt.rs
- [ ] Task 5: Lift rewards.py → src/rewards.rs

### Phase 3: ZOS Integration ⏳
- [ ] Task 6: Add gateway integration
- [ ] Add ZK proof generation
- [ ] Impure derivations for GitHub API

### Phase 4: GitHub Actions ⏳
- [ ] Task 7: Create workflow
- [ ] Test on issue creation
- [ ] Deploy to production

## Quick Start

```bash
# Build (when ready)
cd zos/ai-ticket
cargo build --release

# Run
./target/release/zos-ai-ticket

# Or via Nix (when flake ready)
nix run .#zos-ai-ticket
```

## Architecture

```
GitHub Issue → ZOS Action → Create Ticket → Assign → Execute → Post Result
                                ↓
                            ZK Proof
```

## Files

- `Cargo.toml` - Rust dependencies
- `src/main.rs` - Entry point
- `src/ticket.rs` - Ticket management (from ticket_manager.py)
- `src/proxy.rs` - HTTP server (from proxy_server.py)
- `src/autogpt.rs` - AutoGPT integration (from autogpt_plugin.py)
- `src/rewards.rs` - Reward system (from rewards.py)
- `flake.nix` - Nix build
- `.github/workflows/zos-ticket.yml` - GitHub Action

## Gemini Tasks

See `data/gemini_tasks/queue.json` for task schedule.

## Documentation

- [CRQ-002](../../analysis/CRQ_002_ZOS_AI_TICKET.md) - Complete plan
- [Migration Plan](MIGRATION_PLAN.md) - Detailed migration steps

---

**Lift everything to pure math. Prove everything. Eliminate all duplicates.**
