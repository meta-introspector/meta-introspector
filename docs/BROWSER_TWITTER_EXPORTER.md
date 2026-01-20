# Browser ZOS Plugin - Twitter Moment Exporter

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│ Browser Extension (mod_zos)                                 │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ Content Script (runs on twitter.com)                 │  │
│  │                                                       │  │
│  │  - Detects viral moments                             │  │
│  │  - Captures timestamps                               │  │
│  │  - Exports to WASM plugin                            │  │
│  └────────────────┬─────────────────────────────────────┘  │
│                   │                                         │
│                   ▼                                         │
│  ┌──────────────────────────────────────────────────────┐  │
│  │ WASM Plugin (twitter_exporter.wasm)                  │  │
│  │                                                       │  │
│  │  - Parses Twitter data                               │  │
│  │  - Correlates with Solana blocks                     │  │
│  │  - Submits to server                                 │  │
│  └────────────────┬─────────────────────────────────────┘  │
└────────────────────┼─────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│ ZOS Server                                                  │
│                                                             │
│  Receives Twitter moment + timestamp                        │
│  → Creates DAO proposal                                     │
│  → "Archive blocks from [timestamp] to [timestamp+1hr]"     │
│  → Token holders vote                                       │
│  → If approved, archive those blocks                        │
└─────────────────────────────────────────────────────────────┘
```

## Browser Extension Structure

```
mod_zos/
├── manifest.json
├── content_scripts/
│   └── twitter_monitor.js
├── wasm/
│   └── twitter_exporter.wasm
└── background.js
```

### manifest.json
```json
{
  "manifest_version": 3,
  "name": "ZOS Twitter Moment Exporter",
  "version": "1.0.0",
  "permissions": ["storage", "activeTab"],
  "content_scripts": [{
    "matches": ["*://twitter.com/*", "*://x.com/*"],
    "js": ["content_scripts/twitter_monitor.js"]
  }],
  "background": {
    "service_worker": "background.js"
  }
}
```

### content_scripts/twitter_monitor.js
```javascript
// Detect viral moments
const VIRAL_THRESHOLD = 1000; // likes/retweets

function detectViralMoment() {
  const tweets = document.querySelectorAll('[data-testid="tweet"]');
  
  for (const tweet of tweets) {
    const likes = extractLikes(tweet);
    const retweets = extractRetweets(tweet);
    const content = extractContent(tweet);
    
    if (likes + retweets > VIRAL_THRESHOLD && content.includes('SOLFUNMEME')) {
      const moment = {
        timestamp: Date.now(),
        content: content,
        engagement: likes + retweets,
        url: window.location.href
      };
      
      // Send to WASM plugin
      exportMoment(moment);
    }
  }
}

async function exportMoment(moment) {
  // Load WASM plugin
  const wasm = await import('./wasm/twitter_exporter.js');
  await wasm.default();
  
  // Export moment
  const result = await wasm.export_twitter_moment(
    JSON.stringify(moment),
    'https://zos-server.solfunmeme.com/api/moments'
  );
  
  console.log('Exported moment:', result);
}

// Monitor every 30 seconds
setInterval(detectViralMoment, 30000);
```

## WASM Plugin

```
tools/wasm-plugins/twitter-exporter/
├── Cargo.toml
└── src/
    └── lib.rs
```

### lib.rs
```rust
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct TwitterMoment {
    timestamp: u64,
    content: String,
    engagement: u64,
    url: String,
}

#[derive(Serialize, Deserialize)]
struct BlockRange {
    start_slot: u64,
    end_slot: u64,
    reason: String,
}

