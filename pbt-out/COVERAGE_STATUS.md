# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_ic, English)
> Coverage evidence: file-level (symbol presence) — coverage_gaps returned no .gcda/.profraw; C++ reporter listed unrelated binaries and claimed NOT LINKED. Manual arm audit of encode_ic (system.rs:401-417).

## Summary

| Metric | Value |
|--------|-------|
| Target | encode_ic |
| Source | src/backend/arm/assembler/encoder/system.rs:401 |
| Properties | 9 (6 passing / 3 failing) |
| KAT | 4 passing |
| Regression witnesses | 3 failing (expected until the bugs are fixed) |
| Bugs | 3 medium |
| Sweep | 1/1 — encode_ic_neg_invalid_reg drives the invalid-register Err path |

## This campaign

| Function | Source file | Tested | Notes |
|----------|-------------|--------|-------|
| encode_ic | system.rs | yes | 6 passing / 3 failing; sweep invalid-reg passing |

## File Coverage (scope)

| Source File | Status |
|-------------|--------|
| system.rs (encode_ic only) | covered |

HARD scope: only encode_ic. Other system.rs symbols were not re-tested.
