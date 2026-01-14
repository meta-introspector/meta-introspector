# Rustc Profile Comparison: Enum vs Struct

**Generated**: 2026-01-14
**Tool**: measureme/summarize
**Method**: Compare rustc --self-profile between enum and struct compilation

## Key Differences

### Enum-Specific (Higher Time):
- `metadata_decode_entry_adt_def`: **28.85µs** (enum) vs 18.97µs (struct) - **52% more**
- `check_mod_deathness`: **2.66ms** (enum) vs 1.82ms (struct) - **46% more**

### Struct-Specific (Higher Time):
- `type_of`: 56.50µs (struct) vs **19.13µs** (enum) - **195% more**
- `adt_destructor`: 15.37µs (struct) vs **6.01µs** (enum) - **156% more**

## Full Comparison

| Query | Enum Time | Struct Time | Difference |
|-------|-----------|-------------|------------|
| check_mod_deathness | 2.66ms | 1.82ms | +46% enum |
| metadata_decode_entry_type_of | 55.29µs | 52.94µs | +4% enum |
| check_mod_unstable_api_usage | 46.12µs | 43.23µs | +7% enum |
| check_mod_attrs | 41.01µs | 27.94µs | +47% enum |
| metadata_decode_entry_adt_def | 28.85µs | 18.97µs | +52% enum |
| type_of | 19.13µs | 56.50µs | +195% struct |
| adt_def | 11.05µs | 26.15µs | +137% struct |
| adt_destructor | 6.01µs | 15.37µs | +156% struct |
| check_mod_privacy | 4.42µs | 5.19µs | +17% struct |
| adt_sizedness_constraint | 3.81µs | 3.41µs | +12% enum |

## Auto-Labeling Signatures

Based on profile differences, we can auto-label:

**Enum Signature**:
- High `metadata_decode_entry_adt_def` (28µs+)
- High `check_mod_deathness` (2.5ms+)
- Low `type_of` (<20µs)

**Struct Signature**:
- High `type_of` (50µs+)
- High `adt_destructor` (15µs+)
- Low `metadata_decode_entry_adt_def` (<20µs)

## Next Steps

1. Extract these query names from the 49,655 grammar profiles
2. Match signatures to auto-label enum vs struct grammars
3. Map to character sequences (e→n→u→m vs s→t→r→u→c→t)
4. Build training set for grammar classification
