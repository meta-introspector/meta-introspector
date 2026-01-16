#!/bin/bash
# Launch 24 trading nodes, each on own CPU with own parquet and RAM partition

NUM_NODES=24
TOTAL_RAM_GB=64
RAM_PER_NODE=$((TOTAL_RAM_GB / NUM_NODES))  # ~2.6 GB per node

echo "🚀 Launching $NUM_NODES trading nodes"
echo "💾 RAM per node: ${RAM_PER_NODE}GB"
echo ""

# Create data directory
mkdir -p data/nodes

# Launch each node
for i in $(seq 0 $((NUM_NODES - 1))); do
    CPU_ID=$i
    PORT=$((8000 + i))
    PARQUET_FILE="data/nodes/node_${i}.parquet"
    
    echo "Starting Node $i on CPU $CPU_ID, port $PORT"
    
    # Launch with CPU affinity and memory limit
    taskset -c $CPU_ID \
        systemd-run --scope -p MemoryMax=${RAM_PER_NODE}G \
        cargo run --release --bin trading_node -- \
            --node-id $i \
            --port $PORT \
            --parquet $PARQUET_FILE \
            --peers $(seq -s, 8000 $((8000 + NUM_NODES - 1))) \
            > logs/node_${i}.log 2>&1 &
    
    sleep 0.5
done

echo ""
echo "✅ All nodes launched!"
echo "📊 Monitor: tail -f logs/node_*.log"
echo "🛑 Stop: pkill -f trading_node"
