#!/usr/bin/env bash
# ZOS AI-Ticket Task Runner
# Executes Gemini tasks from the queue

set -euo pipefail

TASK_DIR="data/gemini_tasks"
QUEUE_FILE="$TASK_DIR/queue.json"

# Colors
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log() { echo -e "${GREEN}[$(date +'%H:%M:%S')]${NC} $*"; }
warn() { echo -e "${YELLOW}[$(date +'%H:%M:%S')]${NC} $*"; }
error() { echo -e "${RED}[$(date +'%H:%M:%S')]${NC} $*"; }

# Check if task file exists
if [[ ! -f "$QUEUE_FILE" ]]; then
    error "Queue file not found: $QUEUE_FILE"
    exit 1
fi

# Get next pending task
get_next_task() {
    jq -r '.tasks[] | select(.status == "pending") | .file' "$QUEUE_FILE" | head -1
}

# Execute task
execute_task() {
    local task_file="$1"
    local task_path="$TASK_DIR/$task_file"
    
    if [[ ! -f "$task_path" ]]; then
        error "Task file not found: $task_path"
        return 1
    fi
    
    log "Loading task: $task_file"
    
    local task_id=$(jq -r '.id' "$task_path")
    local task_title=$(jq -r '.title' "$task_path")
    local task_prompt=$(jq -r '.prompt' "$task_path")
    
    log "Task $task_id: $task_title"
    
    # Update status to in_progress
    jq --arg file "$task_file" \
       '(.tasks[] | select(.file == $file) | .status) = "in_progress"' \
       "$QUEUE_FILE" > "$QUEUE_FILE.tmp" && mv "$QUEUE_FILE.tmp" "$QUEUE_FILE"
    
    log "Executing with Gemini..."
    
    # Execute via Gemini
    if nix run .#gemini -- -p "$task_prompt" > "data/gemini_output_task_${task_id}.txt" 2>&1; then
        log "✅ Task $task_id completed successfully"
        
        # Update status to completed
        jq --arg file "$task_file" \
           '(.tasks[] | select(.file == $file) | .status) = "completed"' \
           "$QUEUE_FILE" > "$QUEUE_FILE.tmp" && mv "$QUEUE_FILE.tmp" "$QUEUE_FILE"
        
        # Show output
        cat "data/gemini_output_task_${task_id}.txt"
        
        return 0
    else
        error "❌ Task $task_id failed"
        
        # Update status to failed
        jq --arg file "$task_file" \
           '(.tasks[] | select(.file == $file) | .status) = "failed"' \
           "$QUEUE_FILE" > "$QUEUE_FILE.tmp" && mv "$QUEUE_FILE.tmp" "$QUEUE_FILE"
        
        # Show error
        cat "data/gemini_output_task_${task_id}.txt"
        
        return 1
    fi
}

# Main
main() {
    log "ZOS AI-Ticket Task Runner"
    log "========================="
    
    # Show queue status
    local total=$(jq '.total_tasks' "$QUEUE_FILE")
    local pending=$(jq '.pending' "$QUEUE_FILE")
    local completed=$(jq '.completed' "$QUEUE_FILE")
    
    log "Total tasks: $total"
    log "Pending: $pending"
    log "Completed: $completed"
    echo
    
    # Get next task
    local next_task=$(get_next_task)
    
    if [[ -z "$next_task" ]]; then
        warn "No pending tasks in queue"
        exit 0
    fi
    
    log "Next task: $next_task"
    echo
    
    # Execute
    if execute_task "$next_task"; then
        log "✅ Task completed successfully"
        
        # Check if more tasks
        next_task=$(get_next_task)
        if [[ -n "$next_task" ]]; then
            log "More tasks available. Run again to continue."
        else
            log "🎉 All tasks completed!"
        fi
    else
        error "❌ Task failed. Check output above."
        exit 1
    fi
}

main "$@"
