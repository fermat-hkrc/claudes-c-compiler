# PBT Coverage Status

> Last updated: 2026-10-09 (campaign: encode_bswap)
> Coverage evidence: file-level (symbol presence) — no .gcda/.profraw from cargo host run
> Change-surface target encode_bswap: tested (8 properties, linked in ccc libtest binary per `nm`)

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_bswap (gp_integer.rs:945) |
| Properties | 8 (6 passing, 2 failing) |
| Bugs | 2 medium |
| Test binary | target/debug/deps/ccc-70d56e2a1978a8d3 |
| Symbol evidence | `InstructionEncoder::encode_bswap` present (nm); error strings `bswap requires 1 operand` / `bswap requires register operand` in binary |
| coverage_gaps | no line-level; matcher reported NOT LINKED (Rust name mangling false negative) — overridden by nm + executed KAT/differential |

## Sweep decision

Documented behaviors of encode_bswap:
1. r32 success → differential + invariant + metamorphic (covered, passing)
2. arity ≠ 1 → neg_arity (covered, passing)
3. non-register → neg_non_register (covered, passing)
4. wrong width r16/r8 → neg_wrong_width (covered, failing = bug B1)
5. non-GP aliased → neg_non_gp (covered, failing = bug B2)
6. sreg/cr unknown → neg_unknown_reg (covered, passing; strengthen round)

No further documented branch without a property. Sweep closed.
