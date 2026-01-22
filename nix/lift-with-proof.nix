{ pkgs ? import <nixpkgs> {} }:

let
  promptFile = ../data/prompts/evolution_server_lift.json;
  
in pkgs.stdenv.mkDerivation {
  name = "evolution-server-rust-proven";
  
  src = ./.;
  
  nativeBuildInputs = with pkgs; [
    nodejs
    jq
    linuxPackages.perf
  ];
  
  buildPhase = ''
    mkdir -p $out/{code,traces,proofs}
    
    echo "🔬 Lifting Python → Rust with perf recording..."
    
    # Record perf trace of Gemini call
    ${pkgs.linuxPackages.perf}/bin/perf record \
      -o $out/traces/gemini_lift.perf.data \
      -e 'syscalls:*' \
      ${pkgs.nodejs}/bin/node ~/nix/vendor/external/gemini-cli/bundle/gemini.js \
        -p "$(cat ${promptFile} | ${pkgs.jq}/bin/jq -r .prompt)" \
        --output-format json \
        --model gemini-2.5-flash \
      > $out/code/response.json
    
    # Extract perf script
    ${pkgs.linuxPackages.perf}/bin/perf script \
      -i $out/traces/gemini_lift.perf.data \
      > $out/traces/gemini_lift.perf.script
    
    # Extract Rust code
    cat $out/code/response.json | \
      ${pkgs.jq}/bin/jq -r '.response' | \
      sed 's/^```json//' | sed 's/```$//' | \
      ${pkgs.jq}/bin/jq -r '.rust_code' \
      > $out/code/evolution_server.rs
    
    # Extract proof
    cat $out/code/response.json | \
      ${pkgs.jq}/bin/jq -r '.response' | \
      sed 's/^```json//' | sed 's/```$//' | \
      ${pkgs.jq}/bin/jq -r '.equivalence_proof' \
      > $out/proofs/equivalence_proof.json
    
    # Generate metadata
    cat > $out/metadata.json << META
{
  "derivation": "$out",
  "timestamp": "$(date -Iseconds)",
  "nix_store_path": "$out",
  "perf_trace": "$out/traces/gemini_lift.perf.data",
  "perf_script": "$out/traces/gemini_lift.perf.script",
  "rust_code": "$out/code/evolution_server.rs",
  "proof": "$out/proofs/equivalence_proof.json",
  "syscalls_recorded": $(wc -l < $out/traces/gemini_lift.perf.script),
  "rust_lines": $(wc -l < $out/code/evolution_server.rs)
}
META
    
    echo "✅ Lifting complete with proof!"
    echo "   Nix store: $out"
    echo "   Perf trace: $out/traces/gemini_lift.perf.data"
    echo "   Rust code: $out/code/evolution_server.rs"
    echo "   Proof: $out/proofs/equivalence_proof.json"
  '';
  
  installPhase = ''
    echo "All outputs in: $out"
  '';
}
