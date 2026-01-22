{
  description = "SPARQL: query x=71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.writeShellScriptBin "sparql-71" ''
      ${pkgs.python3.withPackages(ps: [ps.rdflib])}/bin/python3 << 'PYTHON'
from rdflib import Graph, Literal, Namespace, URIRef
from rdflib.namespace import RDF, RDFS

g = Graph()
ns = Namespace("http://example.org/")

g.add((ns.x, RDF.type, ns.Constant))
g.add((ns.x, ns.value, Literal(71)))

query = """
SELECT ?value WHERE {
  ?x <http://example.org/value> ?value .
}
"""

for row in g.query(query):
    print(f"SPARQL result: x = {row.value}")
PYTHON
    '';
  };
}
