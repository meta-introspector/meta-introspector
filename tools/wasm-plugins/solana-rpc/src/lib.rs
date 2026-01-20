use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use web_sys::{Request, RequestInit, RequestMode, Response, Headers};

const CONTRACT: &str = "BwUTq7fS6sfUmHDwAiCQZ3asSiPEapW5zDrsbwtapump";
const RPC_URL: &str = "https://api.mainnet-beta.solana.com";

#[derive(Serialize, Deserialize)]
struct Block {
    slot: u64,
    hash: String,
    transactions: Vec<String>,
    timestamp: u64,
}

#[wasm_bindgen]
pub async fn fetch_block(slot: u64) -> Result<JsValue, JsValue> {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getBlock",
        "params": [slot, {"encoding": "json", "maxSupportedTransactionVersion": 0}]
    });
    
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    
    let headers = Headers::new()?;
    headers.set("Content-Type", "application/json")?;
    opts.headers(&headers);
    
    let request = Request::new_with_str_and_init(RPC_URL, &opts)?;
    
    let window = web_sys::window().unwrap();
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    let json = wasm_bindgen_futures::JsFuture::from(resp.json()?).await?;
    
    Ok(json)
}

#[wasm_bindgen]
pub async fn fetch_signatures(address: &str) -> Result<JsValue, JsValue> {
    let payload = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "getSignaturesForAddress",
        "params": [address, {"limit": 1000}]
    });
    
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    
    let headers = Headers::new()?;
    headers.set("Content-Type", "application/json")?;
    opts.headers(&headers);
    
    let request = Request::new_with_str_and_init(RPC_URL, &opts)?;
    
    let window = web_sys::window().unwrap();
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    let json = wasm_bindgen_futures::JsFuture::from(resp.json()?).await?;
    
    Ok(json)
}

#[wasm_bindgen]
pub async fn submit_to_server(block_json: &str, server_url: &str) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);
    
    let headers = Headers::new()?;
    headers.set("Content-Type", "application/json")?;
    opts.headers(&headers);
    
    let request = Request::new_with_str_and_init(server_url, &opts)?;
    
    let window = web_sys::window().unwrap();
    let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request)).await?;
    let resp: Response = resp_value.dyn_into()?;
    let json = wasm_bindgen_futures::JsFuture::from(resp.json()?).await?;
    
    Ok(json)
}

#[wasm_bindgen]
pub fn get_contract_address() -> String {
    CONTRACT.to_string()
}
