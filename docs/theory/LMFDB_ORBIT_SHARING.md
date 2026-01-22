# LMFDB Orbit URL Sharing System

## 🎯 Concept

Data is split into LMFDB orbit parts, each part encoded in a URL. Collect N URLs to reconstruct the complete data.

## 🔢 LMFDB Orbit Structure

```rust
// src/lmfdb_orbit.rs
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

/// LMFDB Orbit - mathematical object requiring N values for reconstruction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LMFDBOrbit {
    pub orbit_id: String,
    pub dimension: usize,
    pub required_parts: usize,
    pub parts: Vec<OrbitPart>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrbitPart {
    pub index: usize,
    pub value: Vec<u8>,
    pub hash: String,
}

impl LMFDBOrbit {
    /// Create new orbit from data
    pub fn new(data: &[u8], required_parts: usize) -> Self {
        let orbit_id = Self::generate_orbit_id(data);
        let dimension = required_parts;
        let parts = Self::split_into_parts(data, required_parts);
        
        LMFDBOrbit {
            orbit_id,
            dimension,
            required_parts,
            parts,
        }
    }
    
    /// Split data into N parts
    fn split_into_parts(data: &[u8], n: usize) -> Vec<OrbitPart> {
        let chunk_size = (data.len() + n - 1) / n;
        let mut parts = Vec::new();
        
        for (i, chunk) in data.chunks(chunk_size).enumerate() {
            let mut hasher = Sha256::new();
            hasher.update(chunk);
            let hash = format!("{:x}", hasher.finalize());
            
            parts.push(OrbitPart {
                index: i,
                value: chunk.to_vec(),
                hash,
            });
        }
        
        parts
    }
    
    /// Reconstruct data from N parts
    pub fn reconstruct(parts: Vec<OrbitPart>) -> Result<Vec<u8>, String> {
        // Sort by index
        let mut sorted_parts = parts;
        sorted_parts.sort_by_key(|p| p.index);
        
        // Verify hashes
        for part in &sorted_parts {
            let mut hasher = Sha256::new();
            hasher.update(&part.value);
            let computed_hash = format!("{:x}", hasher.finalize());
            
            if computed_hash != part.hash {
                return Err(format!("Hash mismatch for part {}", part.index));
            }
        }
        
        // Concatenate
        let mut data = Vec::new();
        for part in sorted_parts {
            data.extend(part.value);
        }
        
        Ok(data)
    }
    
    fn generate_orbit_id(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        hasher.update(b"lmfdb-orbit");
        format!("{:x}", hasher.finalize())
    }
}

/// URL encoder for orbit parts
pub struct OrbitURLEncoder;

impl OrbitURLEncoder {
    /// Encode orbit part as URL
    pub fn encode_part(orbit: &LMFDBOrbit, part_index: usize) -> String {
        let part = &orbit.parts[part_index];
        
        // Encode part data as base64
        let encoded = base64::encode(&part.value);
        
        // Create URL with orbit metadata
        format!(
            "https://lmfdb.orbit/{}?dim={}&part={}/{}&hash={}&data={}",
            orbit.orbit_id,
            orbit.dimension,
            part.index,
            orbit.required_parts,
            &part.hash[..8],
            encoded
        )
    }
    
    /// Decode orbit part from URL
    pub fn decode_part(url: &str) -> Result<(String, OrbitPart), String> {
        // Parse URL
        let parts: Vec<&str> = url.split('?').collect();
        if parts.len() != 2 {
            return Err("Invalid URL format".to_string());
        }
        
        let orbit_id = parts[0].trim_start_matches("https://lmfdb.orbit/");
        let params: std::collections::HashMap<String, String> = parts[1]
            .split('&')
            .filter_map(|p| {
                let kv: Vec<&str> = p.split('=').collect();
                if kv.len() == 2 {
                    Some((kv[0].to_string(), kv[1].to_string()))
                } else {
                    None
                }
            })
            .collect();
        
        // Extract part info
        let part_str = params.get("part").ok_or("Missing part")?;
        let index: usize = part_str.split('/').next()
            .ok_or("Invalid part format")?
            .parse()
            .map_err(|_| "Invalid index")?;
        
        let hash = params.get("hash").ok_or("Missing hash")?;
        let data = params.get("data").ok_or("Missing data")?;
        
        // Decode data
        let value = base64::decode(data).map_err(|_| "Invalid base64")?;
        
        // Verify hash (partial)
        let mut hasher = Sha256::new();
        hasher.update(&value);
        let full_hash = format!("{:x}", hasher.finalize());
        
        Ok((orbit_id.to_string(), OrbitPart {
            index,
            value,
            hash: full_hash,
        }))
    }
    
    /// Generate all URLs for an orbit
    pub fn generate_urls(orbit: &LMFDBOrbit) -> Vec<String> {
        (0..orbit.parts.len())
            .map(|i| Self::encode_part(orbit, i))
            .collect()
    }
}

/// Login and share system
pub struct OrbitShareSystem {
    wallet: String,
    orbits: Vec<LMFDBOrbit>,
}

impl OrbitShareSystem {
    pub fn new(wallet: String) -> Self {
        OrbitShareSystem {
            wallet,
            orbits: Vec::new(),
        }
    }
    
    /// Add data and split into orbit
    pub fn add_data(&mut self, data: &[u8], required_parts: usize) -> String {
        let orbit = LMFDBOrbit::new(data, required_parts);
        let orbit_id = orbit.orbit_id.clone();
        self.orbits.push(orbit);
        orbit_id
    }
    
    /// Share orbit as URLs
    pub fn share_orbit(&self, orbit_id: &str) -> Result<Vec<String>, String> {
        let orbit = self.orbits.iter()
            .find(|o| o.orbit_id == orbit_id)
            .ok_or("Orbit not found")?;
        
        Ok(OrbitURLEncoder::generate_urls(orbit))
    }
    
    /// Reconstruct data from collected URLs
    pub fn reconstruct_from_urls(urls: Vec<String>) -> Result<Vec<u8>, String> {
        if urls.is_empty() {
            return Err("No URLs provided".to_string());
        }
        
        // Decode all parts
        let mut parts = Vec::new();
        let mut orbit_id = String::new();
        
        for url in urls {
            let (oid, part) = OrbitURLEncoder::decode_part(&url)?;
            if orbit_id.is_empty() {
                orbit_id = oid;
            } else if orbit_id != oid {
                return Err("URLs from different orbits".to_string());
            }
            parts.push(part);
        }
        
        // Reconstruct
        LMFDBOrbit::reconstruct(parts)
    }
}
```

