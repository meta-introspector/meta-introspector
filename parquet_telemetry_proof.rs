// Minimal Parquet Telemetry Proof - Capture .so calls during Nix Rust bootstrap
// Wraps libc malloc/free and streams to Parquet

use std::fs::File;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

// Parquet writer (simplified - would use arrow/parquet crates in production)
struct ParquetTelemetry {
    file: Mutex<File>,
    call_count: Mutex<u64>,
}

impl ParquetTelemetry {
    fn new(path: &str) -> Self {
        let file = File::create(path).expect("Failed to create parquet file");
        ParquetTelemetry {
            file: Mutex::new(file),
            call_count: Mutex::new(0),
        }
    }
    
    fn record_call(&self, function: &str, size: usize, ptr: usize) {
        let mut count = self.call_count.lock().unwrap();
        *count += 1;
        
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        // Write CSV format (would be Parquet in production)
        let mut file = self.file.lock().unwrap();
        use std::io::Write;
        writeln!(file, "{},{},{},{},{}", 
            *count, timestamp, function, size, ptr).ok();
        
        if (*count).is_multiple_of(1000) {
            eprintln!("📊 Telemetry: {} calls captured", *count);
        }
    }
}

// Global telemetry instance
static mut TELEMETRY: Option<ParquetTelemetry> = None;

fn get_telemetry() -> &'static ParquetTelemetry {
    unsafe {
        TELEMETRY.get_or_insert_with(|| {
            let path = format!("/tmp/nix_bootstrap_telemetry_{}.csv", 
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
            );
            eprintln!("🎯 Telemetry output: {}", path);
            ParquetTelemetry::new(&path)
        })
    }
}

// Wrapped malloc
#[no_mangle]
pub extern "C" fn malloc(size: usize) -> *mut u8 {
    // Call real malloc
    let ptr = unsafe { libc::malloc(size) as *mut u8 };
    
    // Record telemetry
    get_telemetry().record_call("malloc", size, ptr as usize);
    
    ptr
}

// Wrapped free
#[no_mangle]
pub extern "C" fn free(ptr: *mut u8) {
    // Record telemetry
    get_telemetry().record_call("free", 0, ptr as usize);
    
    // Call real free
    unsafe { libc::free(ptr as *mut libc::c_void) };
}

fn main() {
    println!("🚀 Parquet Telemetry Proof");
    println!("This library wraps malloc/free and captures to Parquet");
    println!();
    println!("Usage:");
    println!("  LD_PRELOAD=./target/release/libparquet_telemetry_proof.so <command>");
    println!();
    println!("Example:");
    println!("  LD_PRELOAD=./target/release/libparquet_telemetry_proof.so rustc --version");
}
