{
  description = "Pure CUDA const x = 71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      config.allowUnfree = true;
    };
  in {
    packages.${system}.default = pkgs.stdenv.mkDerivation {
      name = "cuda-71";
      src = pkgs.writeText "const71.cu" ''
        #include <stdio.h>
        
        __global__ void const71_kernel(int* result) {
          *result = 71;
        }
        
        int main() {
          int* d_result;
          int h_result;
          
          cudaMalloc(&d_result, sizeof(int));
          const71_kernel<<<1,1>>>(d_result);
          cudaMemcpy(&h_result, d_result, sizeof(int), cudaMemcpyDeviceToHost);
          cudaFree(d_result);
          
          printf("%d\n", h_result);
          return 0;
        }
      '';
      
      nativeBuildInputs = [ 
        pkgs.cudaPackages.cudatoolkit 
        pkgs.cudaPackages.cuda_nvcc
      ];
      
      dontUnpack = true;
      
      buildPhase = ''
        ${pkgs.cudaPackages.cuda_nvcc}/bin/nvcc $src -o const71
        # Can't run GPU code in nix build, just verify compilation
      '';
      
      installPhase = ''
        mkdir -p $out/bin
        cp const71 $out/bin/cuda-71
      '';
    };
  };
}
