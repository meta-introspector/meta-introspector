# 🔮 The Bootstrap Ritual: A Zero-Knowledge Ceremony

## 🎭 The Sacred Performance Captured

```
         ↓
    The ritual begins...
    Every syscall a prayer 🙏
    Every allocation a sacrifice 🔥
    Every cycle a heartbeat 💓
    
    This is the ZK witness:
    "I built trust from nothing"
```

---

## 📜 Canto I: The Bootstrap Paradox

```
🥚 The Chicken-Egg Eternal:

To build a compiler: Need a compiler 💻
To trust the compiler: Must verify 🔍
To verify: Need trusted tools 🔧
To trust tools: Need compiler 💻

The circle closes: 🔄
  Trust → Compiler → Binary → Trust?
  
🚫 BROKEN: The Thompson Attack
  
  Compiler₀ (untrusted) → Compiler₁ (infected)
  Compiler₁ → Compiler₂ (still infected)
  ...
  Compilerₙ → Compilerₙ₊₁ (forever infected)
  
  The virus propagates: 🦠→🦠→🦠
  Through cryptographic rounds: 🔐→🔐→🔐
  Through Kleene iteration: 🔄*
  Through information flow: 📊→📊→📊

The bootstrap breaks the cycle: 💥
  
  Start from NOTHING: ⊥
  Build EVERYTHING: ⊤
  Prove TRUST: ✅
```

---

## 🌱 Canto II: The Minimal Seed - Mes

```
🌰 Mes (Mes Executes Scheme)

The smallest viable seed: 357 bytes of hex 🔢
  stage0: ⊥ → 357 bytes (hand-auditable)
  
This is the axiom: 📐
This is the genesis: 🌅
This is ⊥ in the semiring of trust: 🔐

From 357 bytes, all things emerge:

  357 bytes → hex0 (hex assembler)
  hex0 → hex1 (with symbols)  
  hex1 → hex2 (with macros)
  hex2 → M0 (minimal language)
  M0 → M2-Planet (C subset)
  M2-Planet → Mes (Scheme interpreter)
  Mes → Mes C Library
  Mes C → TinyCC
  TinyCC → GCC 2.95
  GCC 2.95 → GCC 4.7
  GCC 4.7 → Modern GCC
  Modern GCC → The World 🌍

Each step is a cryptographic round: 🔐
Each step is a Kleene join: ⊕
Each step is information bootstrapped: 📊↑

The ZK proof: "I can rebuild civilization from 357 bytes"
```

---

## 🎪 Canto III: The Nix Build - Hermetic Sanctuary

```
🏛️ Nix: The Deterministic Temple

nix-build = Hermetic ritual chamber 🔒
  
  No network: 🚫🌐 (air-gapped trust)
  No time: 🚫⏰ (SOURCE_DATE_EPOCH frozen)
  No randomness: 🚫🎲 (deterministic seeds)
  No user: 🚫👤 (isolated namespaces)
  
Input hash: #️⃣ᵢₙ (cryptographic fingerprint)
  /nix/store/abc123-mes-source
  
Build process: 🔄 (pure function)
  f(#️⃣ᵢₙ) → #️⃣ₒᵤₜ
  
Output hash: #️⃣ₒᵤₜ (cryptographic commitment)
  /nix/store/def456-mes-0.24

The Nix store path IS the ZK commitment: 🔐
  "This hash came from these inputs"
  "No side channels leaked information"
  "The build is reproducible"

Two builders, distant in space: 🌍↔️🌏
  Alice builds: #️⃣ₐ
  Bob builds: #️⃣ᵦ
  
If #️⃣ₐ = #️⃣ᵦ: ✅
  The ritual was pure
  No hidden influence
  Determinism achieved
  
This is the ZK proof: "Same inputs → Same outputs"
```

---

## 🎬 Canto IV: Perf Record - The Witness Emerges

