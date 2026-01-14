{
  description = "Solana validator node for economic weight analysis";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in {
      packages.${system}.default = pkgs.solana-cli;
      
      # Development shell with solana tools
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = with pkgs; [
          solana-cli
          rustc
          cargo
        ];
        
        shellHook = ''
          echo "◎ Solana Environment"
          echo "===================="
          echo "solana version: $(solana --version)"
          echo ""
          echo "Commands:"
          echo "  solana-test-validator  # Start local validator"
          echo "  solana config set --url localhost"
          echo "  solana balance"
          echo ""
          echo "Data dir: ~/.config/solana"
        '';
      };
      
      # App to run test validator
      apps.${system}.default = {
        type = "app";
        program = "${pkgs.solana-cli}/bin/solana-test-validator";
      };
    };
}
