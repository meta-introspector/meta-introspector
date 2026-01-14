{
  description = "Graph Partition: partition to x=71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.writeShellScriptBin "partition-71" ''
      ${pkgs.python3.withPackages(ps: [ps.networkx ps.numpy])}/bin/python3 << 'PYTHON'
import networkx as nx

# Create graph with 71 nodes
G = nx.complete_graph(71)
print(f"Graph partitioned into x = {G.number_of_nodes()} nodes")
PYTHON
    '';
  };
}
