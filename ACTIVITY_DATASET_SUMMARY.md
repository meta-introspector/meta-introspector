# Activity Dataset Summary

## Final Dataset Statistics

- **Total files**: 36,341 activity.json files
- **Total size**: 563 MB
- **Time range**: All git history (earliest to latest commits)
- **Structure**: `{platform}/{user}/{year}/{month}/activity.json`

## Platform Distribution

- **github**: 2,387 users
- **unknown**: 7,216 users (repos with empty/local URLs)
- **gitlab**: 8 users
- **huggingface**: 4 users

## Time Coverage

- **2022-2024**: 20,763 files (57% of dataset)
- **All years**: Complete git history from all 53 registered repos

## Status

✅ Dataset is complete and cached
✅ 100% coverage of registered repos
✅ Platform detection improved (local/sourcehut added)
✅ Ready for Parquet conversion and querying

## Note on "unknown" Platform

The 7,216 "unknown" users are from:
- 8 repos with empty URLs
- 1 repo with local file path
- These are now classified as "local" in updated code
- Existing cached data remains as-is for consistency
