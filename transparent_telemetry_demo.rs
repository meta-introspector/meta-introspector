// 🎯 TRANSPARENT TELEMETRY DEMO
// All API calls automatically wrapped with telemetry!

// Include the auto-generated telemetry macros
// Include the auto-generated telemetry macros
// include!(concat!(env!("OUT_DIR"), "/telemetry_macros.rs"));

use std::ffi::CString;

fn main() {
    println!("🔧 TRANSPARENT TELEMETRY DEMO");
    println!("==============================");
    println!("All API calls automatically captured!");
    
    // These calls will be automatically wrapped with telemetry
    demo_memory_operations();
    demo_file_operations();
    demo_string_operations();
    
    println!("\n✅ All operations completed with automatic telemetry!");
}

fn demo_memory_operations() {
    println!("\n🧠 Memory Operations (auto-wrapped):");
    
    unsafe {
        // These malloc/free calls will be automatically captured
        let ptr = malloc!(1024);
        println!("   Allocated 1024 bytes: {:?}", ptr);
        
        if !ptr.is_null() {
            free!(ptr);
            println!("   Freed memory");
        }
    }
}

fn demo_file_operations() {
    println!("\n📁 File Operations (auto-wrapped):");
    
    let filename = CString::new("/tmp/telemetry_test.txt").unwrap();
    let mode = CString::new("w").unwrap();
    
    unsafe {
        // fopen/fclose calls automatically captured
        let file = fopen!(filename.as_ptr(), mode.as_ptr());
        
        if !file.is_null() {
            println!("   File opened successfully");
            fclose!(file);
            println!("   File closed");
        }
    }
}

fn demo_string_operations() {
    println!("\n📝 String Operations (auto-wrapped):");
    
    let message = CString::new("Hello from transparent telemetry!\n").unwrap();
    
    unsafe {
        // printf call automatically captured
        printf!(message.as_ptr());
    }
}

// Example of custom function that gets wrapped
fn custom_api_call(x: i32, y: i32) -> i32 {
    x + y
}

// This macro would be auto-generated for custom functions too
macro_rules! custom_api_call {
    ($x:expr, $y:expr) => {{
        capture_call!(custom_api_call($x, $y))
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_transparent_telemetry() {
        // Even test calls get telemetry!
        let result = custom_api_call!(5, 3);
        assert_eq!(result, 8);
    }
}