```

As the build ritual unfolds: 🎭
  A shadow records every movement:
  
  perf_event_open() → 👁️ watching
  
  Captures:
    🔥 CPU cycles: The ritual's heartbeat
    💾 Cache misses: Memory's stumbles  
    🌳 Call stacks: The prayer's structure
    ⏱️ Time stamps: The ceremony's rhythm
    🔄 Context switches: The OS's interruptions
    
The perf.data file emerges: 📊
  
  This is the ZK witness: 📜
    "I observed this exact performance"
    "These cycles were consumed"  
    "This call graph was traversed"
    "This is the shape of trust"

The witness contains:
  
  🌊 Event stream (temporal watermark):
    t₀: syscall(open, "mes.c")
    t₁: mmap(heap_start)  
    t₂: exec("hex0")
    t₃: cycles=1000000
    ...
    tₙ: syscall(write, "mes")
    
  📈 Flame graph (spatial structure):
    main → 
      bootstrap_stage0 →
        hex0_assemble → 99% time
      bootstrap_stage1 →
        hex1_assemble → 95% time
      ...
      compile_mes_c →
        tcc_compile → 80% time
        
  🔐 Cryptographic trace:
    Hash(events) = #️⃣ᵥᵥᵢₜₙₑₛₛ
    
This hash is unforgeable: 🚫🔨
  Cannot fake the performance
  Cannot simulate the timing
  Cannot reproduce without actual build
```

---

## 🔬 Canto V: The ZK Witness Structure

```
🎯 Zero-Knowledge Witness = (Statement, Proof, Verifier)

Statement (Public): 💬
  "I built Mes from source to binary"
  "The output hash is #️⃣ₒᵤₜ"
  
Witness (Private): 🤫
  The actual execution trace
  perf.data = complete record
  Every cycle, every instruction
  
Proof (Cryptographic): 🔐
  Commitment to witness:
    C = Hash(perf.data) = #️⃣ᵥᵥᵢₜₙₑₛₛ
    
  Merkle tree of execution:
                🌳
               /  \
              /    \
          🌿      🌿
         / \      / \
        🍃 🍃   🍃 🍃
       [events of build]
       
  Root = #️⃣ᵥᵥᵢₜₙₑₛₛ
  
Verifier checks: ✅
  1. Output hash matches: #️⃣ₒᵤₜ = expected
  2. Witness commitment reveals: #️⃣ᵥᵥᵢₜₙₑₛₛ
  3. Performance is plausible:
     - Cycle count ∈ [min, max]
     - Call graph structure valid
     - Syscall sequence legitimate
     
The ZK property: 🎭
  Verifier learns: "Build happened correctly" ✅
  Verifier doesn't learn: Actual trace details 🤫
  
But here's the twist: 🌀
  In bootstrap, we WANT full transparency!
  The witness should be public and auditable
  
So this is "ZK" in reverse: 🔄
  Maximum knowledge
  Maximum auditability  
  Maximum trust through transparency
```

---

## 🎨 Canto VI: The Three Lenses on Bootstrap

