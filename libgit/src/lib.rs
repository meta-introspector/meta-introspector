use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn git_clone(url: *const c_char, path: *const c_char) -> i32 {
    let url = unsafe { CStr::from_ptr(url).to_str().unwrap() };
    let path = unsafe { CStr::from_ptr(path).to_str().unwrap() };
    
    match gix::prepare_clone(url, path) {
        Ok(_) => 0,
        Err(_) => -1
    }
}
