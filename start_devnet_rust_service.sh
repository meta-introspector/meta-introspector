#!/bin/bash
# 🚀 DEVNET RUST-AS-A-SERVICE LAUNCHER
# Starts both Rust compilation service and ZOS server

set -e

echo "🦀 RUST-AS-A-SERVICE DEVNET LAUNCHER"
echo "=================================="

# Check zombie_driver2 exists
if [ ! -d "/home/mdupont/zombie_driver2" ]; then
    echo "❌ zombie_driver2 not found at /home/mdupont/zombie_driver2"
    exit 1
fi

echo "✅ Found zombie_driver2 at /home/mdupont/zombie_driver2"

# Set environment variables
export ZOMBIE_DRIVER_PATH="/home/mdupont/zombie_driver2"
export RUST_LOG=info

# Function to cleanup background processes
cleanup() {
    echo "🛑 Shutting down services..."
    if [ ! -z "$RUST_SERVICE_PID" ]; then
        kill $RUST_SERVICE_PID 2>/dev/null || true
    fi
    if [ ! -z "$ZOS_SERVER_PID" ]; then
        kill $ZOS_SERVER_PID 2>/dev/null || true
    fi
    exit 0
}

trap cleanup SIGINT SIGTERM

# Start Rust-as-a-Service in background
echo "🦀 Starting Rust-as-a-Service on port 8080..."
cd /mnt/data1/meta-introspector
cargo run --bin rust_as_a_service &
RUST_SERVICE_PID=$!

# Wait for Rust service to start
echo "⏳ Waiting for Rust service to initialize..."
sleep 5

# Check if Rust service is running
if ! curl -s http://localhost:8080/metrics > /dev/null; then
    echo "❌ Rust service failed to start"
    kill $RUST_SERVICE_PID 2>/dev/null || true
    exit 1
fi

echo "✅ Rust-as-a-Service running on http://localhost:8080"

# Start ZOS Server
echo "🌟 Starting ZOS Server on port 8000..."
python3 /mnt/data1/meta-introspector/zos_server_v2.py &
ZOS_SERVER_PID=$!

# Wait for ZOS server to start
sleep 3

# Check if ZOS server is running
if ! curl -s http://localhost:8000/ > /dev/null; then
    echo "❌ ZOS server failed to start"
    cleanup
    exit 1
fi

echo "✅ ZOS Server running on http://localhost:8000"
echo ""
echo "🎯 DEVNET SERVICES READY!"
echo "========================"
echo "🦀 Rust Compilation: http://localhost:8080"
echo "   POST /compile - Compile Rust code"
echo "   GET  /pricing - View pricing"
echo "   GET  /metrics - Service metrics"
echo ""
echo "🌟 ZOS Server: http://localhost:8000"
echo "   POST /load/rust - Load Rust service"
echo "   POST /load/solfunmeme - Load SOLFUNMEME"
echo "   POST /devnet/compile - Pay-per-compilation"
echo "   GET  /services - List loaded services"
echo "   GET  /pricing/rust - Rust pricing"
echo ""
echo "💰 PRICING MODEL:"
echo "   Base cost: 1000 lamports"
echo "   Per line: 10 lamports"
echo "   Optimization: +50% cost"
echo "   Features: 100 lamports each"
echo ""
echo "🔥 EXAMPLE USAGE:"
echo 'curl -X POST http://localhost:8000/devnet/compile \'
echo '  -H "Content-Type: application/json" \'
echo '  -d '"'"'{'
echo '    "source_code": "fn main() { println!(\"Hello, devnet!\"); }",'
echo '    "payment_lamports": 2000'
echo '  }'"'"
echo ""
echo "Press Ctrl+C to stop all services..."

# Wait for user interrupt
wait
