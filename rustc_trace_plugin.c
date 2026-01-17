// QEMU plugin to trace rustc compilation stages
// Build: gcc -shared -fPIC -o librustc_trace.so rustc_trace_plugin.c -I/path/to/qemu/include/qemu

#include <qemu-plugin.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <glib.h>

QEMU_PLUGIN_EXPORT int qemu_plugin_version = QEMU_PLUGIN_VERSION;

static FILE *trace_file;
static GMutex lock;
static uint64_t insn_count = 0;
static uint64_t mem_read_bytes = 0;
static uint64_t mem_write_bytes = 0;

typedef struct {
    uint64_t addr;
    uint64_t size;
    char type; // 'r' or 'w'
} MemAccess;

static void vcpu_mem_access(unsigned int vcpu_index, qemu_plugin_meminfo_t info,
                            uint64_t vaddr, void *userdata)
{
    g_mutex_lock(&lock);
    
    uint64_t size = 1 << qemu_plugin_mem_size_shift(info);
    
    if (qemu_plugin_mem_is_store(info)) {
        mem_write_bytes += size;
    } else {
        mem_read_bytes += size;
    }
    
    g_mutex_unlock(&lock);
}

static void vcpu_insn_exec(unsigned int vcpu_index, void *userdata)
{
    g_atomic_int_inc(&insn_count);
}

static void vcpu_tb_trans(qemu_plugin_id_t id, struct qemu_plugin_tb *tb)
{
    size_t n_insns = qemu_plugin_tb_n_insns(tb);
    
    for (size_t i = 0; i < n_insns; i++) {
        struct qemu_plugin_insn *insn = qemu_plugin_tb_get_insn(tb, i);
        
        qemu_plugin_register_vcpu_insn_exec_cb(
            insn, vcpu_insn_exec, QEMU_PLUGIN_CB_NO_REGS, NULL);
        
        qemu_plugin_register_vcpu_mem_cb(
            insn, vcpu_mem_access,
            QEMU_PLUGIN_CB_NO_REGS,
            QEMU_PLUGIN_MEM_RW, NULL);
    }
}

static void plugin_exit(qemu_plugin_id_t id, void *p)
{
    g_mutex_lock(&lock);
    
    fprintf(trace_file, "\n=== Rustc Execution Trace ===\n");
    fprintf(trace_file, "Instructions executed: %lu\n", insn_count);
    fprintf(trace_file, "Memory read: %lu bytes\n", mem_read_bytes);
    fprintf(trace_file, "Memory written: %lu bytes\n", mem_write_bytes);
    
    fclose(trace_file);
    g_mutex_unlock(&lock);
}

QEMU_PLUGIN_EXPORT int qemu_plugin_install(qemu_plugin_id_t id,
                                           const qemu_info_t *info,
                                           int argc, char **argv)
{
    const char *output = "rustc_trace.txt";
    
    for (int i = 0; i < argc; i++) {
        if (strncmp(argv[i], "output=", 7) == 0) {
            output = &argv[i][7];
        }
    }
    
    trace_file = fopen(output, "w");
    if (!trace_file) {
        fprintf(stderr, "Failed to open trace file: %s\n", output);
        return -1;
    }
    
    g_mutex_init(&lock);
    
    qemu_plugin_register_vcpu_tb_trans_cb(id, vcpu_tb_trans);
    qemu_plugin_register_atexit_cb(id, plugin_exit, NULL);
    
    fprintf(trace_file, "=== Rustc Trace Plugin Started ===\n");
    fprintf(trace_file, "Output: %s\n", output);
    
    return 0;
}
