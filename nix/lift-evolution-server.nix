{ pkgs ? import <nixpkgs> {} }:

let
  # Read the lifting prompt
  promptFile = ../data/prompts/evolution_server_lift.json;
  prompt = builtins.readFile promptFile;
  
in pkgs.stdenv.mkDerivation {
  name = "evolution-server-rust";
  
  src = ./.;
  
  buildInputs = with pkgs; [
    nodejs
    jq
  ];
  
  buildPhase = ''
    mkdir -p $out
    
    # Call Gemini with the lifting prompt
    ${pkgs.nodejs}/bin/node ~/nix/vendor/external/gemini-cli/bundle/gemini.js \
      -p "$(cat ${promptFile} | ${pkgs.jq}/bin/jq -r .prompt)" \
      --output-format json \
      --model gemini-2.5-flash \
      > $out/response.json
    
    # Extract Rust code
    ${pkgs.jq}/bin/jq -r '.rust_code // .code // .content' $out/response.json > $out/evolution_server.rs
    
    # Extract proof
    ${pkgs.jq}/bin/jq -r '.equivalence_proof // .proof' $out/response.json > $out/proof.md
    
    echo "✅ Generated Rust code: $out/evolution_server.rs"
    echo "✅ Generated proof: $out/proof.md"
  '';
  
  installPhase = ''
    echo "Rust code generated at: $out/evolution_server.rs"
  '';
}
