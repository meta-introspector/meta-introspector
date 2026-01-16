#!/bin/bash
# Verify signed binaries

verify_binary() {
    local bin=$1
    if [ -f "$bin" ] && [ -f "$bin.asc" ]; then
        echo "🔍 Verifying $bin..."
        if gpg --verify "$bin.asc" "$bin" 2>&1 | grep -q "Good signature"; then
            echo "✅ Valid signature"
            return 0
        else
            echo "❌ Invalid signature"
            return 1
        fi
    else
        echo "⚠️  Missing $bin or $bin.asc"
        return 1
    fi
}

echo "🔐 Verifying signed binaries..."
echo ""

verify_binary "target/debug/minimal-build-server"
verify_binary "target/debug/liblibnix.so"
verify_binary "libhttp/target/release/liblibhttp.so"
verify_binary "libgit/target/release/liblibgit.so"
