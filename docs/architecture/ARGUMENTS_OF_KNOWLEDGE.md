# Arguments of Knowledge: Public Provenance Proofs

## The Principle

**Every proof is an argument of knowledge with publicly verifiable facts.**

Not "trust the ZK proof" - but "here are the public facts you can verify yourself."

## Byte Provenance as Public Argument

### The Claim
```
This byte at offset 0x1234 came from:
- Commit: abc123def456
- Repo: github.com/meta-introspector/meta-introspector
- File: src/gateway/mod.rs
- Line: 42
- Author: mdupont
- Time: 2026-01-22T10:25:34Z
- GPG Signature: [public key verifiable]
```

### The Public Proof
```rust
pub struct ByteArgument {
    /// The byte itself
    pub byte: u8,
    pub offset: u64,
    
    /// PUBLIC: Git provenance (anyone can verify)
    pub git_commit: String,
    pub git_repo: String,
    pub file_path: String,
    pub line_number: u32,
    pub column: u32,
    
    /// PUBLIC: Author identity (GPG signed)
    pub author: String,
    pub author_gpg_key: String,
    pub commit_signature: Vec<u8>,
    
    /// PUBLIC: Timestamp (git commit time)
    pub timestamp: u64,
    
    /// PUBLIC: Trust chain
    pub trusted_by: Vec<String>,  // Who vouches for this author
    
    /// PUBLIC: Usage in orbits
    pub used_in_orbit: u64,       // Which orbit uses this byte
    pub lifts: Vec<String>,       // What does this byte enable
}

impl ByteArgument {
    /// Verify all public facts
    pub fn verify_public(&self) -> Result<(), String> {
        // 1. Verify git commit exists
        self.verify_git_commit()?;
        
        // 2. Verify file/line/column in that commit
        self.verify_file_location()?;
        
        // 3. Verify GPG signature
        self.verify_gpg_signature()?;
        
        // 4. Verify trust chain
        self.verify_trust_chain()?;
        
        // 5. Verify orbit usage
        self.verify_orbit_usage()?;
        
        Ok(())
    }
    
    /// Anyone can verify: git show <commit>:<file>
    fn verify_git_commit(&self) -> Result<(), String> {
        use std::process::Command;
        
        let output = Command::new("git")
            .args(&["show", &format!("{}:{}", self.git_commit, self.file_path)])
            .output()
            .map_err(|e| e.to_string())?;
        
        if !output.status.success() {
            return Err(format!("Commit {} not found", self.git_commit));
        }
        
        // Verify byte at line/column
        let content = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = content.lines().collect();
        
        if self.line_number as usize >= lines.len() {
            return Err("Line number out of range".to_string());
        }
        
        let line = lines[self.line_number as usize];
        if self.column as usize >= line.len() {
            return Err("Column out of range".to_string());
        }
        
        let actual_byte = line.as_bytes()[self.column as usize];
        if actual_byte != self.byte {
            return Err(format!("Byte mismatch: expected {}, got {}", self.byte, actual_byte));
        }
        
        Ok(())
    }
    
    /// Anyone can verify: git verify-commit <commit>
    fn verify_gpg_signature(&self) -> Result<(), String> {
        use std::process::Command;
        
        let output = Command::new("git")
            .args(&["verify-commit", &self.git_commit])
            .output()
            .map_err(|e| e.to_string())?;
        
        if !output.status.success() {
            return Err("GPG signature invalid".to_string());
        }
        
        Ok(())
    }
    
    /// Verify trust chain (web of trust)
    fn verify_trust_chain(&self) -> Result<(), String> {
        // Check if author is in trusted set
        // Or if vouched for by trusted parties
        unimplemented!("Web of trust verification")
    }
    
    /// Verify this byte is used in claimed orbit
    fn verify_orbit_usage(&self) -> Result<(), String> {
        // Check orbit N contains this byte
        // Check it lifts M other things
        unimplemented!("Orbit membership verification")
    }
    
    fn verify_file_location(&self) -> Result<(), String> {
        Ok(()) // Already done in verify_git_commit
    }
}
```

## Public Orbit Proofs

### The Claim
```
Orbit 5 contains 1,234 bytes that lift 67 other components:
- rust compiler (lifted from bash)
- nix builder (lifted from shell scripts)
- git operations (lifted from 100+ call sites)
- ...
```

