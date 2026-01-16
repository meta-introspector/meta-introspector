fn store_path() -> String {
    #[cfg(target_arch = "wasm32")]
    {
        // WASM uses localStorage as nix store
        "/localStorage/nix-store".to_string()
    }
    #[cfg(target_os = "windows")]
    {
        format!("{}\\AppData\\Local\\meta-store", std::env::var("USERPROFILE").unwrap_or_default())
    }
    #[cfg(target_os = "android")]
    {
        "/data/local/tmp/meta-store".to_string()
    }
    #[cfg(target_os = "ios")]
    {
        format!("{}/Library/meta-store", std::env::var("HOME").unwrap_or_default())
    }
    #[cfg(not(any(target_arch = "wasm32", target_os = "windows", target_os = "android", target_os = "ios")))]
    {
        format!("{}/.meta-store", std::env::var("HOME").unwrap_or_default())
    }
}

#[cfg(target_arch = "wasm32")]
mod wasm_store {
    use wasm_bindgen::prelude::*;
    
    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = localStorage)]
        fn setItem(key: &str, value: &str);
        
        #[wasm_bindgen(js_namespace = localStorage)]
        fn getItem(key: &str) -> Option<String>;
    }
    
    pub fn store_write(path: &str, data: &[u8]) {
        let key = format!("nix-store:{}", path);
        let b64 = base64::encode(data);
        setItem(&key, &b64);
    }
    
    pub fn store_read(path: &str) -> Option<Vec<u8>> {
        let key = format!("nix-store:{}", path);
        getItem(&key).and_then(|s| base64::decode(&s).ok())
    }
}
