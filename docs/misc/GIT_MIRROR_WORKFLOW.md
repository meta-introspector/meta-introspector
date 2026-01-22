# Git Mirror Workflow Documentation

## Overview
Maintain a canonical git mirror at `/mnt/data1/git/` organized by domain/org/repo structure, with local git config redirecting all fetches to use local repos first.

## Phase 1: Discovery - Find All Git Repos

### Initial Scan
```bash
# Full system scan (slow, run once or daily)
find /mnt/data1 /home/mdupont -name ".git" -type d 2>/dev/null \
  | grep -v -E "(nt41|hf_|Trash)" \
  > gitdirs.txt

# Result: gitdirs.txt with ~15K repos
```

### Incremental Scans
```bash
# Hourly: scan only recent changes
find /mnt/data1 /home/mdupont -name ".git" -type d -mtime -0.042 2>/dev/null \
  | grep -v -E "(nt41|hf_|Trash)" \
  > gitdirs-$(date +%Y%m%d-%H).txt

# Merge with master list
cat gitdirs-*.txt | sort -u > gitdirs.txt
```

### Exclusions (`.gitmirror-ignore`)
- `**/nt41*` - Private projects
- `**/hf_*` - HuggingFace tokens in paths
- `**/.local/share/Trash/` - Deleted files

## Phase 2: Analysis - Group by Dependencies

### Analyze Cargo.toml Dependencies
```bash
cargo run --release --bin analyze_cargo_deps
# Output: cargo_deps_groups.json
# Result: 950 unique dependency sets from 1,912 Rust repos
```

### Analyze Workspaces
```bash
cargo run --release --bin analyze_workspaces
# Output: workspaces.json, updated cargo_deps_groups.json
# Result: 917 workspaces, 6,117 unique dependency sets
```

### Build Dependency Graph
```bash
cargo run --release --bin build_dep_graph
# Output: dep_graph.json
# Result: 9,292 nodes, 280 edges, topological order
```

## Phase 3: Mirror Population - Create Canonical Structure

### Link Existing Repos to Mirror
```bash
cargo run --release --bin link_existing_repos
# Reads: gitdirs.txt
# Creates: symlinks at /mnt/data1/git/github.com/org/repo -> actual location
# Handles duplicates: /mnt/data1/git/github.com/org/repo/links/2, /links/3, etc.
```

### Mirror Structure
```
/mnt/data1/git/
├── github.com/
│   ├── org/
│   │   └── repo/          # Primary symlink to first found location
│   │       └── links/     # Duplicates
│   │           ├── 2 -> /other/location
│   │           └── 3 -> /another/location
├── gitlab.com/
├── huggingface.co/
└── ...
```

## Phase 4: Git Configuration - Use Local Mirror

### Configure Git URL Rewriting
```bash
# For all discovered hosts
for host in /mnt/data1/git/*/; do
  hostname=$(basename "$host")
  git config --global url."/mnt/data1/git/$hostname/".insteadOf "https://$hostname/"
  git config --global url."/mnt/data1/git/$hostname/".insteadOf "git://$hostname/"
  git config --global url."/mnt/data1/git/$hostname/".insteadOf "git@$hostname:"
done
```

### Allow File Protocol
```bash
git config --global protocol.file.allow always
```

## Phase 5: Clone Missing Repos

### Fill Gaps in Mirror
```bash
# For repos in queue but not in mirror
./fill_mirror.sh
# Uses: --depth=1 for new clones (shallow)
# Skips: repos already in mirror
```

## Phase 6: Nix Integration

### Configure Nix to Use Local Git
```bash
./configure_nix_all_hosts.sh
# Adds git config for all 47+ discovered hosts
```

### Build with Local Dependencies
```bash
# Nix will now use file:// paths via git config
nix build
nix flake update  # Uses local mirror
```

## Maintenance Schedule

### Hourly
- Incremental git repo discovery
- Update gitdirs.txt with new repos
- Re-run link_existing_repos for new discoveries

### Daily
- Full system scan for git repos
- Regenerate cargo_deps_groups.json
- Update dep_graph.json
- Clean up broken symlinks

### Weekly
- Audit for sensitive data (HF tokens, private keys)
- Verify mirror integrity
- Update documentation

## Key Files

| File | Purpose | Update Frequency |
|------|---------|------------------|
| `gitdirs.txt` | All discovered git repos | Hourly |
| `cargo_deps_groups.json` | Rust dependency grouping | Daily |
| `workspaces.json` | Workspace member tracking | Daily |
| `dep_graph.json` | Dependency graph | Daily |
| `.gitmirror-ignore` | Exclusion patterns | As needed |
| `data/queue_all.txt` | Repos to clone | As needed |
| `data/master_url_list.txt` | All discovered URLs | Daily |

## Statistics (Current)

- **Total repos discovered**: 15,244
- **Repos in mirror**: ~1,000 real + symlinks
- **Mirror size**: 51GB
- **Rust repos**: 1,912
- **Unique dependency sets**: 6,117
- **Git hosts**: 47+
- **URLs to clone**: 13,054

## Tools

| Tool | Purpose |
|------|---------|
| `link_existing_repos` | Create mirror symlinks |
| `analyze_cargo_deps` | Group Rust deps |
| `analyze_workspaces` | Parse workspace members |
| `build_dep_graph` | Build dependency graph |
| `fill_mirror.sh` | Clone missing repos |
| `configure_git_global.sh` | Setup git URL rewriting |
| `configure_nix_all_hosts.sh` | Setup Nix git config |
