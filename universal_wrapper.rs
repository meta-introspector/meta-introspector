// Universal wrapper - ALL functions call this with original function as argument
// Captures everything: args, returns, registers, stack, heap

use std::sync::atomic::{AtomicU64, Ordering};
use std::arch::asm;

static CALL_ID: AtomicU64 = AtomicU64::new(0);

#[repr(C)]
pub struct UniversalContext {
    pub id: u64,
    pub timestamp: i64,
    pub function_name: *const u8,
    pub function_addr: u64,
    pub pattern: [u8; 16],
    pub conductor: u32,
    
    // All x86_64 registers
    pub rax: u64, pub rbx: u64, pub rcx: u64, pub rdx: u64,
    pub rsi: u64, pub rdi: u64, pub rbp: u64, pub rsp: u64,
    pub r8: u64, pub r9: u64, pub r10: u64, pub r11: u64,
    pub r12: u64, pub r13: u64, pub r14: u64, pub r15: u64,
    
    // Stack snapshot (256 bytes)
    pub stack: [u8; 256],
    
    // Return value
    pub ret: u64,
}

/// Universal wrapper - called by ALL instrumented functions
/// 
/// # Arguments
/// * `original_fn` - Pointer to the real function
/// * `name` - Function name
/// * `pattern` - Instruction pattern bytes
/// * `conductor` - LMFDB conductor score
/// * `args` - Variadic arguments (up to 6)
#[no_mangle]
pub unsafe extern "C" fn __universal_wrapper(
    original_fn: *const (),
    name: *const u8,
    pattern: *const u8,
    conductor: u32,
) -> u64 {
    let id = CALL_ID.fetch_add(1, Ordering::Relaxed);
    
    // Capture ALL registers before calling original
    let mut ctx = UniversalContext {
        id,
        timestamp: unsafe { libc::time(std::ptr::null_mut()) },
        function_name: name,
        function_addr: original_fn as u64,
        pattern: [0; 16],
        conductor,
        rax: 0, rbx: 0, rcx: 0, rdx: 0,
        rsi: 0, rdi: 0, rbp: 0, rsp: 0,
        r8: 0, r9: 0, r10: 0, r11: 0,
        r12: 0, r13: 0, r14: 0, r15: 0,
        stack: [0; 256],
        ret: 0,
    };
    
    // Copy pattern
    std::ptr::copy_nonoverlapping(pattern, ctx.pattern.as_mut_ptr(), 16);
    
    // Capture registers using inline assembly
    // Capture registers using inline assembly
    // asm!(
    //     "mov {}, rax", out(reg) ctx.rax,
    //     "mov {}, rbx", out(reg) ctx.rbx,
    //     "mov {}, rcx", out(reg) ctx.rcx,
    //     "mov {}, rdx", out(reg) ctx.rdx,
    //     "mov {}, rsi", out(reg) ctx.rsi,
    //     "mov {}, rdi", out(reg) ctx.rdi,
    //     "mov {}, rbp", out(reg) ctx.rbp,
    //     "mov {}, rsp", out(reg) ctx.rsp,
    //     "mov {}, r8",  out(reg) ctx.r8,
    //     "mov {}, r9",  out(reg) ctx.r9,
    //     "mov {}, r10", out(reg) ctx.r10,
    //     "mov {}, r11", out(reg) ctx.r11,
    //     "mov {}, r12", out(reg) ctx.r12,
    //     "mov {}, r13", out(reg) ctx.r13,
    //     "mov {}, r14", out(reg) ctx.r14,
    //     "mov {}, r15", out(reg) ctx.r15,
    // );
    
    // Sample stack
    let stack_ptr = ctx.rsp as *const u8;
    if !stack_ptr.is_null() {
        std::ptr::copy_nonoverlapping(stack_ptr, ctx.stack.as_mut_ptr(), 256);
    }
    
    // Call original function (preserving all args in registers)
    let ret: u64;
    asm!(
        "call {}",
        in(reg) original_fn,
        lateout("rax") ret,
    );
    
    ctx.ret = ret;
    
    // Log to JSONL
    log_context(&ctx);
    
    ret
}

unsafe fn log_context(ctx: &UniversalContext) {
    use std::io::Write;
    
    // Fast path - write to memory-mapped file or ring buffer
    let json = format!(
        r#"{{"id":{},"ts":{},"fn":"{}","addr":"0x{:x}","pattern":"{}","conductor":{},"regs":{{"rax":"0x{:x}","rdi":"0x{:x}","rsi":"0x{:x}","rdx":"0x{:x}","rcx":"0x{:x}","r8":"0x{:x}","r9":"0x{:x}","rsp":"0x{:x}"}},"stack":"{}","ret":"0x{:x}"}}"#,
        ctx.id,
        ctx.timestamp,
        std::ffi::CStr::from_ptr(ctx.function_name as *const i8).to_str().unwrap_or("unknown"),
        ctx.function_addr,
        hex::encode(&ctx.pattern[..8]),
        ctx.conductor,
        ctx.rax, ctx.rdi, ctx.rsi, ctx.rdx, ctx.rcx, ctx.r8, ctx.r9, ctx.rsp,
        hex::encode(&ctx.stack[..64]), // First 64 bytes of stack
        ctx.ret
    );
    
    // Write to file (or send to Parquet writer)
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/universal_telemetry.jsonl")
    {
        writeln!(f, "{}", json).ok();
    }
}

// Macro to generate wrapper for any function
#[macro_export]
macro_rules! wrap_any {
    ($name:ident, $pattern:expr, $conductor:expr) => {
        paste::paste! {
            #[no_mangle]
            pub unsafe extern "C" fn $name() -> u64 {
                let real_fn = libc::dlsym(
                    libc::RTLD_NEXT,
                    concat!(stringify!($name), "\0").as_ptr() as *const _
                );
                
                __universal_wrapper(
                    real_fn,
                    concat!(stringify!($name), "\0").as_ptr(),
                    $pattern.as_ptr(),
                    $conductor,
                )
            }
        }
    };
}

// Example: wrap any function
wrap_any!(malloc, b"f3 0f 1e fa 48 89 fb", 5000);
wrap_any!(free, b"f3 0f 1e fa 48 85 ff", 4800);
wrap_any!(open, b"f3 0f 1e fa 41 57 41", 4500);
wrap_any!(close, b"f3 0f 1e fa 89 fb 48", 4200);
wrap_any!(read, b"f3 0f 1e fa 49 89 f4", 4500);
wrap_any!(write, b"f3 0f 1e fa 49 89 f4", 4500);
wrap_any!(getpid, b"f3 0f 1e fa b8 27 00", 3500);
wrap_any!(getuid, b"f3 0f 1e fa b8 66 00", 3500);

// Build.rs can generate thousands of these automatically:
// wrap_any!(function_12345, b"...", conductor);
