# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_fclass, English)
> Coverage evidence: file-level (symbol presence) — coverage_gaps returned no .gcda/.profraw; Rust cargo test --lib encode_fclass executed the symbol.

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_fclass (float.rs:110) |
| Properties | 7 (5 passing, 2 failing) |
| KAT | 2 passing |
| Regression witnesses | 2 failing (as designed) |
| Bugs | 2 medium |
| Sweep | 1/1 spent (manual audit; reporter NOT LINKED is a C++-binary gap) |

## File Coverage (scope)

| Source File | Funcs | Candidates this campaign | Tested | Status |
|-------------|-------|--------------------------|--------|--------|
| float.rs | 14 | 1 (encode_fclass) | 1 | covered |

HARD-scope skipped other float.rs symbols (encode_float_load, encode_float_store, encode_fp_arith, encode_fp_arith_d, encode_fp_unary, encode_fp_sgnj, encode_fp_cmp, encode_fcvt_int, encode_fcvt_from_int, encode_fcvt_fp, encode_fmv_x_f, encode_fmv_f_x, encode_fma). ARM64 files outside this path are HARD-scope excluded.
