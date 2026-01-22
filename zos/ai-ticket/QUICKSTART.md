# ZOS AI-Ticket - Quick Start

## What Is This?

Migrating ai-ticket from Python to Rust with:
- Mathematical lifting via perf traces
- ZOS gateway integration
- ZK proofs for all operations
- GitHub Actions deployment

## Current Status

✅ **Initialization Complete**
- Branch created: `feature/CRQ-002-zos-ai-ticket`
- Directory structure ready
- Task queue created (7 tasks)
- CRQ document written

⏳ **Next**: Execute Task 1 (Nixify)

## Execute Task 1 Now

```bash
# View the task
cat data/gemini_tasks/task_001_nixify_ai_ticket.json | jq .

# Execute via evolution server (recommended)
python3 scripts/build/evolution_server.py

# Or manually with Gemini
nix run .#gemini -- -p "$(cat data/gemini_tasks/task_001_nixify_ai_ticket.json | jq -r .prompt)"

# Expected output: zos/ai-ticket/flake.nix
```

## Task Queue

1. ⏳ **Nixify ai-ticket** - Create Nix flake for Python
2. ⏳ **Lift ticket_manager.py** - GitHub API → Rust
3. ⏳ **Lift proxy_server.py** - HTTP server → Rust
4. ⏳ **Lift autogpt_plugin.py** - AutoGPT → Rust
5. ⏳ **Lift rewards.py** - Rewards → Rust
6. ⏳ **ZOS Integration** - Add gateways + ZK proofs
7. ⏳ **GitHub Actions** - Deploy workflow

## File Structure

```
zos/ai-ticket/
├── flake.nix              ← Task 1 creates this
├── Cargo.toml             ✅ Ready
├── src/
│   ├── main.rs           ✅ Stub ready
│   ├── ticket.rs         ← Task 2 creates this
│   ├── proxy.rs          ← Task 3 creates this
│   ├── autogpt.rs        ← Task 4 creates this
│   └── rewards.rs        ← Task 5 creates this
├── python/                ← Copy original Python here
├── proofs/                ← Equivalence proofs
└── .github/workflows/     ← Task 7 creates this
```

## Commands

```bash
# Check queue
cat data/gemini_tasks/queue.json

# Monitor evolution server
curl http://localhost:8081/api/v1/status

# Check branch
git branch
# * feature/CRQ-002-zos-ai-ticket

# Build (when ready)
cd zos/ai-ticket
cargo build --release

# Run (when ready)
./target/release/zos-ai-ticket
```

## Documentation

- [STATUS.md](STATUS.md) - Current status
- [MIGRATION_PLAN.md](MIGRATION_PLAN.md) - Detailed plan
- [README.md](README.md) - Project overview
- [../../analysis/CRQ_002_ZOS_AI_TICKET.md](../../analysis/CRQ_002_ZOS_AI_TICKET.md) - Complete CRQ

## Timeline

- **Week 1**: Nixify (Task 1)
- **Week 2-3**: Rust rewrites (Tasks 2-5)
- **Week 4**: ZOS integration (Task 6)
- **Week 5**: GitHub Actions (Task 7)

## Success Criteria

- [ ] `nix build .#zos-ai-ticket` works
- [ ] All Python replaced with Rust
- [ ] GitHub Action deploys
- [ ] ZK proofs generated
- [ ] Performance improved
- [ ] Type safety proven

---

**Ready to start? Execute Task 1 now!**

```bash
python3 scripts/build/evolution_server.py
```
