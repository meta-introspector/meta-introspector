{
  description = "Mes-Transformer: Computational Omniscience Architecture";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    mes-bootstrap.url = "path:../mes-bootstrap-proof";
  };
  
  outputs = { self, nixpkgs, mes-bootstrap }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      config = {
        allowUnfree = true;
        cudaSupport = true;
      };
    };
    
    libtorch = pkgs.libtorch-bin.override { cudaSupport = true; };
    
  in {
    packages.${system} = {
      # Tiny Transformer with GPU support
      default = pkgs.rustPlatform.buildRustPackage {
        pname = "mes-transformer";
        version = "0.1.0";
        src = ./rust;
        cargoLock.lockFile = ./rust/Cargo.lock;
        
        nativeBuildInputs = [ pkgs.pkg-config ];
        buildInputs = [ 
          pkgs.openssl 
          pkgs.zstd
          libtorch
        ];
        
        LIBTORCH = "${libtorch}";
        LIBTORCH_LIB = "${libtorch}/lib";
        LIBTORCH_INCLUDE = "${libtorch.dev}/include";
        LIBTORCH_CXX11_ABI = "1";
        
        buildFeatures = [ "gpu" ];
      };
    };
    
    devShells.${system}.default = pkgs.mkShell {
      packages = [
        pkgs.rustc
        pkgs.cargo
        libtorch
      ];
      
      shellHook = ''
        export LIBTORCH="${libtorch}"
        export LIBTORCH_LIB="${libtorch}/lib"
        export LIBTORCH_INCLUDE="${libtorch.dev}/include"
        export LD_LIBRARY_PATH="${libtorch}/lib:$LD_LIBRARY_PATH"
        echo "🎮 LibTorch with CUDA ready"
        echo "   LibTorch: ${libtorch}"
      '';
    };
  };
}
