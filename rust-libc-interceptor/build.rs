fn main() {
    // Link against libdl for dlsym
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=pthread");
    
    // Ensure we can intercept libc functions
    println!("cargo:rustc-link-arg=-Wl,--no-as-needed");
}
