#!/bin/bash
# 🌟 DEMO: Unified Nix-as-a-Service with MCP + Solana + Content Addressing

set -e

echo "🚀 ZOS UNIFIED NIX-AS-A-SERVICE DEMO"
echo "===================================="
echo ""

# Check if ZOS server is running
if ! curl -s http://localhost:8000/ > /dev/null; then
    echo "❌ ZOS server not running. Please start it first:"
    echo "   cd ~/zos-server && cargo run"
    exit 1
fi

echo "✅ ZOS server is running"
echo ""

# Demo 1: Load a simple nix flake
echo "📦 DEMO 1: Loading nixpkgs#hello flake"
echo "======================================"

FLAKE_REQUEST='{
    "flake_url": "github:nixos/nixpkgs",
    "outputs": ["hello"],
    "payment_lamports": 5000,
    "mcp_tools_requested": ["list_tools", "call_tool"]
}'

echo "Request:"
echo "$FLAKE_REQUEST" | jq .
echo ""

echo "Loading flake..."
RESPONSE=$(curl -s -X POST http://localhost:8000/api/v1/unified/load-flake \
    -H "Content-Type: application/json" \
    -d "$FLAKE_REQUEST")

if [ $? -eq 0 ]; then
    echo "✅ Flake loaded successfully!"
    echo "Response:"
    echo "$RESPONSE" | jq .
    
    # Extract content address for next demos
    CONTENT_ADDRESS=$(echo "$RESPONSE" | jq -r '.content_address')
    echo ""
    echo "📍 Content Address: $CONTENT_ADDRESS"
else
    echo "❌ Failed to load flake"
    exit 1
fi

echo ""
echo "🔧 DEMO 2: Calling MCP tool on loaded flake"
echo "==========================================="

if [ "$CONTENT_ADDRESS" != "null" ] && [ -n "$CONTENT_ADDRESS" ]; then
    MCP_ARGS='{"input": "list available tools"}'
    
    echo "Calling MCP tool: hello_list_tools"
    echo "Args: $MCP_ARGS"
    echo ""
    
    MCP_RESPONSE=$(curl -s -X POST "http://localhost:8000/api/v1/unified/mcp/$CONTENT_ADDRESS/hello_list_tools" \
        -H "Content-Type: application/json" \
        -d "$MCP_ARGS")
    
    if [ $? -eq 0 ]; then
        echo "✅ MCP tool called successfully!"
        echo "Response:"
        echo "$MCP_RESPONSE" | jq .
    else
        echo "❌ MCP tool call failed"
    fi
else
    echo "⚠️  No content address available, skipping MCP demo"
fi

echo ""
echo "🌌 DEMO 3: Checking Solana orbital transaction"
echo "=============================================="

if [ "$CONTENT_ADDRESS" != "null" ] && [ -n "$CONTENT_ADDRESS" ]; then
    echo "Getting orbital info for: $CONTENT_ADDRESS"
    
    ORBITAL_RESPONSE=$(curl -s "http://localhost:8000/api/v1/unified/orbit/$CONTENT_ADDRESS")
    
    if [ $? -eq 0 ]; then
        echo "✅ Orbital info retrieved!"
        echo "Response:"
        echo "$ORBITAL_RESPONSE" | jq .
    else
        echo "❌ Failed to get orbital info"
    fi
else
    echo "⚠️  No content address available, skipping orbital demo"
fi

echo ""
echo "📚 DEMO 4: Listing loaded libraries"
echo "==================================="

if [ "$CONTENT_ADDRESS" != "null" ] && [ -n "$CONTENT_ADDRESS" ]; then
    echo "Getting libraries for: $CONTENT_ADDRESS"
    
    LIBS_RESPONSE=$(curl -s "http://localhost:8000/api/v1/unified/libraries/$CONTENT_ADDRESS")
    
    if [ $? -eq 0 ]; then
        echo "✅ Libraries info retrieved!"
        echo "Response:"
        echo "$LIBS_RESPONSE" | jq .
    else
        echo "❌ Failed to get libraries info"
    fi
else
    echo "⚠️  No content address available, skipping libraries demo"
fi

echo ""
echo "📊 DEMO 5: Service status and capabilities"
echo "=========================================="

STATUS_RESPONSE=$(curl -s "http://localhost:8000/api/v1/unified/status")

if [ $? -eq 0 ]; then
    echo "✅ Service status retrieved!"
    echo "Response:"
    echo "$STATUS_RESPONSE" | jq .
else
    echo "❌ Failed to get service status"
fi

echo ""
echo "🎯 DEMO COMPLETE!"
echo "================="
echo ""
echo "🌟 What we demonstrated:"
echo "  ✅ Nix flake loading with content addressing"
echo "  ✅ MCP tool discovery and calling"
echo "  ✅ Solana orbital transaction tracking"
echo "  ✅ Dynamic library loading from nix store"
echo "  ✅ Unified service integration"
echo ""
echo "💡 Next steps:"
echo "  - Load your own nix flakes"
echo "  - Create custom MCP tools"
echo "  - Integrate with Solana devnet"
echo "  - Build complex service compositions"
echo ""
echo "🚀 The future is Nix-as-a-Service with MCP + Solana!"