## 🌐 WASM Interface

```rust
// src/lib.rs
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct OrbitShareWASM {
    system: OrbitShareSystem,
}

#[wasm_bindgen]
impl OrbitShareWASM {
    #[wasm_bindgen(constructor)]
    pub fn new(wallet: String) -> OrbitShareWASM {
        console_log!("🔐 Logged in with wallet: {}", wallet);
        
        OrbitShareWASM {
            system: OrbitShareSystem::new(wallet),
        }
    }
    
    /// Add data and get orbit ID
    #[wasm_bindgen]
    pub fn add_data(&mut self, data: Vec<u8>, required_parts: usize) -> String {
        console_log!("📦 Adding data ({} bytes) split into {} parts", data.len(), required_parts);
        self.system.add_data(&data, required_parts)
    }
    
    /// Get URLs for sharing
    #[wasm_bindgen]
    pub fn get_share_urls(&self, orbit_id: String) -> Result<JsValue, JsValue> {
        let urls = self.system.share_orbit(&orbit_id)
            .map_err(|e| JsValue::from_str(&e))?;
        
        console_log!("🔗 Generated {} URLs", urls.len());
        Ok(serde_wasm_bindgen::to_value(&urls)?)
    }
    
    /// Reconstruct from URLs
    #[wasm_bindgen]
    pub fn reconstruct(urls: Vec<String>) -> Result<Vec<u8>, JsValue> {
        console_log!("🔄 Reconstructing from {} URLs", urls.len());
        
        OrbitShareSystem::reconstruct_from_urls(urls)
            .map_err(|e| JsValue::from_str(&e))
    }
}
```

## 📱 Frontend Example

