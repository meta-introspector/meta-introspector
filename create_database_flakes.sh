#!/usr/bin/env bash
# Create const x=71 flakes for databases and query protocols

set -euo pipefail

CONST_DIR="const_71_test"
mkdir -p "$CONST_DIR"/{sql,graphql,sparql,mongodb,redis,neo4j}

echo "🗄️  Creating Database and Query Protocol Flakes for const x=71"
echo "=============================================================="

# SQL (SQLite)
cat > "$CONST_DIR/sql/flake.nix" << 'EOF'
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
EOF

# GraphQL
cat > "$CONST_DIR/graphql/flake.nix" << 'EOF'
{
  description = "GraphQL: query x=71";
  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  outputs = { self, nixpkgs }: let
    system = "x86_64-linux";
    pkgs = nixpkgs.legacyPackages.${system};
  in {
    packages.${system}.default = pkgs.writeShellScriptBin "graphql-71" ''
      ${pkgs.nodejs}/bin/node << 'JS'
const { buildSchema, graphql } = require('graphql');

const schema = buildSchema(`
  type Query {
    x: Int
  }
`);

const root = {
  x: () => 71
};

const query = '{ x }';

graphql({ schema, source: query, rootValue: root }).then(response => {
  console.log('GraphQL result:', response.data.x);
});
JS
    '';
  };
}
EOF

# SPARQL
cat > "$CONST_DIR/sparql/flake.nix" << 'EOF'
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
EOF

# MongoDB
cat > "$CONST_DIR/mongodb/flake.nix" << 'EOF'
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
EOF

# Redis
cat > "$CONST_DIR/redis/flake.nix" << 'EOF'
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
EOF

# Neo4j (Cypher)
cat > "$CONST_DIR/neo4j/flake.nix" << 'EOF'
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
EOF

echo ""
echo "✅ Created 6 Database and Query Protocol flakes"
echo ""
echo "Databases (4):"
echo "  - SQL (SQLite)"
echo "  - MongoDB (document store)"
echo "  - Redis (key-value cache)"
echo "  - Neo4j (graph database with Cypher)"
echo ""
echo "Query Protocols (2):"
echo "  - GraphQL"
echo "  - SPARQL (RDF/semantic web)"
echo ""
echo "Total systems: 32"
echo ""
echo "Build:"
echo "  nix build ./const_71_test/sql#"
echo "  nix build ./const_71_test/graphql#"
echo "  nix build ./const_71_test/sparql#"
