{
  description = "Mes-Transformer: Computational Omniscience Architecture";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/master";
    mes-bootstrap.url = "path:../mes-bootstrap-proof";
  };
  
  outputs = { self, nixpkgs, mes-bootstrap }: let
    system = "x86_64-linux";
    pkgs = import nixpkgs {
      inherit system;
      config.allowUnfree = true;
    };
    
    # Custom libtorch with everything in one output
    libtorch = pkgs.stdenv.mkDerivation {
      pname = "libtorch";
      version = "2.9.0";
      src = pkgs.fetchzip {
        name = "libtorch-shared-with-deps-2.9.0-cu130.zip";
        url = "https://download.pytorch.org/libtorch/cu130/libtorch-shared-with-deps-2.9.0%2Bcu130.zip";
        hash = "sha256-u8l7JIy2rdk6nxv6UxNmFcfOVcpjvZnIEr5CczVNRDQ=";
      };
      
      nativeBuildInputs = [ pkgs.patchelf ];
      
      installPhase = ''
        mkdir -p $out
        cp -r include $out/include
        cp -r share $out/share
        install -Dm755 -t $out/lib lib/*${pkgs.stdenv.hostPlatform.extensions.sharedLibrary}*
        
        # Fix cmake paths
        substituteInPlace $out/share/cmake/Torch/TorchConfig.cmake \
          --replace \''${TORCH_INSTALL_PREFIX}/lib "$out/lib"
        
        substituteInPlace $out/share/cmake/Caffe2/Caffe2Targets-release.cmake \
          --replace \''${_IMPORT_PREFIX}/lib "$out/lib"
      '';
    };
    
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
        LIBTORCH_CXX11_ABI = "1";
        
        buildFeatures = [ "gpu" ];
      };
    };
  };
}
