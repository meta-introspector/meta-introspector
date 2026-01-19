#!/usr/bin/env bash
# Centralized search utilities for find and grep operations
# Source this file: source lib/search_utils.sh

# Find Rust files
find_rust_files() {
    local dir="${1:-.}"
    find "$dir" -name "*.rs" -type f -not -path "*/target/*" 2>/dev/null
}

# Find files by extension
find_by_ext() {
    local dir="$1"
    local ext="$2"
    find "$dir" -name "*.$ext" -type f 2>/dev/null
}

# Find and grep pattern in files
find_grep() {
    local dir="$1"
    local pattern="$2"
    local ext="${3:-rs}"
    find "$dir" -name "*.$ext" -type f -exec grep -l "$pattern" {} \; 2>/dev/null
}

# Find Nix flakes
find_flakes() {
    local dir="${1:-.}"
    find "$dir" -name "flake.nix" -type f 2>/dev/null
}

# Find Cargo workspaces
find_workspaces() {
    local dir="${1:-.}"
    find "$dir" -name "Cargo.toml" -type f -exec grep -l "\\[workspace\\]" {} \; 2>/dev/null
}

# Find binaries in result/bin
find_result_bins() {
    local pattern="${1:-*}"
    find result/bin -name "$pattern" -type f 2>/dev/null
}

# Grep with context
grep_context() {
    local pattern="$1"
    local file="$2"
    local lines="${3:-5}"
    grep -A "$lines" "$pattern" "$file" 2>/dev/null
}

# Grep error codes
grep_errors() {
    local file="$1"
    grep -E "^error\[" "$file" 2>/dev/null | sort | uniq -c | sort -rn
}

# Grep and count
grep_count() {
    local pattern="$1"
    local file="$2"
    grep -c "$pattern" "$file" 2>/dev/null || echo "0"
}

# Find .so files in strace logs
find_so_loads() {
    local log="$1"
    grep "openat.*\\.so.*= [0-9]" "$log" 2>/dev/null | \
        sed 's/.*"\([^"]*\.so[^"]*\)".*/\1/' | sort -u
}

# Find execve calls
find_execve() {
    local log="$1"
    grep "execve(" "$log" 2>/dev/null | \
        sed 's/.*execve("\([^"]*\)".*/\1/' | sort | uniq -c | sort -nr
}

# Find git repos
find_git_repos() {
    local dir="${1:-.}"
    find "$dir" -name ".git" -type d 2>/dev/null | sed 's|/.git$||'
}

# Find files modified in last N days
find_recent() {
    local dir="$1"
    local days="${2:-7}"
    find "$dir" -type f -mtime -"$days" 2>/dev/null
}

# Find large files
find_large() {
    local dir="$1"
    local size="${2:-100M}"
    find "$dir" -type f -size +"$size" 2>/dev/null
}

# Grep for specific patterns in logs
grep_log_pattern() {
    local log="$1"
    local pattern="$2"
    grep "$pattern" "$log" 2>/dev/null | head -1 | cut -c1-80
}

# Find and count by pattern
find_count_pattern() {
    local dir="$1"
    local name_pattern="$2"
    find "$dir" -name "$name_pattern" -type f 2>/dev/null | wc -l
}

# Grep multiple patterns (OR)
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

# Grep with line numbers
grep_lines() {
    local pattern="$1"
    local file="$2"
    grep -n "$pattern" "$file" 2>/dev/null
}

# Find empty files
find_empty() {
    local dir="${1:-.}"
    find "$dir" -type f -empty 2>/dev/null
}

# Grep and extract field
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
    local name_args=""
    for ext in "${exts[@]}"; do
        name_args="$name_args -o -name '*.$ext'"
    done
    name_args="${name_args# -o }"
    eval find "$dir" -type f \( $name_args \) 2>/dev/null
}
