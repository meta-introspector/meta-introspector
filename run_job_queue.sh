#!/bin/bash

# Job Queue Runner for Value Lattice Analysis
# Runs analysis on all 3 core repos with job management

REPOS=(
    "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/rust-build"
    "/mnt/data1/nix/vendor/rust/cargo2nix/submodules/split-decls-rs" 
    "/home/mdupont/zos-server"
)

ANALYZER="/mnt/data1/meta-introspector/target/release/crossbeam_value_lattice"
LOG_DIR="/mnt/data1/meta-introspector/analysis/logs"
PROGRESS_DIR="/mnt/data1/meta-introspector/analysis"

mkdir -p "$LOG_DIR"

echo "🚀 VALUE LATTICE JOB QUEUE RUNNER"
echo "================================="
echo "📊 Output: $PROGRESS_DIR/value-lattice/"
echo "📝 Logs: $LOG_DIR/"
echo "💾 Progress: $PROGRESS_DIR/progress.json"
echo ""

for i in "${!REPOS[@]}"; do
    repo="${REPOS[$i]}"
    job_num=$((i + 1))
    
    if [ -d "$repo" ]; then
        echo "🎯 Job $job_num/3: $(basename "$repo")"
        echo "   📂 Path: $repo"
        
        # Run in background with logging
        cd "$repo"
        echo "   ⚡ Starting 20-core analysis..."
        
        # Run with nohup for persistence
        nohup "$ANALYZER" > "$LOG_DIR/job_${job_num}_$(basename "$repo").log" 2>&1 &
        JOB_PID=$!
        
        echo "   🔢 PID: $JOB_PID"
        echo "   📊 Monitor: tail -f $LOG_DIR/job_${job_num}_$(basename "$repo").log"
        echo "   📈 Progress: cat $PROGRESS_DIR/progress.json"
        echo ""
        
        # Wait a moment before starting next job
        sleep 2
    else
        echo "❌ Job $job_num/3: Repository not found: $repo"
    fi
done

echo "🎉 All jobs queued!"
echo ""
echo "📋 Monitor commands:"
echo "   ps aux | grep crossbeam_value_lattice"
echo "   ls -la $PROGRESS_DIR/value-lattice/"
echo "   tail -f $LOG_DIR/*.log"
