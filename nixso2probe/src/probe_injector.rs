use crate::parquet_streamer::{ProbeEvent, EventType, ParquetStreamer};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tokio::sync::mpsc;
use tracing::{info, debug, warn, error};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProbeConfig {
    pub probes: Vec<ProbeDefinition>,
    pub global_settings: GlobalSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProbeDefinition {
    pub id: String,
    pub probe_type: ProbeType,
    pub target: ProbeTarget,
    pub events: Vec<String>,
    pub filters: Option<ProbeFilters>,
    pub sampling_rate: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ProbeType {
    Uprobe,    // User-space function entry/exit
    Kprobe,    // Kernel function entry/exit
    Tracepoint, // Kernel tracepoints
    Usdt,      // User-space static tracepoints
    Perf,      // Perf events (CPU, cache, etc.)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProbeTarget {
    pub binary_path: Option<PathBuf>,
    pub function_name: String,
    pub address_offset: Option<u64>,
    pub process_filter: Option<ProcessFilter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProcessFilter {
    pub pid: Option<u32>,
    pub process_name: Option<String>,
    pub uid: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProbeFilters {
    pub min_duration_ns: Option<u64>,
    pub max_frequency_hz: Option<f64>,
    pub stack_trace_depth: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GlobalSettings {
    pub buffer_size: usize,
    pub batch_timeout_ms: u64,
    pub enable_stack_traces: bool,
    pub compression_level: i32,
}

impl Default for GlobalSettings {
    fn default() -> Self {
        Self {
            buffer_size: 10000,
            batch_timeout_ms: 1000,
            enable_stack_traces: true,
            compression_level: 6,
        }
    }
}

pub struct ProbeInjector {
    config: ProbeConfig,
    active_probes: HashMap<String, ActiveProbe>,
    event_sender: Option<mpsc::Sender<ProbeEvent>>,
}

struct ActiveProbe {
    id: String,
    probe_type: ProbeType,
    bpf_program: Option<libbpf_rs::Program>,
    perf_buffer: Option<libbpf_rs::PerfBuffer>,
}

impl ProbeInjector {
    pub fn new(config: ProbeConfig) -> Self {
        Self {
            config,
            active_probes: HashMap::new(),
            event_sender: None,
        }
    }
    
    pub async fn inject_into_target(&mut self, target: &str) -> Result<()> {
        info!("🎯 Injecting probes into target: {}", target);
        
        // Parse target (PID, process name, or "system")
        let target_filter = self.parse_target(target)?;
        
        for probe_def in &self.config.probes.clone() {
            if let Err(e) = self.inject_single_probe(probe_def, &target_filter).await {
                warn!("Failed to inject probe {}: {}", probe_def.id, e);
            } else {
                info!("✅ Injected probe: {}", probe_def.id);
            }
        }
        
        Ok(())
    }
    
    pub async fn inject_system_wide(&mut self) -> Result<()> {
        info!("🌐 Injecting system-wide probes");
        
        for probe_def in &self.config.probes.clone() {
            if let Err(e) = self.inject_single_probe(probe_def, &None).await {
                warn!("Failed to inject system probe {}: {}", probe_def.id, e);
            } else {
                info!("✅ Injected system probe: {}", probe_def.id);
            }
        }
        
        Ok(())
    }
    
    async fn inject_single_probe(
        &mut self, 
        probe_def: &ProbeDefinition,
        target_filter: &Option<ProcessFilter>
    ) -> Result<()> {
        debug!("Injecting probe: {} ({:?})", probe_def.id, probe_def.probe_type);
        
        match probe_def.probe_type {
            ProbeType::Uprobe => self.inject_uprobe(probe_def, target_filter).await,
            ProbeType::Kprobe => self.inject_kprobe(probe_def).await,
            ProbeType::Tracepoint => self.inject_tracepoint(probe_def).await,
            ProbeType::Usdt => self.inject_usdt(probe_def, target_filter).await,
            ProbeType::Perf => self.inject_perf_event(probe_def).await,
        }
    }
    
    async fn inject_uprobe(
        &mut self, 
        probe_def: &ProbeDefinition,
        _target_filter: &Option<ProcessFilter>
    ) -> Result<()> {
        // Generate BPF program for uprobe
        let bpf_code = self.generate_uprobe_bpf(probe_def)?;
        
        // Load and attach BPF program
        let mut object = libbpf_rs::ObjectBuilder::default()
            .debug(true)
            .open_memory("uprobe", bpf_code.as_bytes())?
            .load()?;
        
        let program = object
            .prog_mut(&format!("{}_entry", probe_def.id))
            .ok_or_else(|| anyhow::anyhow!("Program not found"))?;
        
        // Attach to target function
        let binary_path = probe_def.target.binary_path
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Binary path required for uprobe"))?;
        
        let _link = program.attach_uprobe(
            false, // not retprobe
            -1,    // any PID
            binary_path,
            probe_def.target.address_offset.unwrap_or(0),
        )?;
        
        // Set up perf buffer for events
        let perf_buffer = libbpf_rs::PerfBufferBuilder::new(object.map("events")?)
            .sample_cb(self.create_perf_callback(probe_def.id.clone()))
            .build()?;
        
        let active_probe = ActiveProbe {
            id: probe_def.id.clone(),
            probe_type: probe_def.probe_type.clone(),
            bpf_program: Some(program.clone()),
            perf_buffer: Some(perf_buffer),
        };
        
        self.active_probes.insert(probe_def.id.clone(), active_probe);
        
        Ok(())
    }
    
    async fn inject_kprobe(&mut self, probe_def: &ProbeDefinition) -> Result<()> {
        let bpf_code = self.generate_kprobe_bpf(probe_def)?;
        
        let mut object = libbpf_rs::ObjectBuilder::default()
            .debug(true)
            .open_memory("kprobe", bpf_code.as_bytes())?
            .load()?;
        
        let program = object
            .prog_mut(&format!("{}_entry", probe_def.id))
            .ok_or_else(|| anyhow::anyhow!("Program not found"))?;
        
        let _link = program.attach_kprobe(false, &probe_def.target.function_name)?;
        
        let perf_buffer = libbpf_rs::PerfBufferBuilder::new(object.map("events")?)
            .sample_cb(self.create_perf_callback(probe_def.id.clone()))
            .build()?;
        
        let active_probe = ActiveProbe {
            id: probe_def.id.clone(),
            probe_type: probe_def.probe_type.clone(),
            bpf_program: Some(program.clone()),
            perf_buffer: Some(perf_buffer),
        };
        
        self.active_probes.insert(probe_def.id.clone(), active_probe);
        
        Ok(())
    }
    
    async fn inject_tracepoint(&mut self, probe_def: &ProbeDefinition) -> Result<()> {
        // Parse tracepoint name (e.g., "syscalls:sys_enter_openat")
        let parts: Vec<&str> = probe_def.target.function_name.split(':').collect();
        if parts.len() != 2 {
            return Err(anyhow::anyhow!("Invalid tracepoint format: {}", probe_def.target.function_name));
        }
        
        let bpf_code = self.generate_tracepoint_bpf(probe_def)?;
        
        let mut object = libbpf_rs::ObjectBuilder::default()
            .debug(true)
            .open_memory("tracepoint", bpf_code.as_bytes())?
            .load()?;
        
        let program = object
            .prog_mut(&format!("{}_tp", probe_def.id))
            .ok_or_else(|| anyhow::anyhow!("Program not found"))?;
        
        let _link = program.attach_tracepoint(parts[0], parts[1])?;
        
        let perf_buffer = libbpf_rs::PerfBufferBuilder::new(object.map("events")?)
            .sample_cb(self.create_perf_callback(probe_def.id.clone()))
            .build()?;
        
        let active_probe = ActiveProbe {
            id: probe_def.id.clone(),
            probe_type: probe_def.probe_type.clone(),
            bpf_program: Some(program.clone()),
            perf_buffer: Some(perf_buffer),
        };
        
        self.active_probes.insert(probe_def.id.clone(), active_probe);
        
        Ok(())
    }
    
    async fn inject_usdt(&mut self, _probe_def: &ProbeDefinition, _target_filter: &Option<ProcessFilter>) -> Result<()> {
        // TODO: Implement USDT probe injection
        warn!("USDT probes not yet implemented");
        Ok(())
    }
    
    async fn inject_perf_event(&mut self, _probe_def: &ProbeDefinition) -> Result<()> {
        // TODO: Implement perf event probes
        warn!("Perf event probes not yet implemented");
        Ok(())
    }
    
    pub async fn start_collection(&mut self, mut streamer: ParquetStreamer) -> Result<()> {
        info!("🚀 Starting probe data collection");
        
        let sender = streamer.get_sender();
        self.event_sender = Some(sender);
        
        // Start Parquet streaming in background
        tokio::spawn(async move {
            if let Err(e) = streamer.start_streaming().await {
                error!("Parquet streaming failed: {}", e);
            }
        });
        
        // Poll all perf buffers
        loop {
            for (probe_id, active_probe) in &mut self.active_probes {
                if let Some(perf_buffer) = &mut active_probe.perf_buffer {
                    if let Err(e) = perf_buffer.poll(std::time::Duration::from_millis(100)) {
                        warn!("Perf buffer poll failed for {}: {}", probe_id, e);
                    }
                }
            }
            
            tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
        }
    }
    
    fn parse_target(&self, target: &str) -> Result<Option<ProcessFilter>> {
        if target == "system" {
            return Ok(None);
        }
        
        // Try to parse as PID
        if let Ok(pid) = target.parse::<u32>() {
            return Ok(Some(ProcessFilter {
                pid: Some(pid),
                process_name: None,
                uid: None,
            }));
        }
        
        // Treat as process name
        Ok(Some(ProcessFilter {
            pid: None,
            process_name: Some(target.to_string()),
            uid: None,
        }))
    }
    
    fn create_perf_callback(&self, probe_id: String) -> impl Fn(&[u8]) + Send + Sync + 'static {
        let sender = self.event_sender.clone();
        
        move |data: &[u8]| {
            // Parse BPF event data
            if let Ok(event) = Self::parse_bpf_event(data, &probe_id) {
                if let Some(ref sender) = sender {
                    let _ = sender.try_send(event);
                }
            }
        }
    }
    
    fn parse_bpf_event(data: &[u8], probe_id: &str) -> Result<ProbeEvent> {
        // This is a simplified parser - real implementation would use proper BPF event structures
        if data.len() < 32 {
            return Err(anyhow::anyhow!("Event data too short"));
        }
        
        let timestamp_ns = u64::from_ne_bytes(data[0..8].try_into()?);
        let process_id = u32::from_ne_bytes(data[8..12].try_into()?);
        let thread_id = u64::from_ne_bytes(data[12..20].try_into()?);
        let cpu_id = u16::from_ne_bytes(data[20..22].try_into()?);
        
        Ok(ProbeEvent {
            timestamp_ns,
            probe_id: probe_id.to_string(),
            process_id,
            thread_id,
            function_name: "unknown".to_string(), // TODO: Extract from BPF data
            event_type: EventType::FunctionEntry,  // TODO: Determine from probe type
            data_payload: data[32..].to_vec(),
            stack_trace: None,
            cpu_id,
            duration_ns: None,
        })
    }
    
    fn generate_uprobe_bpf(&self, probe_def: &ProbeDefinition) -> Result<String> {
        Ok(format!(r#"
#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

struct event {{
    __u64 timestamp;
    __u32 pid;
    __u64 tid;
    __u16 cpu;
    char comm[16];
    __u64 args[6];
}};

struct {{
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u32));
}} events SEC(".maps");

SEC("uprobe/{}")
int {}_entry(struct pt_regs *ctx) {{
    struct event e = {{}};
    
    e.timestamp = bpf_ktime_get_ns();
    e.pid = bpf_get_current_pid_tgid() >> 32;
    e.tid = bpf_get_current_pid_tgid();
    e.cpu = bpf_get_smp_processor_id();
    bpf_get_current_comm(&e.comm, sizeof(e.comm));
    
    // Capture function arguments
    e.args[0] = PT_REGS_PARM1(ctx);
    e.args[1] = PT_REGS_PARM2(ctx);
    e.args[2] = PT_REGS_PARM3(ctx);
    e.args[3] = PT_REGS_PARM4(ctx);
    e.args[4] = PT_REGS_PARM5(ctx);
    e.args[5] = PT_REGS_PARM6(ctx);
    
    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &e, sizeof(e));
    return 0;
}}

char LICENSE[] SEC("license") = "GPL";
"#, probe_def.target.function_name, probe_def.id))
    }
    
    fn generate_kprobe_bpf(&self, probe_def: &ProbeDefinition) -> Result<String> {
        Ok(format!(r#"
#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

struct event {{
    __u64 timestamp;
    __u32 pid;
    __u64 tid;
    __u16 cpu;
    char comm[16];
    __u64 args[6];
}};

struct {{
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u32));
}} events SEC(".maps");

SEC("kprobe/{}")
int {}_entry(struct pt_regs *ctx) {{
    struct event e = {{}};
    
    e.timestamp = bpf_ktime_get_ns();
    e.pid = bpf_get_current_pid_tgid() >> 32;
    e.tid = bpf_get_current_pid_tgid();
    e.cpu = bpf_get_smp_processor_id();
    bpf_get_current_comm(&e.comm, sizeof(e.comm));
    
    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &e, sizeof(e));
    return 0;
}}

char LICENSE[] SEC("license") = "GPL";
"#, probe_def.target.function_name, probe_def.id))
    }
    
    fn generate_tracepoint_bpf(&self, probe_def: &ProbeDefinition) -> Result<String> {
        Ok(format!(r#"
#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

struct event {{
    __u64 timestamp;
    __u32 pid;
    __u64 tid;
    __u16 cpu;
    char comm[16];
}};

struct {{
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u32));
}} events SEC(".maps");

SEC("tracepoint/{}")
int {}_tp(void *ctx) {{
    struct event e = {{}};
    
    e.timestamp = bpf_ktime_get_ns();
    e.pid = bpf_get_current_pid_tgid() >> 32;
    e.tid = bpf_get_current_pid_tgid();
    e.cpu = bpf_get_smp_processor_id();
    bpf_get_current_comm(&e.comm, sizeof(e.comm));
    
    bpf_perf_event_output(ctx, &events, BPF_F_CURRENT_CPU, &e, sizeof(e));
    return 0;
}}

char LICENSE[] SEC("license") = "GPL";
"#, probe_def.target.function_name, probe_def.id))
    }
}

pub async fn load_config(config_path: &Path) -> Result<ProbeConfig> {
    let content = tokio::fs::read_to_string(config_path).await?;
    let config: ProbeConfig = toml::from_str(&content)?;
    Ok(config)
}

// Configuration generator
pub struct ConfigGenerator;

impl ConfigGenerator {
    pub fn new() -> Self {
        Self
    }
    
    pub async fn analyze_and_generate(&self, target: &Path) -> Result<ProbeConfig> {
        info!("🔍 Analyzing target for probe generation: {:?}", target);
        
        // Use objdump or similar to extract function symbols
        let functions = self.extract_functions(target).await?;
        
        let mut probes = Vec::new();
        
        // Generate probes for interesting functions
        for (i, function) in functions.iter().take(50).enumerate() {
            probes.push(ProbeDefinition {
                id: format!("probe_{}", i),
                probe_type: ProbeType::Uprobe,
                target: ProbeTarget {
                    binary_path: Some(target.to_path_buf()),
                    function_name: function.clone(),
                    address_offset: None,
                    process_filter: None,
                },
                events: vec!["entry".to_string(), "exit".to_string()],
                filters: Some(ProbeFilters {
                    min_duration_ns: Some(1000), // 1μs minimum
                    max_frequency_hz: Some(1000.0),
                    stack_trace_depth: Some(10),
                }),
                sampling_rate: Some(0.1), // 10% sampling
            });
        }
        
        Ok(ProbeConfig {
            probes,
            global_settings: GlobalSettings::default(),
        })
    }
    
    pub async fn generate_system_wide_config(&self, _interfaces: &[crate::perf_interface::ProbeableInterface]) -> Result<ProbeConfig> {
        // Generate system-wide probes for common syscalls and kernel functions
        let mut probes = Vec::new();
        
        let common_syscalls = vec![
            "sys_openat", "sys_read", "sys_write", "sys_close",
            "sys_mmap", "sys_munmap", "sys_brk", "sys_clone",
            "sys_execve", "sys_exit", "sys_socket", "sys_connect"
        ];
        
        for (i, syscall) in common_syscalls.iter().enumerate() {
            probes.push(ProbeDefinition {
                id: format!("syscall_{}", syscall),
                probe_type: ProbeType::Tracepoint,
                target: ProbeTarget {
                    binary_path: None,
                    function_name: format!("syscalls:sys_enter_{}", syscall.trim_start_matches("sys_")),
                    address_offset: None,
                    process_filter: None,
                },
                events: vec!["enter".to_string()],
                filters: Some(ProbeFilters {
                    min_duration_ns: None,
                    max_frequency_hz: Some(100.0), // Limit to 100 Hz per syscall
                    stack_trace_depth: Some(5),
                }),
                sampling_rate: Some(0.01), // 1% sampling for system-wide
            });
        }
        
        Ok(ProbeConfig {
            probes,
            global_settings: GlobalSettings::default(),
        })
    }
    
    async fn extract_functions(&self, target: &Path) -> Result<Vec<String>> {
        use std::process::Command;
        
        let output = Command::new("objdump")
            .args(&["-t", target.to_str().unwrap()])
            .output()?;
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let functions: Vec<String> = stdout
            .lines()
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 6 && parts[3] == "F" {
                    Some(parts[5].to_string())
                } else {
                    None
                }
            })
            .collect();
        
        Ok(functions)
    }
}
