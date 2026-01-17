use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ContentAddress {
    pub hash: String,
    pub algorithm: String,
}

/// Generate content address from input and metadata
#[no_mangle]
pub extern "C" fn generate_content_address(input: *const u8, input_len: usize) -> *mut std::os::raw::c_char {
    let input_slice = unsafe { std::slice::from_raw_parts(input, input_len) };
    let hash = generate_content_address_rust(input_slice, &[]);
    std::ffi::CString::new(hash).unwrap().into_raw()
}

pub fn generate_content_address_rust(input: &[u8], metadata: &[String]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input);
    for m in metadata {
        hasher.update(m.as_bytes());
    }
    format!("ca:sha256:{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_content_address() {
        let addr = generate_content_address_rust(b"hello", &[]);
        assert!(addr.starts_with("ca:sha256:"));
    }
}
