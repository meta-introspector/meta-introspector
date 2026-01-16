# AI-First Observability: Killing Datadog with Intelligence

## Vision
Replace Datadog's UI/query model with AI that understands the complete build→runtime→profile pipeline for Rust systems.

## What We Have (Foundation)

### 1. Ingestion ✅
- **LD_PRELOAD telemetry** - Captures every libc call with safe syscall logging
- **Perf capture** - Real runtime profiling with `linux-perf-data` parser
- **Build telemetry** - 32 binaries, 92 libraries, 37k symbols from real Nix builds
- **Symbol dissolution** - ABI, bytes, source extraction via goblin

### 2. Storage ✅
- **Structured Parquet** - Typed, compressed, queryable
- **JSONL streams** - Real-time append-only logs
- **Symbol database** - LMFDB-ranked symbols with conductor scores
- **Build provenance** - Complete Nix derivation → crate → symbol chain

### 3. Correlation ⚠️ (In Progress)
- **Logistical graph** - Dependencies: Build, Link, Call, Nix
- **Perf ranking** - Runtime hotness merged with static analysis
- **Call chains** - Stack traces from perf + symbol resolution

## What We Need (AI Layer)

### 4. Unified Schema 🎯
```rust
struct UnifiedObservability {
    // Build-time
    build_id: String,
    derivation: String,
    crate_name: String,
    crate_version: String,
    
    // Symbol-level
    symbol: String,
    symbol_abi: String,
    lmfdb_conductor: u32,
    
    // Runtime
    perf_samples: u64,
    call_count: u64,
    cpu_time_ns: u64,
    
    // Correlation
    trace_id: String,
    span_id: String,
    parent_span: Option<String>,
    
    // Context
    timestamp: i64,
    hostname: String,
    service: String,
    environment: String,
}
```

### 5. AI Query Interface 🤖

**Instead of Datadog dashboards, ask:**

```
Q: "Why did latency spike at 19:42?"
A: Perf shows malloc calls increased 3x. Build telemetry shows 
   commit abc123 changed allocator. Symbol __libc_malloc went from 
   1000 samples/sec to 3000. Trace correlation shows request_handler 
   now allocates 2MB per request vs 500KB before.

Q: "Which symbols should I wrap first?"
A: Top 20 by perf samples × LMFDB conductor:
   1. __libc_open (462 samples, conductor 8500) - HIGH PRIORITY
   2. __libc_write (380 samples, conductor 7200) - HIGH PRIORITY
   ...

Q: "What changed between builds?"
A: Build 1768332029 → 1768351567:
   - 5 new symbols in libstd
   - malloc usage +40% in rustc_codegen
   - Critical path now includes LLVM optimization pass
```

### 6. Correlation Engine 🔗

**Cross-reference everything:**
- Perf sample → Symbol → Binary → Derivation → Commit
- Trace span → Function call → Perf hotspot → LMFDB rank
- Latency spike → CPU profile → Memory allocation → Source line

## Datadog Killer Features

### What AI Replaces
1. **Manual dashboards** → Natural language queries
2. **Query languages** → "Show me what's slow"
3. **Alert rules** → "Tell me when something looks wrong"
4. **Correlation** → "Why is this related to that?"

### What We Keep Better
1. **Build provenance** - Nix gives us perfect reproducibility
2. **Symbol-level insight** - LMFDB ranking + goblin parsing
3. **Zero overhead** - Compile-time wrapper generation
4. **Complete stack** - Kernel (perf) → libc (LD_PRELOAD) → Rust (OTLP)

## Implementation Roadmap

### Phase 1: Unified Data (Current)
- [x] Perf capture from Nix builds
- [x] Symbol extraction with goblin
- [x] LMFDB ranking system
- [ ] Parse perf.data with linux-perf-data
- [ ] Merge perf + symbols + build data

### Phase 2: Correlation
- [ ] Build logistical graph from real data
- [ ] Link perf samples to symbols
- [ ] Trace call chains through stack
- [ ] Compute critical path

### Phase 3: AI Interface
- [ ] Unified schema in Parquet
- [ ] Vector embeddings for symbols
- [ ] LLM query interface
- [ ] Auto-correlation engine

### Phase 4: Production
- [ ] OTLP ingestion from Rust services
- [ ] Real-time streaming to Parquet
- [ ] Distributed tracing integration
- [ ] Alert generation via AI

## Cost Comparison

**Datadog**: $15-31/host/month + $0.10/GB ingested + $1.27/million spans
**Our stack**: 
- Storage: S3 ~$0.023/GB/month
- Compute: Self-hosted (already have)
- AI: Claude/GPT API ~$0.01/query
- **Total**: ~95% cheaper at scale

## The Moat

**Why this beats Datadog:**
1. **Build-aware** - We know what changed, not just that it changed
2. **Symbol-level** - Not just "service slow", but "malloc in request_handler"
3. **Provenance** - Nix gives us perfect reproducibility
4. **AI-native** - Query interface is natural language, not dashboards
5. **Open** - Own your data, own your infra, own your insights

## Next Steps

1. **Fix rust_perf_decoder** - Parse perf.data with linux-perf-data
2. **Merge datasets** - Perf samples + symbols + build provenance
3. **Build correlation** - Link everything in logistical graph
4. **AI prototype** - Simple LLM that answers "why is X slow?"

The foundation is there. Now we make AI the interface. 🚀
