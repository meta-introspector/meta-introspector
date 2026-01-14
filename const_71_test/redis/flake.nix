{
  description = "Redis: cache x=71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.writeShellScriptBin "redis-71" ''
      ${pkgs.python3.withPackages(ps: [ps.redis])}/bin/python3 << 'PYTHON'
# Simulated Redis operation (no server needed)
cache = {"x": 71}
print(f"Redis GET x: {cache['x']}")
PYTHON
    '';
  };
}
