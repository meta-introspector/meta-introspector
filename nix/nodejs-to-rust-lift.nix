{
  description = "Node.js → Rust lifting via MES bootstrap tracing";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };
  
  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
      # Step 1: Compile Node.js with full tracing
      nodejsTraced = pkgs.stdenv.mkDerivation {
        name = "nodejs-traced";
        src = pkgs.nodejs_22.src;
        
        __impure = true;
        
        buildInputs = with pkgs; [
          python3
          gcc
          gnumake
          perf
        ];
        
        buildPhase = ''
          # Record Node.js compilation with perf
          perf record -o $out/nodejs_compile.perf.data \
            -e 'syscalls:*' \
            -e 'sched:*' \
            --call-graph dwarf \
            ./configure --prefix=$out
          
          perf record -o $out/nodejs_build.perf.data \
            -e 'syscalls:*' \
            make -j$(nproc)
          
          make install
          
          # Parse traces
          perf script -i $out/nodejs_compile.perf.data > $out/nodejs_compile.trace
          perf script -i $out/nodejs_build.perf.data > $out/nodejs_build.trace
          
          # Extract compilation curve
          cat > $out/compile_analysis.json << EOF
          {
            "compiler": "gcc",
            "syscalls": $(grep -c "syscalls:" $out/nodejs_compile.trace),
            "build_syscalls": $(grep -c "syscalls:" $out/nodejs_build.trace)
          }
          EOF
        '';
      };
      
      # Step 2: Trace Gemini CLI compilation with Node.js
      geminiTraced = pkgs.stdenv.mkDerivation {
        name = "gemini-cli-traced";
        
        __impure = true;
        
        buildInputs = [
          nodejsTraced
          pkgs.perf
        ];
        
        src = pkgs.fetchFromGitHub {
          owner = "meta-introspector";
          repo = "gemini-cli";
          rev = "feature/CRQ-016-nixify-2025-10-06";
          sha256 = ""; # Will be filled
        };
        
        buildPhase = ''
          # Trace npm install
          perf record -o $out/npm_install.perf.data \
            -e 'syscalls:*' \
            ${nodejsTraced}/bin/npm install
          
          # Trace bundling
          perf record -o $out/bundle.perf.data \
            -e 'syscalls:*' \
            ${nodejsTraced}/bin/npm run bundle
          
          # Parse traces
          perf script -i $out/npm_install.perf.data > $out/npm_install.trace
          perf script -i $out/bundle.perf.data > $out/bundle.trace
          
          cp bundle/gemini.js $out/
        '';
      };
      
      # Step 3: Trace Gemini execution
      geminiExecutionTrace = pkgs.writeScriptBin "trace-gemini-execution" ''
        #!/bin/bash
        
        TRACE_DIR="$1"
        mkdir -p "$TRACE_DIR"
        
        # Trace Gemini execution
        perf record -o "$TRACE_DIR/gemini_exec.perf.data" \
          -e 'syscalls:*' \
          -e 'sched:*' \
          --call-graph dwarf \
          ${nodejsTraced}/bin/node ${geminiTraced}/gemini.js \
          -p "Test prompt" \
          --output-format json
        
        # Parse trace
        perf script -i "$TRACE_DIR/gemini_exec.perf.data" > "$TRACE_DIR/gemini_exec.trace"
        
        # Extract syscall curve
        grep "syscalls:" "$TRACE_DIR/gemini_exec.trace" | \
          awk '{print $1, $4}' > "$TRACE_DIR/syscall_curve.txt"
        
        # Analyze
        cat > "$TRACE_DIR/execution_analysis.json" << EOF
        {
          "total_syscalls": $(grep -c "syscalls:" "$TRACE_DIR/gemini_exec.trace"),
          "node_syscalls": $(grep "node" "$TRACE_DIR/gemini_exec.trace" | grep -c "syscalls:"),
          "v8_syscalls": $(grep "v8" "$TRACE_DIR/gemini_exec.trace" | grep -c "syscalls:")
        }
        EOF
        
        echo "✅ Execution trace saved to $TRACE_DIR"
      '';
      
      # Step 4: MES + GCC + LLVM bootstrap tracing
      mesBootstrapTrace = pkgs.stdenv.mkDerivation {
        name = "mes-bootstrap-trace";
        
        __impure = true;
        
        buildInputs = with pkgs; [
          mes
          gcc
          llvm
          perf
        ];
        
        buildPhase = ''
          mkdir -p $out/traces
          
          # Trace MES bootstrap
          perf record -o $out/traces/mes_bootstrap.perf.data \
            -e 'syscalls:*' \
            ${pkgs.mes}/bin/mes --version
          
          # Trace GCC compilation
          echo "int main() { return 0; }" > test.c
          perf record -o $out/traces/gcc_compile.perf.data \
            -e 'syscalls:*' \
            ${pkgs.gcc}/bin/gcc test.c -o test
          
          # Trace LLVM compilation
          perf record -o $out/traces/llvm_compile.perf.data \
            -e 'syscalls:*' \
            ${pkgs.llvm}/bin/clang test.c -o test_llvm
          
          # Parse all traces
          for trace in $out/traces/*.perf.data; do
            perf script -i "$trace" > "''${trace%.perf.data}.trace"
          done
          
          # Create bootstrap analysis
          cat > $out/bootstrap_analysis.json << EOF
          {
            "mes_syscalls": $(grep -c "syscalls:" $out/traces/mes_bootstrap.trace),
            "gcc_syscalls": $(grep -c "syscalls:" $out/traces/gcc_compile.trace),
            "llvm_syscalls": $(grep -c "syscalls:" $out/traces/llvm_compile.trace),
            "total_bootstrap_syscalls": $((
              $(grep -c "syscalls:" $out/traces/mes_bootstrap.trace) +
              $(grep -c "syscalls:" $out/traces/gcc_compile.trace) +
              $(grep -c "syscalls:" $out/traces/llvm_compile.trace)
            ))
          }
          EOF
        '';
      };
      
      # Step 5: Comprehend Node.js via traces
      comprehendNodejs = pkgs.writeScriptBin "comprehend-nodejs" ''
        #!/bin/bash
        
        OUTPUT="$1"
        mkdir -p "$OUTPUT"
        
        echo "🔍 Comprehending Node.js via MES + GCC + LLVM traces..."
        
        # Aggregate all traces
        cat ${nodejsTraced}/nodejs_compile.trace \
            ${nodejsTraced}/nodejs_build.trace \
            ${mesBootstrapTrace}/traces/*.trace \
            > "$OUTPUT/full_bootstrap.trace"
        
        # Extract compilation patterns
        grep "gcc\|clang\|ld" "$OUTPUT/full_bootstrap.trace" > "$OUTPUT/compiler_syscalls.txt"
        
        # Extract V8 patterns
        grep "v8" "$OUTPUT/full_bootstrap.trace" > "$OUTPUT/v8_syscalls.txt"
        
        # Create comprehension map
        cat > "$OUTPUT/nodejs_comprehension.json" << EOF
        {
          "bootstrap_chain": "MES → GCC → LLVM → Node.js",
          "total_syscalls": $(wc -l < "$OUTPUT/full_bootstrap.trace"),
          "compiler_syscalls": $(wc -l < "$OUTPUT/compiler_syscalls.txt"),
          "v8_syscalls": $(wc -l < "$OUTPUT/v8_syscalls.txt"),
          "comprehension": "Complete bootstrap trace from MES to Node.js"
        }
        EOF
        
        echo "✅ Node.js comprehension complete: $OUTPUT/nodejs_comprehension.json"
      '';
      
      # Step 6: Lift to Rust with pure math
      liftToRust = pkgs.writeScriptBin "lift-gemini-to-rust" ''
        #!/bin/bash
        
        TRACE_DIR="$1"
        OUTPUT="$2"
        
        echo "📊 Lifting Gemini CLI: Node.js → Rust (pure math)"
        
        # Analyze all traces
        TOTAL_SYSCALLS=$(cat \
          ${nodejsTraced}/nodejs_compile.trace \
          ${geminiTraced}/npm_install.trace \
          ${geminiTraced}/bundle.trace \
          "$TRACE_DIR/gemini_exec.trace" \
          | grep -c "syscalls:")
        
        # Create lifting prompt
        cat > "$OUTPUT/lift_prompt.json" << EOF
        {
          "task": "Lift Gemini CLI from Node.js to Rust with pure math",
          "input": {
            "nodejs_compilation": "${nodejsTraced}/nodejs_compile.trace",
            "gemini_bundle": "${geminiTraced}/bundle.trace",
            "execution_trace": "$TRACE_DIR/gemini_exec.trace",
            "mes_bootstrap": "${mesBootstrapTrace}/bootstrap_analysis.json"
          },
          "total_syscalls": $TOTAL_SYSCALLS,
          "prompt": "Analyze these traces and create pure Rust implementation that:
            1. Replaces Node.js runtime with Rust
            2. Replaces V8 with pure math
            3. Preserves Gemini API behavior
            4. Proves equivalence via syscall curves
            5. Uses: reqwest (HTTP), serde (JSON), tokio (async)
            
            Mathematical proof:
            - Node.js syscall curve: $TOTAL_SYSCALLS syscalls
            - Rust syscall curve: <predicted> syscalls
            - Equivalence: same API behavior, better performance
            
            Output: Complete Rust crate that replaces gemini.js"
        }
        EOF
        
        echo "✅ Lifting prompt created: $OUTPUT/lift_prompt.json"
        echo "   Total syscalls analyzed: $TOTAL_SYSCALLS"
      '';
      
    in {
      packages.${system} = {
        nodejs-traced = nodejsTraced;
        gemini-traced = geminiTraced;
        mes-bootstrap = mesBootstrapTrace;
        
        trace-execution = geminiExecutionTrace;
        comprehend = comprehendNodejs;
        lift = liftToRust;
        
        # Complete pipeline
        default = pkgs.writeScriptBin "nodejs-to-rust-pipeline" ''
          #!/bin/bash
          
          echo "🚀 Node.js → Rust Lifting Pipeline"
          echo "===================================="
          echo ""
          
          WORK_DIR="./nodejs_lift_work"
          mkdir -p "$WORK_DIR"
          
          # Step 1: Trace Gemini execution
          echo "[1/4] Tracing Gemini execution..."
          ${geminiExecutionTrace}/bin/trace-gemini-execution "$WORK_DIR/execution"
          
          # Step 2: Comprehend Node.js
          echo "[2/4] Comprehending Node.js via bootstrap traces..."
          ${comprehendNodejs}/bin/comprehend-nodejs "$WORK_DIR/comprehension"
          
          # Step 3: Create lifting prompt
          echo "[3/4] Creating Rust lifting prompt..."
          ${liftToRust}/bin/lift-gemini-to-rust \
            "$WORK_DIR/execution" \
            "$WORK_DIR/lift"
          
          # Step 4: Summary
          echo "[4/4] Summary"
          echo "=============="
          cat "$WORK_DIR/comprehension/nodejs_comprehension.json"
          echo ""
          echo "✅ Ready to lift to Rust!"
          echo "   Prompt: $WORK_DIR/lift/lift_prompt.json"
        '';
      };
    };
}
