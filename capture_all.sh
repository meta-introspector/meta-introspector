#!/bin/bash
# 🔥 CAPTURE & ANALYZE: Get all binaries + libs from nix build

echo "🔥 CAPTURING ALL BINARIES & LIBS FROM NIX BUILD"

# 1. Run strace on nix build with verbose output
echo "📊 Running strace on nix build -vvv..."
strace -f -o /tmp/nix_strace.log -e trace=execve,openat \
    ~/meta-introspector/rust-overlay-test/runbuild.sh

# 2. Extract executed binaries
echo "🔧 Extracting executed binaries..."
grep "execve(" /tmp/nix_strace.log | \
    sed 's/.*execve("\([^"]*\)".*/\1/' | \
    sort | uniq > /tmp/executed_binaries.txt

# 3. Extract opened .so files  
echo "📚 Extracting opened .so files..."
grep "openat.*\.so.*= [0-9]" /tmp/nix_strace.log | \
    sed 's/.*"\([^"]*\.so[^"]*\)".*/\1/' | \
    sort | uniq > /tmp/opened_libs.txt

# 4. Run ldd on all binaries
echo "🔍 Running ldd on all binaries..."
> /tmp/ldd_libs.txt
while read binary; do
    if [ -f "$binary" ]; then
        ldd "$binary" 2>/dev/null | grep "\.so" | \
            awk '{print $3}' | grep "^/" >> /tmp/ldd_libs.txt
    fi
done < /tmp/executed_binaries.txt

# 5. Combine and deduplicate all libraries
echo "📋 Combining all libraries..."
cat /tmp/opened_libs.txt /tmp/ldd_libs.txt | sort | uniq > /tmp/all_libs.txt

# 6. Summary
BINARIES=$(wc -l < /tmp/executed_binaries.txt)
OPENED_LIBS=$(wc -l < /tmp/opened_libs.txt) 
LDD_LIBS=$(wc -l < /tmp/ldd_libs.txt)
ALL_LIBS=$(wc -l < /tmp/all_libs.txt)

echo "✅ SUMMARY:"
echo "   🔧 Binaries executed: $BINARIES"
echo "   📂 .so files opened: $OPENED_LIBS"
echo "   📚 ldd dependencies: $LDD_LIBS"
echo "   📋 Total unique libs: $ALL_LIBS"
