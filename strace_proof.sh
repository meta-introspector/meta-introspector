#!/bin/bash
# 🔥 STRACE PROOF: Wrap nix build with strace to prove LD_PRELOAD capture

echo "🔥 STRACE PROOF OF LD_PRELOAD CAPTURE"
echo "====================================="

# Set up telemetry
export PROJECT_NAME="strace_proof"
export LD_PRELOAD="/mnt/data1/meta-introspector/rust_preload_interceptor/target/release/librust_preload_interceptor.so"

# Check if LD_PRELOAD library exists
if [ ! -f "$LD_PRELOAD" ]; then
    echo "❌ LD_PRELOAD library not found: $LD_PRELOAD"
    exit 1
fi

echo "✅ LD_PRELOAD library found: $LD_PRELOAD"

# Run nix build wrapped in strace
echo "🔍 Running nix build with strace capture..."
strace -f -o /tmp/strace_nix_proof.log \
       -e trace=execve,openat,access,stat \
       nix build ./rustc-only-build --print-out-paths

echo "📊 STRACE ANALYSIS:"
echo "=================="

# Count system calls
execve_count=$(grep -c "execve(" /tmp/strace_nix_proof.log)
openat_count=$(grep -c "openat(" /tmp/strace_nix_proof.log)
access_count=$(grep -c "access(" /tmp/strace_nix_proof.log)
stat_count=$(grep -c "stat(" /tmp/strace_nix_proof.log)

echo "📋 execve calls: $execve_count"
echo "📂 openat calls: $openat_count"
echo "🔍 access calls: $access_count"
echo "📊 stat calls: $stat_count"

# Show executed binaries
echo ""
echo "🔧 EXECUTED BINARIES:"
grep "execve(" /tmp/strace_nix_proof.log | sed 's/.*execve("\([^"]*\)".*/\1/' | sort | uniq -c | sort -nr

# Show opened shared libraries
echo ""
echo "📚 OPENED SHARED LIBRARIES:"
grep "openat.*\.so" /tmp/strace_nix_proof.log | sed 's/.*"\([^"]*\.so[^"]*\)".*/\1/' | sort | uniq -c | sort -nr

# Check telemetry capture
echo ""
echo "🔍 TELEMETRY CAPTURE:"
telemetry_files=$(ls -la /mnt/data1/meta-introspector/data/telemetry/strace_proof_*.jsonl 2>/dev/null | wc -l)
echo "📄 Telemetry files created: $telemetry_files"

if [ $telemetry_files -gt 0 ]; then
    echo "✅ Telemetry captured successfully!"
    latest_file=$(ls -t /mnt/data1/meta-introspector/data/telemetry/strace_proof_*.jsonl | head -1)
    echo "📋 Latest telemetry: $latest_file"
    cat "$latest_file"
else
    echo "❌ No telemetry files found"
fi

echo ""
echo "📄 Full strace log: /tmp/strace_nix_proof.log"
echo "🎯 PROOF COMPLETE!"
