# GitHub Activity Collection - Complete Documentation

## Overview

Complete GitHub activity collection system storing events verbatim in canonical format.

## Data Structure

```
data/
├── activity/
│   └── github/
│       └── {user}/
│           └── {year}/
│               └── {month}/
│                   ├── events.json      # GitHub API events (verbatim)
│                   └── activity.json    # Local git commits
├── github-activity/
│   ├── raw/
│   │   ├── {user}_all_events.json      # Complete event dump
│   │   └── {org}_org_events.json       # Org events
│   ├── by-month/
│   │   └── YYYY-MM.json                # Events grouped by month
│   └── contributions/
│       └── YYYY-MM.json                # GraphQL contributions data
└── my-activity/
    ├── {user}_{year}_{month}_activity.json  # Personal activity extracts
    └── github_{user}_recent_events.json     # Recent events summary
```

## Tools

### 1. github-events-collector.rs
**Purpose**: Fetch and store GitHub events in canonical format

**Usage**:
```bash
cargo run --bin github-events-collector -- <user> <start-year> <start-month> <end-year> <end-month>
```

**Example**:
```bash
cargo run --bin github-events-collector -- jmikedupont2 2024 1 2026 1
```

**Output**:
- Raw: `data/github-activity/raw/{user}_all_events.json`
- Canonical: `data/activity/github/{user}/{year}/{month}/events.json`

### 2. fetch-month-contributions.sh
**Purpose**: Fetch GitHub contributions using GraphQL API

**Usage**:
```bash
./fetch-month-contributions.sh <user> <year> <month>
```

**Example**:
```bash
./fetch-month-contributions.sh jmikedupont2 2024 1
```

**Output**: JSON with contribution counts and per-day breakdown

### 3. gh api (direct)
**Purpose**: Quick event fetching

**Examples**:
```bash
# User events
gh api /users/jmikedupont2/events --paginate

# Org events
gh api /orgs/meta-introspector/events --paginate
```

## Data Collected

### For @jmikedupont2

**Time Range**: 2020-11 to 2026-01 (21 months with activity)

**Total Events**: 270

**Event Types**:
- PushEvent: 150 (commits)
- WatchEvent: 45 (stars)
- ForkEvent: 36 (forks)
- CreateEvent: 31 (new repos/branches)
- IssueCommentEvent: 4
- IssuesEvent: 3
- PullRequestEvent: 1

**Monthly Breakdown**:
- 2020-11: 1 month
- 2023: 3 months (Jul, Aug, Sep)
- 2024: 7 months (Mar, Jul, Aug, Sep, Nov, Dec)
- 2025: 9 months (Jan-Sep)
- 2026: 1 month (Jan)

### For meta-introspector org

**Total Events**: 272

**Storage**: `data/github-activity/raw/meta-introspector_org_events.json`

## API Limitations

### REST Events API
- **Limit**: ~300 most recent events
- **Time window**: Last 30-90 days typically
- **Cannot retrieve**: Arbitrary historical months beyond this window

### GraphQL Contributions API
- **Limit**: No event limit, but only contribution counts
- **Time window**: Any date range
- **Data**: Aggregated counts, not individual events

## Canonical Storage Benefits

1. **Unified Format**: GitHub events + local git commits in same structure
2. **Verbatim Preservation**: No data loss, exact API responses
3. **Easy Querying**: Month-based organization
4. **Reproducible**: Can regenerate from raw files

## Next Steps

1. ✅ Collect GitHub events (done)
2. ✅ Store in canonical format (done)
3. ⏳ Check if all repos mentioned are registered
4. ⏳ Check if repos are cloned locally
5. ⏳ Fill gaps with local git history
6. ⏳ Convert to Parquet for efficient querying

## Files

- `github-events-collector.rs` - Main collector tool
- `fetch-month-contributions.sh` - GraphQL contributions fetcher
- `github-comprehensive-activity.rs` - Octocrab-based collector (WIP)
- `GITHUB_ACTIVITY_COLLECTION.md` - This file
