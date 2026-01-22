{
  description = "Burn tensor framework test outputting 71";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "burn-71";
          version = "0.1.0";
          src = ./.;
          
          cargoLock = {
            lockFile = ./Cargo.lock;
            outputHashes = {
              "burn-0.21.0" = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
              "cubecl-0.9.0" = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
            };
          };

          nativeBuildInputs = with pkgs; [ pkg-config ];
          
          doCheck = false;
          
          installPhase = ''
            mkdir -p $out/bin
            cp target/release/burn-71 $out/bin/
            $out/bin/burn-71 > $out/output.txt
            if [ "$(cat $out/output.txt)" != "71" ]; then
              echo "Expected 71, got $(cat $out/output.txt)"
              exit 1
            fi
          '';
        };
      }
    );
}
