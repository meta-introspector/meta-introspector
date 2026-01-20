use wasm_bindgen::prelude::*;
use libp2p::identity;

#[wasm_bindgen]
pub struct P2PClient {
    peer_id: String,
}

#[wasm_bindgen]
impl P2PClient {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let keypair = identity::Keypair::generate_ed25519();
        let peer_id = keypair.public().to_peer_id().to_string();
        
        web_sys::console::log_1(&format!("🆔 Peer ID: {}", peer_id).into());
        
        Self { peer_id }
    }
    
    pub fn get_peer_id(&self) -> String {
        self.peer_id.clone()
    }
    
    pub async fn connect(&self, server_addr: &str) -> Result<JsValue, JsValue> {
        web_sys::console::log_1(&format!("🔌 Connecting to: {}", server_addr).into());
        
        let ws = web_sys::WebSocket::new(server_addr)?;
        ws.set_binary_type(web_sys::BinaryType::Arraybuffer);
        
        Ok(JsValue::from_str(&format!("Connected to {}", server_addr)))
    }
    
    pub async fn submit_block(&self, block_json: &str) -> Result<JsValue, JsValue> {
        web_sys::console::log_1(&format!("📦 Submitting block: {}", block_json).into());
        
        let ws = web_sys::WebSocket::new("ws://localhost:9000")?;
        ws.send_with_str(block_json)?;
        
        Ok(JsValue::from_str("Block submitted"))
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
}
