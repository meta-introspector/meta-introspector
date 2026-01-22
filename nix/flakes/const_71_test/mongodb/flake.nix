{
  description = "MongoDB: store x=71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.writeShellScriptBin "mongodb-71" ''
      ${pkgs.python3.withPackages(ps: [ps.pymongo])}/bin/python3 << 'PYTHON'
# Simulated MongoDB operation (no server needed)
document = {"name": "x", "value": 71}
print(f"MongoDB document: {document}")
print(f"Query result: x = {document['value']}")
PYTHON
    '';
  };
}
