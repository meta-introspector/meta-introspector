use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Write;
use std::os::raw::{c_char, c_int, c_void};
use std::sync::Mutex;
use parquet::file::properties::WriterProperties;
use parquet::file::writer::SerializedFileWriter;
use parquet::schema::parser::parse_message_type;
use arrow::array::{UInt64Array, UInt8Array, ArrayRef};
use arrow::datatypes::{Schema, Field, DataType};
use arrow::record_batch::RecordBatch;
use parquet::arrow::ArrowWriter;
use std::sync::Arc;

// QEMU plugin API types
type QemuPluginId = u64;
type VCPUIndex = u32;

#[repr(C)]
struct QemuInfo {
    _unused: [u8; 0],
}

#[repr(C)]
struct QemuPluginTb {
    _unused: [u8; 0],
}

#[repr(C)]
struct QemuPluginInsn {
    _unused: [u8; 0],
}

type QemuPluginMeminfo = u32;

const QEMU_PLUGIN_VERSION: c_int = 1;
const QEMU_PLUGIN_CB_NO_REGS: c_int = 0;
const QEMU_PLUGIN_MEM_RW: c_int = 3;

// External QEMU functions
extern "C" {
    fn qemu_plugin_register_vcpu_tb_trans_cb(
        id: QemuPluginId,
        cb: extern "C" fn(QemuPluginId, *mut QemuPluginTb),
    );
    fn qemu_plugin_register_atexit_cb(
        id: QemuPluginId,
        cb: extern "C" fn(QemuPluginId, *mut c_void),
        userdata: *mut c_void,
    );
    fn qemu_plugin_tb_n_insns(tb: *const QemuPluginTb) -> usize;
    fn qemu_plugin_tb_get_insn(tb: *const QemuPluginTb, idx: usize) -> *mut QemuPluginInsn;
    fn qemu_plugin_insn_vaddr(insn: *const QemuPluginInsn) -> u64;
    fn qemu_plugin_register_vcpu_insn_exec_cb(
        insn: *mut QemuPluginInsn,
        cb: extern "C" fn(VCPUIndex, *mut c_void),
        flags: c_int,
        userdata: *mut c_void,
    );
    fn qemu_plugin_register_vcpu_mem_cb(
        insn: *mut QemuPluginInsn,
        cb: extern "C" fn(VCPUIndex, QemuPluginMeminfo, u64, *mut c_void),
        flags: c_int,
        rw: c_int,
        userdata: *mut c_void,
    );
    fn qemu_plugin_mem_size_shift(info: QemuPluginMeminfo) -> u32;
    fn qemu_plugin_mem_is_store(info: QemuPluginMeminfo) -> bool;
}

#[derive(Default)]
struct ByteProvenance {
    input_offsets: Vec<usize>,
    insn_addrs: Vec<u64>,
}

#[derive(Clone)]
struct ReachabilityRecord {
    input_offset: u64,
    insn_addr: u64,
    insn_bytes: Vec<u8>,
    insn_mnemonic: String,
    output_offset: u64,
}

struct PluginState {
    output_file: Option<File>,
    byte_provenance: HashMap<usize, ByteProvenance>,
    taint_map: HashMap<u64, usize>,
    current_insn: u64,
    current_insn_bytes: Vec<u8>,
    input_base: u64,
    input_size: u64,
    output_base: u64,
    output_size: u64,
    records: Vec<ReachabilityRecord>,
    parquet_path: String,
}

static STATE: Mutex<Option<PluginState>> = Mutex::new(None);

impl PluginState {
    fn new(output_path: &str) -> Self {
        Self {
            output_file: File::create(output_path).ok(),
            byte_provenance: HashMap::new(),
            taint_map: HashMap::new(),
            current_insn: 0,
            current_insn_bytes: Vec::new(),
            input_base: 0,
            input_size: 0,
            output_base: 0,
            output_size: 0,
            records: Vec::new(),
            parquet_path: output_path.replace(".txt", ".parquet"),
        }
    }

    fn add_input_taint(&mut self, mem_addr: u64, input_offset: usize) {
        self.taint_map.insert(mem_addr, input_offset);
    }

    fn track_output_write(&mut self, output_offset: usize, mem_addr: u64) {
        let prov = self.byte_provenance.entry(output_offset).or_default();
        
        if prov.insn_addrs.len() < 64 {
            prov.insn_addrs.push(self.current_insn);
        }
        
        if let Some(&input_offset) = self.taint_map.get(&mem_addr) {
            if prov.input_offsets.len() < 32 {
                prov.input_offsets.push(input_offset);
            }
        }
    }

    fn write_report(&mut self) {
        if let Some(ref mut file) = self.output_file {
            let _ = writeln!(file, "\n=== Byte Reachability Report ===");
            let _ = writeln!(file, "Tracked output bytes: {}\n", self.byte_provenance.len());
            
            for (offset, prov) in &self.byte_provenance {
                let _ = writeln!(file, "\nOutput byte {}:", offset);
                
                if !prov.input_offsets.is_empty() {
                    let _ = write!(file, "  Input bytes: ");
                    for off in &prov.input_offsets {
                        let _ = write!(file, "{} ", off);
                    }
                    let _ = writeln!(file);
                }
                
                if !prov.insn_addrs.is_empty() {
                    let _ = write!(file, "  Instructions: ");
                    for (i, addr) in prov.insn_addrs.iter().take(10).enumerate() {
                        let _ = write!(file, "0x{:x} ", addr);
                    }
                    if prov.insn_addrs.len() > 10 {
                        let _ = write!(file, "... ({} total)", prov.insn_addrs.len());
                    }
                    let _ = writeln!(file);
                }
            }
        }
    }
}

