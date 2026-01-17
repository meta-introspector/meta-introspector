// Minimal LD_PRELOAD Parquet Telemetry for Nix Rust Build
// Wraps malloc/free and streams to CSV (Parquet-ready format)

use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

static TELEMETRY: Mutex<Option<Telemetry>> = Mutex::new(None);

struct Telemetry {
    file: std::fs::File,
    count: u64,
}

impl Telemetry {
    fn new() -> Self {
        let session = std::env::var("TELEMETRY_SESSION_ID")
            .unwrap_or_else(|_| format!("session_{}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()));
        
        let path = format!("/mnt/data1/meta-introspector/data/parquet_telemetry/{}.csv", session);
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .expect("Failed to create telemetry file");
        
        writeln!(file, "timestamp_ms,call_count,function,size,ptr").ok();
        eprintln!("📊 Telemetry: {}", path);
        
        Telemetry { file, count: 0 }
    }
    
    fn record(&mut self, function: &str, size: usize, ptr: usize) {
        self.count += 1;
        let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
        writeln!(self.file, "{},{},{},{},{}", ts, self.count, function, size, ptr).ok();
        
        if self.count % 10000 == 0 {
            eprintln!("📊 Captured {} calls", self.count);
        }
    }
}

fn get_telemetry() -> &'static Mutex<Option<Telemetry>> {
    &TELEMETRY
}

#[no_mangle]
pub extern "C" fn malloc(size: usize) -> *mut u8 {
    let ptr = unsafe { libc::malloc(size) as *mut u8 };
    
    let mut guard = get_telemetry().lock().unwrap();
    if guard.is_none() {
        *guard = Some(Telemetry::new());
    }
    if let Some(ref mut t) = *guard {
        t.record("malloc", size, ptr as usize);
    }
    
    ptr
}

#[no_mangle]
pub extern "C" fn free(ptr: *mut u8) {
    let mut guard = get_telemetry().lock().unwrap();
    if let Some(ref mut t) = *guard {
        t.record("free", 0, ptr as usize);
    }
    
    unsafe { libc::free(ptr as *mut libc::c_void) };
}

fn main() {
    println!("nix_telemetry_preload - add usage here");
}
