# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_fcvt_int, English, standard)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; C++ reporter listed unrelated binaries; Rust cargo tests are not those binaries.

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_fcvt_int (float.rs:116) |
| Properties | 8 |
| Passing / failing | 6 / 2 |
| Bugs | 2 |
| Sweep | 1 round (manual audit of 2-op / rm / R-type / ABI / dyn / arity-class / extra / non-rm-third) |

## File Coverage (scope)

| Source File | Funcs | This-campaign target | Tested this campaign | Status |
|-------------|-------|----------------------|----------------------|--------|
| float.rs | 13 | encode_fcvt_int | encode_fcvt_int | covered (6 pass / 2 fail) |
