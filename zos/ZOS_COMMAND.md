# ZOS Command System

Zero Operating System - Unified command interface for managing all repos.

## Commands

### `zos cargo audit`

Audits all repos in intelligent order:

1. **Discovery**: Finds all repos from:
   - Current repo (meta-introspector)
   - Git submodules
   - master_url_list.txt (13,686 repos)

2. **Dependency Analysis**: Builds dependency graph from Cargo.toml

3. **Topological Sort**: Orders repos by:
   - Dependencies (build deps first)
   - Priority (current repo > submodules > mirrored repos)

4. **Execution**: Runs cargo build on each repo

5. **Reporting**: Generates ZOS_AUDIT_SUMMARY.md with:
   - Success/failure counts
   - Failed build details
   - Quarantine recommendations

## Usage

```bash
# Build zos command
cargo build --release --bin zos

# Run audit
./target/release/zos cargo audit

# Check results
cat ZOS_AUDIT_SUMMARY.md
```

## Architecture

```
zos cargo audit
  ↓
Discover 13,699 repos
  ↓
Build dependency graph
  ↓
Topological sort + priority
  ↓
Audit each repo
  ↓
Generate summary
```

## Benefits

- **Smart ordering**: Dependencies built first
- **Priority-based**: Important repos first
- **Scalable**: Handles thousands of repos
- **Atomic**: Each audit is isolated
- **Traceable**: Full audit trail in summary
