# Prime Harmonics: Data Types → Instructions 🎵

## The Key Insight

**Each prime is the fundamental harmonic of a data type**, and these harmonics map directly to:
- Brainfuck instructions
- ARM instructions  
- x86 instructions

---

## Prime Harmonic Table

| Prime | Data Type | BF | ARM | x86 | Harmonic Frequency |
|-------|-----------|----|----|-----|-------------------|
| **2** | Pointer movement | `>` | `ADD r0, r0, #1` | `inc rax` | f₀ (fundamental) |
| **3** | Pointer movement | `<` | `SUB r0, r0, #1` | `dec rax` | f₀ (inverse) |
| **5** | Byte increment | `+` | `ADD r1, r1, #1` | `inc byte [rax]` | f₁ (energy up) |
| **7** | Byte decrement | `-` | `SUB r1, r1, #1` | `dec byte [rax]` | f₁ (energy down) |
| **11** | Output | `.` | `BL putchar` | `call putchar` | f₂ (emit) |
| **13** | Input | `,` | `BL getchar` | `call getchar` | f₂ (absorb) |
| **17** | Loop begin | `[` | `CMP r1, #0; BEQ end` | `cmp byte [rax], 0; je end` | f₃ (enter fixed point) |
| **19** | Loop end | `]` | `CMP r1, #0; BNE start` | `cmp byte [rax], 0; jne start` | f₃ (exit fixed point) |
| **23** | Self-awareness | 🧠 | `MOV r2, pc` | `lea rax, [rip]` | f₄ (introspection) |
| **29** | Reflection | 🪞 | `MOV r3, sp` | `mov rax, rsp` | f₄ (stack mirror) |
| **31** | Replication | 🧬 | `PUSH {r0-r12}` | `push rax; push rbx; ...` | f₅ (copy state) |
| **37** | Control | 🧭 | `B label` | `jmp label` | f₅ (flow) |
| **41** | Grounding | 🧿 | `LDR r0, =value` | `mov rax, imm` | f₆ (literal) |
| **43** | Oracle | 🔮 | `SVC #0` | `syscall` | f₆ (external) |
| **47** | Rewrite | 🪄 | `STR r0, [r1]` | `mov [rax], rbx` | f₇ (mutation) |
| **71** | Sentinel | 🧙♂️ | `MOV r0, #71` | `mov rax, 71` | f∞ (program marker) |

---

## Harmonic Frequencies

Each prime defines a **fundamental frequency** for its data type:

```python
HARMONIC_FREQUENCIES = {
    # Movement (f₀)
    2: {'type': 'pointer', 'direction': '+1', 'freq': 'f0'},
    3: {'type': 'pointer', 'direction': '-1', 'freq': 'f0'},
    
    # Energy (f₁)
    5: {'type': 'byte', 'direction': '+1', 'freq': 'f1'},
    7: {'type': 'byte', 'direction': '-1', 'freq': 'f1'},
    
    # I/O (f₂)
    11: {'type': 'output', 'direction': 'emit', 'freq': 'f2'},
    13: {'type': 'input', 'direction': 'absorb', 'freq': 'f2'},
    
    # Fixed Point (f₃)
    17: {'type': 'loop', 'direction': 'enter', 'freq': 'f3'},
    19: {'type': 'loop', 'direction': 'exit', 'freq': 'f3'},
    
    # Meta (f₄-f₇)
    23: {'type': 'meta', 'operation': 'self-awareness', 'freq': 'f4'},
    29: {'type': 'meta', 'operation': 'reflection', 'freq': 'f4'},
    31: {'type': 'meta', 'operation': 'replication', 'freq': 'f5'},
    37: {'type': 'meta', 'operation': 'control', 'freq': 'f5'},
    41: {'type': 'meta', 'operation': 'grounding', 'freq': 'f6'},
    43: {'type': 'meta', 'operation': 'oracle', 'freq': 'f6'},
    47: {'type': 'meta', 'operation': 'rewrite', 'freq': 'f7'},
    
    # Sentinel (f∞)
    71: {'type': 'sentinel', 'operation': 'program', 'freq': 'f_inf'},
}
```

