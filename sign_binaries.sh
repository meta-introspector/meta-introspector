#!/bin/bash
# Sign binaries with GPG

LIBS=(
    "target/debug/minimal-build-server"
    "target/debug/liblibnix.so"
    "libhttp/target/release/liblibhttp.so"
    "libgit/target/release/liblibgit.so"
)

for lib in "${LIBS[@]}"; do
    if [ -f "$lib" ]; then
        echo "🔏 Signing $lib..."
        gpg --detach-sign --armor "$lib"
        echo "✅ Created $lib.asc"
    fi
done

echo ""
echo "📦 Signed binaries:"
ls -lh target/debug/minimal-build-server.asc \
       target/debug/liblibnix.so.asc \
       libhttp/target/release/liblibhttp.so.asc \
       libgit/target/release/liblibgit.so.asc 2>/dev/null || true