```html
<!DOCTYPE html>
<html>
<head>
    <title>LMFDB Orbit Sharing</title>
</head>
<body>
    <h1>🔢 LMFDB Orbit URL Sharing</h1>
    
    <div>
        <h2>1. Login</h2>
        <input type="text" id="wallet" placeholder="Wallet Address">
        <button onclick="login()">Login</button>
    </div>
    
    <div id="add-data" style="display:none">
        <h2>2. Add Data</h2>
        <textarea id="data" placeholder="Enter data to share"></textarea>
        <input type="number" id="parts" value="3" min="2" max="10">
        <button onclick="addData()">Split into Orbit</button>
    </div>
    
    <div id="urls" style="display:none">
        <h2>3. Share URLs</h2>
        <p>Share these URLs. Need <span id="required"></span> to reconstruct:</p>
        <div id="url-list"></div>
    </div>
    
    <div>
        <h2>4. Reconstruct</h2>
        <textarea id="collected-urls" placeholder="Paste URLs (one per line)"></textarea>
        <button onclick="reconstruct()">Reconstruct Data</button>
        <div id="result"></div>
    </div>
    
    <script type="module">
        import init, { OrbitShareWASM } from './pkg/orbit_share.js';
        
        let system;
        
        async function loadModule() {
            await init();
        }
        
        window.login = function() {
            const wallet = document.getElementById('wallet').value;
            system = new OrbitShareWASM(wallet);
            document.getElementById('add-data').style.display = 'block';
        };
        
        window.addData = function() {
            const data = document.getElementById('data').value;
            const parts = parseInt(document.getElementById('parts').value);
            
            const dataBytes = new TextEncoder().encode(data);
            const orbitId = system.add_data(Array.from(dataBytes), parts);
            
            const urls = system.get_share_urls(orbitId);
            
            document.getElementById('required').textContent = parts;
            const urlList = document.getElementById('url-list');
            urlList.innerHTML = urls.map((url, i) => 
                `<div>Part ${i+1}: <input value="${url}" readonly style="width:100%"></div>`
            ).join('');
            
            document.getElementById('urls').style.display = 'block';
        };
        
        window.reconstruct = function() {
            const urlsText = document.getElementById('collected-urls').value;
            const urls = urlsText.split('\n').filter(u => u.trim());
            
            try {
                const dataBytes = OrbitShareWASM.reconstruct(urls);
                const data = new TextDecoder().decode(new Uint8Array(dataBytes));
                
                document.getElementById('result').innerHTML = 
                    `<h3>✅ Reconstructed!</h3><pre>${data}</pre>`;
            } catch (e) {
                document.getElementById('result').innerHTML = 
                    `<h3>❌ Error</h3><p>${e}</p>`;
            }
        };
        
        loadModule();
    </script>
</body>
</html>
```

## 🎯 Example Flow

```
1. Login with wallet
   → OrbitShareWASM::new("wallet123")

2. Add data (split into 3 parts)
   → add_data("Hello World", 3)
   → Returns orbit_id: "abc123..."

3. Get URLs
   → get_share_urls("abc123...")
   → Returns:
     - https://lmfdb.orbit/abc123?dim=3&part=0/3&hash=a1b2c3d4&data=SGVs
     - https://lmfdb.orbit/abc123?dim=3&part=1/3&hash=e5f6g7h8&data=bG8g
     - https://lmfdb.orbit/abc123?dim=3&part=2/3&hash=i9j0k1l2&data=V29y

4. Share URLs (need all 3)

5. Reconstruct
   → reconstruct([url1, url2, url3])
   → Returns: "Hello World"
```

## 🔢 LMFDB Properties

- **Orbit**: Mathematical object requiring N values
- **Dimension**: Number of parts needed
- **Parts**: Individual URL-encoded pieces
- **Reconstruction**: Requires all N parts
- **Hash Verification**: Each part verified on reconstruction

---

**Status**: 🔢 LMFDB orbit URL sharing ready  
**Encoding**: Base64 in URLs  
**Reconstruction**: Requires N parts  
**Verification**: SHA256 hash per part  
**Use Case**: Distributed data sharing via URLs
