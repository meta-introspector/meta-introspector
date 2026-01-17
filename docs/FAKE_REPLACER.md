# Fake Data Replacer

Automatically replaces fake/test data with `panic!()` calls to force real implementations.

## What It Does

Finds and replaces:
- **Hardcoded integers**: 42, 123, 999, 1234
- **Suspicious strings**: "test", "example", "placeholder", "mock", "fake", "demo"

With:
```rust
panic!("FAKE DATA DETECTED: hardcoded value 42 - replace with real data source")
panic!("FAKE DATA DETECTED: suspicious string \"test\" - replace with real data source")
```

## Usage

```bash
# Build
cargo build --release --bin fake_replacer

# Replace in single file
./target/release/fake_replacer src/my_file.rs

# Replace in directory
./target/release/fake_replacer src/
```

## Example

### Before
```rust
fn get_data() -> i32 {
    42  // Fake test value
}

fn get_name() -> String {
    "test".to_string()  // Placeholder
}
```

### After
```rust
fn get_data() -> i32 {
    panic!("FAKE DATA DETECTED: hardcoded value 42 - replace with real data source")
}

fn get_name() -> String {
    panic!("FAKE DATA DETECTED: suspicious string \"test\" - replace with real data source")
}
```

## Why This Works

1. **Forces awareness**: Code panics immediately when fake data is accessed
2. **Clear message**: Tells developer exactly what's wrong
3. **Prevents merge**: Tests will fail if fake data remains
4. **Traceable**: Stack trace shows where fake data was used

## Integration

### Pre-commit Hook
```bash
#!/bin/bash
./target/release/fake_replacer src/
if git diff --quiet; then
    echo "✅ No fake data found"
else
    echo "⚠️  Fake data replaced with panic!() - review changes"
    git add -u
fi
```

### CI Check
```yaml
- name: Replace fake data
  run: |
    cargo build --release --bin fake_replacer
    ./target/release/fake_replacer src/
    
- name: Verify no panics
  run: |
    if grep -r "FAKE DATA DETECTED" src/; then
      echo "❌ Fake data still present"
      exit 1
    fi
```

## Safe Patterns

These won't be replaced:
```rust
const CONFIG_PORT: u16 = 8080;  // Not in fake list
let version = "1.0.0";          // Not suspicious
let count = 100;                // Not in fake list
```

## Detected Patterns

### Integers
- 42 (classic test value)
- 123 (sequential test)
- 999 (placeholder)
- 1234 (sequential test)

### Strings (case-insensitive)
- "test"
- "example"
- "placeholder"
- "mock"
- "fake"
- "demo"

## Workflow

1. **Run replacer**: `fake_replacer src/`
2. **Review changes**: Check what was replaced
3. **Fix panics**: Replace with real data sources
4. **Test**: Ensure no panics at runtime
5. **Commit**: Only real data remains

## Example Fix

```rust
// After replacement (panics)
fn get_data() -> i32 {
    panic!("FAKE DATA DETECTED: hardcoded value 42 - replace with real data source")
}

// Fixed (real data)
fn get_data() -> Result<i32> {
    let config = load_config()?;
    Ok(config.data_value)
}
```

## Benefits

- **Enforces Demo2Code policy**: No fake data survives
- **Clear errors**: Developers know exactly what to fix
- **Automated**: No manual review needed
- **Safe**: Only replaces known fake patterns

## Limitations

- May flag legitimate uses of 42, 123, etc.
- Requires manual review of replacements
- Doesn't detect all fake patterns
- Modifies source files (use version control!)

## Recovery

If replacer makes mistakes:
```bash
git checkout src/  # Revert changes
# Or review and selectively revert
git diff src/my_file.rs
git checkout src/my_file.rs
```
