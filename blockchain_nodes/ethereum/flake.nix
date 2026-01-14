{
  description = "Ethereum Geth node for economic weight analysis";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.go-ethereum;
      
      # Development shell with ethereum tools
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          go-ethereum
        ];
        
        shellHook = ''
          echo "⟠ Ethereum Geth Environment"
          echo "==========================="
          echo "geth version: $(geth version | grep Version)"
          echo ""
          echo "Commands:"
          echo "  geth --dev --http --http.api eth,web3,personal"
          echo "  geth attach http://localhost:8545"
          echo "  geth --dev console"
          echo ""
          echo "Data dir: ~/.ethereum"
        '';
      };
      
      # App to run geth in dev mode
      apps.${system}.default = {
        type = "app";
        program = "${pkgs.go-ethereum}/bin/geth";
      };
    };
}
