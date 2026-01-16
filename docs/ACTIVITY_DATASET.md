# Canonical Git Activity Dataset

Comprehensive git activity dataset organized by platform, user, and time.

## Structure

```
data/activity/
├── github/
│   ├── {user}/
│   │   ├── {year}/
│   │   │   ├── {month}/
│   │   │   │   └── activity.json
├── codeberg/
├── huggingface/
└── unknown/
```

## Dataset Statistics

- **Total activity files**: 36,341
- **Total size**: 563 MB
- **Repositories processed**: 53
- **Format**: JSON (ready for Parquet conversion)

## Data Schema

Each `activity.json` contains an array of commits:

```json
{
  "commit_hash": "abc123...",
  "author_name": "John Doe",
  "author_email": "john@example.com",
  "author_date": "2025-01-15T10:30:00Z",
  "committer_name": "John Doe",
  "committer_email": "john@example.com",
  "committer_date": "2025-01-15T10:30:00Z",
  "message": "Add feature X",
  "repo_name": "my-project",
  "repo_url": "https://github.com/user/my-project",
  "platform": "github",
  "files_changed": 5,
  "insertions": 120,
  "deletions": 45
}
```

## Platforms

- **github**: GitHub repositories
- **codeberg**: Codeberg repositories
- **huggingface**: HuggingFace datasets/models
- **unknown**: Local or custom git remotes

## Usage

### Build Dataset
```bash
cargo build --bin build-activity-dataset --release
./target/release/build-activity-dataset
```

### Query with DataFusion
```sql
-- Load all activity for a user in 2025
SELECT * FROM 'data/activity/github/mike.dupont/2025/*/*.json'

-- Count commits by month
SELECT 
  author_date::DATE as month,
  COUNT(*) as commits
FROM 'data/activity/github/mike.dupont/2025/*/*.json'
GROUP BY month
ORDER BY month
```

### Convert to Parquet
```bash
# TODO: Add parquet conversion tool
```

## Integration with git-sources

The dataset is built from all repos in the git-sources registry:
- 53 repos registered
- Activity extracted from all branches
- Includes commit stats (files, insertions, deletions)

## Next Steps

1. ✅ Extract activity from all registered repos
2. ✅ Organize by platform/user/year/month
3. ⏳ Convert JSON to Parquet
4. ⏳ Check which repos are in registry
5. ⏳ Upload to HuggingFace as dataset
6. ⏳ Create query interface

## Files

- `build-activity-dataset.rs` - Dataset builder
- `data/activity/` - Activity dataset (563 MB)
- `data/git-sources-registry.json` - Source repos
