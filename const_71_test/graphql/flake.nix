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
