# Unified System Architecture

## Use Existing zos-server P2PLibWrapper

**Location**: `~/zos-server/p2p_lib_wrapper.rs`

Already implements:
- `libloading::Library` for .so loading
- Verb-based API (LoadLib, ListSymbols, InvokeSymbol)
- P2P peer ID
- Results storage

## Package meta-introspector as .so

### Cargo.toml
```toml
[lib]
name = "meta_introspector"
crate-type = ["cdylib", "rlib"]
```

### Exports
```rust
#[no_mangle]
pub extern "C" fn git_temporal_morphisms() -> *const u8 { ... }

#[no_mangle]
pub extern "C" fn byte_provenance_track() -> *const u8 { ... }

#[no_mangle]
pub extern "C" fn bootstrap_arrow_chain() -> *const u8 { ... }
```

## Load via zos-server

```rust
let mut wrapper = P2PLibWrapper::new();
wrapper.execute_verb(LibVerb::LoadLib("/path/to/libmeta_introspector.so"))?;
wrapper.execute_verb(LibVerb::InvokeSymbol("git_temporal_morphisms"))?;
```

## Security Contexts

**Minimal** (always loaded):
- libssl.so
- libcurl.so
- libgit2.so

**On-demand** (load when needed):
- libmeta_introspector.so (our code)
- libp2p.so (only for P2P operations)

## Next Steps

1. Add `crate-type = ["cdylib"]` to meta-introspector/Cargo.toml
2. Add `#[no_mangle] pub extern "C"` exports
3. Build: `cargo build --release --lib`
4. Load via zos-server's P2PLibWrapper
5. Delete duplicate server code

## Don't Build

- ❌ New .so loader (use P2PLibWrapper)
- ❌ New server (use zos-server)
- ❌ New architecture (unify existing)