```
🔐 CRYPTOGRAPHIC View:

The bootstrap is a hash chain: ⛓️
  
  H₀ = Hash(357 bytes) = #️⃣ₛₜₐ𝓰ₑ₀
  H₁ = Hash(H₀ ∥ build_hex0) = #️⃣ₕₑₓ₀
  H₂ = Hash(H₁ ∥ build_hex1) = #️⃣ₕₑₓ₁
  ...
  Hₙ = Hash(Hₙ₋₁ ∥ build_mes) = #️⃣ₘₑₛ
  
Each stage:
  Input: Previous hash (key material) 🔑
  Process: Compilation (encryption round) 🔐
  Output: New hash (commitment) #️⃣
  
Tamper-evidence: 💥
  Change ANY bit in stage i
  → Avalanche effect in all j > i
  → Final hash changes
  
The perf.data is the MAC: 🔏
  "This hash was computed THIS way"
  "With THIS performance signature"
  "Verifiable by reconstruction"

---

🔄 KLEENE View:

The bootstrap is iterated join: ⊕*
  
  Capability₀ = {357 bytes of hex} (⊥)
  Capability₁ = Capability₀ ⊕ {hex0}
  Capability₂ = Capability₁ ⊕ {hex1}
  Capability₃ = Capability₂ ⊕ {hex2}
  ...
  Capabilityₙ = Capabilityₙ₋₁ ⊕ {mes, tcc, gcc, ...}
  
Fixed-point: 🎯
  When Capability* includes "full compiler toolchain"
  
The lattice of trust: 📊
  
        ⊤ (Modern GCC - full trust)
       /|\
      / | \
    GCC 4.7
      |
    GCC 2.95
      |  
    TinyCC
      |
    Mes C
      |
    Mes Scheme
      |
    M2-Planet
      |
    M0
      |
    hex2
      |
    hex1
      |
    hex0
      |
    ⊥ (357 bytes - minimal trust)

Each step up: Information increases ↑
Each step up: Trust propagates ↑
Each step up: Capabilities expand ↑

The perf trace shows: 🎥
  The actual path through lattice
  The convergence to fixed-point
  The iteration count (build stages)

---

📊 INFORMATION View:

The bootstrap is entropy resolution: 🌀→🎯

Initially: 🌫️
  H(toolchain | 357 bytes) = ∞
  "Unknown what we can build"
  
Stage 0: hex0 assembler
  H(toolchain | stage0) = HIGH
  "Can assemble hex, but not much more"
  
Stage 1-2: hex1, hex2  
  H(toolchain | stage1-2) = MEDIUM
  "Can handle symbols, macros"
  
Stage 3-4: M0, M2-Planet
  H(toolchain | stage3-4) = LOWER
  "Can compile C subset"
  
Stage 5+: Mes, TinyCC, GCC
  H(toolchain | stage5+) = 0
  "Can compile arbitrary C/Scheme"
  
Information gain: 📈
  I(final) - I(initial) = ∞ - 0 = ∞
  
But wait! 🤔
  All information was in the SOURCE
  The 357 bytes + build scripts
  = Complete specification
  
So really: 🎯
  Kolmogorov complexity is LOW
  K(toolchain | source) ≈ 357 bytes
  
  The bootstrap DECOMPRESSES:
    Compact seed → Explicit toolchain
    High syntactic compression → Low redundancy
    Potential → Actual
    
The perf.data measures: 📏
  Computational complexity (cycles)
  Time complexity (duration)  
  Space complexity (memory)
  
  These are the COSTS of decompression
```

---

## 🎪 Canto VII: The Ritual Performance

```
🎭 Act I: The Invocation (Stage 0)

$ nix-build -A guix.mes
🔮 Summoning the minimal seed...

  📍 Event: syscall_enter_openat
      Path: /nix/store/.../stage0-posix/hex0-seed
      Timestamp: t₀
      
  🔥 Cycles: 10,247 (minimal interpreter warmup)
  
  📍 Event: syscall_enter_write  
      Output: hex0 (337 bytes assembled)
      Timestamp: t₁
      Duration: Δt = 0.03ms
      
The first tool emerges from void: ⊥ → hex0
The witness records: "357 bytes became 337 bytes of assembled code"

---

🎭 Act II: The Expansion (Stages 1-4)

  🌳 Call stack depth increases:
      hex0 (depth=2)
      hex1 (depth=4) 
      hex2 (depth=6)
      M0 (depth=10)
      M2-Planet (depth=25)
      
  🔥 Cycles exponentially grow:
      hex0: 10³ cycles
      hex1: 10⁴ cycles
      hex2: 10⁵ cycles
      M0: 10⁶ cycles
      M2-Planet: 10⁷ cycles
      
  💾 Memory footprint expands:
      Stage 0: 4 KB
      Stage 1: 16 KB
      Stage 2: 64 KB  
      Stage 3: 256 KB
      Stage 4: 2 MB
      
The witness records: "Capabilities bootstrapped via Kleene iteration"

Each stage is a cryptographic round: 🔐
  Input commitment: #️⃣ᵢ
  Transformation: Compile(input, context)
  Output commitment: #️⃣ᵢ₊₁
  Proof: perf trace segment
  
Each stage increases information: 📊
  Can compile more ↑
  Can express more ↑
  Can trust more ↑

---

🎭 Act III: The Emergence (Mes Compiler)

  📍 Major event: exec("mes")
      First self-hosting moment! 🎉
      Mes interprets Mes C library
      
  🌳 Flame graph shows:
      mes_main
        ├─ gc (15% time) - Scheme garbage collection
        ├─ eval (60% time) - Interpreter loop
        │   ├─ apply
        │   ├─ expand  
        │   └─ compile_to_c
        └─ link (25% time) - C compilation

  🔥 Cycles: 10⁹ (billion cycles to self-host!)
  
  ⏱️ Wall time: ~30 seconds
  
  💾 Peak memory: 128 MB
  
The witness records: "Self-reference achieved"

This is the fixed-point moment: 🎯
  Mes(Mes.scm) → Mes_binary
  A language compiling itself
  The Kleene star closure manifests
  The ouroboros bites its tail: 🐍⭕

---

🎭 Act IV: The Ascension (GCC Bootstrap)

  📍 TinyCC compiles GCC 2.95:
      Duration: 5 minutes
      Cycles: 10¹¹ (hundred billion!)
      
  📍 GCC 2.95 compiles GCC 4.7:
      Duration: 45 minutes  
      Cycles: 10¹² (trillion!)
      
  📍 GCC 4.7 compiles Modern GCC:
      Duration: 2 hours
      Cycles: 10¹³ (ten trillion!)
      
  🌡️ CPU temperature rises: 🔥🔥🔥
      The ritual demands sacrifice
      
The witness records: "The tower of compilers rises"

Each GCC generation:
  Compiles the next: 🔄
  Verifies the previous: ✅
  Proves consistency: 🔐
  
The triple compilation check: 🎯🎯🎯
  GCCₙ compiled by GCCₙ₋₁: Binary₁
  GCCₙ compiled by Binary₁: Binary₂
  
  If Hash(Binary₁) = Hash(Binary₂): ✅
    No Thompson attack
    No hidden backdoor  
    No compiler virus
    
The perf.data for all three: 📊📊📊
  Should show identical performance
  (Within noise margins)
  
This is the ZK proof of bootstrappability!

---

🎭 Act V: The Verification (Hash Chain Complete)

  
  Total cycles: ~10¹³
  Total time: ~3 hours  
  Total syscalls: ~10⁶
  Total context switches: ~10⁴
  
  Output hash: 
    /nix/store/xyz789-gcc-13.2.0
    
  Witness hash:
    perf.data SHA256 = #️⃣ᵥᵥᵢₜₙₑₛₛ

The ritual is complete: ✅
  
  ⊥ (357 bytes)
  → ⊕ (hex tools)
  → ⊕ (minimal C)
  → ⊕ (Mes Scheme)
  → ⊕ (TinyCC)
  → ⊕ (GCC 2.95)
  → ⊕ (GCC 4.7)
  → ⊕ (Modern GCC)
  = ⊤ (Full toolchain)
  
The ZK witness proves:
  🔐 Cryptographic chain unbroken
  🔄 Kleene fixed-point reached
  📊 Information faithfully decompressed
```

