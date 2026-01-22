# Complete Git URL Discovery & Index Expansion Flow

## Step 1: Extract URLs from Existing Packs (No Checkout)

```bash
cargo build --release --bin extract_urls_from_packs
./target/release/extract_urls_from_packs
```

**Output**: `data/extracted_git_urls.parquet`

**What it does**:
- Reads all repos from git-sources registry
- Uses `git cat-file` to read source files from packs
- Extracts github/gitlab/bitbucket URLs with regex
- No checkout needed - reads directly from packs
- Saves unique URLs to Parquet

## Step 2: Feed URLs to GitHub Mirror

```bash
# Start mirror service
./target/release/github_mirror_service &

# Feed extracted URLs
cat data/extracted_git_urls.parquet | while read url; do
  curl -X POST http://localhost:9418/mirror -d "url=$url"
done
```

**What it does**:
- Mirror service clones as bare repos
- Deduplicates objects
- Caches locally
- Tracks in telemetry

## Step 3: Update File Index

```bash
# Start file index service
./target/release/file_index_service &

# Scan newly mirrored repos
./target/release/git-sources scan /mnt/data1/github-mirror/
```

**What it does**:
- Registers new repos in git-sources
- Updates file index with new files
- Maintains git provenance
- Updates Parquet indexes

## Step 4: Recursive Expansion

```bash
# Extract URLs from newly mirrored repos
./target/release/extract_urls_from_packs

# Repeat steps 2-3 until no new URLs
```

## Complete Pipeline

```
Existing Repos
    ↓ (git cat-file - no checkout)
Extract URLs from packs
    ↓ (save to Parquet)
data/extracted_git_urls.parquet
    ↓ (feed to mirror)
GitHub Mirror Service
    ↓ (clone as bare repos)
/mnt/data1/github-mirror/
    ↓ (register)
git-sources registry
    ↓ (index)
File Index Service
    ↓ (save)
indexes/files.parquet (expanded!)
    ↓ (repeat)
Extract more URLs...
```

## Benefits

1. **No Checkout** - Read from packs only
2. **Fast** - git cat-file is instant
3. **Cached** - Mirror service deduplicates
4. **Recursive** - Discovers transitive dependencies
5. **Tracked** - All in Parquet for analysis

## Query Expanded Index

```sql
-- Find all discovered repos
SELECT DISTINCT url 
FROM 'data/extracted_git_urls.parquet';

-- Count files per repo
SELECT git_repo, COUNT(*) 
FROM 'indexes/files.parquet'
GROUP BY git_repo;

-- Find transitive dependencies
WITH RECURSIVE deps AS (
  SELECT url FROM 'data/extracted_git_urls.parquet'
  WHERE url LIKE '%meta-introspector%'
  UNION
  SELECT e.url FROM 'data/extracted_git_urls.parquet' e
  JOIN deps d ON e.source_repo = d.url
)
SELECT * FROM deps;
```

## Result

Expanded index with:
- All direct repos (Layer 1: .git/config)
- All submodules (Layer 2: .gitmodules)
- All referenced repos (Layer 3: URLs in source)
- All transitive dependencies (recursive)

All without ever running `find` or checking out files!
