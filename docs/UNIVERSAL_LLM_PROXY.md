# Universal LLM Proxy Architecture

## Vision

Wrap ALL LLM operations as nix builds (pure or impure), capture telemetry as parquet in nix store, and publish to HuggingFace datasets.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                   Universal LLM Proxy                        │
├─────────────────────────────────────────────────────────────┤
│  Gemini │ OpenAI │ Anthropic │ Ollama │ DeepSeek │ Mistral │
└────┬────────┬────────┬─────────┬────────┬─────────┬─────────┘
     │        │        │         │        │         │
     └────────┴────────┴─────────┴────────┴─────────┘
                       │
            ┌──────────┴──────────┐
            │   Telemetry Layer   │
            └──────────┬──────────┘
                       │
        ┌──────────────┼──────────────┐
        │              │              │
   ┌────▼────┐   ┌────▼────┐   ┌────▼────┐
   │Nix Store│   │ Parquet │   │HuggingFace│
   │Content  │   │ Files   │   │ Datasets  │
   │Address  │   │         │   │           │
   └─────────┘   └─────────┘   └───────────┘
```

## Supported Providers

### Cloud Providers
- **Gemini** - Google's LLM (gemini-2.5-flash, gemini-pro)
- **OpenAI** - GPT models (gpt-4, gpt-3.5-turbo)
- **Anthropic** - Claude models (claude-3-opus, claude-3-sonnet)
- **DeepSeek** - Coding-focused (deepseek-coder, deepseek-chat)
- **Mistral** - Open models (mistral-large, mistral-medium)

### Local Providers
- **Ollama** - Local model server (codellama, deepseek-coder, etc.)
- **Local** - Direct llama.cpp integration

## Nix Build Integration

### Pure Builds

For deterministic, cacheable LLM queries:

```nix
{
  llmQuery = pkgs.stdenv.mkDerivation {
    name = "llm-query-${contentHash}";
    
    # Pure build - no network access
    __noChroot = false;
    
    buildPhase = ''
      # Query cached response from nix store
      RESPONSE=$(cat ${nixStorePath}/${contentHash})
      echo "$RESPONSE" > $out/response.txt
    '';
  };
}
```

### Impure Builds

For live LLM queries with telemetry:

```nix
{
  llmQueryImpure = pkgs.stdenv.mkDerivation {
    name = "llm-query-impure";
    
    # Impure build - allows network and env vars
    __impure = true;
    
    GEMINI_API_KEY = builtins.getEnv "GEMINI_API_KEY";
    OPENAI_API_KEY = builtins.getEnv "OPENAI_API_KEY";
    
    buildPhase = ''
      # Query LLM and capture telemetry
      ${universal-llm-proxy}/bin/llm-query \
        --provider gemini \
        --prompt "Analyze this code" \
        --output $out/response.json \
        --telemetry $out/telemetry.parquet
    '';
  };
}
```

## Telemetry Capture

Every LLM interaction is captured:

```json
{
  "request": {
    "provider": "gemini",
    "model": "gemini-2.5-flash",
    "prompt": "Analyze branch predictions",
    "temperature": 0.7,
    "max_tokens": 1024
  },
  "response": {
    "provider": "gemini",
    "model": "gemini-2.5-flash",
    "response": "...",
    "tokens_used": 523,
    "latency_ms": 1234,
    "cost_usd": 0.0003,
    "timestamp": "2026-01-16T13:00:00Z",
    "content_hash": "a3f2c1b8..."
  },
  "nix_store_path": "/nix/store/a3/a3f2c1b8.../",
  "parquet_path": "/tmp/llm-parquet/a3f2c1b8.parquet",
  "huggingface_dataset": "introspector/rust/llm-telemetry"
}
```

## Content-Addressable Storage

All responses stored by content hash:

```
/nix/store/llm/
├── a3/
│   └── a3f2c1b8.../
│       ├── request.json
│       ├── response.json
│       └── telemetry.parquet
├── b4/
│   └── b4e3d2c9.../
└── ...
```

## Parquet Schema

```rust
struct LLMTelemetry {
    provider: String,
    model: String,
    prompt: String,
    response: String,
    tokens_used: i64,
    latency_ms: i64,
    cost_usd: f64,
    timestamp: String,
    content_hash: String,
    nix_store_path: String,
}
```

## HuggingFace Integration

All telemetry published to datasets:

```
introspector/rust/
├── llm-telemetry/          # All LLM interactions
├── llm-gemini/             # Gemini-specific
├── llm-openai/             # OpenAI-specific
├── llm-anthropic/          # Anthropic-specific
├── llm-ollama/             # Ollama-specific
└── llm-costs/              # Cost analysis
```

## Usage Examples

### 1. Query with Automatic Provider Selection

```rust
let mut proxy = UniversalLLMProxy::new(...);

