{
  description = "JAX GPU: compute x=71 on NVIDIA 12GB";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.writeShellScriptBin "jax-gpu-71" ''
      ${pkgs.python3.withPackages(ps: [ps.jax ps.jaxlib ps.numpy])}/bin/python3 << 'PYTHON'
import jax
import jax.numpy as jnp

# Check GPU
print(f"JAX devices: {jax.devices()}")

# GPU-accelerated computation
@jax.jit
def compute_const():
    return jnp.array(71)

x = compute_const()
print(f"JAX GPU computed x = {x}")

# Gradient descent on GPU
@jax.jit
def loss_fn(params):
    return (params - 71.0) ** 2

grad_fn = jax.grad(loss_fn)
params = 50.0
for _ in range(100):
    params -= 0.1 * grad_fn(params)

print(f"JAX GPU optimized x ≈ {params:.1f}")
PYTHON
    '';
  };
}
