# GPU 71 Plan: 71 Ways to Write 71 on GPU

## Goal
Write "const x = 71" targeting NVIDIA RTX 3080 Ti in 71 different ways

## GPU Targets

### 1. CUDA (Pure)
- `const_71_test/cuda/` - Pure CUDA C++
- Direct kernel: `__global__ void const71() { int x = 71; }`

### 2. IREE Compiler
- `const_71_test/llvm/iree/` - IREE targeting CUDA
- Compile ML model to GPU
- Use t1 example as template

### 3. Mojo GPU
- `const_71_test/python/mojo/` - Mojo with GPU support
- Python-like syntax, GPU execution

### 4. PyTorch CUDA
- `const_71_test/pytorch/` - PyTorch tensor on GPU
- `torch.tensor([71]).cuda()`

### 5. JAX GPU
- `const_71_test/jax_gpu/` - Already exists!
- JAX JIT compilation to GPU

### 6. TensorFlow GPU
- `const_71_test/tensorflow/` - Already exists!
- TF tensor on GPU

### 7. OpenCL
- `const_71_test/opencl/` - Cross-platform GPU
- Kernel: `__kernel void const71(__global int* x) { *x = 71; }`

### 8. Vulkan Compute
- `const_71_test/vulkan/` - Vulkan compute shaders
- SPIR-V shader outputting 71

### 9. Metal (if we add macOS)
- `const_71_test/metal/` - Apple GPU
- Metal shader language

### 10. SYCL
- `const_71_test/sycl/` - C++ GPU abstraction
- OneAPI SYCL kernel

## Implementation Steps

### Phase 1: Pure CUDA ✅ Next
```bash
cd const_71_test
mkdir cuda
cat > cuda/flake.nix << 'EOF'
{
  description = "Pure CUDA const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "cuda-71";
      src = pkgs.writeText "const71.cu" ''
        #include <stdio.h>
        __global__ void const71() {
          int x = 71;
          printf("%d\n", x);
        }
        int main() {
          const71<<<1,1>>>();
          cudaDeviceSynchronize();
          return 0;
        }
      '';
      nativeBuildInputs = [ pkgs.cudaPackages.cudatoolkit ];
      buildPhase = ''
        nvcc $src -o const71
        ./const71 > output.txt
        grep -q "71" output.txt || exit 1
      '';
      installPhase = ''
        mkdir -p $out/bin
        cp const71 $out/bin/
      '';
    };
  };
}
EOF
```

### Phase 2: IREE Compiler
```bash
cd const_71_test/llvm/iree
# Update t1 example to output 71
# Compile with IREE targeting CUDA backend
```

### Phase 3: Update Existing GPU Tests
- pytorch/ - Add .cuda() call
- tensorflow/ - Add GPU placement
- jax_gpu/ - Verify GPU execution

### Phase 4: Add New GPU Languages
- opencl/
- vulkan/
- sycl/

## Verification

Each test must:
1. ✅ Output "71" to stdout
2. ✅ Execute on RTX 3080 Ti
3. ✅ Build with nix
4. ✅ Record perf data

## GPU Perf Analysis

After building all GPU versions:
```bash
# Record GPU execution
for gpu_test in cuda iree mojo pytorch jax tensorflow opencl vulkan sycl; do
  nix build ./const_71_test/$gpu_test
  nvidia-smi --query-gpu=utilization.gpu --format=csv -l 1 &
  NSYS_PID=$!
  ./result/bin/*-71
  kill $NSYS_PID
done

# Compare GPU vs CPU
# - Latency
# - Memory usage
# - Power consumption
# - Instruction diversity
```

## Expected Results

- **10 GPU implementations** of "const x = 71"
- **Perf traces** for each
- **Instruction fingerprints** on GPU
- **Convergence proof**: All GPU paths → same CUDA PTX

## Next Actions

1. ✅ Build t1 IREE example
2. ✅ Create pure CUDA version
3. ✅ Update IREE to latest
4. ✅ Target RTX 3080 Ti
5. ✅ Add remaining GPU languages
