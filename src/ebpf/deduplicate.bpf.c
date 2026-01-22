// eBPF program for runtime deduplication
// Stops duplicate code execution at kernel level

#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>
#include <linux/ptrace.h>

// Map: instruction signature -> execution count
struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1000000);
    __type(key, __u64);      // eBPF signature
    __type(value, __u64);    // execution count
} execution_map SEC(".maps");

// Map: duplicate events
struct {
    __uint(type, BPF_MAP_TYPE_PERF_EVENT_ARRAY);
    __uint(key_size, sizeof(__u32));
    __uint(value_size, sizeof(__u32));
} duplicate_events SEC(".maps");

// Duplicate event structure
struct duplicate_event {
    __u64 signature;
    __u32 pid;
    __u64 timestamp;
    char comm[16];
};

// Compute eBPF signature from instruction pointer
static __always_inline __u64 compute_signature(struct pt_regs *ctx) {
    __u64 ip = PT_REGS_IP(ctx);
    __u64 sp = PT_REGS_SP(ctx);
    
    // Hash: instruction pointer + stack pointer
    // This creates unique signature per code path
    return (ip << 32) | (sp & 0xFFFFFFFF);
}

// Detect and block duplicate executions
SEC("kprobe/sys_execve")
int detect_duplicate_execve(struct pt_regs *ctx) {
    __u64 signature = compute_signature(ctx);
    
    // Check if already executed
    __u64 *count = bpf_map_lookup_elem(&execution_map, &signature);
    
    if (count && *count > 0) {
        // DUPLICATE DETECTED!
        
        // Log event
        struct duplicate_event evt = {};
        evt.signature = signature;
        evt.pid = bpf_get_current_pid_tgid() >> 32;
        evt.timestamp = bpf_ktime_get_ns();
        bpf_get_current_comm(&evt.comm, sizeof(evt.comm));
        
        bpf_perf_event_output(ctx, &duplicate_events, BPF_F_CURRENT_CPU,
                              &evt, sizeof(evt));
        
        // Block execution
        bpf_override_return(ctx, -EALREADY);
        return 1;
    }
    
    // First execution - record and allow
    __u64 one = 1;
    bpf_map_update_elem(&execution_map, &signature, &one, BPF_ANY);
    return 0;
}

// Track all syscalls
SEC("kprobe/sys_read")
int detect_duplicate_read(struct pt_regs *ctx) {
    return detect_duplicate_execve(ctx);
}

SEC("kprobe/sys_write")
int detect_duplicate_write(struct pt_regs *ctx) {
    return detect_duplicate_execve(ctx);
}

SEC("kprobe/sys_open")
int detect_duplicate_open(struct pt_regs *ctx) {
    return detect_duplicate_execve(ctx);
}

SEC("kprobe/sys_close")
int detect_duplicate_close(struct pt_regs *ctx) {
    return detect_duplicate_execve(ctx);
}

char LICENSE[] SEC("license") = "GPL";
