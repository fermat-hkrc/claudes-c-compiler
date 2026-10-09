# PBT Coverage Status

> Last updated: 2026-10-09 (campaign: encode_lmsw)
> Coverage evidence: file-level (symbol presence) — no .gcda/.profraw from Rust run; `coverage_gaps` listed unrelated OH C++ binaries and reported encode_lmsw NOT LINKED there. Real execution confirmed by `cargo test --lib encode_lmsw` (12 pass / 9 fail including KAT/regression).

## Summary

| Metric | Value |
|--------|-------|
| Campaign target | encode_lmsw |
| Source | src/backend/i686/assembler/encoder/system.rs:203 |
| Properties | 11 (8 passing, 3 failing) |
| Bugs | 2 root-cause / 3 reports (missing segment prefix base+disp + SIB; accepts non-r16 register) |
| Documented behaviors with properties | arity, 0F 01 /6 invariant, r16 form, base/disp/SIB/abs mem, segment prefix, non-r16 reject, imm/label reject, metamorphic vs lidt |
| Contract-surface sweep | 1 round (standard) — all documented behaviors property-covered; coverage_gaps file-level only; no additional gap properties owed |

## This campaign

| Function | Source file | Tested | Result |
|----------|-------------|--------|--------|
| encode_lmsw | system.rs | yes | 8 pass / 3 fail (2 root-cause bugs) |

## Oracle Type Distribution (this campaign)

| Oracle Type | Count |
|-------------|-------|
| differential | 7 |
| algebraic.invariant | 1 |
| algebraic.metamorphic | 1 |
| negative_error | 2 |
