# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_ldp_stp, English, standard)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw
> Files: 11/11 scanned | Functions: 307 total | PBT candidates: 177 | This campaign tested: encode_ldp_stp

## This campaign

| Function | Source file | Tested | Result |
|----------|-------------|--------|--------|
| encode_ldp_stp | load_store.rs | yes | 7 passing / 3 failing (3 bugs) |

## Sweep

Round 1/1: coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Manual audit of encode_ldp_stp: added encode_ldp_stp_diff_simd_llvm_mc and encode_ldp_stp_diff_alt_spellings (both passing). Closed: tier round spent; remaining documented gaps are the failing negative-contract paths already filed as bugs.
