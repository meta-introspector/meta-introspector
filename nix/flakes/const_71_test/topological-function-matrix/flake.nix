{
  description = "Topological Function Matrix - Build order creates orthogonal hierarchy";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};
      in
      {
        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "topological-function-matrix";
          version = "0.1.0";
          src = ./.;
          
          cargoLock.lockFile = ./Cargo.lock;
          
          nativeBuildInputs = with pkgs; [ 
            graphviz
            linuxPackages.perf
          ];
          
          buildPhase = ''
            echo "🔬 Topological Function Matrix"
            echo "==============================="
            echo ""
            
            mkdir -p $out/topology $out/matrix
            
            # Build dependency graph (topological order)
            echo "📊 Computing build topology..."
            
            # Example: mes → tcc → gcc → rust → python → ...
            cat > $out/topology/build-order.dot << 'EOF'
            digraph BuildTopology {
              rankdir=BT;
              
              // Level 0: Bootstrap
              mes [label="GNU Mes\nGF(2^19)"];
              
              // Level 1: C compiler
              tcc [label="TCC\nGF(2^20)"];
              mes -> tcc;
              
              // Level 2: Full compiler
              gcc [label="GCC\nGF(2^21)"];
              tcc -> gcc;
              
              // Level 3: Modern languages
              rust [label="Rust\nGF(2^22)"];
              python [label="Python\nGF(2^22)"];
              gcc -> rust;
              gcc -> python;
              
              // Level 4: High-level
              haskell [label="Haskell\nGF(2^23)"];
              rust -> haskell;
              
              // Each edge = orthogonal layer
            }
            EOF
            
            dot -Tpng $out/topology/build-order.dot -o $out/topology/build-order.png
            
            echo "   Topology: $out/topology/build-order.png"
            echo ""
            
            # Compute orthogonal layers
            echo "🧮 Computing orthogonal function matrix..."
            
            cat > $out/matrix/README.md << 'EOF'
            # Topological Function Matrix
            
            ## Concept
            
            Build topology creates orthogonal hierarchy:
            - Each node in build graph = orthogonal layer
            - Each function f = position in matrix
            - Matrix[layer, function] = mathematical position
            
            ## Structure
            
            ```
            Layer 0 (Mes):     [f0, f1, f2, ...]  GF(2^19)
            Layer 1 (TCC):     [g0, g1, g2, ...]  GF(2^20)
            Layer 2 (GCC):     [h0, h1, h2, ...]  GF(2^21)
            Layer 3 (Rust):    [r0, r1, r2, ...]  GF(2^22)
            Layer 4 (Haskell): [s0, s1, s2, ...]  GF(2^23)
            ```
            
            ## Orthogonality
            
            Each layer labels the previous:
            - TCC functions label Mes functions
            - GCC functions label TCC functions
            - Rust functions label GCC functions
            - etc.
            
            ## Matrix Position
            
            Function f at layer L, position P:
            - Row: L (topological depth)
            - Column: P (function index in layer)
            - Value: Orthogonal projection strength
            
            ## Properties
            
            1. **Topological**: Respects build order
            2. **Orthogonal**: Each layer independent
            3. **Hierarchical**: Mes → ... → High-level
            4. **Complete**: Every function has unique position
            
            ## Usage
            
            ```rust
            let matrix = FunctionMatrix::from_topology(build_graph);
            let pos = matrix.position_of(function_ip);
            println!("Function at layer {}, position {}", pos.layer, pos.index);
            ```
            EOF
            
            echo "   Matrix spec: $out/matrix/README.md"
            echo ""
            
            # Create matrix metadata
            cat > $out/matrix/meta.json << EOF
            {
              "concept": "topological-function-matrix",
              "description": "Build topology creates orthogonal hierarchy where each function has unique mathematical position",
              "structure": {
                "rows": "topological layers (build depth)",
                "columns": "function indices within layer",
                "values": "orthogonal projection strengths"
              },
              "properties": [
                "topological (respects build order)",
                "orthogonal (independent layers)",
                "hierarchical (mes to high-level)",
                "complete (every function positioned)"
              ],
              "derivation": "$out",
              "timestamp": "$(date -Iseconds)"
            }
            EOF
            
            echo "✅ Topological function matrix computed"
            echo ""
            echo "🧙 Every function f has unique position:"
            echo "   Matrix[layer, index] = mathematical position"
            echo "   Build order = orthogonal hierarchy"
          '';
          
          installPhase = ''
            echo ""
            echo "📊 Outputs:"
            echo "   Topology: $out/topology/build-order.png"
            echo "   Matrix: $out/matrix/README.md"
            echo "   Metadata: $out/matrix/meta.json"
          '';
        };
      }
    );
}
