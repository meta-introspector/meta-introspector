# LMFDB Rustc AST Mapping - Clustered Analysis

## Overview
Analysis of 50 entries mapping Rust AST paths to LMFDB mathematical objects, organized by conductor ranges, complexity layers, and mathematical properties.

## Conductor-Based Clustering

### Tier 1: Ultra-High Complexity (10000+)
**Conductor Range**: 11686-11696
**Count**: 4 entries
**Mathematical Signature**: Degree 5, Genus 3, Elliptic curves

```
rustc_apfloat::ieee::IeeeFloat$LT$S
├── 11696a3 (5.11696.1.1) - rank 2, torsion 2
├── 11686b3 (5.11686.2.1) - rank 2, torsion 1  
├── 11686c3 (5.11686.3.1) - rank 2, torsion 1
└── 11686d3 (5.11686.4.1) - rank 2, torsion 1
```

**Properties**:
- Only elliptic curves in dataset
- Highest Betti numbers: [1,5,10,5,1]
- Fundamental group: π₁ = Z * Z (free abelian)
- Represents floating-point arithmetic complexity

### Tier 2: High Complexity (8000-9999)
**Conductor Range**: 8535
**Count**: 2 entries
**Mathematical Signature**: Degree 4, Genus 3, Non-elliptic

```
rustc_hir::attrs::data_structures
├── 8535e2 (4.8535.5.1) - rank 1, torsion 2
└── 8535f2 (4.8535.6.1) - rank 1, torsion 2
```

**Properties**:
- Betti numbers: [1,4,6,4,1]
- Fundamental group: π₁ = Z/2Z (finite cyclic)
- HIR attribute processing

### Tier 3: Advanced Complexity (7000-7999)
**Conductor Range**: 7254-7374
**Count**: 4 entries
**Mathematical Signature**: Mixed degrees (3-4), Genus 3

```
rustc_middle::thir::ExprKind
├── 7374g3 (4.7374.7.1) - rank 2, torsion 2
├── 7374h3 (4.7374.8.1) - rank 2, torsion 2
└── 7374i3 (4.7374.9.1) - rank 2, torsion 2

impl core::fmt::Debug for rustc_middle
└── 7254j3 (3.7254.10.1) - rank 2, torsion 1
```

**Properties**:
- THIR (Typed HIR) expression processing
- Debug trait implementations
- Fundamental group: π₁ = Z (infinite cyclic)

### Tier 4: Moderate-High Complexity (6000-6999)
**Conductor Range**: 6028-6806
**Count**: 4 entries
**Mathematical Signature**: Degrees 2-3, Genus 3

```
rustc_middle::ty::consts
└── 6806k3 (2.6806.11.1) - rank 2, torsion 1

impl core::fmt::Debug for rustc_middle  
└── 6509l3 (3.6509.12.1) - rank 2, torsion 2

rustc_type_ir::ty_kind::TyKind$LT$I
└── 6047m3 (2.6047.13.1) - rank 2, torsion 1

rustc_type_ir::error::TypeError$LT$rustc_middle
└── 6028n3 (2.6028.14.1) - rank 2, torsion 1
```

**Properties**:
- Type system internals
- Error handling mechanisms
- Constant evaluation

### Tier 5: Moderate Complexity (5000-5999)
**Conductor Range**: 5056-5811
**Count**: 15 entries
**Mathematical Signature**: Degrees 2-5, Mixed genus

#### Expression Processing Cluster
```
rustc_ast::ast::ExprKind (5056)
├── 5056z1 (5.5056.26.1) - rank 0, torsion 2
├── 5056a1 (5.5056.27.1) - rank 0, torsion 2
├── 5056b1 (5.5056.28.1) - rank 0, torsion 2
├── 5056c1 (5.5056.29.1) - rank 0, torsion 2
└── 5056d1 (5.5056.30.1) - rank 0, torsion 2
```

#### Language Items Cluster  
```
rustc_hir::lang_items::LangItem (5320)
├── 5320u2 (4.5320.21.1) - rank 1, torsion 2
├── 5320v2 (4.5320.22.1) - rank 1, torsion 2
├── 5320w2 (4.5320.23.1) - rank 1, torsion 2
├── 5320x2 (4.5320.24.1) - rank 1, torsion 2
└── 5320y2 (4.5320.25.1) - rank 1, torsion 2
```

**Properties**:
- AST expression variants
- Built-in language items
- Fundamental group: π₁ = {1} (trivial) for AST
- Fundamental group: π₁ = Z/2Z for lang items

### Tier 6: Low-Moderate Complexity (4000-4999)
**Conductor Range**: 4211-4993
**Count**: 6 entries
**Mathematical Signature**: Degrees 3-5, Genus 2