extern "C" fn vcpu_mem_access(
    _vcpu_index: VCPUIndex,
    info: QemuPluginMeminfo,
    vaddr: u64,
    _userdata: *mut c_void,
) {
    let mut state = STATE.lock().unwrap();
    if let Some(ref mut s) = *state {
        let size = 1u64 << unsafe { qemu_plugin_mem_size_shift(info) };
        let is_write = unsafe { qemu_plugin_mem_is_store(info) };
        
        // Track input reads
        if !is_write && vaddr >= s.input_base && vaddr < s.input_base + s.input_size {
            let input_offset = (vaddr - s.input_base) as usize;
            s.add_input_taint(vaddr, input_offset);
        }
        
        // Track output writes
        if is_write && vaddr >= s.output_base && vaddr < s.output_base + s.output_size {
            for i in 0..size {
                let output_offset = (vaddr + i - s.output_base) as usize;
                if output_offset < 10 * 1024 * 1024 {
                    s.track_output_write(output_offset, vaddr + i);
                }
            }
        }
    }
}

extern "C" fn vcpu_insn_exec(_vcpu_index: VCPUIndex, userdata: *mut c_void) {
    let insn_addr = userdata as u64;
    let mut state = STATE.lock().unwrap();
    if let Some(ref mut s) = *state {
        s.current_insn = insn_addr;
    }
}

extern "C" fn vcpu_tb_trans(id: QemuPluginId, tb: *mut QemuPluginTb) {
    unsafe {
        let n_insns = qemu_plugin_tb_n_insns(tb);
        
        for i in 0..n_insns {
            let insn = qemu_plugin_tb_get_insn(tb, i);
            let insn_addr = qemu_plugin_insn_vaddr(insn);
            
            qemu_plugin_register_vcpu_insn_exec_cb(
                insn,
                vcpu_insn_exec,
                QEMU_PLUGIN_CB_NO_REGS,
                insn_addr as *mut c_void,
            );
            
            qemu_plugin_register_vcpu_mem_cb(
                insn,
                vcpu_mem_access,
                QEMU_PLUGIN_CB_NO_REGS,
                QEMU_PLUGIN_MEM_RW,
                std::ptr::null_mut(),
            );
        }
    }
}

extern "C" fn plugin_exit(_id: QemuPluginId, _p: *mut c_void) {
    let mut state = STATE.lock().unwrap();
    if let Some(ref mut s) = *state {
        s.write_report();
    }
}

#[no_mangle]
pub extern "C" fn qemu_plugin_version() -> c_int {
    QEMU_PLUGIN_VERSION
}

#[no_mangle]
pub extern "C" fn qemu_plugin_install(
    id: QemuPluginId,
    _info: *const QemuInfo,
    argc: c_int,
    argv: *const *const c_char,
) -> c_int {
    let mut output = "reachability.txt".to_string();
    let mut input_base = 0u64;
    let mut input_size = 0u64;
    let mut output_base = 0u64;
    let mut output_size = 0u64;
    
    unsafe {
        for i in 0..argc {
            let arg = CStr::from_ptr(*argv.offset(i as isize));
            if let Ok(s) = arg.to_str() {
                if let Some(val) = s.strip_prefix("output=") {
                    output = val.to_string();
                } else if let Some(val) = s.strip_prefix("input_base=") {
                    input_base = u64::from_str_radix(val.trim_start_matches("0x"), 16).unwrap_or(0);
                } else if let Some(val) = s.strip_prefix("input_size=") {
                    input_size = val.parse().unwrap_or(0);
                } else if let Some(val) = s.strip_prefix("output_base=") {
                    output_base = u64::from_str_radix(val.trim_start_matches("0x"), 16).unwrap_or(0);
                } else if let Some(val) = s.strip_prefix("output_size=") {
                    output_size = val.parse().unwrap_or(0);
                }
            }
        }
    }
    
    let mut state = PluginState::new(&output);
    state.input_base = input_base;
    state.input_size = input_size;
    state.output_base = output_base;
    state.output_size = output_size;
    
    if let Some(ref mut file) = state.output_file {
        let _ = writeln!(file, "=== Byte Reachability Tracer (Rust) ===");
        let _ = writeln!(file, "Input: 0x{:x} - 0x{:x} ({} bytes)", 
                        input_base, input_base + input_size, input_size);
        let _ = writeln!(file, "Output: 0x{:x} - 0x{:x} ({} bytes)\n",
                        output_base, output_base + output_size, output_size);
    }
    
    *STATE.lock().unwrap() = Some(state);
    
    unsafe {
        qemu_plugin_register_vcpu_tb_trans_cb(id, vcpu_tb_trans);
        qemu_plugin_register_atexit_cb(id, plugin_exit, std::ptr::null_mut());
    }
    
    0
}
