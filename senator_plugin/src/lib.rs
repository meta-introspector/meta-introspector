// senator_plugin/src/lib.rs
use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub struct SenatorPlugin {
    rank: u32,
    attestations: Vec<u8>,
}

#[wasm_bindgen]
impl SenatorPlugin {
    #[wasm_bindgen(constructor)]
    pub fn new(rank: u32) -> SenatorPlugin {
        console_log!("🏛️ Senator Plugin initialized for rank {}", rank);
        
        SenatorPlugin {
            rank,
            attestations: Vec::new(),
        }
    }
    
    #[wasm_bindgen]
    pub fn verify_senator(&self) -> bool {
        self.rank > 0 && self.rank <= 100
    }
    
    #[wasm_bindgen]
    pub fn add_attestation(&mut self, data: Vec<u8>) {
        console_log!("📝 Adding attestation ({} bytes)", data.len());
        self.attestations.extend(data);
    }
    
    #[wasm_bindgen]
    pub fn get_rank(&self) -> u32 {
        self.rank
    }
    
    #[wasm_bindgen]
    pub fn generate_proof(&self) -> String {
        console_log!("🔐 Generating execution proof");
        format!("proof_rank_{}_attestations_{}", self.rank, self.attestations.len())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelfLiftProof {
    pub wasm_hash: String,
    pub timestamp: f64,
    pub browser_env: String,
}

#[wasm_bindgen]
pub fn generate_self_lift_proof() -> JsValue {
    let proof = SelfLiftProof {
        wasm_hash: "generated_at_runtime".to_string(),
        timestamp: js_sys::Date::now(),
        browser_env: get_browser_env(),
    };
    
    serde_wasm_bindgen::to_value(&proof).unwrap()
}

fn get_browser_env() -> String {
    web_sys::window()
        .and_then(|w| w.navigator().user_agent().ok())
        .unwrap_or_else(|| "unknown".to_string())
}
