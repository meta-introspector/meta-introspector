{
  description = "Gemini Evolution with Impure Derivations, Output Merging, and Ranking";
  
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    gemini-telemetry.url = "path:/mnt/data1/nix/source/github/meta-introspector/streamofrandom/2025/09/27/7-concepts/6-qa-testing/tests/2025-01-27-build-time-gemini-capture";
    zos-impure.url = "path:/mnt/data1/nix/source/github/meta-introspector/streamofrandom/2025/10/15/zos/impure-wrapper";
  };
  
  outputs = { self, nixpkgs, gemini-telemetry, zos-impure }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
      # Create impure Gemini call with ZKP
      createGeminiCall = { prompt, iteration }:
        zos-impure.lib.${system}.createImpureCall {
          callSpec = {
            name = "gemini-iter-${toString iteration}";
            description = "Gemini triage for iteration ${toString iteration}";
            command = ''
              # Call Gemini with telemetry
              ${gemini-telemetry.packages.${system}.default}/bin/gemini \
                -p "${prompt}" \
                --output-format json \
                --model gemini-2.5-flash \
                > $out/response.json 2> $out/telemetry.log
              
              # Capture perf trace
              perf record -o $out/trace.perf.data \
                -e 'syscalls:*' \
                ${gemini-telemetry.packages.${system}.default}/bin/gemini \
                -p "${prompt}" \
                --output-format json \
                --model gemini-2.5-flash
              
              # Extract metrics
              perf script -i $out/trace.perf.data > $out/trace.txt
              
              # Test generated code if present
              if jq -e '.commands' $out/response.json > /dev/null; then
                echo "Testing generated commands..."
                jq -r '.commands[]' $out/response.json > $out/commands.sh
                bash -n $out/commands.sh && echo "✅ Syntax valid" > $out/test_result.txt
              fi
              
              # Compile if code present
              if jq -e '.files' $out/response.json > /dev/null; then
                echo "Compiling generated code..."
                jq -r '.files[].content' $out/response.json > $out/generated.rs
                rustc --crate-type lib $out/generated.rs -o $out/generated.rlib 2> $out/compile.log || true
              fi
              
              # Rank output
              SYSCALLS=$(grep -c "syscalls:" $out/trace.txt || echo 0)
              DURATION=$(tail -1 $out/trace.txt | awk '{print $4}')
              
              cat > $out/rank.json << EOF
              {
                "iteration": ${toString iteration},
                "syscalls": $SYSCALLS,
                "duration": "$DURATION",
                "has_code": $(jq 'has("files")' $out/response.json),
                "has_commands": $(jq 'has("commands")' $out/response.json),
                "rank_score": $((SYSCALLS / 100))
              }
              EOF
            '';
            typeSignature = "Prompt -> (Response, Trace, Rank)";
            outputs = {
              response = "JSON";
              telemetry = "Log";
              trace = "PerfData";
              rank = "JSON";
            };
            buildInputs = with pkgs; [
              jq
              perf
              rustc
              bash
            ];
          };
        };
      
      # Merge multiple Gemini outputs
      mergeGeminiOutputs = outputs:
        pkgs.runCommand "merged-gemini-outputs" {
          __impure = true;
          buildInputs = [ pkgs.jq ];
        } ''
          mkdir -p $out/{responses,traces,ranks}
          
          # Merge all responses
          ${pkgs.lib.concatMapStringsSep "\n" (out: ''
            cp ${out.executor}/response.json $out/responses/$(basename ${out.executor}).json || true
            cp ${out.executor}/trace.perf.data $out/traces/$(basename ${out.executor}).perf.data || true
            cp ${out.executor}/rank.json $out/ranks/$(basename ${out.executor}).json || true
          '') outputs}
          
          # Aggregate ranks
          jq -s 'sort_by(.rank_score) | reverse' $out/ranks/*.json > $out/ranked_outputs.json
          
          # Aggregate traces
          cat $out/traces/*.perf.data > $out/all_traces.perf.data || true
          
          # Summary
          cat > $out/summary.json << EOF
          {
            "total_iterations": $(ls $out/responses/*.json | wc -l),
            "total_syscalls": $(jq -s 'map(.syscalls) | add' $out/ranks/*.json),
            "best_iteration": $(jq -r '.[0].iteration' $out/ranked_outputs.json),
            "timestamp": "$(date -Iseconds)"
          }
          EOF
          
          echo "✅ Merged $(ls $out/responses/*.json | wc -l) outputs"
        '';
      
      # Evolution with impure derivations
      evolutionWithImpure = { max_iterations ? 10 }:
        let
          # Create impure calls for each iteration
          geminiCalls = builtins.genList (i:
            createGeminiCall {
              prompt = "Fix error in iteration ${toString i}";
              iteration = i;
            }
          ) max_iterations;
          
          # Build all calls
          builtCalls = map (call: call.executor) geminiCalls;
          
          # Merge outputs
          merged = mergeGeminiOutputs geminiCalls;
        in
        {
          inherit geminiCalls builtCalls merged;
          
          # ZKP proofs for all calls
          proofs = map (call: call.zkpProver) geminiCalls;
          
          # Verifiers
          verifiers = map (call: call.zkpVerifier) geminiCalls;
        };
      
    in {
      packages.${system} = {
        # Single Gemini call
        gemini-call = (createGeminiCall {
          prompt = "Test prompt";
          iteration = 1;
        }).executor;
        
        # Evolution with 10 iterations
        evolution-10 = (evolutionWithImpure { max_iterations = 10; }).merged;
        
        # Evolution with 100 iterations
        evolution-100 = (evolutionWithImpure { max_iterations = 100; }).merged;
        
        # Full 10k evolution
        evolution-10k = (evolutionWithImpure { max_iterations = 10000; }).merged;
        
        default = self.packages.${system}.evolution-10;
      };
      
      lib.${system} = {
        inherit createGeminiCall mergeGeminiOutputs evolutionWithImpure;
      };
    };
}
