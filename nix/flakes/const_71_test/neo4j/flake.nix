{
  description = "Neo4j Cypher: graph query x=71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.writeShellScriptBin "neo4j-71" ''
      ${pkgs.python3.withPackages(ps: [ps.neo4j])}/bin/python3 << 'PYTHON'
# Simulated Neo4j Cypher query
cypher = "CREATE (x:Constant {value: 71}) RETURN x.value"
print(f"Cypher query: {cypher}")
print(f"Result: x = 71")
PYTHON
    '';
  };
}