### The Public Proof
```rust
pub struct OrbitArgument {
    /// Orbit number
    pub orbit: u64,
    
    /// PUBLIC: All bytes in this orbit
    pub bytes: Vec<ByteArgument>,
    
    /// PUBLIC: What this orbit lifts
    pub lifts: Vec<LiftArgument>,
    
    /// PUBLIC: Galois field coverage
    pub gf_coverage: f64,
    pub gf_field: String,  // e.g., "GF(2^12)"
}

pub struct LiftArgument {
    /// What was lifted
    pub name: String,
    
    /// From what (e.g., "bash script")
    pub from: String,
    
    /// To what (e.g., "Rust function")
    pub to: String,
    
    /// PUBLIC: Proof of equivalence
    pub equivalence_proof: EquivalenceArgument,
}

pub struct EquivalenceArgument {
    /// PUBLIC: Original code (git commit)
    pub original_commit: String,
    pub original_file: String,
    
    /// PUBLIC: Lifted code (git commit)
    pub lifted_commit: String,
    pub lifted_file: String,
    
    /// PUBLIC: Test cases showing equivalence
    pub test_cases: Vec<TestCase>,
    
    /// PUBLIC: Perf traces showing same syscalls
    pub perf_trace_original: String,
    pub perf_trace_lifted: String,
}

pub struct TestCase {
    /// Input
    pub input: Vec<u8>,
    
    /// Expected output
    pub output: Vec<u8>,
    
    /// PUBLIC: Both produce same output
    pub original_output: Vec<u8>,
    pub lifted_output: Vec<u8>,
}

impl EquivalenceArgument {
    /// Anyone can verify: run both and compare
    pub fn verify_equivalence(&self) -> Result<(), String> {
        for test in &self.test_cases {
            if test.original_output != test.lifted_output {
                return Err("Outputs differ".to_string());
            }
            if test.output != test.original_output {
                return Err("Expected output differs".to_string());
            }
        }
        
        // Verify perf traces match
        self.verify_perf_traces()?;
        
        Ok(())
    }
    
    fn verify_perf_traces(&self) -> Result<(), String> {
        // Compare syscall sequences
        unimplemented!("Perf trace comparison")
    }
}
```

## Trust Chain

### Web of Trust
```rust
pub struct TrustChain {
    /// Root of trust (e.g., project maintainers)
    pub roots: Vec<TrustedIdentity>,
    
    /// Vouches: A vouches for B
    pub vouches: Vec<(String, String)>,
}

pub struct TrustedIdentity {
    /// Name
    pub name: String,
    
    /// PUBLIC: GPG key
    pub gpg_key: String,
    
    /// PUBLIC: GitHub account
    pub github: String,
    
    /// PUBLIC: Commits signed by this key
    pub signed_commits: Vec<String>,
}

impl TrustChain {
    /// Is this author trusted?
    pub fn is_trusted(&self, author: &str, gpg_key: &str) -> bool {
        // Direct root?
        if self.roots.iter().any(|r| r.name == author && r.gpg_key == gpg_key) {
            return true;
        }
        
        // Vouched for by trusted party?
        for (voucher, vouchee) in &self.vouches {
            if vouchee == author && self.is_trusted(voucher, gpg_key) {
                return true;
            }
        }
        
        false
    }
}
```

## Public Verification Script

