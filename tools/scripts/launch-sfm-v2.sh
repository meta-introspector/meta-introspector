#!/bin/bash
# Launch solfunmeme-dioxus v2 as ZOS server

set -e

echo "🚀 Launching SOLFUNMEME v2 on ZOS Server"
echo "========================================="

# Build WASM
echo "📦 Building WASM..."
cd /mnt/data1/nix/time/2025/06/01/solfunmeme-dioxus
cargo build --target wasm32-unknown-unknown --release --bin solfunmeme-dioxus

# Build plugin
echo "🔌 Building solana-p2p plugin..."
cd plugins/solana-p2p
cargo build --target wasm32-unknown-unknown --release

# Copy to ZOS server
echo "📂 Deploying to ZOS server..."
mkdir -p ~/zos-server/www/plugins
cp ../../target/wasm32-unknown-unknown/release/solfunmeme_dioxus.wasm ~/zos-server/www/
cp target/wasm32-unknown-unknown/release/solana_p2p_plugin.wasm ~/zos-server/www/plugins/

# Create index.html
cat > ~/zos-server/www/index.html << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>SOLFUNMEME v2</title>
    <style>
        body { font-family: sans-serif; margin: 20px; }
        .app { max-width: 800px; margin: 0 auto; }
        button { padding: 10px 20px; margin: 10px 0; }
        .result { background: #f0f0f0; padding: 10px; margin: 10px 0; }
    </style>
</head>
<body>
    <div id="main"></div>
    <script type="module">
        import init from './solfunmeme_dioxus.js';
        init();
    </script>
</body>
</html>
EOF

# Start ZOS server
echo "🌐 Starting ZOS server on http://localhost:8080"
cd ~/zos-server
cargo run --release --bin zos_server serve