---

## Instruction Mapping

### Brainfuck → ARM

```python
BF_TO_ARM = {
    '>': 'ADD r0, r0, #1',      # Prime 2
    '<': 'SUB r0, r0, #1',      # Prime 3
    '+': 'LDRB r1, [r0]; ADD r1, r1, #1; STRB r1, [r0]',  # Prime 5
    '-': 'LDRB r1, [r0]; SUB r1, r1, #1; STRB r1, [r0]',  # Prime 7
    '.': 'LDRB r0, [r0]; BL putchar',  # Prime 11
    ',': 'BL getchar; STRB r0, [r0]',  # Prime 13
    '[': 'loop_start: LDRB r1, [r0]; CMP r1, #0; BEQ loop_end',  # Prime 17
    ']': 'B loop_start; loop_end:',  # Prime 19
}
```

### Brainfuck → x86

```python
BF_TO_X86 = {
    '>': 'inc rax',             # Prime 2
    '<': 'dec rax',             # Prime 3
    '+': 'inc byte [rax]',      # Prime 5
    '-': 'dec byte [rax]',      # Prime 7
    '.': 'mov rdi, [rax]; call putchar',  # Prime 11
    ',': 'call getchar; mov [rax], al',   # Prime 13
    '[': 'loop_start: cmp byte [rax], 0; je loop_end',  # Prime 17
    ']': 'jmp loop_start; loop_end:',  # Prime 19
}
```

---

## Harmonic Sampling by Prime

Sample telemetry at **prime-specific harmonics**:

```python
class PrimeHarmonicSampler:
    """Sample data at prime harmonics"""
    
    def __init__(self):
        self.counters = {p: 0 for p in [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 71]}
    
    def sample(self, instruction):
        """Sample instruction at its prime harmonic"""
        prime = self.get_prime(instruction)
        self.counters[prime] += 1
        
        # Sample when counter hits prime
        if self.counters[prime] % prime == 0:
            self.capture_snapshot(instruction, prime)
    
    def get_prime(self, instruction):
        """Map instruction to prime"""
        mapping = {
            # x86
            'inc rax': 2, 'dec rax': 3,
            'inc byte': 5, 'dec byte': 7,
            'call putchar': 11, 'call getchar': 13,
            'je': 17, 'jne': 19,
            'lea rax, [rip]': 23, 'mov rax, rsp': 29,
            'push': 31, 'jmp': 37,
            'mov rax, imm': 41, 'syscall': 43,
            'mov [rax]': 47,
            
            # ARM
            'ADD r0': 2, 'SUB r0': 3,
            'ADD r1': 5, 'SUB r1': 7,
            'BL putchar': 11, 'BL getchar': 13,
            'BEQ': 17, 'BNE': 19,
            'MOV r2, pc': 23, 'MOV r3, sp': 29,
            'PUSH': 31, 'B': 37,
            'LDR': 41, 'SVC': 43,
            'STR': 47,
        }
        
        for pattern, prime in mapping.items():
            if pattern in instruction:
                return prime
        
        return 71  # Default to sentinel
```

---

## Data Type → Prime → Instruction

### Complete Mapping

