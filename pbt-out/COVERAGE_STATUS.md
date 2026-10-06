# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_csri, English)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; C++ reporter listed unrelated binaries and claimed NOT LINKED for encode_csri. Execution evidence is cargo test --lib encode_csri (5 passing / 3 failing properties plus KAT and regression witnesses).

## Summary

| Metric | Value |
|--------|-------|
| Campaign target | encode_csri (system.rs:56) |
| Effort tier | standard |
| Properties | 8 (5 passing, 3 failing) |
| KAT | 2 passing |
| Regression witnesses | 3 failing (as expected while bugs stand) |
| Bugs | 3 |
| Sweep | 1/1 spent (manual audit; no LLVM profraw) |

## This campaign

| Function | Source file | Tested | Notes |
|----------|-------------|--------|-------|
| encode_csri | system.rs | yes | 5 pass / 3 fail; extra, zimm OOB, csr OOB bugs |

## Skipped (HARD scope)

encode_fence, encode_sfence_vma, encode_csr, get_csr_num, csr_name_to_num — test only encode_csri.