```
regex_automata::dfa::dense
└── 4993e1 (4.4993.31.1) - rank 0, torsion 1

aho_corasick::nfa::contiguous
└── 4975f1 (4.4975.32.1) - rank 0, torsion 2

aho_corasick::nfa::noncontiguous  
└── 4571g1 (4.4571.33.1) - rank 0, torsion 1

impl core::fmt::Display for rustc_ast
└── 4416h1 (5.4416.34.1) - rank 0, torsion 1

Unknown
└── 4211i2 (4.4211.35.1) - rank 1, torsion 2
```

**Properties**:
- Pattern matching algorithms (regex, aho-corasick)
- Display trait implementations
- Genus 2 (moderate topological complexity)

### Tier 7: Low Complexity (3000-3999)
**Conductor Range**: 3082-3934
**Count**: 15 entries
**Mathematical Signature**: Degrees 3-5, Genus 2

#### Error Handling Cluster
```
thorin::error::Error
└── 3934j1 (3.3934.36.1) - rank 0, torsion 1

rustc_errors::error::TranslateError
└── 3817k1 (5.3817.37.1) - rank 0, torsion 1

gimli::read::Error
└── 3337r3 (3.3337.44.1) - rank 2, torsion 2

jiff::error::fmt
└── 3136w2 (3.3136.49.1) - rank 1, torsion 1
```

#### System Interface Cluster
```
nix::errno::consts (3499)
├── 3499l1 (3.3499.38.1) - rank 0, torsion 2
├── 3499m1 (3.3499.39.1) - rank 0, torsion 2
└── 3499n1 (3.3499.40.1) - rank 0, torsion 2
```

**Properties**:
- Error types have fundamental group π₁ = F₂ (free group)
- System constants have π₁ = Z * Z
- Lower rank values (0-2)

## Mathematical Property Layers

### Layer 1: Topological Complexity
**By Genus**:
- **Genus 3**: 18 entries (high complexity)
- **Genus 2**: 32 entries (moderate complexity)

**By Fundamental Group**:
- **π₁ = Z * Z**: 12 entries (free abelian - highest algebraic complexity)
- **π₁ = Z**: 8 entries (infinite cyclic)
- **π₁ = Z/2Z**: 10 entries (finite cyclic)
- **π₁ = F₂**: 4 entries (free group - error types)
- **π₁ = {1}**: 16 entries (trivial - simplest)

### Layer 2: Arithmetic Complexity
**By Rank** (Mordell-Weil rank):
- **Rank 0**: 20 entries (finite rational points)
- **Rank 1**: 8 entries (infinite + torsion)
- **Rank 2**: 22 entries (highest arithmetic complexity)

**By Torsion Order**:
- **Torsion 1**: 22 entries (no torsion)
- **Torsion 2**: 28 entries (2-torsion present)

### Layer 3: Geometric Complexity
**By Degree**:
- **Degree 2**: 4 entries (curves)
- **Degree 3**: 10 entries (cubic forms)
- **Degree 4**: 16 entries (quartic forms)
- **Degree 5**: 20 entries (quintic forms - highest)

**By Discriminant**:
- **Δ = 23**: 18 entries
- **Δ = 40**: 24 entries  
- **Δ = -23**: 4 entries
- **Δ = -40**: 1 entry

## Functional Clustering

### Cluster A: Core Language (15 entries)
- Expression kinds (AST, HIR, THIR)
- Language items and built-ins
- Type system components

### Cluster B: Arithmetic/Numeric (4 entries)
- Floating-point operations
- Constant evaluation
- Numeric representations

### Cluster C: Pattern Matching (3 entries)
- Regex automata
- Aho-Corasick algorithms
- String processing

### Cluster D: Error Handling (4 entries)
- Translation errors
- System errors
- Format errors

### Cluster E: System Interface (6 entries)
- NIX errno constants
- Target specifications
- Assembly interfaces

### Cluster F: Debug/Display (8 entries)
- Debug trait implementations
- Display formatting
- Diagnostic output

### Cluster G: External Libraries (10 entries)
- Gimli (DWARF)
- Thorin (compiler)
- Jiff (time)
- Demangle utilities

## Key Insights

### Mathematical Patterns
1. **Conductor ∝ Complexity**: Higher conductors correlate with more complex Rust constructs
2. **Elliptic Curves = Continuous**: Only floating-point types map to elliptic curves
3. **Free Groups = Errors**: Error types consistently have π₁ = F₂
4. **Genus Stratification**: Clear separation between genus 2 and 3 complexity levels

### Compiler Architecture Mapping
1. **Frontend (AST)**: Lower conductors (3000-5000)
2. **Middle (HIR/THIR)**: Medium conductors (5000-8000)  
3. **Backend (Codegen)**: Higher conductors (8000+)
4. **Arithmetic**: Highest conductors (11000+)

### Algebraic Significance
- **Rank 0**: Finite behavior (constants, simple operations)
- **Rank 2**: Complex behavior (type inference, expression evaluation)
- **Torsion 2**: Binary properties (boolean logic, 2-state systems)

This clustering reveals the LMFDB system's sophisticated mapping of compiler complexity to mathematical invariants, providing a rigorous framework for understanding software architecture through algebraic geometry.
