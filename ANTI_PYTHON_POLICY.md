# Anti-Python Policy

**Effective**: 2026-01-18  
**Status**: Active

## Policy

All Python code must be converted to Rust for:
- Performance (10-100x faster)
- Memory safety (no runtime errors)
- Type safety (compile-time guarantees)
- Nix integration (better caching)
- Binary distribution (.so loading)

## Migration Strategy

1. **Identify Python files** in codebase
2. **Convert to Rust** with equivalent functionality
3. **Build as .so** for dynamic loading
4. **Load via ZOS server** (existing infrastructure)
5. **Delete Python** after verification

## Rust Advantages

- Compiles to native code
- No interpreter overhead
- Better error messages
- Cargo ecosystem
- Nix-friendly builds

## Exceptions

None. All Python must be converted.

## Implementation

Use existing ZOS server .so loading mechanism instead of Python scripts.
