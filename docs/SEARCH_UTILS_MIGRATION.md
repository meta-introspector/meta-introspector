# Search Utils Migration Guide

## Overview

All `find` and `grep` operations are now centralized in `lib/search_utils.sh`.

## Usage

```bash
source lib/search_utils.sh
```

## Function Reference

### File Finding

| Function | Usage | Old Pattern |
|----------|-------|-------------|
| `find_rust_files [dir]` | Find all .rs files | `find . -name "*.rs" -type f` |
| `find_by_ext <dir> <ext>` | Find by extension | `find $dir -name "*.$ext"` |
| `find_flakes [dir]` | Find flake.nix files | `find . -name "flake.nix"` |
| `find_workspaces [dir]` | Find Cargo workspaces | `find . -name "Cargo.toml" -exec grep -l "\[workspace\]" {} \;` |
| `find_git_repos [dir]` | Find git repositories | `find . -name ".git" -type d` |
| `find_recent <dir> [days]` | Files modified in N days | `find $dir -type f -mtime -7` |
| `find_large <dir> [size]` | Find large files | `find $dir -type f -size +100M` |
| `find_empty [dir]` | Find empty files | `find . -type f -empty` |
| `find_multi_ext <dir> <ext1> <ext2>...` | Multiple extensions | `find . -name "*.rs" -o -name "*.toml"` |

### Grep Operations

| Function | Usage | Old Pattern |
|----------|-------|-------------|
| `grep_context <pattern> <file> [lines]` | Grep with context | `grep -A 5 "$pattern" "$file"` |
| `grep_errors <file>` | Extract error codes | `grep -E "^error\[" $file \| sort \| uniq -c` |
| `grep_count <pattern> <file>` | Count matches | `grep -c "$pattern" "$file"` |
| `grep_lines <pattern> <file>` | Grep with line numbers | `grep -n "$pattern" "$file"` |
| `grep_field <pattern> <file> [field]` | Extract field from match | `grep "$pattern" $file \| awk '{print $2}'` |
| `grep_multi <file> <pat1> <pat2>...` | Multiple patterns (OR) | `grep -E "pat1\|pat2" $file` |
| `grep_log_pattern <log> <pattern>` | Extract log pattern | `grep "$pattern" $log \| head -1 \| cut -c1-80` |

### Combined Operations

| Function | Usage | Old Pattern |
|----------|-------|-------------|
| `find_grep <dir> <pattern> [ext]` | Find files with pattern | `find . -name "*.rs" -exec grep -l "$pattern" {} \;` |
| `find_so_loads <log>` | Find .so loads in strace | `grep "openat.*\\.so" $log \| sed...` |
| `find_execve <log>` | Find execve calls | `grep "execve(" $log \| sed...` |
| `find_count_pattern <dir> <pattern>` | Count files matching | `find $dir -name "$pattern" \| wc -l` |

## Migration Examples

### Before
```bash
find . -name "*.rs" -exec grep -l "OciClient\|oracle" {} \; 2>/dev/null | head -3
```

### After
```bash
source lib/search_utils.sh
find_grep . "OciClient\|oracle" rs | head -3
```

---

### Before
```bash
grep "error:" "$log" | head -1 | cut -c1-80
```

### After
```bash
source lib/search_utils.sh
grep_log_pattern "$log" "error:"
```

---

### Before
```bash
find "$dir" -name "*.rs" -o -name "*.toml" -o -name "*.nix" 2>/dev/null
```

### After
```bash
source lib/search_utils.sh
find_multi_ext "$dir" rs toml nix
```

## Scripts to Migrate

Priority order based on complexity:

1. ✅ `quick-find.sh` - Example migration below
2. `generate_error_matrix.sh` - Heavy grep usage
3. `prove_so_wrapping.sh` - Complex grep patterns
4. `strace_proof.sh` - Multiple grep operations
5. `analyze_repo_ownership.sh` - Mixed find/grep
6. `find-active-repos.sh` - Complex find logic
7. All other scripts in `*.sh`

## Example: quick-find.sh Migration

### Before
```bash
find "$proj" -name "*.rs" -exec grep -l "OciClient\|oracle\|oci_core" {} \; 2>/dev/null | head -3
find "$proj" -name "*.rs" -exec grep -l "axum::Router\|#\[tokio::main\]" {} \; 2>/dev/null | head -3
```

### After
```bash
source lib/search_utils.sh
find_grep "$proj" "OciClient\|oracle\|oci_core" rs | head -3
find_grep "$proj" "axum::Router\|#\[tokio::main\]" rs | head -3
```

## Benefits

- **Consistency**: Same patterns across all scripts
- **Maintainability**: Fix once, apply everywhere
- **Readability**: Semantic function names
- **Error handling**: Centralized 2>/dev/null
- **Testing**: Single place to test search logic
- **Performance**: Can optimize implementations once

## Testing

```bash
# Test the library
source lib/search_utils.sh

# Test find operations
find_rust_files . | head -5
find_flakes . | head -5

# Test grep operations
grep_errors build.log | head -5
grep_count "error:" build.log
```

## Next Steps

1. Source `lib/search_utils.sh` in scripts
2. Replace patterns with function calls
3. Test each script after migration
4. Remove redundant error handling (2>/dev/null)
5. Document any custom patterns not covered
