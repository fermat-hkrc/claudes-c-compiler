# Coverage Status — encode_push campaign

**Date:** 2026-10-09
**Tier:** standard
**Coverage evidence:** file-level (symbol presence) + cargo execution

## Summary

| Metric | Value |
|--------|-------|
| Target function | encode_push |
| Source | gp_integer.rs:346 |
| Test file | encode_push_pbt.rs |
| Properties | 12 (6 passing, 6 failing) |
| Bugs | 4 |
| Line-level .profraw | none (coverage_gaps: no Rust instrumentation data; C++ binary fallback N/A) |
| Execution evidence | `cargo test --lib encode_push_` → 14 passed, 11 failed (real symbol exercised) |

## Branches / behaviors

| Behavior | Property | Status |
|----------|----------|--------|
| r32 short 50+n | p1, p5 | passing |
| imm8/imm32 6A/68 | p2, p6 | passing |
| mem FF /6 no seg | p3 | passing |
| mem + segment override | p4, p7 | failing (B1) |
| arity ≠1 Err | p8 | passing |
| reject xmm | p9 | failing (B2) |
| reject r8 | p10 | failing (B2) |
| Sreg push forms | p11 | failing (B3) |
| r16 + 0x66 | p12 | failing (B4) |
| symbol imm 0x68+reloc (sweep) | encode_push_invariant_symbol_imm / KAT | passing |
| mixed extra ops (sweep) | encode_push_neg_extra_mixed | passing |

## Sweep

One `coverage_gaps` round (standard tier). Tool: no line data; file-level C++ binaries unrelated. Cargo evidence + added symbol-imm and mixed-arity properties cover remaining documented arms. Close.
