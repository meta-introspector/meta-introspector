#!/usr/bin/env bash
# Centralized search utilities - Version 2.0
# Now uses file-index service for cached, intelligent queries

# Server URL
FILE_INDEX_SERVER="${FILE_INDEX_SERVER:-http://127.0.0.1:3030}"
FILE_INDEX_CLI="${FILE_INDEX_CLI:-file-index}"

# Check if service is available
check_service() {
    if command -v "$FILE_INDEX_CLI" &> /dev/null; then
        if curl -s "$FILE_INDEX_SERVER/health" &> /dev/null; then
            return 0
        fi
    fi
    return 1
}

# Find Rust files
find_rust_files() {
    local dir="${1:-.}"
    if check_service; then
        $FILE_INDEX_CLI query ext rs
    else
        # Fallback to traditional find
        find "$dir" -name "*.rs" -type f -not -path "*/target/*" 2>/dev/null
    fi
}

# Find files by extension
find_by_ext() {
    local dir="$1"
    local ext="$2"
    if check_service; then
        $FILE_INDEX_CLI query ext "$ext"
    else
        find "$dir" -name "*.$ext" -type f 2>/dev/null
    fi
}

# Find and grep pattern in files
find_grep() {
    local dir="$1"
    local pattern="$2"
    local ext="${3:-rs}"
    if check_service; then
        # Use service to find files, then grep locally
        $FILE_INDEX_CLI query ext "$ext" | xargs grep -l "$pattern" 2>/dev/null
    else
        find "$dir" -name "*.$ext" -type f -exec grep -l "$pattern" {} \; 2>/dev/null
    fi
}

# Find Nix flakes
find_flakes() {
    local dir="${1:-.}"
    if check_service; then
        $FILE_INDEX_CLI query name "flake.nix"
    else
        find "$dir" -name "flake.nix" -type f 2>/dev/null
    fi
}

# Find Cargo workspaces
find_workspaces() {
    local dir="${1:-.}"
    if check_service; then
        $FILE_INDEX_CLI query name "Cargo.toml" | xargs grep -l "\[workspace\]" 2>/dev/null
    else
        find "$dir" -name "Cargo.toml" -type f -exec grep -l "\\[workspace\\]" {} \; 2>/dev/null
    fi
}

# Find binaries in result/bin
find_result_bins() {
    local pattern="${1:-*}"
    if check_service; then
        $FILE_INDEX_CLI query pattern "result/bin/$pattern"
    else
        find result/bin -name "$pattern" -type f 2>/dev/null
    fi
}

# Get top priority files (most likely to be accessed)
find_priority() {
    local limit="${1:-100}"
    if check_service; then
        $FILE_INDEX_CLI priority --limit "$limit"
    else
        echo "Service not available" >&2
        return 1
    fi
}

# Get predicted queries
predict_queries() {
    if check_service; then
        $FILE_INDEX_CLI predict
    else
        echo "Service not available" >&2
        return 1
    fi
}

# Get index statistics
index_stats() {
    if check_service; then
        $FILE_INDEX_CLI stats
    else
        echo "Service not available" >&2
        return 1
    fi
}

# Refresh index
refresh_index() {
    if check_service; then
        $FILE_INDEX_CLI refresh
    else
        echo "Service not available" >&2
        return 1
    fi
}

# Grep with context (still local)
grep_context() {
    local pattern="$1"
    local file="$2"
    local lines="${3:-5}"
    grep -A "$lines" "$pattern" "$file" 2>/dev/null
}

# Grep error codes (still local)
grep_errors() {
    local file="$1"
    grep -E "^error\[" "$file" 2>/dev/null | sort | uniq -c | sort -rn
}

# Grep and count (still local)
grep_count() {
    local pattern="$1"
    local file="$2"
    grep -c "$pattern" "$file" 2>/dev/null || echo "0"
}

# Find .so files in strace logs (still local)
find_so_loads() {
    local log="$1"
    grep "openat.*\\.so.*= [0-9]" "$log" 2>/dev/null | \
        sed 's/.*"\([^"]*\.so[^"]*\)".*/\1/' | sort -u
}

# Find execve calls (still local)
find_execve() {
    local log="$1"
    grep "execve(" "$log" 2>/dev/null | \
        sed 's/.*execve("\([^"]*\)".*/\1/' | sort | uniq -c | sort -nr
}

# Find git repos
find_git_repos() {
    local dir="${1:-.}"
    if check_service; then
        $FILE_INDEX_CLI query name ".git" | sed 's|/.git$||'
    else
        find "$dir" -name ".git" -type d 2>/dev/null | sed 's|/.git$||'
    fi
}

# Find recent files
find_recent() {
    local dir="$1"
    local days="${2:-7}"
    # This still uses find as it requires mtime
    find "$dir" -type f -mtime -"$days" 2>/dev/null
}

# Find large files
find_large() {
    local dir="$1"
    local size="${2:-100M}"
    # This still uses find as it requires size filtering
    find "$dir" -type f -size +"$size" 2>/dev/null
}

# Grep for specific patterns in logs (still local)
grep_log_pattern() {
    local log="$1"
    local pattern="$2"
    grep "$pattern" "$log" 2>/dev/null | head -1 | cut -c1-80
}

# Find and count by pattern
find_count_pattern() {
    local dir="$1"
    local name_pattern="$2"
    if check_service; then
        $FILE_INDEX_CLI query pattern "$name_pattern" --format count
    else
        find "$dir" -name "$name_pattern" -type f 2>/dev/null | wc -l
    fi
}

# Grep multiple patterns (OR) (still local)
grep_multi() {
    local file="$1"
    shift
    local patterns="$*"
    grep -E "$(echo "$patterns" | tr ' ' '|')" "$file" 2>/dev/null
}

# Find files excluding patterns
find_exclude() {
    local dir="$1"
    local pattern="$2"
    shift 2
    local excludes=("$@")
    local exclude_args=""
    for ex in "${excludes[@]}"; do
        exclude_args="$exclude_args -not -path '*/$ex/*'"
    done
    eval find "$dir" -name "$pattern" -type f $exclude_args 2>/dev/null
}

# Grep with line numbers (still local)
grep_lines() {
    local pattern="$1"
    local file="$2"
    grep -n "$pattern" "$file" 2>/dev/null
}

# Find empty files
find_empty() {
    local dir="${1:-.}"
    # Still uses find as it requires empty check
    find "$dir" -type f -empty 2>/dev/null
}

# Grep and extract field (still local)
grep_field() {
    local pattern="$1"
    local file="$2"
    local field="${3:-2}"
    grep "$pattern" "$file" 2>/dev/null | awk "{print \$$field}"
}

# Find by multiple extensions
find_multi_ext() {
    local dir="$1"
    shift
    local exts=("$@")
    
    if check_service; then
        for ext in "${exts[@]}"; do
            $FILE_INDEX_CLI query ext "$ext"
        done
    else
        local name_args=""
        for ext in "${exts[@]}"; do
            name_args="$name_args -o -name '*.$ext'"
        done
        name_args="${name_args# -o }"
        eval find "$dir" -type f \( $name_args \) 2>/dev/null
    fi
}

# Show service status
service_status() {
    if check_service; then
        echo "✅ File Index Service: ONLINE"
        echo "   Server: $FILE_INDEX_SERVER"
        index_stats
    else
        echo "❌ File Index Service: OFFLINE"
        echo "   Falling back to traditional find/grep"
        echo ""
        echo "To start the service:"
        echo "  cargo run --bin file-index-server"
    fi
}
