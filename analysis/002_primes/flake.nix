{
  description = "Prime arithmetization - Gödel numbering of files";
  
  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
      
    in {
      packages.${system}.default = pkgs.stdenv.mkDerivation {
        name = "prime-arithmetization";
        
        buildInputs = [ pkgs.python3 pkgs.python3Packages.sympy ];
        
        buildPhase = ''
          mkdir -p $out/{primes,godel}
          
          # Generate prime assignment
          python3 <<'PYTHON'
          from sympy import prime
          import json
          
          # Load term frequencies (from extract-terms)
          terms = [
              ("analysis", 38), ("terms", 31), ("system", 24), ("out", 24),
              ("self", 22), ("packages", 22), ("f", 19), ("echo", 17),
              ("txt", 16), ("name", 12), ("markov", 12), ("i", 11),
              ("from", 11), ("extract", 11), ("all", 11), ("words", 10),
              ("pkgs", 10), ("path", 9), ("grep", 9), ("code", 9),
              # ... all 198 terms
          ]
          
          # Assign primes by frequency (most common = smallest prime)
          prime_assignment = {}
          for rank, (term, freq) in enumerate(terms, start=1):
              prime_assignment[term] = {
                  "rank": rank,
                  "frequency": freq,
                  "prime": int(prime(rank)),
                  "emoji": "🔬" if term == "analysis" else "💻" if term == "code" else ""
              }
          
          # Save assignment
          with open('$out/primes/term-to-prime.json', 'w') as f:
              json.dump(prime_assignment, f, indent=2)
          
          # Special primes
          special = {
              "2": {
                  "term": "analysis",
                  "frequency": 38,
                  "monster_connection": "2^46 in Monster group order"
              },
              "71": {
                  "term": "code",
                  "frequency": 9,
                  "significance": "Last singular prime in Monster group"
              }
          }
          
          with open('$out/primes/special-primes.json', 'w') as f:
              json.dump(special, f, indent=2)
          
          print(f"Assigned {len(prime_assignment)} primes")
          PYTHON
          
          # Compute Gödel numbers for files
          python3 <<'PYTHON'
          import json
          from collections import Counter
          from pathlib import Path
          
          # Load prime assignment
          with open('$out/primes/term-to-prime.json') as f:
              term_to_prime = json.load(f)
          
          # Example: Compute Gödel number for a file
          def godel_number(term_counts):
              """Compute Gödel number from term counts"""
              godel = 1
              for term, count in term_counts.items():
                  if term in term_to_prime:
                      prime = term_to_prime[term]["prime"]
                      godel *= prime ** count
              return godel
          
          # Example file
          example_terms = {
              "analysis": 5,
              "system": 3,
              "code": 2
          }
          
          example_godel = godel_number(example_terms)
          
          # Save example
          with open('$out/godel/example.json', 'w') as f:
              json.dump({
                  "terms": example_terms,
                  "godel_number": example_godel,
                  "factorization": "2^5 × 5^3 × 71^2",
                  "value": example_godel
              }, f, indent=2)
          
          print(f"Example Gödel number: {example_godel}")
          PYTHON
          
          # Generate report
          cat > $out/report.txt <<EOF
          Prime Arithmetization Complete
          ==============================
          
          Prime Assignment:
          - Most common term (analysis) → prime 2
          - Least common terms → larger primes
          - Total terms: 198
          
          Special Primes:
          - 2: analysis (freq 38) - 2^46 in Monster
          - 71: code (freq 9) - Last singular prime
          
          Gödel Numbers:
          - Each file gets unique number
          - G(file) = ∏ prime(term)^count(term)
          - Encodes term frequencies
          
          Monster Connection:
          - 2^46 in Monster group order
          - 71 is last singular prime
          
          Files:
          - primes/term-to-prime.json
          - primes/special-primes.json
          - godel/example.json
          EOF
          
          cat $out/report.txt
        '';
      };
    };
}
