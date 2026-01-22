{
  description = "Code quality analysis - detect fake/placeholder terms";
  
  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
    in {
      packages.${system} = {
        # Job: Extract all terms from our code
        extract-terms = pkgs.stdenv.mkDerivation {
          name = "extract-terms";
          src = ./.;
          
          buildInputs = [ pkgs.ripgrep pkgs.jq ];
          
          buildPhase = ''
            mkdir -p $out/analysis
            
            # Extract all words from sh/rs/nix/md files (include docs for weight)
            find . -maxdepth 3 -type f \
              \( -name "*.sh" -o -name "*.rs" -o -name "*.nix" -o -name "*.md" \) \
              ! -path "./nix/flakes/*" \
              ! -path "./.git/*" \
              ! -path "./submodules/*" \
              ! -path "./research/experimental/*" \
              -exec cat {} \; | \
              grep -oE '\b[a-z_][a-z_0-9]*\b' | \
              sort | uniq -c | sort -rn > $out/analysis/all-terms.txt
            
            # Extract suspicious patterns
            grep -iE "place|holder|stub|dummy|mock|fake|temp|tmp|test|example|sample|xxx|yyy|zzz|aaa|bbb|ccc|foo|bar|baz" \
              $out/analysis/all-terms.txt > $out/analysis/suspicious-terms.txt || true
            
            # Count occurrences
            TOTAL=$(wc -l < $out/analysis/all-terms.txt)
            SUSPICIOUS=$(wc -l < $out/analysis/suspicious-terms.txt)
            
            cat > $out/analysis/summary.json <<EOF
            {
              "total_terms": $TOTAL,
              "suspicious_terms": $SUSPICIOUS,
              "timestamp": "$(date -Iseconds)"
            }
            EOF
            
            echo "Extracted $TOTAL unique terms, $SUSPICIOUS suspicious"
          '';
        };
        
        # Job: N-gram analysis
        ngram-analysis = pkgs.stdenv.mkDerivation {
          name = "ngram-analysis";
          src = ./.;
          
          buildInputs = [ pkgs.python3 self.packages.${system}.extract-terms ];
          
          buildPhase = ''
            mkdir -p $out/ngrams
            
            # Extract 2-grams, 3-grams from code
            find . -maxdepth 3 -type f \
              \( -name "*.sh" -o -name "*.rs" -o -name "*.nix" \) \
              ! -path "./nix/flakes/*" \
              ! -path "./.git/*" \
              ! -path "./submodules/*" \
              ! -path "./research/experimental/*" \
              -exec cat {} \; > all-code.txt
            
            # 2-grams
            python3 <<'PYTHON'
            import sys
            from collections import Counter
            
            with open('all-code.txt') as f:
                words = f.read().lower().split()
            
            bigrams = [f"{words[i]} {words[i+1]}" for i in range(len(words)-1)]
            counts = Counter(bigrams)
            
            with open('$out/ngrams/bigrams.txt', 'w') as f:
                for bigram, count in counts.most_common(1000):
                    f.write(f"{count}\t{bigram}\n")
            PYTHON
            
            # 3-grams
            python3 <<'PYTHON'
            import sys
            from collections import Counter
            
            with open('all-code.txt') as f:
                words = f.read().lower().split()
            
            trigrams = [f"{words[i]} {words[i+1]} {words[i+2]}" for i in range(len(words)-2)]
            counts = Counter(trigrams)
            
            with open('$out/ngrams/trigrams.txt', 'w') as f:
                for trigram, count in counts.most_common(1000):
                    f.write(f"{count}\t{trigram}\n")
            PYTHON
            
            echo "N-gram analysis complete"
          '';
        };
        
        # Job: Markov chain analysis
        markov-analysis = pkgs.stdenv.mkDerivation {
          name = "markov-analysis";
          src = ./.;
          
          buildInputs = [ pkgs.python3 self.packages.${system}.extract-terms ];
          
          buildPhase = ''
            mkdir -p $out/markov
            
            # Build Markov chain of term transitions
            python3 <<'PYTHON'
            import json
            from collections import defaultdict, Counter
            
            # Read all terms
            with open('${self.packages.${system}.extract-terms}/analysis/all-terms.txt') as f:
                terms = [line.split()[1] for line in f if line.strip()]
            
            # Build transition matrix
            transitions = defaultdict(Counter)
            for i in range(len(terms)-1):
                transitions[terms[i]][terms[i+1]] += 1
            
            # Convert to JSON
            markov_chain = {}
            for term, next_terms in transitions.items():
                markov_chain[term] = dict(next_terms.most_common(10))
            
            with open('$out/markov/transitions.json', 'w') as f:
                json.dump(markov_chain, f, indent=2)
            
            print(f"Markov chain: {len(markov_chain)} terms")
            PYTHON
            
            echo "Markov analysis complete"
          '';
        };
        
        # Job: Generate updated pre-commit hook
        generate-precommit-hook = pkgs.stdenv.mkDerivation {
          name = "generate-precommit-hook";
          
          buildInputs = [ 
            self.packages.${system}.extract-terms
            self.packages.${system}.ngram-analysis
            self.packages.${system}.markov-analysis
          ];
          
          buildPhase = ''
            mkdir -p $out
            
            # Extract banned terms from analysis
            BANNED_TERMS=$(cat ${self.packages.${system}.extract-terms}/analysis/suspicious-terms.txt | \
              awk '{print $2}' | tr '\n' '|' | sed 's/|$//')
            
            # Generate pre-commit hook
            cat > $out/pre-commit <<'EOF'
            #!/bin/bash
            # Demo2Code Policy Enforcement - Auto-generated from code analysis
            
            echo "🔍 Checking for fake/demo code..."
            
            # Banned patterns (auto-generated)
            BANNED_PATTERNS="BANNED_TERMS_PLACEHOLDER"
            
            # Smart filtering: exclude legitimate uses
            if git diff --cached --name-only | \
               grep "\.rs$\|\.sh$\|\.nix$" | \
               grep -v "fake_detector\|fake_replacer\|demos/archived\|demo2code-lint\|research/experimental" | \
               xargs grep -iE "$BANNED_PATTERNS" 2>/dev/null | \
               grep -v "Connection" | \
               grep -v '"/demos/archived/"' | \
               grep -v "// Parse from\|// Parsed from" | \
               grep -v "implementation pending"; then
                echo ""
                echo "❌ COMMIT REJECTED: Fake/demo code detected"
                echo ""
                echo "Banned patterns found: $BANNED_PATTERNS"
                echo ""
                echo "Policy: DEMO2CODE_POLICY.md"
                echo "Audit: QUALITY_AUDIT.md"
                echo ""
                echo "Excluded from checks: fake_detector.rs, fake_replacer.rs, demos/archived/, demo2code-lint/, research/experimental/"
                echo "Allowed: *Connection types, path literals, implementation notes"
                exit 1
            fi
            
            echo "✅ No fake code detected (smart context filtering)"
            exit 0
            EOF
            
            # Replace placeholder with actual terms
            sed -i "s|BANNED_TERMS_PLACEHOLDER|$BANNED_TERMS|g" $out/pre-commit
            
            chmod +x $out/pre-commit
            
            # Generate report
            cat > $out/report.txt <<EOF
            Pre-commit Hook Generated
            ==========================
            
            Banned terms: $(echo "$BANNED_TERMS" | tr '|' '\n' | wc -l)
            
            Source analysis:
            - Total terms: $(cat ${self.packages.${system}.extract-terms}/analysis/summary.json | jq -r .total_terms)
            - Suspicious: $(cat ${self.packages.${system}.extract-terms}/analysis/summary.json | jq -r .suspicious_terms)
            
            N-grams analyzed:
            - Bigrams: $(wc -l < ${self.packages.${system}.ngram-analysis}/ngrams/bigrams.txt)
            - Trigrams: $(wc -l < ${self.packages.${system}.ngram-analysis}/ngrams/trigrams.txt)
            
            Markov chain:
            - Transitions: $(cat ${self.packages.${system}.markov-analysis}/markov/transitions.json | jq 'length')
            EOF
          '';
        };
        
        # Complete quality analysis
        default = pkgs.stdenv.mkDerivation {
          name = "quality-analysis-complete";
          
          buildInputs = [
            self.packages.${system}.extract-terms
            self.packages.${system}.ngram-analysis
            self.packages.${system}.markov-analysis
            self.packages.${system}.generate-precommit-hook
          ];
          
          buildPhase = ''
            mkdir -p $out/{analysis,ngrams,markov,hooks}
            
            # Collect all outputs
            cp -r ${self.packages.${system}.extract-terms}/analysis/* $out/analysis/
            cp -r ${self.packages.${system}.ngram-analysis}/ngrams/* $out/ngrams/
            cp -r ${self.packages.${system}.markov-analysis}/markov/* $out/markov/
            cp ${self.packages.${system}.generate-precommit-hook}/pre-commit $out/hooks/
            cp ${self.packages.${system}.generate-precommit-hook}/report.txt $out/
            
            echo "Quality analysis complete: $out"
          '';
        };
      };
    };
}