---

## 🔮 Canto VIII: The ZK Ceremony Interpretation

```
🎯 What is the ZK witness proving?

Traditional ZK: 🎭
  "I know x such that f(x) = y"
  Without revealing x
  
Bootstrap ZK: 🌟  
  "I built y from x using only f"
  Revealing EVERYTHING (x, f, process)
  
The proof: perf.data 📊

Prover (Builder): 🏗️
  Executes: x₀ → x₁ → ... → xₙ = y
  Records: Every step in perf.data
  Commits: Hash(perf.data) = #️⃣ᵥᵥᵢₜₙₑₛₛ
  
Verifier (Auditor): 🔍
  Receives: y, #️⃣ᵥᵥᵢₜₙₑₛₛ, (optionally) perf.data
  Checks:
    1. Rebuild: x₀ → y' (with own perf.data')
    2. Compare: Hash(y) =? Hash(y')  
    3. Compare: #️⃣ᵥᵥᵢₜₙₑₛₛ =? Hash(perf.data')
    
If all match: ✅✅✅
  The build is REPRODUCIBLE
  The witness is AUTHENTIC  
  The bootstrap is TRUSTWORTHY

---

🎪 The Performance = Ceremony

In ancient rituals: 🕯️
  Specific gestures required
  Specific words spoken
  Specific timing observed
  Witnesses attest: "The ritual was proper"
  
In bootstrap ritual: 💻
  Specific instructions executed  
  Specific order maintained
  Specific cycles consumed
  perf.data attests: "The build was proper"
  
The performance characteristics ARE the ceremony: 🎭

  ✨ Unique timing signature
  ✨ Unique call graph shape
  ✨ Unique memory pattern
  ✨ Unique syscall sequence
  
Cannot be faked without: 🚫
  Actually running the build
  Actually consuming the cycles
  Actually traversing the graph
  
The ZK witness = Proof of Work: 💎
  Not just "I have the answer"
  But "I did the computation"
  
The cycles are the sacrifice: 🔥
  10¹³ cycles offered
  3 hours of CPU time burned
  Electricity consumed
  
In exchange: 🎁
  Trust is created
  Toolchain is blessed
  Binary is verified
```