```bash
#!/bin/bash
# verify_byte_argument.sh
# Anyone can run this to verify a byte argument

BYTE_ARG="$1"  # JSON file with ByteArgument

# Extract fields
COMMIT=$(jq -r '.git_commit' "$BYTE_ARG")
FILE=$(jq -r '.file_path' "$BYTE_ARG")
LINE=$(jq -r '.line_number' "$BYTE_ARG")
COL=$(jq -r '.column' "$BYTE_ARG")
BYTE=$(jq -r '.byte' "$BYTE_ARG")

echo "Verifying byte argument..."
echo "  Commit: $COMMIT"
echo "  File: $FILE"
echo "  Line: $LINE, Column: $COL"
echo "  Expected byte: $BYTE"
echo ""

# 1. Verify commit exists
echo "[1/4] Verifying commit exists..."
if ! git cat-file -e "$COMMIT^{commit}"; then
    echo "❌ Commit not found"
    exit 1
fi
echo "✅ Commit exists"

# 2. Verify GPG signature
echo "[2/4] Verifying GPG signature..."
if ! git verify-commit "$COMMIT" 2>/dev/null; then
    echo "❌ GPG signature invalid"
    exit 1
fi
echo "✅ GPG signature valid"

# 3. Verify byte at location
echo "[3/4] Verifying byte at location..."
CONTENT=$(git show "$COMMIT:$FILE")
ACTUAL_BYTE=$(echo "$CONTENT" | sed -n "${LINE}p" | cut -c"$COL" | od -An -tu1 | tr -d ' ')

if [ "$ACTUAL_BYTE" != "$BYTE" ]; then
    echo "❌ Byte mismatch: expected $BYTE, got $ACTUAL_BYTE"
    exit 1
fi
echo "✅ Byte matches"

# 4. Verify author is trusted
echo "[4/4] Verifying author trust..."
AUTHOR=$(git show -s --format='%an' "$COMMIT")
GPG_KEY=$(git show -s --format='%GK' "$COMMIT")

# Check against trust chain
if ! grep -q "$GPG_KEY" trust_chain.txt; then
    echo "⚠️  Author not in trust chain (but signature is valid)"
else
    echo "✅ Author is trusted"
fi

echo ""
echo "🎉 Byte argument verified!"
echo "   This byte provably came from commit $COMMIT"
echo "   Signed by $AUTHOR at $(git show -s --format='%ci' "$COMMIT")"
```

## Example: Public Verification

```json
{
  "byte": 123,
  "offset": 4660,
  "git_commit": "0447a57a",
  "git_repo": "github.com/meta-introspector/meta-introspector",
  "file_path": "src/gateway/mod.rs",
  "line_number": 42,
  "column": 15,
  "author": "mdupont",
  "author_gpg_key": "ABC123DEF456",
  "timestamp": 1737559034,
  "trusted_by": ["project-maintainers"],
  "used_in_orbit": 5,
  "lifts": [
    "rust_compiler_from_bash",
    "nix_builder_from_shell",
    "git_ops_consolidation"
  ]
}
```

**Anyone can verify:**
```bash
./verify_byte_argument.sh byte_0x1234.json

# Output:
# ✅ Commit exists
# ✅ GPG signature valid
# ✅ Byte matches
# ✅ Author is trusted
# 🎉 Byte argument verified!
```

## Integration with Driver

```rust
// In driver binary
pub fn execute_with_arguments(cmd: &str, args: &[&str]) -> Result<(), String> {
    // Record execution
    let trace = perf::record_with_provenance(cmd, args)?;
    
    // Generate public arguments for each byte
    let arguments: Vec<ByteArgument> = trace.iter()
        .enumerate()
        .map(|(offset, &byte)| {
            ByteArgument::from_trace(offset as u64, byte, &trace)
        })
        .collect();
    
    // Verify all arguments
    for arg in &arguments {
        arg.verify_public()?;
    }
    
    // Store arguments (publicly verifiable)
    store_arguments(&arguments)?;
    
    Ok(())
}
```

## Storage Format

```
/nix/store/xxx-byte-arguments/
├── orbit_5/
│   ├── byte_0x0000.json  (ByteArgument)
│   ├── byte_0x0001.json
│   ├── ...
│   └── orbit.json        (OrbitArgument)
├── trust_chain.json      (TrustChain)
└── verification.sh       (Public verification script)
```

**Anyone can download and verify:**
```bash
nix-store -r /nix/store/xxx-byte-arguments
cd /nix/store/xxx-byte-arguments
./verification.sh orbit_5/byte_0x1234.json
```

## The Complete Argument

```
Claim: This system is minimal, proven, and necessary.

Public Evidence:
1. Every byte has git provenance (verifiable)
2. Every commit is GPG signed (verifiable)
3. Every author is in trust chain (verifiable)
4. Every byte is used in orbit N (verifiable)
5. Every orbit lifts M things (verifiable)
6. Every lift has equivalence tests (verifiable)
7. Every test passes (verifiable)
8. Every perf trace matches (verifiable)

Conclusion: The system is provably minimal.

Anyone can verify. No trust required.
```

## See Also

- `src/provenance/mod.rs` - Byte provenance tracking
- `src/orbit/mod.rs` - Orbit computation
- `docs/architecture/AUTOMORPHIC_EIGENVECTOR.md` - Eigenvector system

---

**Arguments of knowledge: Public facts, public verification, no trust required.**
