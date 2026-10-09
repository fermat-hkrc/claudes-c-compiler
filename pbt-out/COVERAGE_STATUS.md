# Coverage Status — encode_bit_count campaign

**Tier:** standard
**Coverage evidence:** file-level (symbol presence) — no native line coverage (.gcda/.profraw absent)
**coverage_gaps:** reported NOT LINKED for `encode_bit_count` (Rust mangling false negative). nm confirms:

```
_ZN3ccc7backend4i6869assembler7encoder10gp_integer76_$LT$impl$u20$ccc..backend..i686..assembler..encoder..InstructionEncoder$GT$16encode_bit_count17h0e4a317da3c81d5eE
```

in `target/debug/deps/ccc-70d56e2a1978a8d3`. KAT + differential properties execute the real symbol.

| Metric | Value |
|--------|-------|
| Target function | encode_bit_count |
| Properties | 9 (6 pass / 3 fail) |
| KAT | 4 pass |
| Strengthen round | 1 (meta mnemonic opcodes) pass |
| Regression witnesses | 3 fail (as expected) |
| Bugs filed | 3 |
| Sweep | 1 round; documented behaviors covered (rr success, mem, arity, width, non-GP, imm/label, opc invariant/meta) |

**Sweep close reason:** tier rounds done — all documented encode_bit_count behaviors have properties; no additional gap properties required after nm/KAT proof of link/execution.