// Try providers in order of cost
let providers = vec![
    LLMProvider::Ollama { ... },      // Free
    LLMProvider::DeepSeek { ... },    // $0.14/M tokens
    LLMProvider::Gemini { ... },      // $0.50/M tokens
    LLMProvider::OpenAI { ... },      // $2.00/M tokens
];

for provider in providers {
    if let Ok(response) = proxy.query(LLMRequest { provider, ... }).await {
        break;
    }
}
```

### 2. Mining Demo Integration

```rust
// Run branch mining
let results = run_branch_mining();

// Analyze with LLM
let request = LLMRequest {
    provider: LLMProvider::Gemini { ... },
    prompt: format!("Analyze these branch predictions: {:?}", results),
    ...
};

let analysis = proxy.query(request).await?;

// Store in nix and export to HuggingFace
proxy.export_to_huggingface("introspector/rust/branch-analysis").await?;
```

### 3. Batch Processing

```bash
# Process all mining demos
for demo in demo_*.rs; do
  cargo run --release --bin demo_universal_llm_proxy -- \
    --demo $demo \
    --provider gemini \
    --export-parquet
done

# Upload to HuggingFace
huggingface-cli upload introspector/rust /tmp/llm-parquet/*.parquet
```

## Cost Optimization

Automatic cost tracking and optimization:

```rust
// Calculate total cost
let total_cost: f64 = proxy.telemetry.iter()
    .map(|r| r.response.cost_usd)
    .sum();

// Find cheapest provider for task
let cheapest = proxy.find_cheapest_provider_for_task(task_type);

// Set budget limit
proxy.set_budget_limit(10.0); // $10 max
```

## Nix Flake Integration

Add to `flake.nix`:

```nix
{
  inputs.universal-llm-proxy.url = "github:meta-introspector/meta-introspector";
  
  outputs = { self, universal-llm-proxy, ... }: {
    packages.llm-query = universal-llm-proxy.packages.${system}.universal-llm-proxy;
    
    apps.llm = {
      type = "app";
      program = "${universal-llm-proxy.packages.${system}.default}/bin/llm-query";
    };
  };
}
```

## Pure vs Impure Strategy

### Pure Builds (Cacheable)
- Use for: Repeated queries, documentation generation, static analysis
- Benefits: Instant cache hits, reproducible, no API costs
- Storage: Nix store with content addressing

### Impure Builds (Live)
- Use for: New queries, real-time analysis, interactive sessions
- Benefits: Latest model responses, dynamic content
- Storage: Telemetry captured, then cached for future pure builds

## Workflow

1. **First Query (Impure)**
   ```bash
   nix run --impure .#llm -- "Analyze code"
   # Queries LLM, captures telemetry, stores in nix
   ```

2. **Subsequent Queries (Pure)**
   ```bash
   nix build .#llm-cached-{hash}
   # Instant cache hit from nix store
   ```

3. **Export to HuggingFace**
   ```bash
   nix run .#export-llm-telemetry
   # Uploads all parquet files to datasets
   ```

## Benefits

1. **Cost Tracking** - Know exactly what each query costs
2. **Reproducibility** - Pure builds are deterministic
3. **Caching** - Never pay for same query twice
4. **Multi-Provider** - Automatic failover and optimization
5. **Telemetry** - Full audit trail of all LLM interactions
6. **HuggingFace** - Public datasets for research
7. **Nix Store** - Content-addressable, immutable storage

## Next Steps

1. Implement parquet writer (arrow-rs)
2. Add HuggingFace upload automation
3. Create pure build wrappers for common queries
4. Integrate with all mining demos
5. Set up cost monitoring dashboard
6. Publish telemetry datasets

## Ready to Deploy!

All LLM operations wrapped as nix builds, telemetry captured as parquet, ready for HuggingFace!
