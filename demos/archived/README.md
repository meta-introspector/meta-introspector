# Archived Demos

These files are **archived** and not part of the production codebase.

## Status: ARCHIVED

All files in this directory violate the Demo2Code policy:
- Contains demo/mock/fake patterns
- Uses hardcoded test data
- Incomplete implementations
- Educational/exploratory code only

## Do Not Use

These files are kept for historical reference only. They are:
- ❌ Not compiled in production builds
- ❌ Not tested
- ❌ Not maintained
- ❌ Not supported

## Migration Path

To use concepts from these demos:
1. Extract the core idea
2. Implement with real data sources
3. Add proper error handling
4. Write integration tests
5. Place in appropriate module

## Files

Total archived demos: 48

Categories:
- **Compression**: demo_compression_*.rs
- **Content**: demo_content_*.rs
- **Network**: demo_p2p_*.rs, demo_shared_memory.rs
- **Analysis**: demo_scan_*.rs, demo_perf_*.rs
- **Proof**: demo_proof_*.rs
- **Compilation**: demo_self_compilation.rs, demo_rustc_*.rs
- **Other**: Various exploratory demos

## Why Archived

Per Demo2Code policy:
- No more demos in production code
- All code must be production-ready
- Real data sources required
- Complete error handling required

See: [DEMO2CODE_POLICY.md](../../DEMO2CODE_POLICY.md)
