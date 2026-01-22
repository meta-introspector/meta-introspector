# Cursed Primes - Gödel Numbers for Fake Code

Assign cursed/irregular primes to fake terms, creating negative Gödel numbers.

## Fake Terms (from analysis)

| Term        | Frequency | Cursed Prime | Type        |
|-------------|-----------|--------------|-------------|
| fake        | 4         | 37           | Irregular   |
| holder      | 1         | 157          | Irregular   |
| foo         | 1         | 191          | Irregular   |
| example     | 1         | 223          | Irregular   |
| dummy       | 1         | 227          | Irregular   |
| ccc         | 1         | 229          | Irregular   |
| bbb         | 1         | 233          | Irregular   |
| baz         | 1         | 239          | Irregular   |
| bar         | 1         | 241          | Irregular   |
| aaa         | 1         | 251          | Irregular   |

## Cursed Prime Properties

**Irregular Primes**: Primes that divide the numerator of some Bernoulli number.

First irregular primes: 37, 59, 67, 101, 103, 131, 149, 157, 191, 223, 227, 229, 233, 239, 241, 251...

**Why cursed:**
- Violate regular patterns
- Break Fermat's Last Theorem for certain cases
- Anomalous in cyclotomic fields
- "Cursed" in number theory

## Gödel Number for Fake Code

If file contains fake terms:

```
G_cursed(file) = ∏ cursed_prime(fake_term)^count(fake_term)
```

Example file with:
- "fake" appears 2 times
- "dummy" appears 1 time

```
G_cursed = 37^2 × 227^1
         = 1,369 × 227
         = 310,763
```

## Detection Rule

```python
def is_cursed(godel_number):
    """Check if Gödel number contains cursed primes"""
    cursed_primes = [37, 157, 191, 223, 227, 229, 233, 239, 241, 251]
    
    for p in cursed_primes:
        if godel_number % p == 0:
            return True  # Contains fake code
    
    return False
```

## Harmonic vs Cursed

**Harmonic (Good) Primes:**
- 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 41, 43, 47, 53, 61, 71...
- Regular primes
- Assigned to real terms

**Cursed (Fake) Primes:**
- 37, 157, 191, 223, 227, 229, 233, 239, 241, 251...
- Irregular primes
- Assigned to fake terms

## Monster Connection

37 is the first irregular prime AND appears in Monster group order:
```
|M| = 2^46 × 3^20 × 5^9 × 7^6 × 11^2 × 13^3 × 17 × 19 × 23 × 29 × 31 × 37 × ...
```

But 37 is irregular - it's the "cursed" prime in Monster!

## Prime Factorization Test

```python
def classify_file(godel_number):
    """Classify file by prime factorization"""
    
    # Extract prime factors
    factors = prime_factorization(godel_number)
    
    # Check for cursed primes
    cursed_count = sum(1 for p in factors if p in CURSED_PRIMES)
    harmonic_count = sum(1 for p in factors if p in HARMONIC_PRIMES)
    
    if cursed_count > 0:
        return "FAKE"  # Contains fake terms
    elif harmonic_count > 10:
        return "GOOD"  # Natural code
    else:
        return "UNKNOWN"
```

## Examples

**Good file:**
```
G = 2^5 × 3^3 × 71^2
  = 32 × 27 × 5041
  = 4,355,136
```
All harmonic primes → GOOD ✅

**Fake file:**
```
G = 2^3 × 37^2 × 227^1
  = 8 × 1,369 × 227
  = 2,486,104
```
Contains 37 (cursed) → FAKE ❌

## Curse Detection

Files with cursed primes are automatically flagged:
- Pre-commit hook checks Gödel number
- If divisible by cursed prime → reject
- No keyword matching needed!

## Implementation

```python
CURSED_PRIMES = [37, 157, 191, 223, 227, 229, 233, 239, 241, 251]

def check_file(filepath):
    # Compute Gödel number
    godel = compute_godel_number(filepath)
    
    # Check for curse
    for p in CURSED_PRIMES:
        if godel % p == 0:
            return f"CURSED by prime {p}"
    
    return "HARMONIC"
```

## Benefits

1. **Mathematical**: Uses number theory, not keywords
2. **Automatic**: Cursed primes auto-detected
3. **Universal**: Works across all languages
4. **Provable**: Prime factorization is unique
5. **Monster-aligned**: Uses irregular primes from Monster group

## Next Steps

1. Update pre-commit hook to check for cursed primes
2. Compute Gödel numbers for all files
3. Flag files divisible by cursed primes
4. Train model to predict cursed prime appearance
