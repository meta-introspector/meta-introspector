# Nix Store Security Levels

## The Split

```
/nix/store
  ├── /public      (pure functions, no proof needed)
  ├── /verified    (cryptographic proofs required)
  ├── /trusted     (signed by known parties)
  └── /private     (encrypted, access controlled)
```

## Level 0: Public (Pure Functions)

**No proof needed, only consensus**

```
/nix/store/public/abc123-pure-add
/nix/store/public/def456-pure-sha256
/nix/store/public/ghi789-pure-parser
```

Properties:
- Pure functions only
- Self-verifying
- Anyone can use
- Consensus on value
- Published to IPFS

## Level 1: Verified (Cryptographic Proofs)

**ZK proofs of correct execution**

```
/nix/store/verified/abc123-compiled-binary
  ├── binary
  ├── proof.zk
  └── trace.json
```

Properties:
- Has ZK proof
- Execution verified
- Reproducible build
- Godel number matches

## Level 2: Trusted (Signed)

**Signed by known parties**

```
/nix/store/trusted/abc123-nixpkgs-hello
  ├── binary
  ├── signature.gpg
  └── builder: nixpkgs-maintainers
```

Properties:
- GPG signed
- Known builder
- Trust chain
- Audit trail

## Level 3: Private (Encrypted)

**Access controlled**

```
/nix/store/private/abc123-user-secrets
  ├── data.enc
  ├── acl.json
  └── owner: user@example.com
```

Properties:
- Encrypted
- Access control
- Private keys
- User data

## Classification Algorithm

```rust
fn classify_store_path(path: &str) -> SecurityLevel {
    let binary = read_binary(path);
    
    // Check if pure function
    if is_pure_function(&binary) {
        return SecurityLevel::Public;
    }
    
    // Check for ZK proof
    if has_zk_proof(path) {
        return SecurityLevel::Verified;
    }
    
    // Check for signature
    if has_valid_signature(path) {
        return SecurityLevel::Trusted;
    }
    
    // Default to private
    SecurityLevel::Private
}
```

## Migration

```bash
# Scan existing nix store
for path in /nix/store/*; do
    level=$(classify $path)
    
    case $level in
        public)
            # Publish to IPFS
            ipfs add $path
            ln -s $path /nix/store/public/
            ;;
        verified)
            # Generate ZK proof
            generate_proof $path
            mv $path /nix/store/verified/
            ;;
        trusted)
            # Verify signature
            verify_signature $path
            mv $path /nix/store/trusted/
            ;;
        private)
            # Encrypt if needed
            mv $path /nix/store/private/
            ;;
    esac
done
```

## Access Control

```rust
fn can_access(user: &User, path: &str) -> bool {
    let level = get_security_level(path);
    
    match level {
        SecurityLevel::Public => true,  // Anyone
        SecurityLevel::Verified => verify_proof(path),  // If proof valid
        SecurityLevel::Trusted => check_trust_chain(user, path),  // If trusted
        SecurityLevel::Private => check_acl(user, path),  // If authorized
    }
}
```

## Replication

```rust
// Public: Replicate everywhere
replicate_to_ipfs("/nix/store/public/*");

// Verified: Replicate with proofs
replicate_with_proofs("/nix/store/verified/*");

// Trusted: Replicate to trusted nodes
replicate_to_trusted_nodes("/nix/store/trusted/*");

// Private: No replication (or encrypted)
// stays local or encrypted backup
```

## Benefits

### Public Level
- Maximum distribution
- No barriers
- Self-verifying
- Consensus-based value

### Verified Level
- Cryptographic guarantees
- Reproducible
- Trustless verification

### Trusted Level
- Known provenance
- Audit trail
- Social trust

### Private Level
- User privacy
- Access control
- Secrets management

## Example: rustc

```
/nix/store/public/abc123-rustc-pure-parser
  └── Pure parser functions extracted from rustc

/nix/store/verified/def456-rustc-1.75.0
  ├── rustc binary
  ├── proof.zk (proves correct compilation)
  └── trace.json

/nix/store/trusted/ghi789-rustc-official
  ├── rustc binary
  ├── signature.gpg (signed by Rust Foundation)
  └── builder: rust-lang/rust

/nix/store/private/jkl012-my-rustc-fork
  ├── rustc binary (my modifications)
  └── owner: me
```

## Integration with Consensus

```rust
// Public level uses consensus
let consensus = get_consensus("pure-add");
let path = consensus.most_voted_implementation;
// /nix/store/public/abc123-pure-add

// Verified level uses proofs
let verified = get_verified("rustc-1.75.0");
assert!(verify_proof(verified.proof));

// Trusted level uses signatures
let trusted = get_trusted("nixpkgs-hello");
assert!(verify_signature(trusted.signature));

// Private level uses ACL
let private = get_private("my-secrets");
assert!(check_permission(current_user, private));
```

## The Vision

Split nix store by security model:
- **Public**: Pure functions, consensus-based, IPFS
- **Verified**: ZK proofs, trustless
- **Trusted**: Signatures, social trust
- **Private**: Encrypted, access controlled

Each level has appropriate guarantees and distribution model.
