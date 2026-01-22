{
  description = "Burn CUDA tensor framework test outputting 71";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          config.allowUnfree = true;
        };
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "burn-cuda-71";
          version = "0.1.0";
          src = ./.;
          
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "burn-0.21.0" = "sha256-mw1fY+7iBgB9JDWR3bGQ3+Ht6lqo3h6WDjqP+Uiusm0=";
              "cubecl-0.9.0" = "sha256-Eg6mIYs9C5PR52xW35LEvUKkn6Dv/6rU2iURvi3ce5c=";
              "cubek-0.1.0" = "sha256-BNg1XXx4EHaIbkAcqFGmuOuwNS2tIUkA1dkVn7AvjmE=";
            };
          };

          nativeBuildInputs = with pkgs; [ 
            pkg-config 
            cudaPackages.cuda_nvcc
            makeWrapper
          ];
          
          buildInputs = with pkgs; [
            cudaPackages.cuda_cudart
            cudaPackages.libcublas
            linuxPackages.nvidia_x11
          ];
          
          CUDA_PATH = "${pkgs.cudaPackages.cuda_cudart}";
          CUDA_INCLUDE_PATH = "${pkgs.cudaPackages.cuda_cudart}/include";
          
          doCheck = false;
          
          installPhase = ''
            runHook preInstall
            mkdir -p $out/bin
            install -Dm755 target/*/release/burn-cuda-71 $out/bin/burn-cuda-71
            
            # Wrap with nvidia driver in LD_LIBRARY_PATH
            wrapProgram $out/bin/burn-cuda-71 \
              --prefix LD_LIBRARY_PATH : "${pkgs.lib.makeLibraryPath [ pkgs.linuxPackages.nvidia_x11 ]}"
            
            runHook postInstall
          '';
        };
      }
    );
}
