# Demo2Code Custom Clippy Lints

Custom lints to enforce the Demo2Code policy.

## Lints

### 1. `DEMO_CODE` (deny)
Detects banned patterns in code:
- Function/variable names containing: demo, mock, fake, stub, placeholder
- Hardcoded test values: 42, 123
- Suspicious strings: "test", "example", "placeholder"

### 2. `TRIVIAL_FUNCTION` (warn)
Detects functions with < 5 statements (excluding tests)

### 3. `MISSING_ERROR_HANDLING` (warn)
Detects `unwrap()` and `expect()` calls
Suggests using `?` operator instead

### 4. `EXCESSIVE_CONSTANTS` (warn)
Detects when > 10% of items are constants
Suggests moving to config file

## Usage

### Build the lint
```bash
cd demo2code-lint
cargo build --release
```

### Use with rustc
```bash
rustc -L ./demo2code-lint/target/release \
      -Z extra-plugins=demo2code_lint \
      your_file.rs
```

### Use with cargo
Add to `.cargo/config.toml`:
```toml
[build]
rustflags = [
  "-L", "demo2code-lint/target/release",
  "-Z", "extra-plugins=demo2code_lint"
]
```

Then:
```bash
cargo build
```

## Examples

### ❌ Violations

```rust
// DEMO_CODE: banned pattern
fn demo_function() {}
fn mock_data() {}
let fake_value = 42;

// TRIVIAL_FUNCTION: too simple
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// MISSING_ERROR_HANDLING: unwrap
let data = fs::read("file").unwrap();

// EXCESSIVE_CONSTANTS: too many
const A: i32 = 1;
const B: i32 = 2;
const C: i32 = 3;
// ... 20 more constants
```

### ✅ Compliant

```rust
// Real function names
fn analyze_complexity() -> Result<Analysis> {
    // Full implementation > 5 lines
    let trace = load_trace()?;
    let clusters = cluster_tests(&trace)?;
    let harmonics = compute_harmonics(&clusters)?;
    let classification = classify(&harmonics)?;
    Ok(classification)
}

// Proper error handling
let data = fs::read("file")?;

// Config instead of constants
let config = load_config()?;
```

## Integration

### With CI
```yaml
- name: Run custom lints
  run: |
    cd demo2code-lint && cargo build --release
    cargo rustc -- -L demo2code-lint/target/release -Z extra-plugins=demo2code_lint
```

### With pre-commit
```bash
#!/bin/bash
cargo rustc -- -L demo2code-lint/target/release -Z extra-plugins=demo2code_lint
if [ $? -ne 0 ]; then
    echo "❌ Demo2Code lint failed"
    exit 1
fi
```

## Customization

Edit `src/lib.rs` to add more lints:

```rust
declare_lint! {
    pub YOUR_LINT,
    Deny,
    "your lint description"
}

declare_lint_pass!(YourLint => [YOUR_LINT]);

impl EarlyLintPass for YourLint {
    fn check_item(&mut self, cx: &EarlyContext<'_>, item: &ast::Item) {
        // Your lint logic
    }
}
```

## Lint Levels

- `Deny`: Compilation fails
- `Warn`: Warning only
- `Allow`: Disabled

Change in `declare_lint!` macro.
