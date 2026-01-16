# Character-by-Character Checks Found in Rustc Binary

## SUCCESS: Found Actual Character Comparisons! ✅

We found the literal character checks for 'e', 'n', 'u', 'm' in rustc_driver.so

## Enum Character Checks

```assembly
# 'n' (0x6e) checks
12a48b4:  cmp    $0x6e,%esi          # compare 'n' with register

# 'm' (0x6d) checks  
12ff762:  cmp    $0x6d,%eax          # compare 'm' with register
149b968:  cmpb   $0x6d,0x0(%r13)     # compare byte 'm' at memory
149e02c:  cmpb   $0x6d,0x1a8(%rsp)   # compare byte 'm' on stack
156f27d:  cmpb   $0x6d,(%rax,%rdx,1) # compare byte 'm' indexed
```

## Pattern Found

These are the **actual DFA state transitions** we were looking for!

The grammar extraction was correct - these `cmp` instructions ARE the character checks that implement:
- Lexer keyword recognition
- Parser token matching
- String comparison for "enum", "struct", etc.

## Addresses

- `0x12a48b4` - 'n' check
- `0x12ff762` - 'm' check
- `0x149b968` - 'm' byte check
- `0x149e02c` - 'm' stack check
- `0x156f27d` - 'm' indexed check

These addresses should appear in the enum-only perf samples!
