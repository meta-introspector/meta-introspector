
#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <dlfcn.h>
#include <unistd.h>
#include <sys/types.h>
#include <time.h>
#include <string.h>

// Function pointers for original functions
static int (*real_execve)(const char *pathname, char *const argv[], char *const envp[]) = NULL;
static FILE* (*real_fopen)(const char *pathname, const char *mode) = NULL;
static void* (*real_malloc)(size_t size) = NULL;
static void (*real_free)(void *ptr) = NULL;

// Telemetry log
static FILE* telemetry_log = NULL;
static int initialized = 0;

static void init_interceptor() {
    if (initialized) return;
    initialized = 1;
    
    // Load original functions
    real_execve = dlsym(RTLD_NEXT, "execve");
    real_fopen = dlsym(RTLD_NEXT, "fopen");
    real_malloc = dlsym(RTLD_NEXT, "malloc");
    real_free = dlsym(RTLD_NEXT, "free");
    
    // Open telemetry log
    char log_path[512];
    snprintf(log_path, sizeof(log_path), "/tmp/preload_intercept_%d.log", getpid());
    telemetry_log = fopen(log_path, "a");
    
    if (telemetry_log) {
        fprintf(telemetry_log, "🔥 PRELOAD INTERCEPTOR ACTIVE PID:%d\n", getpid());
        fflush(telemetry_log);
    }
    
    fprintf(stderr, "🔥 LD_PRELOAD interceptor active for PID %d\n", getpid());
}

// Intercept execve
int execve(const char *pathname, char *const argv[], char *const envp[]) {
    init_interceptor();
    
    if (telemetry_log) {
        fprintf(telemetry_log, "🎯 EXECVE: %s\n", pathname ? pathname : "NULL");
        fflush(telemetry_log);
    }
    
    fprintf(stderr, "🎯 INTERCEPTED EXECVE: %s\n", pathname ? pathname : "NULL");
    
    if (real_execve) {
        return real_execve(pathname, argv, envp);
    }
    return -1;
}

// Intercept fopen
FILE* fopen(const char *pathname, const char *mode) {
    init_interceptor();
    
    if (telemetry_log && pathname) {
        fprintf(telemetry_log, "📁 FOPEN: %s (%s)\n", pathname, mode ? mode : "?");
        fflush(telemetry_log);
    }
    
    if (real_fopen) {
        return real_fopen(pathname, mode);
    }
    return NULL;
}

// Intercept malloc
void* malloc(size_t size) {
    init_interceptor();
    
    if (telemetry_log) {
        fprintf(telemetry_log, "🧠 MALLOC: %zu bytes\n", size);
        fflush(telemetry_log);
    }
    
    if (real_malloc) {
        return real_malloc(size);
    }
    return NULL;
}

// Intercept free
void free(void *ptr) {
    init_interceptor();
    
    if (telemetry_log && ptr) {
        fprintf(telemetry_log, "🧠 FREE: %p\n", ptr);
        fflush(telemetry_log);
    }
    
    if (real_free) {
        real_free(ptr);
    }
}

// Constructor - called when library is loaded
__attribute__((constructor))
void preload_constructor() {
    init_interceptor();
}

// Destructor - called when library is unloaded
__attribute__((destructor))
void preload_destructor() {
    if (telemetry_log) {
        fprintf(telemetry_log, "🔥 PRELOAD INTERCEPTOR SHUTDOWN PID:%d\n", getpid());
        fclose(telemetry_log);
    }
}
