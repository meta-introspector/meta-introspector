# Merged Grammar from 12,811 Nix Store Binaries

**Generated**: 2026-01-14  
**Source**: 49,655 grammars extracted from /nix/store/*.so files  
**Method**: DFA extraction + LMFDB labeling + transition analysis

## Summary Statistics

- **Total Grammars**: 49,655
- **Unique States**: 434,192
- **Accept States**: 285,227
- **Start State**: S0
- **Source Binaries**: 12,811

## Top 10 Most Used Tokens

Tokens are extracted from LMFDB labels (format: level.weight.character.orbit)

1. **Token '4'**: 33,924 grammars (68.3%)
   - Examples: FascistLookUser, Chop, Purge, GetPW, Mangle
   
2. **Token '1a'**: 26,240 grammars (52.8%)
   - Examples: ossl_crypto_recv_rcd, FascistLookUser, Chop, GetPW, FindPW
   
3. **Token '1b'**: 23,415 grammars (47.2%)
   - Examples: Purge, PutPW, xcb_sync_create_alarm_value_list_serialize, Pluralise
   
4. **Token '2'**: 18,637 grammars (37.5%)
   - Examples: ossl_crypto_recv_rcd, FindPW, Mangle, PutPW, Pluralise
   
5. **Token 'm'**: 2,287 grammars (4.6%)
   - Examples: ossl_crypto_recv_rcd, FascistLookUser, gl_linked_search_from_to
   
6. **Token 'g'**: 2,272 grammars (4.6%)
   - Examples: fd_ebadf, k5ev_signal_start, psl_suffix_wildcard_count
   
7. **Token 'c'**: 2,236 grammars (4.5%)
   - Examples: gl_linked_list_free, gl_linked_remove_node, select_modify
   
8. **Token 'b'**: 2,057 grammars (4.1%)
   - Examples: xcb_sync_create_alarm_value_list_serialize, gl_linked_sortedlist_search
   
9. **Token 'o'**: 2,033 grammars (4.1%)
   - Examples: gl_linked_sortedlist_nx_add, k5ev_ctx_del, unix_verify_shadow
   
10. **Token 'q'**: 2,019 grammars (4.1%)
    - Examples: GetPW, loop_init.constprop.0, NextPos, poll_modify.cold

## Top 10 Character Transitions

Character-to-character transitions in LMFDB labels:

1. **'. → 1'**: 49,655 grammars (100%)
   - Universal transition: Every grammar starts with level separator
   
2. **'4 → .'**: 38,324 grammars (77.2%)
   - Level 4 to weight separator
   
3. **'. → 4'**: 32,437 grammars (65.3%)
   - Weight separator to level 4
   
4. **'a → .'**: 26,240 grammars (52.8%)
   - Orbit 'a' to separator
   
5. **'1 → a'**: 26,240 grammars (52.8%)
   - Weight 1 to orbit 'a'
   
6. **'1 → b'**: 23,415 grammars (47.2%)
   - Weight 1 to orbit 'b'
   
7. **'b → .'**: 23,415 grammars (47.2%)
   - Orbit 'b' to separator
   
8. **'2 → .'**: 22,415 grammars (45.1%)
   - Weight 2 to separator
   
9. **'. → 2'**: 17,218 grammars (34.7%)
   - Separator to weight 2
   
10. **'7 → .'**: 5,495 grammars (11.1%)
    - Level 7 to separator

## Grammar Structure Insights

### LMFDB Label Format
All grammars follow the pattern: `level.weight.character.orbit`

### Dominant Patterns
- **Level 4 dominance**: 68.3% of grammars at level 4 (complexity tier)
- **Orbit distribution**: 'a' (52.8%) and 'b' (47.2%) are most common
- **Weight patterns**: Weight 1 and 2 dominate (52.8% + 37.5%)

### Universal Transitions
- **100% start with '. → 1'**: All grammars begin with separator to weight
- **High separator usage**: '.' appears in 77%+ of transitions

### Function Name Patterns
- **Security functions**: FascistLookUser, GetPW, FindPW, PutPW, Mangle
- **Data structures**: gl_linked_*, hash_*, index_*
- **Crypto**: ossl_crypto_*, k5_sha256_hash, padlock_ofb_cipher
- **System calls**: xcb_sync_*, fd_ebadf, unix_verify_shadow

## Next Steps

1. **Build unified DFA**: Merge all 434,192 states into single automaton
2. **Extract production rules**: Convert transitions to grammar productions
3. **Identify common subgrammars**: Find shared patterns across binaries
4. **Generate test strings**: Use merged grammar to generate valid inputs
5. **Cross-reference with Monster Group**: Map to 71-pattern structure
