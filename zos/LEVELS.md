# ZOS 7-Level Security Architecture

Hierarchical security model where each level can only communicate downward.

## The 7 Levels

```
Level 6: User Interface (LLM, CLI)
   ↓ (can only talk down)
Level 5: Application Code
   ↓
Level 4: Language Runtime (Rust, LLVM)
   ↓
Level 3: Build System (Nix)
   ↓
Level 2: System Services (DNS, Proxies)
   ↓
Level 1: Hypervisor/SELinux
   ↓
Level 0: Hardware/Kernel
```

## Level Definitions

### Level 0: Hardware/Kernel
- **Domain**: `zos_level0_t`
- **Components**: CPU, Memory, Kernel
- **Cannot talk to**: Anyone (lowest level)

### Level 1: Hypervisor/SELinux
- **Domain**: `zos_level1_t`
- **Components**: SELinux, cgroups, namespaces
- **Can talk to**: Level 0 only
- **Enforces**: All security policies

### Level 2: System Services
- **Domain**: `zos_level2_t`
- **Components**: DNS, File proxy, Git proxy, Nix proxy, LLM proxy
- **Can talk to**: Level 1 only
- **Provides**: Network isolation, logging

### Level 3: Build System
- **Domain**: `zos_level3_t`
- **Components**: Nix, derivations, store
- **Can talk to**: Level 2 only
- **Enforces**: Reproducible builds

### Level 4: Language Runtime
- **Domain**: `zos_level4_t`
- **Components**: Rust compiler, LLVM, cargo
- **Can talk to**: Level 3 only
- **Provides**: Type safety, memory safety

### Level 5: Application Code
- **Domain**: `zos_level5_t`
- **Components**: User programs, binaries
- **Can talk to**: Level 4 only
- **Runs**: Actual workloads

### Level 6: User Interface
- **Domain**: `zos_level6_t`
- **Components**: CLI, LLM interface, APIs
- **Can talk to**: Level 5 only
- **Provides**: Human interaction

## Security Properties

### Downward-Only Communication
```
Level N can ONLY communicate with Level N-1
```

### No Upward Flow
```
neverallow zos_level0_t zos_level1_t:process *;
```
Lower levels **cannot** influence higher levels.

### No Level Skipping
```
neverallow zos_level3_t zos_level1_t:process *;
```
Must go through intermediate levels.

## Example: Cargo Build

```
User (L6) → cargo (L5) → rustc (L4) → nix (L3) → proxy (L2) → SELinux (L1) → kernel (L0)
```

Each step enforced by SELinux at kernel level.

## Running at Specific Level

```bash
# Run at Level 6 (user interface)
runcon -t zos_level6_t ./my-cli

# Run at Level 5 (application)
runcon -t zos_level5_t ./my-app

# Run at Level 3 (build system)
runcon -t zos_level3_t nix build
```

## Advantages

1. **Information Flow Control**: Data can only flow down
2. **Privilege Separation**: Each level isolated
3. **Kernel Enforced**: Cannot be bypassed
4. **Audit Trail**: All cross-level communication logged
5. **Provable Security**: Formal model

## Comparison to Bell-LaPadula

| Property | Bell-LaPadula | ZOS Levels |
|----------|---------------|------------|
| Read Down | ✅ | ✅ |
| Write Up | ❌ | ❌ |
| Enforcement | Policy | Kernel |
| Levels | Classified | Architectural |

This creates a **mathematically provable** security model where compromise of any level cannot affect lower levels.
