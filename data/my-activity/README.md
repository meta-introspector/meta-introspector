# My Git Activity (2022-2025)

Personal git activity extracted from the canonical activity dataset.

## Statistics

- **Total commits**: 1,395
- **Time range**: 2022-2025 (last 3+ years)
- **Total size**: 816 KB
- **Files**: 16 monthly activity files

## Monthly Breakdown

- **2023**: 3 months (Jul, Aug, Sep) - 479 KB (peak activity in Aug)
- **2024**: 4 months (Feb, Mar, Apr, Dec) - 84 KB
- **2025**: 9 months (Jan-Aug, Nov, Dec) - 253 KB

## Peak Activity

- **2023-08**: 479 KB (largest month)
- **2024-12**: 84 KB
- **2025-01**: 41 KB
- **2025-02**: 32 KB

## Files

Each file contains commits for that month:
- Format: `{user}_{year}_{month}_activity.json`
- Schema: Same as canonical dataset (commit_hash, author, date, message, repo, stats)

## Usage

```bash
# Count commits per month
for f in *.json; do echo "$f: $(jq 'length' $f)"; done

# Get all commit messages
jq -r '.[].message' *.json

# Get repos worked on
jq -r '.[].repo_name' *.json | sort -u
```
