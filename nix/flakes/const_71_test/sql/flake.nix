{
  description = "SQL: query x=71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.writeShellScriptBin "sql-71" ''
      ${pkgs.sqlite}/bin/sqlite3 :memory: << 'SQL'
CREATE TABLE constants (name TEXT, value INTEGER);
INSERT INTO constants VALUES ('x', 71);
SELECT value FROM constants WHERE name = 'x';
SQL
    '';
  };
}
