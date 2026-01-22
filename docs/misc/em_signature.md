# 📡 EM Signature: Side-Channel Witness

## The Unintended Broadcast

```
CPU executes instruction:
  Electrons flow through gates
  Current changes rapidly
  ΔI/Δt = dI/dt
  
Maxwell's equations:
  ∇×B = μ₀ε₀∂E/∂t + μ₀J
  Changing current → EM radiation
  
CPU becomes antenna:
  Unintentional EM emission
  Frequency: DC to GHz
  Power: μW to mW
  
WiFi antenna nearby:
  Receives EM radiation
  Records side-channel signal
  = Unintended witness
```

## The EM Fingerprint

```
Different instructions → Different EM signatures:

mov %rax, %rbx:
  Simple register transfer
  Minimal current change
  Low EM emission
  Signature: ___/‾‾‾\___
  
add %rax, %rbx:
  ALU activation
  Carry propagation
  Medium EM emission
  Signature: __/‾‾‾‾‾\__
  
mul %rax, %rbx:
  Full multiplier active
  Many gates switching
  High EM emission
  Signature: _/‾‾‾‾‾‾‾‾\_
  
Each instruction = Unique EM pattern
= Electromagnetic fingerprint
```

## The Compilation Signature

```
Mes compiling TinyCC:
  Specific instruction sequence
  Specific EM pattern over time
  
EM trace:
  t₀: Parse phase (string ops, high EM)
  t₁: Type check (comparison ops, medium EM)
  t₂: Codegen (write ops, high EM)
  t₃: Link (I/O ops, variable EM)
  
Spectrogram:
     Frequency
        ↑
    GHz |     ████
        |   ██    ██
        |  █        █
        | █          █
        |█            █
        └──────────────→ Time
        Parse  Type  Code  Link
        
Unique pattern = Compilation fingerprint
```

## The Heat Signature

```
Power dissipation:
  P = I²R (Joule heating)
  I = current through transistors
  R = resistance
  
Instruction execution:
  High activity → High current → High heat
  Low activity → Low current → Low heat
  
Thermal camera view:
  Hot spots = Active cores
  Cool spots = Idle cores
  
Temperature over time:
     Temp (°C)
        ↑
     80 |        ╱‾‾‾╲
        |       ╱     ╲
     60 |      ╱       ╲___
        |     ╱
     40 |____╱
        └──────────────────→ Time
        Idle  Compile  Done
        
Heat signature = Thermal witness
```

## The WiFi Side-Channel

```
WiFi antenna as unintended receiver:

Antenna characteristics:
  Frequency: 2.4 GHz or 5 GHz (carrier)
  Bandwidth: 20-160 MHz
  Sensitivity: -90 dBm
  
CPU EM leakage:
  Frequency: 0.1-3 GHz (harmonics)
  Power: -60 to -40 dBm (nearby)
  Modulation: Instruction-dependent
  
Coupling:
  CPU EM → WiFi antenna
  Unintended reception
  Side-channel signal
  
Demodulation:
  Extract baseband from carrier
  Reveals instruction patterns
  = EM side-channel attack
```

## The TEMPEST Threat

```
TEMPEST: Telecommunications Electronics Material 
         Protected from Emanating Spurious Transmissions

Compromising emanations:
  EM radiation from CPU
  Can be received remotely
  Can reconstruct computation
  
Attack scenario:
  Attacker: WiFi antenna + SDR
  Distance: 10-100 meters
  Receives: CPU EM leakage
  Reconstructs: Instruction stream
  
Defense:
  Faraday cage (EM shielding)
  Spread spectrum (noise injection)
  Distance (signal attenuation)
```

## The Signature Database

```
Build EM signature database:

For each instruction:
  Execute in isolation
  Record EM emission
  Store signature
  
Database:
  mov: [EM_pattern_mov]
  add: [EM_pattern_add]
  mul: [EM_pattern_mul]
  ...
  
Pattern matching:
  Receive EM signal
  Correlate with database
  Identify instruction
  Reconstruct execution
  
This is EM-based perf tracing!
  No software instrumentation
  Pure physical measurement
  Unintended witness
```

## The Bootstrap EM Signature

```
Mes → TinyCC → GCC → LLVM → Mes

Each stage has unique EM signature:

Stage 0 (hex0):
  Simple string processing
  Low complexity
  EM: Low frequency, steady
  
Stage 1-4 (hex1, hex2, M0, M2):
  Increasing complexity
  More ALU operations
  EM: Medium frequency, variable
  
Stage 5 (Mes):
  Interpreter loop
  High branching
  EM: High frequency, chaotic
  
Stage 6+ (GCC):
  Massive compilation
  All units active
  EM: Maximum power, complex pattern
  
Spectrogram of full bootstrap:
  Unique "EM fingerprint"
  Proves bootstrap occurred
  Physical witness
```

## The Coherence Signature

```
Bootstrap cycle: Mes → ... → Mes

First iteration:
  EM signature: S₁(t)
  
Second iteration:
  EM signature: S₂(t)
  
Coherence check:
  S₁(t) ≈ S₂(t) ?
  
If yes: ✅
  Same computation
  Same EM emission
  Physical coherence
  
If no: ❌
  Different computation
  Different EM emission
  Coherence broken
  
EM correlation:
  ρ = ∫ S₁(t)·S₂(t) dt / √(∫S₁²dt · ∫S₂²dt)
  ρ ≈ 1: Coherent
  ρ < 1: Incoherent
```

## The Multi-Modal Witness

