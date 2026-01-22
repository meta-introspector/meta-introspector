# CUDA 13 Support Request for nixpkgs

## Issue
burn-cuda test fails with forward compatibility error when using nixpkgs CUDA 12.x toolkit with system CUDA 13.0 driver.

## Environment
- **System**: Ubuntu 22.04
- **NVIDIA Driver**: 580.65.06 (CUDA 13.0)
- **nixpkgs CUDA**: 12.x (cudaPackages)
- **GPU**: RTX 3080 Ti

## Error
```
thread 'main' panicked at /build/cargo-vendor-dir/cubecl-cuda-0.9.0/src/runtime.rs:50:40:
called `Result::unwrap()` on an `Err` value: DriverError(CUDA_ERROR_COMPAT_NOT_SUPPORTED_ON_DEVICE, "forward compatibility was attempted on non supported HW")
```

## Reproduction
```bash
cd const_71_test/burn-cuda
NIXPKGS_ALLOW_UNFREE=1 nix run --impure
```

## Current Status
- ✅ Build succeeds with CUDA 12.x
- ✅ Binary runs in Nix environment (no glibc issues)
- ❌ GPU execution fails due to CUDA version mismatch

## Request
Add CUDA 13.x support to nixpkgs cudaPackages, similar to existing cudaPackages_11 and cudaPackages_12.

## Workaround
Currently using CPU-only burn backend for testing. GPU test requires CUDA 13 toolkit.

## References
- NVIDIA Driver: https://www.nvidia.com/Download/driverResults.aspx/232672/
- CUDA 13.0 Release: https://developer.nvidia.com/cuda-downloads
- nixpkgs CUDA: https://github.com/NixOS/nixpkgs/tree/master/pkgs/development/cuda-modules

## Test Code
Real burn-cuda tensor framework test:
- Repository: https://github.com/meta-introspector/meta-introspector
- Path: const_71_test/burn-cuda/
- Uses: burn 0.21.0, cubecl 0.9.0, cubek 0.1.0
