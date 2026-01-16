use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn http_get(url: *const c_char, out: *mut *mut c_char) -> i32 {
    let url = unsafe { CStr::from_ptr(url).to_str().unwrap() };
    
    let rt = tokio::runtime::Runtime::new().unwrap();
    let result = rt.block_on(async {
        reqwest::get(url).await?.text().await
    });
    
    match result {
        Ok(body) => {
            let c_str = CString::new(body).unwrap();
            unsafe { *out = c_str.into_raw(); }
            0
        }
        Err(_) => -1
    }
}

#[no_mangle]
pub extern "C" fn http_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        unsafe { drop(CString::from_raw(ptr)); }
    }
}