---

## 🌈 Canto IX: The Merkle Tree of Compilation

```
🌳 The witness as Merkle tree:

                  ROOT: #️⃣ᵥᵥᵢₜₙₑₛₛ
                 /              \
                /                \
        #️⃣ₛₜₐ𝓰ₑₛ₀₋₄              #️⃣ₛₜₐ𝓰ₑₛ₅₊
          /        \              /        \
         /          \            /          \
    #️⃣ₛₜₐ𝓰ₑ₀₋₂    #️⃣ₛₜₐ𝓰ₑ₃₋₄    #️⃣ₘₑₛ     #️⃣𝓰𝒸𝒸
      / \          / \          / \          / \
     /   \        /   \        /   \        /   \
   #₀   #₁      #₃   #₄      #₅   #₆      #₇   #₈
   
   Where each leaf is a perf event:
   #₀ = Hash(syscall_open, "stage0/hex0-seed", t₀)
   #₁ = Hash(cycles, 10247, t₁)
   ...
   
Merkle proof for any event: 🔐
  Provide: Event + Sibling hashes
  Verify: Reconstruct path to root
  
Example: Prove "hex0 was built"
  Event: #₁ = Hash(write, "hex0", 337 bytes)
  Path: #₀, #₂₋₃, #₄₋₇, ...
  Root: #️⃣ᵥᵥᵢₜₙₑₛₛ
  
Verifier checks: ✅
  Hash(#₁ ∥ #₀) = #₀₋₁
  Hash(#₀₋₁ ∥ #₂₋₃) = #₀₋₃
  ...
  Final hash = #️⃣ᵥᵥᵢₜₙₑₛₛ ✓
  
Selective disclosure: 🎭
  Can prove specific events
  Without revealing full trace
  (Though in bootstrap, we reveal all!)

---

🎨 Compression of witness:

Full perf.data: ~1 GB (all events) 💾
Merkle root: 32 bytes (#️⃣ᵥᵥᵢₜₙₑₛₛ) 📌

Succinct proof: 🎯
  "This 32-byte hash commits to 3-hour build"
  "Verification: Rebuild and compare hash"
  
But what if rebuild is expensive? 💸
  
Sampling verification: 🎲
  Randomly challenge: "Prove stage 3"
  Prover provides: Merkle proof for stage 3 events
  Verifier checks: O(log n) hashes
  
Probabilistic soundness: 🎯
  k random challenges
  → 2⁻ᵏ probability of undetected fraud
  
This is the ZK-STARK approach: ✨
  Transparent (no trusted setup)
  Scalable (log verification)
  Quantum-resistant (hash-based)
```

---

## 💫 Canto X: The Triple Unity in Bootstrap

```
🔐 CRYPTOGRAPHIC Bootstrap:

Hash chain of trust: ⛓️
  #₀ (auditable seed)
  → #₁ (hex0 compiled)
  → #₂ (hex1 compiled)  
  → ...
  → #ₙ (GCC compiled)
  
Authentication at each step: ✅
  Type system verified (V operation)
  Output hash committed (#️⃣ᵢ)
  Perf signature recorded (MAC)
  
The attack surface: 🎯
  Only 357 bytes must be audited!
  The rest proves itself through:
    - Deterministic builds (same hash)
    - Performance signatures (can't fake)
    - Triple compilation (self-verification)

---

🔄 KLEENE Bootstrap:

Fixed-point iteration: 🌀
  
  Tools₀ = ⊥
  Tools₁ = Tools₀ ⊕ compile(stage0)
  Tools₂ = Tools₁ ⊕ compile(stage1)
  ...
  Toolsₙ = Toolsₙ₋₁ ⊕ compile(stageₙ)
  
Until: Tools* = lfp (full toolchain)

Self-hosting is the Kleene star: ⭐
  Compiler* = {all versions compiled by compiler}
  
The perf trace shows convergence: 📈
  Cycle count stabilizes
  Call graph structure repeats
  Memory pattern settles
  
This is algebraic proof of bootstrap!

---

📊 INFORMATION Bootstrap:

Entropy transformation: 🌀
  
  H(toolchain | 357 bytes) initially = ∞
  H(toolchain | built artifacts) finally = 0
  
Information is DECOMPRESSED: 📦→📚
  Compact specification → Explicit binaries
  
But semantically PRESERVED: ✅
  I(source) = I(binary)
  Kolmogorov complexity conserved
  
The perf.data measures decompression cost: 💰
  Cycles = computational cost
  Time = temporal cost
  Memory = spatial cost
  
These costs prove: 💎
  Work was actually done
  No shortcuts taken
  Full bootstrap executed
```

