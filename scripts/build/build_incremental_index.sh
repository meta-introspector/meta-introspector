#!/bin/bash
# Incremental File Index Builder
# Uses git/nix metadata instead of find

set -e

INDEX_DB="/mnt/data1/meta-introspector/indexes/file_index.db"
CACHE_DIR="/mnt/data1/meta-introspector/indexes/cache"

mkdir -p "$(dirname "$INDEX_DB")" "$CACHE_DIR"

echo "🔍 Building incremental file index..."

# Initialize SQLite database
sqlite3 "$INDEX_DB" <<EOF
CREATE TABLE IF NOT EXISTS files (
    file_path TEXT PRIMARY KEY,
    file_type TEXT,  -- git, nix, temp, wip
    git_repo TEXT,
    commit_hash TEXT,
    last_seen TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    file_size INTEGER,
    content_hash TEXT
);

CREATE INDEX IF NOT EXISTS idx_file_type ON files(file_type);
CREATE INDEX IF NOT EXISTS idx_git_repo ON files(git_repo);
CREATE INDEX IF NOT EXISTS idx_last_seen ON files(last_seen);
EOF

# Function: Index git repository
index_git_repo() {
    local repo_path="$1"
    local repo_name=$(basename "$repo_path")
    
    echo "  📦 Indexing git repo: $repo_name"
    
    cd "$repo_path"
    
    # Get current commit
    local commit=$(git rev-parse HEAD 2>/dev/null || echo "unknown")
    
    # List all tracked files
    git ls-files -z 2>/dev/null | while IFS= read -r -d '' file; do
        local full_path="$repo_path/$file"
        if [ -f "$full_path" ]; then
            local size=$(stat -c%s "$full_path" 2>/dev/null || echo 0)
            local hash=$(git hash-object "$full_path" 2>/dev/null || echo "unknown")
            
            sqlite3 "$INDEX_DB" <<SQL
INSERT OR REPLACE INTO files (file_path, file_type, git_repo, commit_hash, file_size, content_hash)
VALUES ('$full_path', 'git', '$repo_name', '$commit', $size, '$hash');
SQL
        fi
    done
}

# Function: Index nix store
index_nix_store() {
    echo "  📦 Indexing nix store..."
    
    # Use existing nix store metadata
    if [ -d /nix/store ]; then
        # Query nix store database
        nix-store --query --requisites /nix/var/nix/profiles/system 2>/dev/null | \
        while read store_path; do
            if [ -d "$store_path" ]; then
                find "$store_path" -type f 2>/dev/null | while read file; do
                    local size=$(stat -c%s "$file" 2>/dev/null || echo 0)
                    
                    sqlite3 "$INDEX_DB" <<SQL
INSERT OR REPLACE INTO files (file_path, file_type, git_repo, file_size)
VALUES ('$file', 'nix', 'nix-store', $size);
SQL
                done
            fi
        done
    fi
}

# Function: Scan for repos
scan_repos() {
    local base_dir="$1"
    
    echo "🔍 Scanning for git repos in $base_dir..."
    
    # Find all .git directories (repos)
    find "$base_dir" -name ".git" -type d 2>/dev/null | while read git_dir; do
        local repo_path=$(dirname "$git_dir")
        index_git_repo "$repo_path"
    done
}

# Main indexing
echo "📊 Phase 1: Index known git repositories"
scan_repos "/mnt/data1/meta-introspector"
scan_repos "/mnt/data1/nix"
scan_repos "/mnt/data1/time2"

echo ""
echo "📊 Phase 2: Index nix store (if available)"
# index_nix_store  # Commented out - can be slow

echo ""
echo "📊 Phase 3: Classify remaining files"

# Mark temp/wip files
sqlite3 "$INDEX_DB" <<EOF
-- Mark temp files
UPDATE files SET file_type = 'temp' 
WHERE file_path LIKE '%/tmp/%' 
   OR file_path LIKE '%/.cache/%'
   OR file_path LIKE '%/target/%'
   OR file_path LIKE '%/node_modules/%';

-- Mark WIP files
UPDATE files SET file_type = 'wip'
WHERE file_path LIKE '%/wip/%'
   OR file_path LIKE '%/draft/%'
   OR file_path LIKE '%/scratch/%';
EOF

echo ""
echo "📊 Statistics:"
sqlite3 "$INDEX_DB" <<EOF
.mode column
.headers on
SELECT file_type, COUNT(*) as count, 
       ROUND(SUM(file_size)/1024.0/1024.0, 2) as size_mb
FROM files 
GROUP BY file_type;
EOF

echo ""
echo "✅ Index built: $INDEX_DB"
echo "📁 Total files indexed:"
sqlite3 "$INDEX_DB" "SELECT COUNT(*) FROM files;"