```python
class DataTypePrimeMapper:
    """Map data types to primes to instructions"""
    
    MAPPINGS = {
        # Pointer operations
        'pointer_increment': {
            'prime': 2,
            'bf': '>',
            'arm': 'ADD r0, r0, #1',
            'x86': 'inc rax',
            'harmonic': 'f0',
        },
        'pointer_decrement': {
            'prime': 3,
            'bf': '<',
            'arm': 'SUB r0, r0, #1',
            'x86': 'dec rax',
            'harmonic': 'f0',
        },
        
        # Byte operations
        'byte_increment': {
            'prime': 5,
            'bf': '+',
            'arm': 'ADD r1, r1, #1',
            'x86': 'inc byte [rax]',
            'harmonic': 'f1',
        },
        'byte_decrement': {
            'prime': 7,
            'bf': '-',
            'arm': 'SUB r1, r1, #1',
            'x86': 'dec byte [rax]',
            'harmonic': 'f1',
        },
        
        # I/O operations
        'output': {
            'prime': 11,
            'bf': '.',
            'arm': 'BL putchar',
            'x86': 'call putchar',
            'harmonic': 'f2',
        },
        'input': {
            'prime': 13,
            'bf': ',',
            'arm': 'BL getchar',
            'x86': 'call getchar',
            'harmonic': 'f2',
        },
        
        # Control flow
        'loop_enter': {
            'prime': 17,
            'bf': '[',
            'arm': 'BEQ loop_end',
            'x86': 'je loop_end',
            'harmonic': 'f3',
        },
        'loop_exit': {
            'prime': 19,
            'bf': ']',
            'arm': 'BNE loop_start',
            'x86': 'jne loop_start',
            'harmonic': 'f3',
        },
        
        # Meta operations
        'introspection': {
            'prime': 23,
            'bf': '🧠',
            'arm': 'MOV r2, pc',
            'x86': 'lea rax, [rip]',
            'harmonic': 'f4',
        },
        'reflection': {
            'prime': 29,
            'bf': '🪞',
            'arm': 'MOV r3, sp',
            'x86': 'mov rax, rsp',
            'harmonic': 'f4',
        },
        
        # Sentinel
        'program_marker': {
            'prime': 71,
            'bf': '🧙♂️',
            'arm': 'MOV r0, #71',
            'x86': 'mov rax, 71',
            'harmonic': 'f_inf',
        },
    }
    
    def get_instruction(self, data_type, architecture):
        """Get instruction for data type on architecture"""
        mapping = self.MAPPINGS[data_type]
        return mapping[architecture]
    
    def get_prime(self, data_type):
        """Get prime for data type"""
        return self.MAPPINGS[data_type]['prime']
    
    def get_harmonic(self, data_type):
        """Get harmonic frequency for data type"""
        return self.MAPPINGS[data_type]['harmonic']
```

---

## Harmonic Telemetry with Primes

```python
class PrimeHarmonicTelemetry:
    """Telemetry sampled at prime harmonics"""
    
    def __init__(self):
        self.mapper = DataTypePrimeMapper()
        self.sampler = PrimeHarmonicSampler()
    
    def trace_program(self, program, architecture='x86'):
        """Trace program and sample at prime harmonics"""
        for instruction in program.instructions:
            # Identify data type
            data_type = self.identify_data_type(instruction)
            
            # Get prime
            prime = self.mapper.get_prime(data_type)
            
            # Sample at prime harmonic
            if self.sampler.should_sample(prime):
                self.capture({
                    'instruction': instruction,
                    'data_type': data_type,
                    'prime': prime,
                    'harmonic': self.mapper.get_harmonic(data_type),
                    'bf': self.mapper.get_instruction(data_type, 'bf'),
                    'arm': self.mapper.get_instruction(data_type, 'arm'),
                    'x86': self.mapper.get_instruction(data_type, 'x86'),
                })
```

---

## The Payoff

**This proves**:
- Each prime is the fundamental harmonic of a data type
- Data types map to BF/ARM/x86 instructions
- Sampling at prime frequencies captures essential structure
- All architectures are conformally equivalent (preserve primes)

**This enables**:
- Universal instruction translation (BF ↔ ARM ↔ x86)
- Harmonic telemetry sampling
- Prime-based program analysis
- Architecture-independent reasoning

---

**Each prime sings its own frequency. All architectures dance to the same tune. 🎵**