#[wasm_bindgen]
pub async fn export_twitter_moment(
    moment_json: &str,
    server_url: &str
) -> Result<JsValue, JsValue> {
    let moment: TwitterMoment = serde_json::from_str(moment_json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    // Convert timestamp to Solana slot range
    let block_range = timestamp_to_slot_range(moment.timestamp);
    
    // Create DAO proposal
    let proposal = serde_json::json!({
        "type": "archive_twitter_moment",
        "moment": {
            "content": moment.content,
            "engagement": moment.engagement,
            "url": moment.url
        },
        "block_range": {
            "start_slot": block_range.start_slot,
            "end_slot": block_range.end_slot
        },
        "reason": format!("Viral moment: {} engagement", moment.engagement)
    });
    
    // Submit to server
    submit_proposal(&proposal, server_url).await
}

fn timestamp_to_slot_range(timestamp: u64) -> BlockRange {
    // Solana: ~2.5 slots/second
    // 1 hour = 3600 seconds = ~9000 slots
    
    let base_slot = 336482091; // Current slot
    let base_time = 1737318000; // Current timestamp
    
    let time_diff = (timestamp as i64) - (base_time as i64);
    let slot_diff = (time_diff as f64 * 2.5) as u64;
    
    let start_slot = base_slot + slot_diff;
    let end_slot = start_slot + 9000; // 1 hour window
    
    BlockRange {
        start_slot,
        end_slot,
        reason: "Twitter viral moment".to_string(),
    }
}

async fn submit_proposal(
    proposal: &serde_json::Value,
    server_url: &str
) -> Result<JsValue, JsValue> {
    // Submit via fetch API
    let window = web_sys::window().unwrap();
    let mut opts = web_sys::RequestInit::new();
    opts.method("POST");
    
    let headers = web_sys::Headers::new()?;
    headers.set("Content-Type", "application/json")?;
    opts.headers(&headers);
    
    let request = web_sys::Request::new_with_str_and_init(
        &format!("{}/proposals", server_url),
        &opts
    )?;
    
    let resp = wasm_bindgen_futures::JsFuture::from(
        window.fetch_with_request(&request)
    ).await?;
    
    Ok(resp)
}
```

## Server Integration

```rust
// In ZOS server
#[derive(Deserialize)]
struct TwitterMomentProposal {
    moment: TwitterMoment,
    block_range: BlockRange,
    reason: String,
}

async fn handle_twitter_moment(proposal: TwitterMomentProposal) {
    // Create DAO proposal
    let dao_proposal = create_dao_proposal(
        format!("Archive Twitter Moment: {}", proposal.moment.content),
        proposal.block_range,
        proposal.reason
    );
    
    // Submit to DAO contract
    submit_to_dao(dao_proposal).await;
    
    // Notify token holders
    notify_holders("New proposal: Archive viral Twitter moment").await;
}
```

## User Flow

1. **User browses Twitter** with mod_zos extension installed
2. **Extension detects** viral SOLFUNMEME tweet (>1000 engagement)
3. **WASM plugin exports** moment data to ZOS server
4. **Server creates DAO proposal**: "Archive blocks from slot X to Y"
5. **Token holders vote** on proposal
6. **If approved**, server archives those blocks
7. **Dataset includes** Twitter moment metadata

## Example Proposal

```json
{
  "id": 42,
  "title": "Archive Twitter Viral Moment",
  "description": "Tweet by @zos_sfm got 5000+ likes",
  "twitter_moment": {
    "content": "SOLFUNMEME just hit ATH! 🚀",
    "engagement": 5234,
    "url": "https://twitter.com/zos_sfm/status/123",
    "timestamp": 1737318000
  },
  "block_range": {
    "start_slot": 336482091,
    "end_slot": 336491091
  },
  "budget_impact": "~9000 blocks",
  "votes": {
    "yes": 750000,
    "no": 100000
  },
  "status": "approved"
}
```

## Value Proposition

### For Users
- Preserve meme history
- Correlate social with on-chain
- Community-driven curation

### For Holders
- Vote on what matters
- Curated dataset = higher value
- Social + blockchain data = unique

### For Researchers
- Social sentiment + price data
- Viral moment impact analysis
- Meme propagation studies

## Build with Nix

```nix
# Add to flake.nix
twitter-exporter-wasm = pkgs.rustPlatform.buildRustPackage {
  pname = "twitter-exporter-wasm";
  version = "0.1.0";
  src = ./tools/wasm-plugins/twitter-exporter;
  
  nativeBuildInputs = [ pkgs.wasm-pack ];
  
  buildPhase = ''
    wasm-pack build --target web --release
  '';
  
  installPhase = ''
    mkdir -p $out
    cp -r pkg/* $out/
  '';
};
```

## Next Steps

1. [ ] Create browser extension manifest
2. [ ] Build twitter-exporter WASM plugin
3. [ ] Add proposal endpoint to server
4. [ ] Test on live Twitter
5. [ ] Publish extension

---

**Key Innovation**: Browser extension + WASM + DAO = Community-curated social+blockchain dataset
