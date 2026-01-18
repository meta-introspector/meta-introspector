{
  description = "DuckDB for querying Parquet build logs";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    duckdb-nix.url = "github:rupurt/duckdb-nix";
  };

  outputs = { self, nixpkgs, duckdb-nix }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      packages.${system}.default = pkgs.writeShellScriptBin "query-build-logs" ''
        ${duckdb-nix.packages.${system}.default}/bin/duckdb "$@"
      '';
      
      devShells.${system}.default = pkgs.mkShell {
        buildInputs = [
          duckdb-nix.packages.${system}.default
        ];
        
        shellHook = ''
          echo "🦆 DuckDB ready for Parquet queries"
          echo ""
          echo "Example queries:"
          echo "  duckdb -c \"SELECT * FROM 'nix_build_logs.parquet' LIMIT 5\""
          echo "  duckdb -c \"SELECT build_status, COUNT(*) FROM 'nix_build_logs.parquet' GROUP BY build_status\""
          echo "  duckdb -c \"SELECT project, exit_code FROM 'nix_build_logs.parquet' WHERE build_status='failed'\""
        '';
      };
    };
}
