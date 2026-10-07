# PBT Coverage Status

> Last updated: 2026-10-07 (campaign: encode_vmv_v_x)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw and encode_vmv_v_x NOT LINKED in C++ reporter binaries. The function executed under `cargo test --lib encode_vmv_v_x`.

## Summary

| Metric | Value |
|--------|-------|
| Campaign target | encode_vmv_v_x |
| Source file | vector.rs |
| PBT properties | 8 |
| Passing | 6 |
| Failing (bugs) | 2 |
| KAT | 3 passing |
| Regression witnesses | 2 failing |
| Sweep | 1 round (standard); remaining gaps are the extra-operand and v0.t bugs |

## This campaign

| Function | Source | Tested | Notes |
|----------|--------|--------|-------|
| encode_vmv_v_x | vector.rs | yes | 6 passing / 2 failing; extra operand and trailing v0.t filed as bugs |

## Other vector.rs symbols

HARD-scope excluded this campaign (test only encode_vmv_v_x). Already-covered symbols were not re-tested.
