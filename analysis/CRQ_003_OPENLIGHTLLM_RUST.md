# CRQ-003: OpenLightLLM → Rust Migration

**Status**: Planning  
**Priority**: High  
**Branch**: feature/CRQ-003-openlightllm-rust  
**Dependencies**: CRQ-002 (AI-Ticket ZOS)

## Objective

Migrate openlightllm (LiteLLM fork) from Python to Rust:
1. Rebase changes from upstream
2. Recompose application architecture
3. Convert Python → Rust via mathematical lifting
4. Integrate with ZOS + AI-Ticket

## Current State

**Location**: `/home/mdupont/projects/agentartificial/devops/vendor/litellm`  
**Your fork**: https://github.com/jmikedupont2/openlightllm  
**Upstream**: https://github.com/BerriAI/litellm  
**Language**: Python (FastAPI)

## Why Rust?

1. **Type Safety** - Catch errors at compile time
2. **Performance** - 10-100x faster than Python
3. **Memory Safety** - No GC pauses
4. **Concurrency** - Tokio async runtime
5. **Integration** - Native with AI-Ticket (Rust)

## Migration Plan

### Phase 1: Rebase & Analyze (Week 1)

```bash
# Rebase your changes on latest upstream
cd /home/mdupont/projects/agentartificial/devops/vendor/litellm
git fetch upstream
git rebase upstream/main

# Analyze Python codebase
find litellm -name "*.py" | wc -l  # Count files
cloc litellm/                       # Lines of code
```

**Deliverables**:
- Rebased fork
- Code analysis report
- Component inventory

### Phase 2: Architecture Recomposition (Week 2)

**Core Components**:
1. **Router** - Route requests to providers
2. **Proxy Server** - FastAPI → Axum
3. **Provider Adapters** - OpenAI, Anthropic, Google, etc.
4. **Rate Limiting** - Token bucket algorithm
5. **Cost Tracking** - Per-key budgets
6. **Fallback Logic** - Retry with exponential backoff

**New Rust Architecture**:
```
openlightllm-rust/
├── Cargo.toml
├── src/
│   ├── main.rs           # Entry point
│   ├── server.rs         # Axum server (replaces FastAPI)
│   ├── router.rs         # Request routing
│   ├── providers/
│   │   ├── mod.rs
│   │   ├── openai.rs     # OpenAI adapter
│   │   ├── anthropic.rs  # Claude adapter
│   │   ├── google.rs     # Gemini adapter
│   │   └── bedrock.rs    # AWS Bedrock
│   ├── ratelimit.rs      # Token bucket
│   ├── budget.rs         # Cost tracking
│   └── fallback.rs       # Retry logic
└── config/
    └── config.yaml       # Provider config
```

### Phase 3: Python → Rust Lifting (Week 3-6)

**Lifting Pipeline** (per component):
```bash
# For each Python module
python3 scripts/build/lift_python.py \
  /home/mdupont/projects/agentartificial/devops/vendor/litellm/litellm/router.py

# Pipeline:
# 1. script2test - Generate test cases
# 2. test2perf - Record perf traces
# 3. perf2prompt - Create lifting prompt
# 4. Gemini - Generate Rust code + proof
# 5. Verify - Compile and test
```

**Priority Order**:
1. **router.py** → `src/router.rs` (core routing logic)
2. **proxy_server.py** → `src/server.rs` (FastAPI → Axum)
3. **litellm_core_utils/llm_request.py** → `src/providers/mod.rs`
4. **litellm_core_utils/token_counter.py** → `src/ratelimit.rs`
5. **litellm_core_utils/budget_manager.py** → `src/budget.rs`

### Phase 4: ZOS Integration (Week 7)

**Add ZOS Features**:
- Gateway abstraction for all HTTP calls
- ZK proofs for request/response
- Nix store for config
- P2P gossip for distributed routing

```rust
// src/zos.rs
use crate::gateway;

pub async fn proxy_request(req: Request) -> Result<Response> {
    // Generate ZK proof of request
    let proof = gateway::gateway().net().http_post(
        &req.url,
        &req.body
    )?;
    
    // Store in Nix
    let hash = store_in_nix(&req, &proof)?;
    
    Ok(Response { proof, hash })
}
```

### Phase 5: Testing & Deployment (Week 8)

**Testing**:
- Unit tests (per module)
- Integration tests (full proxy)
- Load tests (compare Python vs Rust)
- Compatibility tests (OpenAI API format)

**Deployment**:
```bash
# Build with Nix
nix build .#openlightllm-rust

# Run
./result/bin/openlightllm --config config.yaml --port 4000
```

## Stack

```toml
[dependencies]
# Web server (replaces FastAPI)
axum = "0.7"
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace"] }

# HTTP client (for provider APIs)
reqwest = { version = "0.11", features = ["json"] }

# Async runtime
tokio = { version = "1", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"

# Rate limiting
governor = "0.6"

# Metrics
prometheus = "0.13"

# Config
config = "0.14"

# Error handling
anyhow = "1.0"
thiserror = "1.0"
```

## Gemini Task Schedule

### Task 1: Rebase & Analyze
```json
{
  "id": 1,
  "title": "Rebase openlightllm and analyze codebase",
  "prompt": "Rebase /home/mdupont/projects/agentartificial/devops/vendor/litellm on upstream. Generate component inventory and migration plan."
}
```

### Task 2: Lift router.py
```json
{
  "id": 2,
  "title": "Lift router.py to Rust",
  "prompt": "Lift litellm/router.py to Rust. Core routing logic with provider selection and fallback."
}
```

### Task 3: Lift proxy_server.py
```json
{
  "id": 3,
  "title": "Lift proxy_server.py to Rust",
  "prompt": "Lift litellm/proxy/proxy_server.py to Rust. FastAPI → Axum. OpenAI-compatible endpoints."
}
```

### Task 4-10: Lift provider adapters
- OpenAI adapter
- Anthropic adapter
- Google adapter
- AWS Bedrock adapter
- Azure adapter
- HuggingFace adapter
- Local model adapter

### Task 11: ZOS Integration
```json
{
  "id": 11,
  "title": "Add ZOS integration",
  "prompt": "Add ZOS gateway, ZK proofs, and Nix store integration to openlightllm-rust."
}
```

## Success Criteria

- [ ] All Python code replaced with Rust
- [ ] OpenAI API compatibility maintained
- [ ] Performance improved (10x+ faster)
- [ ] All providers working
- [ ] Rate limiting functional
- [ ] Cost tracking accurate
- [ ] Fallback logic working
- [ ] ZOS integration complete
- [ ] Nix build successful

## Timeline

- Week 1: Rebase & analyze
- Week 2: Architecture design
- Week 3-6: Python → Rust lifting (core + providers)
- Week 7: ZOS integration
- Week 8: Testing & deployment

## Benefits

✅ **Type Safety** - Rust compiler catches errors  
✅ **Performance** - 10-100x faster than Python  
✅ **Memory Safety** - No GC pauses  
✅ **Integration** - Native with AI-Ticket  
✅ **ZOS Features** - Gateways, ZK proofs, Nix  
✅ **Maintainability** - Modern Rust tooling  

---

**CRQ-003: Migrate openlightllm to Rust with ZOS integration**
