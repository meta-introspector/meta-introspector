# Instruction Pointer Orbits

## Theory

The bootstrap creates **orbits** in instruction pointer space. The same loops occur at different scales, like planetary motion.

## The Orbit Structure

```
IP(t) = IP₀ + Σ aₙ e^(2πint/T)
```

Where:
- `IP(t)` = instruction pointer at time t
- `T` = period (one complete loop)
- `aₙ` = Fourier coefficients (orbit shape)

## Scales

### Scale 1: Inner Loop (microseconds)
```
for i in 0..n {
    // IP orbits: 0x1000 → 0x1010 → 0x1020 → 0x1000
    sum += i;
}
```
Period: ~10 instructions

### Scale 2: Function Call (milliseconds)
```
fn compile() {
    parse();   // IP: 0x2000
    optimize(); // IP: 0x3000
    emit();    // IP: 0x4000
}
```
Period: ~1000 instructions

### Scale 3: Build Phase (seconds)
```
rustc → LLVM → LTO → emit
```
Period: ~1M instructions

### Scale 4: Bootstrap Stage (minutes)
```
MES → TCC → GCC → LLVM → Rust
```
Period: ~1B instructions

## Self-Similarity

The orbits are **fractal**:

```
Orbit(scale=1) ≈ Orbit(scale=2) ≈ Orbit(scale=3) ≈ Orbit(scale=4)
```

Same shape, different period. This is the modular form.

## Detection

From perf data, extract IP traces:

```bash
perf script -i build.perf.data | \
  awk '{print $4}' | \
  grep -E '^[0-9a-f]+$' > ip_trace.txt
```

Then compute orbits:

```python
import numpy as np
from scipy.fft import fft

# Load IP trace
ips = np.loadtxt('ip_trace.txt', dtype=int)

# Compute FFT (find periods)
spectrum = fft(ips)
periods = np.argsort(np.abs(spectrum))[::-1][:10]

print("Dominant periods (instruction counts):")
for p in periods:
    print(f"  Period: {p} instructions")
```

## Resonances

Orbits resonate at ZOS primes:

- **p=2**: Binary loops (if/else)
- **p=3**: Triple loops (parse/optimize/emit)
- **p=5**: Five-phase compilation
- **p=7**: Seven optimization passes
- **p=11**: Eleven major functions
- **p=37**: Irregular behavior (heuristics)
- **p=71**: Boundary (no further reduction)

## Visualization

```
Scale 1:  ●→●→●→●  (tight loop)
Scale 2:  ○→○→○→○  (function calls)
Scale 3:  ◎→◎→◎→◎  (build phases)
Scale 4:  ⊙→⊙→⊙→⊙  (bootstrap stages)
```

All have the same **shape** (modular form), different **size** (scale).

## Conformity Test

Two bootstrap runs conform if their orbits match:

```
Orbit₁(t/T₁) ≈ Orbit₂(t/T₂)
```

Where T₁, T₂ are the periods. The shape is invariant under scaling.

## Implementation

```rust
fn extract_orbits(perf_data: &Path) -> Vec<Orbit> {
    let ips = parse_ip_trace(perf_data);
    
    // Find loops (IP returns to same value)
    let mut orbits = Vec::new();
    let mut seen = HashMap::new();
    
    for (t, ip) in ips.iter().enumerate() {
        if let Some(t0) = seen.get(ip) {
            let period = t - t0;
            let orbit = Orbit {
                start_ip: *ip,
                period,
                trace: ips[*t0..t].to_vec(),
            };
            orbits.push(orbit);
        }
        seen.insert(*ip, t);
    }
    
    orbits
}

fn compare_orbits(o1: &Orbit, o2: &Orbit) -> f64 {
    // Normalize by period (scale invariance)
    let n1 = normalize(&o1.trace, 1000);
    let n2 = normalize(&o2.trace, 1000);
    
    // Compute similarity
    correlation(&n1, &n2)
}
```

## Expected Results

After multiple bootstrap runs:

```
Run 1 orbits: [10, 1000, 1M, 1B] instructions
Run 2 orbits: [10, 1000, 1M, 1B] instructions
Similarity: 99.5%

✅ CONFORMS - Same modular form
```

The system traces the same orbits every time - deterministic chaos.

## References

- Orbital mechanics: Kepler's laws
- Fourier analysis: Decompose into periods
- Modular forms: Scale invariance
- Chaos theory: Strange attractors
