fn main() {
    use libloading::{Library, Symbol};
    
    let libnix_path = "./target/debug/liblibnix.so";
    
    println!("Loading libnix from {}...", libnix_path);
    
    unsafe {
        let lib = Library::new(libnix_path).expect("Failed to load libnix");
        
        let load_fn: Symbol<extern "C" fn(*const *const i8, usize) -> i32> = 
            lib.get(b"libnix_load").expect("Failed to find libnix_load");
        
        let libs = vec![
            std::ffi::CString::new("ssl").unwrap(),
        ];
        let ptrs: Vec<*const i8> = libs.iter().map(|s| s.as_ptr()).collect();
        
        println!("Calling libnix_load...");
        let result = load_fn(ptrs.as_ptr(), ptrs.len());
        
        if result == 0 {
            println!("✅ Success!");
        } else {
            println!("❌ Failed with code {}", result);
        }
    }
}
