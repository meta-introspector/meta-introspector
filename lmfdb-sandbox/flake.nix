{
  description = "LMFDB server sandbox for data collection";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };
  
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system: let
      pkgs = nixpkgs.legacyPackages.${system};
      
      lmfdb = pkgs.python3Packages.buildPythonApplication {
        pname = "lmfdb";
        version = "2024.01";
        
        src = pkgs.fetchFromGitHub {
          owner = "LMFDB";
          repo = "lmfdb";
          rev = "main";
          hash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
        };
        
        propagatedBuildInputs = with pkgs.python3Packages; [
          flask
          pymongo
          psycopg2
          pyyaml
          sage
        ];
        
        doCheck = false;
      };
      
      lmfdb-data-collector = pkgs.writeShellScriptBin "lmfdb-collect" ''
        #!/bin/bash
        set -e
        
        DATA_DIR="$PWD/data/lmfdb-collected"
        mkdir -p "$DATA_DIR"
        
        echo "🔍 LMFDB Data Collector"
        echo "Output: $DATA_DIR"
        echo ""
        
        # Elliptic curves (conductor <= 10000)
        echo "📊 Collecting elliptic curves..."
        ${pkgs.curl}/bin/curl -s \
          'https://www.lmfdb.org/api/ec_curvedata/?conductor={$lte:10000}&_format=json' \
          > "$DATA_DIR/elliptic_curves.json"
        echo "  ✓ $(${pkgs.jq}/bin/jq '.data | length' "$DATA_DIR/elliptic_curves.json") curves"
        
        # Modular forms (level <= 100)
        echo "📊 Collecting modular forms..."
        ${pkgs.curl}/bin/curl -s \
          'https://www.lmfdb.org/api/mf_newforms/?level={$lte:100}&_format=json' \
          > "$DATA_DIR/modular_forms.json"
        echo "  ✓ $(${pkgs.jq}/bin/jq '.data | length' "$DATA_DIR/modular_forms.json") forms"
        
        # L-functions (degree <= 4)
        echo "📊 Collecting L-functions..."
        ${pkgs.curl}/bin/curl -s \
          'https://www.lmfdb.org/api/lfunc_instances/?degree={$lte:4}&_format=json' \
          > "$DATA_DIR/lfunctions.json"
        echo "  ✓ $(${pkgs.jq}/bin/jq '.data | length' "$DATA_DIR/lfunctions.json") L-functions"
        
        # Number fields (degree <= 6)
        echo "📊 Collecting number fields..."
        ${pkgs.curl}/bin/curl -s \
          'https://www.lmfdb.org/api/nf_fields/?degree={$lte:6}&_format=json' \
          > "$DATA_DIR/number_fields.json"
        echo "  ✓ $(${pkgs.jq}/bin/jq '.data | length' "$DATA_DIR/number_fields.json") fields"
        
        echo ""
        echo "✅ Collection complete!"
        ls -lh "$DATA_DIR"
      '';
      
      lmfdb-local-server = pkgs.writeShellScriptBin "lmfdb-serve" ''
        #!/bin/bash
        set -e
        
        echo "🚀 Starting LMFDB local server..."
        echo "Port: 37777"
        echo "Data: $PWD/data/lmfdb-collected"
        echo ""
        
        # Start MongoDB for local data
        ${pkgs.mongodb}/bin/mongod \
          --dbpath "$PWD/data/lmfdb-db" \
          --port 37778 \
          --bind_ip 127.0.0.1 &
        MONGO_PID=$!
        
        # Wait for MongoDB
        sleep 2
        
        # Start LMFDB server
        cd ${lmfdb}
        LMFDB_MONGO_URI="mongodb://127.0.0.1:37778" \
        ${pkgs.python3}/bin/python start-lmfdb.py --port 37777 &
        LMFDB_PID=$!
        
        echo "✅ LMFDB running at http://localhost:37777"
        echo "MongoDB PID: $MONGO_PID"
        echo "LMFDB PID: $LMFDB_PID"
        echo ""
        echo "Press Ctrl+C to stop"
        
        trap "kill $MONGO_PID $LMFDB_PID" EXIT
        wait
      '';
      
    in {
      packages = {
        default = lmfdb-data-collector;
        collector = lmfdb-data-collector;
        server = lmfdb-local-server;
      };
      
      apps = {
        default = {
          type = "app";
          program = "${lmfdb-data-collector}/bin/lmfdb-collect";
        };
        collect = {
          type = "app";
          program = "${lmfdb-data-collector}/bin/lmfdb-collect";
        };
        serve = {
          type = "app";
          program = "${lmfdb-local-server}/bin/lmfdb-serve";
        };
      };
    });
}
