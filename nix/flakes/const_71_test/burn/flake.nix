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
              "burn-0.21.0" = "sha256-mw1fY+7iBgB9JDWR3bGQ3+Ht6lqo3h6WDjqP+Uiusm0=";
              "cubecl-0.9.0" = "sha256-Eg6mIYs9C5PR52xW35LEvUKkn6Dv/6rU2iURvi3ce5c=";
              "cubek-0.1.0" = "sha256-BNg1XXx4EHaIbkAcqFGmuOuwNS2tIUkA1dkVn7AvjmE=";
            };
          };

          nativeBuildInputs = with pkgs; [ pkg-config ];
          
          doCheck = false;
          
          installPhase = ''
            runHook preInstall
            mkdir -p $out/bin
            install -Dm755 target/*/release/burn-71 $out/bin/burn-71
            $out/bin/burn-71 > $out/output.txt
            if [ "$(cat $out/output.txt)" != "71" ]; then
              echo "Expected 71, got $(cat $out/output.txt)"
              exit 1
            fi
            runHook postInstall
          '';
        };
      }
    );
}
