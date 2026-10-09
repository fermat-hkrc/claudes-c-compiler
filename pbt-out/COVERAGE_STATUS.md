# PBT Coverage Status

> Last updated: 2026-10-09 (campaign: encode_system_table)
> Coverage evidence: file-level (symbol presence) — no .gcda/.profraw from Rust run; `coverage_gaps` listed unrelated OH C++ binaries. Real execution confirmed by `cargo test --lib encode_system_table` (13 pass / 6 fail including KAT/regression).

## Summary

| Metric | Value |
|--------|-------|
| Campaign target | encode_system_table |
| Source | src/backend/i686/assembler/encoder/system.rs:170 |
| Properties | 12 (10 passing, 2 failing) |
| Bugs | 1 (missing segment prefix) |
| Documented behaviors with properties | arity, l-suffix, /N ext table, memory base/disp/SIB/abs, label form, non-mem reject, segment prefix |
| Contract-surface sweep | 1 round (standard) — all documented behaviors covered; no additional gap properties owed |

## This campaign

| Function | Source file | Tested | Result |
|----------|-------------|--------|--------|
| encode_system_table | system.rs | yes | 10 pass / 2 fail (1 bug) |

## Oracle Type Distribution (this campaign)

| Oracle Type | Count |
|-------------|-------|
| differential | 6 |
| algebraic.invariant | 2 |
| algebraic.metamorphic | 2 |
| negative_error | 2 |
