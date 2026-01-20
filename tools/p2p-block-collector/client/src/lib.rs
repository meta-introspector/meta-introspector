use wasm_bindgen::prelude::*;
use p2p_shared::*;

#[wasm_bindgen]
pub struct Client {
    peer_id: String,
    rpc_url: String,
}

#[wasm_bindgen]
impl Client {
    #[wasm_bindgen(constructor)]
    pub fn new(rpc_url: String) -> Self {
        console_log("P2P Client initializing...");
        Self {
            peer_id: format!("peer_{}", js_sys::Math::random()),
            rpc_url,
        }
    }
    
    pub fn get_peer_id(&self) -> String {
        self.peer_id.clone()
    }
    
    pub async fn fetch_block(&self, slot: u64) -> Result<JsValue, JsValue> {
        console_log(&format!("Fetching block at slot {}", slot));
        
        let request = web_sys::Request::new_with_str_and_init(
            &self.rpc_url,
            web_sys::RequestInit::new()
                .method("POST")
                .body(Some(&JsValue::from_str(&format!(
                    r#"{{"jsonrpc":"2.0","id":1,"method":"getBlock","params":[{}]}}"#,
                    slot
                )))),
        )?;
        
        let window = web_sys::window().ok_or("no window")?;
        let resp = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;
        let resp: web_sys::Response = resp.dyn_into()?;
        let json = wasm_bindgen_futures::JsFuture::from(resp.json()?).await?;
        
        Ok(json)
    }
    
    pub async fn start(&self) -> Result<(), JsValue> {
        console_log("Client started");
        Ok(())
    }
}

fn console_log(s: &str) {
    web_sys::console::log_1(&JsValue::from_str(s));
}
