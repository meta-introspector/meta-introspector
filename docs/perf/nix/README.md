# Perf + Nix Integration

Same as `docs/nix/perf/` but from perf perspective.

## Core Concept

Use Nix builds as reproducible perf data sources.

## Why Nix for Perf?

1. **Reproducible**: Same derivation = same perf data
2. **Immutable**: Perf data stored in `/nix/store/`
3. **Versioned**: Each build = unique hash
4. **Distributed**: Multiple nodes, merge results

## Workflow

```
                              ↓
                         Reproducible training data
```

## Tools

### perf-wrapper

See: `perf-wrapper/README.md`

### perf-recorder
Record perf data from Nix builds.

See: `perf-recorder/README.md`

## Data Location

All perf data in nix store:
```
/nix/store/xxx-package/perf/build.perf.data
/nix/store/yyy-mes/perf/mes-bootstrap.perf.data
/nix/store/zzz-rust/perf/rust-build.perf.data
```

## Analysis

### perf-complexity
Label instruction data using nix store perf:

```bash
perf-complexity \
  --basis-system /nix/store/xxx-mes/perf/build.perf.data \
  --target-system /nix/store/yyy-rust/perf/build.perf.data
```

### Meta-Perf
Self-referential analysis:

```bash
nix build .#meta-perf
ls result/convergence/level*.ips
```

## References

- Nix Integration: `docs/nix/perf/README.md`
- Perf Tools: `docs/perf/README.md`