```
Three witnesses of same computation:

1. Software (perf.data):
   Instrumented trace
   Cycle counts
   Cache misses
   
2. EM (WiFi antenna):
   Radiated signature
   Frequency spectrum
   Power levels
   
3. Thermal (IR camera):
   Heat dissipation
   Temperature map
   Thermal dynamics
   
All three must agree:
  perf.data says: "Compiled GCC"
  EM signature says: "Compiled GCC"
  Thermal signature says: "Compiled GCC"
  
If all match: ✅ Strong witness
If mismatch: ❌ Tampering detected
```

## The Fourier Analysis

```
EM signal in time domain:
  S(t) = Σᵢ Aᵢ sin(ωᵢt + φᵢ)
  
Fourier transform to frequency domain:
  S(ω) = ∫ S(t) e^(-iωt) dt
  
Power spectral density:
  P(ω) = |S(ω)|²
  
Instruction signatures in frequency space:
  mov: Peak at ω₁
  add: Peak at ω₂
  mul: Peak at ω₃
  
Compilation = Sequence of peaks:
  P(ω, t) = time-frequency spectrogram
  Unique pattern per program
```

## The Antenna Equation

```
Received power at WiFi antenna:

Friis transmission equation:
  P_r = P_t · G_t · G_r · (λ/4πd)²
  
Where:
  P_t = CPU radiated power (~1 μW)
  G_t = CPU "antenna" gain (~0.01)
  G_r = WiFi antenna gain (~2)
  λ = wavelength (~0.1 m at 3 GHz)
  d = distance (~1 m)
  
Result:
  P_r ≈ 10⁻⁹ W = -60 dBm
  
Above WiFi sensitivity (-90 dBm)
∴ Detectable! ✓
```

## The Correlation Attack

```
Attacker strategy:

1. Collect EM traces:
   Record WiFi antenna during compilation
   Store S_observed(t)
   
2. Build reference database:
   Compile known programs
   Record EM signatures
   Store S_reference_i(t)
   
3. Correlate:
   For each reference i:
     ρᵢ = Corr(S_observed, S_reference_i)
   
4. Identify:
   i* = argmax ρᵢ
   "Victim compiled program i*"
   
5. Reconstruct:
   From program i*, infer:
     - Source code structure
     - Algorithms used
     - Secrets processed
```

## The Quantum Limit

```
Minimum detectable EM signal:

Quantum noise floor:
  P_quantum = hf (photon energy)
  h = Planck constant
  f = frequency
  
At 3 GHz:
  P_quantum = 6.6×10⁻³⁴ × 3×10⁹
            = 2×10⁻²⁴ W
            = -204 dBm
  
Thermal noise floor:
  P_thermal = kTB
  k = Boltzmann constant
  T = temperature (300 K)
  B = bandwidth (1 GHz)
  
  P_thermal = 1.4×10⁻²³ × 300 × 10⁹
            = 4×10⁻¹² W
            = -114 dBm
  
CPU emission (-60 dBm) >> Thermal noise
∴ Easily detectable above noise floor
```

## The Shielding Equation

```
Faraday cage effectiveness:

Shielding effectiveness:
  SE = 20 log₁₀(E_incident / E_transmitted)
  
For copper mesh:
  Thickness: 1 mm
  Frequency: 3 GHz
  SE ≈ 100 dB
  
Attenuation:
  E_transmitted = E_incident × 10^(-SE/20)
                = E_incident × 10⁻⁵
  
CPU emission: -60 dBm
After shielding: -60 - 100 = -160 dBm
Below noise floor: ✓
∴ Shielding effective
```

## The Witness Equation

```
Complete witness W:

W = (W_software, W_EM, W_thermal)

W_software = perf.data
  Instrumented trace
  Hash: #️⃣_perf
  
W_EM = EM signature
  Recorded spectrum
  Hash: #️⃣_EM = Hash(S(ω,t))
  
W_thermal = Thermal trace
  Temperature map
  Hash: #️⃣_thermal = Hash(T(x,y,t))
  
Combined witness:
  #️⃣_total = Hash(#️⃣_perf ∥ #️⃣_EM ∥ #️⃣_thermal)
  
Verification:
  All three must be consistent
  Perf ⟺ EM ⟺ Thermal
  
If consistent: ✅ Authentic witness
If inconsistent: ❌ Tampering detected
```

## The Side-Channel Proof

```
Theorem: EM signature proves computation

Proof:

1. Instruction → EM emission (physics)
   Each instruction has unique EM pattern
   
2. Sequence → Signature (composition)
   Instruction sequence → EM signature sequence
   
3. Program → Spectrum (Fourier)
   Program execution → Unique EM spectrum
   
4. Spectrum → Program (inverse)
   EM spectrum → Infer program (with database)
   
5. Bootstrap → Unique spectrum
   Mes → TinyCC → GCC → LLVM → Mes
   Has unique EM fingerprint
   
6. Reproduce → Same spectrum
   Same bootstrap → Same EM signature
   Different bootstrap → Different signature
   
∴ EM signature is proof of specific computation

QED ✓

Caveat: Requires EM database and correlation
```

## The Meta-Signature

```
This document has EM signature:

As you read this:
  CPU decodes text
  Renders to screen
  Specific instruction pattern
  Specific EM emission
  
Your CPU is broadcasting:
  "Reading about EM signatures"
  Detectable by nearby antenna
  Unintended meta-witness
  
The irony:
  Learning about side-channels
  Creates side-channel
  The witness witnesses itself
  
🔄 Self-referential EM emission
```

---

**CPU execution → EM radiation**  
**WiFi antenna → Unintended receiver**  
**EM signature → Physical fingerprint**  
**Bootstrap → Unique spectrum**  
**Multi-modal witness → Strong proof**  
**Side-channel = Unintended witness**

📡 = ⚡ = 💭 (EM = Electrons = Thought)
