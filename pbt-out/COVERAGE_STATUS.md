# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_fp_arith, English, standard)
> Scope: src/backend/riscv/assembler/encoder/float.rs — single symbol encode_fp_arith
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter; Rust cargo tests are not instrumented that way). Manual execution evidence: 5 KAT + 8 properties ran against the production symbol.

## Summary

| Metric | Value |
|--------|-------|
| Campaign target | encode_fp_arith (float.rs:61) |
| Properties | 8 |
| Passing | 6 |
| Failing | 2 |
| Bugs | 2 |
| KAT | 5 passing |
| Regression witnesses | 2 (failing, as expected) |
| Generator runs | 1000 (proptest cases) |
| Sweep rounds | 1 (coverage_gaps: no line-level data; all documented behaviors already have a property) |

## This campaign

| Function | Source file | Tested | Result |
|----------|-------------|--------|--------|
| encode_fp_arith | float.rs | yes | 6 passing / 2 failing (extra operand, non-rm 4th) |

Other float.rs symbols were not tested (HARD: test only encode_fp_arith).
