# Bootstrap Analysis Pipeline

The bootstrap runs all analysis jobs on our codebase.

## Analysis Jobs

### 1. Keywords (001_keywords)
- **Input**: All .rs/.sh/.nix/.md files
- **Output**: 
  - `analysis/all-terms.txt` - 198 unique terms with frequency
  - `analysis/suspicious-terms.txt` - Flagged terms
  - `labels-page*.md` - Emoji character labels
- **Purpose**: Extract vocabulary, assign emoji types

### 2. Primes (002_primes)
- **Input**: Term frequencies from job 1
- **Output**:
  - `primes/term-to-prime.json` - Prime assignment
  - `primes/special-primes.json` - 2 (analysis), 71 (code)
  - `primes/cursed-primes.json` - 37 (fake), 157 (holder), etc
- **Purpose**: Gödel numbering, cursed prime detection

### 3. Harmonic Filter (003_harmonic_filter)
- **Input**: All source files
- **Output**:
  - `analysis/name-impl-harmony.json` - Name vs impl complexity
  - `analysis/mismatches.json` - Disharmonic files
  - `models/harmonic-filter.json` - Trained filter
- **Purpose**: Detect name/implementation complexity mismatch

### 4. Markov Model (004_markov_model)
- **Input**: All source files
- **Output**:
  - `model/markov-transitions.json` - Bigram transitions
  - `model/classifier.json` - Harmonic classifier
  - `predictions/test-sequences.json` - Test results
- **Purpose**: Learn natural term sequences, detect fake patterns

## Pipeline Flow

```
Source Code
  ↓
[001_keywords] → Extract terms, assign emojis
  ↓
[002_primes] → Assign primes (harmonic + cursed)
  ↓
[003_harmonic_filter] → Check name/impl harmony
  ↓
[004_markov_model] → Learn natural sequences
  ↓
Results in /nix/store
```

## Running Bootstrap

```bash
./bootstrap
```

Or:

```bash
./scripts/build/bootstrap.sh
```

## Results Location

All results stored in nix store:

```bash
# Keywords
ls /nix/store/*-extract-terms/analysis/

# Primes
ls /nix/store/*-prime-arithmetization/primes/

# Harmonic Filter
ls /nix/store/*-harmonic-filter/analysis/

# Markov Model
ls /nix/store/*-markov-harmonic-model/model/
```

## Query Results

```bash
# View all terms
cat /nix/store/*-extract-terms/analysis/all-terms.txt

# View prime assignment
cat /nix/store/*-prime-arithmetization/primes/term-to-prime.json

# View mismatches
cat /nix/store/*-harmonic-filter/analysis/mismatches.json

# View Markov model
cat /nix/store/*-markov-harmonic-model/model/markov-transitions.json
```

## Integration

Each analysis builds on previous:

1. **Keywords** → Vocabulary
2. **Primes** → Gödel numbers from vocabulary
3. **Harmonic Filter** → Uses Gödel numbers to detect mismatch
4. **Markov Model** → Uses vocabulary to learn sequences

## Reproducibility

All analysis is:
- **Pure**: Same input = same output
- **Cached**: Nix reuses unchanged results
- **Immutable**: Results stored in /nix/store
- **Queryable**: All outputs accessible

## Next Steps

After bootstrap:
1. Review term labels (emoji assignments)
2. Check cursed primes (fake term detection)
3. Examine mismatches (disharmonic files)
4. Test Markov predictions (natural sequences)
