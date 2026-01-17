// Demo: Universal LLM Proxy with Mining Integration

mod universal_llm_proxy;
use universal_llm_proxy::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 UNIVERSAL LLM PROXY");
    println!("======================\n");
    println!("Multi-provider LLM proxy with telemetry and nix store integration\n");
    
    // Initialize proxy
    let mut proxy = UniversalLLMProxy::new(
        PathBuf::from("/tmp/llm-nix-store"),
        PathBuf::from("/tmp/llm-parquet"),
    );
    
    println!("📋 Supported Providers:");
    println!("  • Gemini (gemini-2.5-flash, gemini-pro)");
    println!("  • OpenAI (gpt-4, gpt-3.5-turbo)");
    println!("  • Anthropic (claude-3-opus, claude-3-sonnet)");
    println!("  • Ollama (local models)");
    println!("  • DeepSeek (deepseek-coder, deepseek-chat)");
    println!("  • Mistral (mistral-large, mistral-medium)");
    println!("  • Local (llama.cpp models)\n");
    
    // Example 1: Query Gemini for branch prediction analysis
    println!("🔍 Example 1: Branch Prediction Analysis with Gemini");
    let gemini_request = LLMRequest {
        provider: LLMProvider::Gemini {
            model: "gemini-2.5-flash".to_string(),
            api_key: std::env::var("GEMINI_API_KEY").unwrap_or_default(),
        },
        prompt: "Analyze these rustc branch predictions and identify the 5 most critical hot paths".to_string(),
        context: vec![],
        temperature: 0.7,
        max_tokens: 1024,
        metadata: [("demo".to_string(), "branch_mining".to_string())].iter().cloned().collect(),
    };
    
    if let Ok(response) = proxy.query(gemini_request).await {
        println!("  Provider: {}", response.provider);
        println!("  Model: {}", response.model);
        println!("  Latency: {}ms", response.latency_ms);
        println!("  Cost: ${:.4}", response.cost_usd);
        println!("  Hash: {}", response.content_hash);
        println!("  Response: {}...\n", &response.response[..100.min(response.response.len())]);
    }
    
    // Example 2: Query Ollama for Markov chain analysis
    println!("🔍 Example 2: Markov Chain Analysis with Ollama");
    let ollama_request = LLMRequest {
        provider: LLMProvider::Ollama {
            model: "codellama:7b".to_string(),
            endpoint: "http://localhost:11434".to_string(),
        },
        prompt: "What grammar patterns emerge from these character transitions?".to_string(),
        context: vec![],
        temperature: 0.5,
        max_tokens: 512,
        metadata: [("demo".to_string(), "markov_mining".to_string())].iter().cloned().collect(),
    };
    
    if let Ok(response) = proxy.query(ollama_request).await {
        println!("  Provider: {}", response.provider);
        println!("  Model: {}", response.model);
        println!("  Latency: {}ms", response.latency_ms);
        println!("  Cost: ${:.4} (free!)", response.cost_usd);
        println!("  Hash: {}", response.content_hash);
        println!("  Response: {}...\n", &response.response[..100.min(response.response.len())]);
    }
    
    // Example 3: Query OpenAI for lattice proof verification
    println!("🔍 Example 3: Lattice Proof Verification with OpenAI");
    let openai_request = LLMRequest {
        provider: LLMProvider::OpenAI {
            model: "gpt-4".to_string(),
            api_key: std::env::var("OPENAI_API_KEY").unwrap_or_default(),
        },
        prompt: "Verify this lattice proof shows 100% uniqueness for 11 syn types → 103 IPs".to_string(),
        context: vec![],
        temperature: 0.3,
        max_tokens: 2048,
        metadata: [("demo".to_string(), "lattice_proof".to_string())].iter().cloned().collect(),
    };
    
    if let Ok(response) = proxy.query(openai_request).await {
        println!("  Provider: {}", response.provider);
        println!("  Model: {}", response.model);
        println!("  Latency: {}ms", response.latency_ms);
        println!("  Cost: ${:.4}", response.cost_usd);
        println!("  Hash: {}", response.content_hash);
        println!("  Response: {}...\n", &response.response[..100.min(response.response.len())]);
    }
    
    // Show telemetry summary
    println!("\n📊 Telemetry Summary:");
    println!("  Total queries: {}", proxy.telemetry.len());
    
    let total_cost: f64 = proxy.telemetry.iter()
        .map(|r| r.response.cost_usd)
        .sum();
    println!("  Total cost: ${:.4}", total_cost);
    
    let avg_latency: u64 = proxy.telemetry.iter()
        .map(|r| r.response.latency_ms)
        .sum::<u64>() / proxy.telemetry.len().max(1) as u64;
    println!("  Average latency: {}ms", avg_latency);
    
    // Show nix store paths
    println!("\n📦 Nix Store Paths:");
    for record in &proxy.telemetry {
        if let Some(path) = &record.nix_store_path {
            println!("  {}", path);
        }
    }
    
    // Show parquet files
    println!("\n📄 Parquet Files:");
    for record in &proxy.telemetry {
        if let Some(path) = &record.parquet_path {
            println!("  {}", path);
        }
    }
    
    // Export to HuggingFace
    proxy.export_to_huggingface("introspector/rust/llm-telemetry").await?;
    
    println!("\n💡 Use Cases:");
    println!("  • Multi-provider LLM queries with automatic failover");
    println!("  • Cost tracking across all providers");
    println!("  • Content-addressable storage in nix store");
    println!("  • Parquet export for HuggingFace datasets");
    println!("  • Telemetry capture for all LLM interactions");
    println!("  • Pure and impure nix builds supported");
    
    println!("\n✅ All LLM operations captured and stored!");
    
    Ok(())
}
