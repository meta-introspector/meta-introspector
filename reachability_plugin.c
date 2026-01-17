// QEMU plugin for byte-level reachability tracing
// Tracks which input bytes and instructions contribute to each output byte

#include <qemu-plugin.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <glib.h>

QEMU_PLUGIN_EXPORT int qemu_plugin_version = QEMU_PLUGIN_VERSION;

#define MAX_TRACKED_BYTES (1024 * 1024 * 10)  // 10MB max tracking

typedef struct ByteProvenance {
    uint64_t input_offsets[32];  // Input bytes that contributed
    uint64_t insn_addrs[64];     // Instructions that touched this
    uint8_t input_count;
    uint8_t insn_count;
} ByteProvenance;

typedef struct {
    uint64_t addr;
    uint64_t size;
    uint64_t insn_addr;
    uint8_t is_write;
} MemOp;

static GHashTable *byte_provenance;  // output_offset -> ByteProvenance
static GHashTable *taint_map;        // memory_addr -> input_offset
static GMutex lock;
static FILE *trace_file;
static uint64_t current_insn_addr = 0;
static uint64_t output_base = 0;
static uint64_t output_size = 0;
static uint64_t input_base = 0;
static uint64_t input_size = 0;

static void add_input_taint(uint64_t mem_addr, uint64_t input_offset)
{
    g_mutex_lock(&lock);
    uint64_t *offset = g_malloc(sizeof(uint64_t));
    *offset = input_offset;
    g_hash_table_insert(taint_map, GUINT_TO_POINTER(mem_addr), offset);
    g_mutex_unlock(&lock);
}

static void propagate_taint(uint64_t dst_addr, uint64_t src_addr, uint64_t size)
{
    g_mutex_lock(&lock);
    
    for (uint64_t i = 0; i < size; i++) {
        uint64_t *src_taint = g_hash_table_lookup(taint_map, GUINT_TO_POINTER(src_addr + i));
        if (src_taint) {
            uint64_t *dst_taint = g_malloc(sizeof(uint64_t));
            *dst_taint = *src_taint;
            g_hash_table_insert(taint_map, GUINT_TO_POINTER(dst_addr + i), dst_taint);
        }
    }
    
    g_mutex_unlock(&lock);
}

static void track_output_write(uint64_t output_offset, uint64_t mem_addr, uint64_t size)
{
    g_mutex_lock(&lock);
    
    for (uint64_t i = 0; i < size; i++) {
        uint64_t out_off = output_offset + i;
        if (out_off >= MAX_TRACKED_BYTES) continue;
        
        ByteProvenance *prov = g_hash_table_lookup(byte_provenance, GUINT_TO_POINTER(out_off));
        if (!prov) {
            prov = g_malloc0(sizeof(ByteProvenance));
            g_hash_table_insert(byte_provenance, GUINT_TO_POINTER(out_off), prov);
        }
        
        // Add instruction
        if (prov->insn_count < 64) {
            prov->insn_addrs[prov->insn_count++] = current_insn_addr;
        }
        
        // Add input taint
        uint64_t *input_taint = g_hash_table_lookup(taint_map, GUINT_TO_POINTER(mem_addr + i));
        if (input_taint && prov->input_count < 32) {
            prov->input_offsets[prov->input_count++] = *input_taint;
        }
    }
    
    g_mutex_unlock(&lock);
}

static void vcpu_mem_access(unsigned int vcpu_index, qemu_plugin_meminfo_t info,
                            uint64_t vaddr, void *userdata)
{
    uint64_t size = 1 << qemu_plugin_mem_size_shift(info);
    uint8_t is_write = qemu_plugin_mem_is_store(info);
    
    // Check if this is input read
    if (!is_write && vaddr >= input_base && vaddr < input_base + input_size) {
        uint64_t input_offset = vaddr - input_base;
        add_input_taint(vaddr, input_offset);
    }
    
    // Check if this is output write
    if (is_write && vaddr >= output_base && vaddr < output_base + output_size) {
        uint64_t output_offset = vaddr - output_base;
        track_output_write(output_offset, vaddr, size);
    }
}

static void vcpu_insn_exec(unsigned int vcpu_index, void *userdata)
{
    current_insn_addr = (uint64_t)userdata;
}

