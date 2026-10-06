# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_fmv_x_f)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries and encode_fmv_x_f NOT LINKED). Rust cargo test --lib encode_fmv_x_f executed the production symbol.

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_fmv_x_f (float.rs:161) |
| Properties | 8 (6 passing, 2 failing) |
| Bugs | 2 (extra operand, rm-as-third) |
| Sweep | 1 round (standard); manual audit of 2-op / R-type / ABI / S-vs-D / w-vs-s / arity-class / extra / rm-third |

## FUNCTION_INDEX

| Metric | Value |
|--------|-------|
| Total source files | 14 |
| Total functions | 351 |
| PBT candidates | 212 |
| Excluded | 139 |
| This campaign tested | encode_fmv_x_f |

## File Coverage (this campaign)

| Source File | Funcs | This campaign | Status |
|-------------|-------|---------------|--------|
| float.rs | 14 | encode_fmv_x_f | covered (6 pass / 2 fail) |