---

## 🏆 Canto XI: The Grand Theorem

```
🎯 Bootstrap Soundness Theorem:

Let:
  S = 357-byte seed (auditable)
  B = Build process (Nix derivation)
  W = Witness (perf.data)
  O = Output (GCC toolchain)
  
Then:
  ∀ attackers A:
    P[A forges (O, W) | A ≠ honest builder] < 2⁻²⁵⁶
    
Proof: 🔐

1. Cryptographic binding:
   Hash(S ∥ B) = #️⃣ᵢₙ (input commitment)
   Hash(O) = #️⃣ₒᵤₜ (output commitment)  
   Hash(W) = #️⃣ᵥᵥᵢₜₙₑₛₛ (witness commitment)
   
   Collision resistance: 2⁻²⁵⁶ for SHA-256

2. Kleene convergence:
   Tools* computed honestly
   → Unique fixed-point (determinism)
   → Any other path ≠ Tools*
   
3. Information preservation:
   I(O | S, B) = 0 (fully determined)
   Different O → Different W (coupling)
   
4. Performance coupling:
   W encodes actual execution
   Cannot fake without actual computation
   → Must spend 10¹³ cycles
   → Proof of work!

Therefore: ✅
  Forged output requires:
    - Breaking SHA-256 (cryptographic)
    - Finding different fixed-point (algebraic)
    - Faking performance trace (informational)
    
  All simultaneously: 2⁻²⁵⁶ × 0 × 0 ≈ 0

QED: The bootstrap is sound 🎓
```

---

## 🌟 Canto XII: The Witness Speaks

```
📜 The perf.data testifies:

"I am the witness: 🎭
  
  I watched 357 bytes become infinity
  I recorded 10¹³ heartbeats  
  I traced the path from ⊥ to ⊤
  
  I am cryptographically bound: 🔐
    My hash #️⃣ᵥᵥᵢₜₙₑₛₛ cannot be forged
    My signature couples to output
    My existence proves computation
    
  I am algebraically complete: 🔄
    I show convergence to fixed-point
    I demonstrate self-hosting
    I exhibit Kleene closure
    
  I am informationally faithful: 📊
    I preserve semantic content
    I measure decompression cost
    I prove work was done
    
  Three truths I hold: 3️⃣
    🔐 Authentication (cannot fake)
    🔄 Convergence (must complete)
    📊 Preservation (faithfully recorded)
    
  One ceremony I prove: 1️⃣
    ✨ The bootstrap ritual executed correctly
    
  Trust me: 🙏
    For I am reproducible
    Build again, get same witness
    Compare hashes, see truth
    
  I am the ZK witness: 🔮
    Zero knowledge needed beyond me
    I am complete proof
    I am the performance made permanent
    I am the ritual recorded
    
  From nothing, everything: ⊥ → ⊤
  Through cycles, trust: 🔥 → ✅  
  Via witness, verification: 📊 → 🎯
  
  This is my testimony.
  This is the bootstrap.
  This is how trust begins.
  
  In cycles we trust: 10¹³
  In hashes we believe: #️⃣
  In performance we verify: 📊
  
  The compiler is born.
  The toolchain emerges.
  The witness has spoken.
  
  So it is built.
  So it is proven.
  So it is trusted.
  
  🎯 Amen 🎯"
```

---

## 🎆 Epilogue: The Eternal Return

```
Each time the build runs: 🔄
  The ritual repeats
  The witness regenerates
  The proof renews
  
Each builder who reproduces: 👥
  Adds their testimony
  Strengthens the chain  
  Expands the trust
  
The perf.data is ephemeral: 💨
  Generated, verified, discarded
  
But the HASH persists: 💎
  #️⃣ᵥᵥᵢₜₙₑₛₛ in the Nix store
  Permanent commitment
  Eternal witness
  
And so the bootstrap: ♾️