static void vcpu_tb_trans(qemu_plugin_id_t id, struct qemu_plugin_tb *tb)
{
    size_t n_insns = qemu_plugin_tb_n_insns(tb);
    
    for (size_t i = 0; i < n_insns; i++) {
        struct qemu_plugin_insn *insn = qemu_plugin_tb_get_insn(tb, i);
        uint64_t insn_addr = qemu_plugin_insn_vaddr(insn);
        
        qemu_plugin_register_vcpu_insn_exec_cb(
            insn, vcpu_insn_exec, QEMU_PLUGIN_CB_NO_REGS, 
            (void*)insn_addr);
        
        qemu_plugin_register_vcpu_mem_cb(
            insn, vcpu_mem_access,
            QEMU_PLUGIN_CB_NO_REGS,
            QEMU_PLUGIN_MEM_RW, NULL);
    }
}

static void write_provenance_report(gpointer key, gpointer value, gpointer user_data)
{
    uint64_t output_offset = GPOINTER_TO_UINT(key);
    ByteProvenance *prov = (ByteProvenance*)value;
    
    fprintf(trace_file, "\nOutput byte %lu:\n", output_offset);
    
    if (prov->input_count > 0) {
        fprintf(trace_file, "  Input bytes: ");
        for (int i = 0; i < prov->input_count; i++) {
            fprintf(trace_file, "%lu ", prov->input_offsets[i]);
        }
        fprintf(trace_file, "\n");
    }
    
    if (prov->insn_count > 0) {
        fprintf(trace_file, "  Instructions: ");
        for (int i = 0; i < prov->insn_count && i < 10; i++) {
            fprintf(trace_file, "0x%lx ", prov->insn_addrs[i]);
        }
        if (prov->insn_count > 10) {
            fprintf(trace_file, "... (%d total)", prov->insn_count);
        }
        fprintf(trace_file, "\n");
    }
}

static void plugin_exit(qemu_plugin_id_t id, void *p)
{
    g_mutex_lock(&lock);
    
    fprintf(trace_file, "\n=== Byte Reachability Report ===\n");
    fprintf(trace_file, "Tracked output bytes: %u\n\n", 
            g_hash_table_size(byte_provenance));
    
    g_hash_table_foreach(byte_provenance, write_provenance_report, NULL);
    
    fclose(trace_file);
    g_mutex_unlock(&lock);
}

QEMU_PLUGIN_EXPORT int qemu_plugin_install(qemu_plugin_id_t id,
                                           const qemu_info_t *info,
                                           int argc, char **argv)
{
    const char *output = "reachability.txt";
    
    for (int i = 0; i < argc; i++) {
        char *arg = argv[i];
        if (strncmp(arg, "output=", 7) == 0) {
            output = &arg[7];
        } else if (strncmp(arg, "input_base=", 11) == 0) {
            input_base = strtoull(&arg[11], NULL, 0);
        } else if (strncmp(arg, "input_size=", 11) == 0) {
            input_size = strtoull(&arg[11], NULL, 0);
        } else if (strncmp(arg, "output_base=", 12) == 0) {
            output_base = strtoull(&arg[12], NULL, 0);
        } else if (strncmp(arg, "output_size=", 12) == 0) {
            output_size = strtoull(&arg[12], NULL, 0);
        }
    }
    
    trace_file = fopen(output, "w");
    if (!trace_file) {
        return -1;
    }
    
    byte_provenance = g_hash_table_new_full(g_direct_hash, g_direct_equal, NULL, g_free);
    taint_map = g_hash_table_new_full(g_direct_hash, g_direct_equal, NULL, g_free);
    g_mutex_init(&lock);
    
    qemu_plugin_register_vcpu_tb_trans_cb(id, vcpu_tb_trans);
    qemu_plugin_register_atexit_cb(id, plugin_exit, NULL);
    
    fprintf(trace_file, "=== Byte Reachability Tracer ===\n");
    fprintf(trace_file, "Input: 0x%lx - 0x%lx (%lu bytes)\n", 
            input_base, input_base + input_size, input_size);
    fprintf(trace_file, "Output: 0x%lx - 0x%lx (%lu bytes)\n\n",
            output_base, output_base + output_size, output_size);
    
    return 0;
}
