# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_lr)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; Rust cargo test --lib encode_lr executed the production symbol.
> Files: 13/13 scanned | Functions: 337 total | PBT candidates: 197 | This campaign tested: encode_lr

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 13 |
| Files scanned | 13 / 13 |
| Total functions (all files) | 337 |
| PBT candidates (from FUNCTION_INDEX) | 197 |
| This campaign target | encode_lr |
| This campaign properties | 7 (5 passing, 2 failing) |
| Coverage evidence | file-level (symbol presence) |

## This campaign

| Function | Source | Tested | Result |
|----------|--------|--------|--------|
| encode_lr | atomics.rs | yes | 5 passing / 2 failing |

## Oracle Type Distribution (encode_lr)

| Oracle Type | Count | Result |
|-------------|-------|--------|
| differential | 1 | passing |
| algebraic.invariant | 1 | passing |
| algebraic.metamorphic | 1 | passing |
| negative_error | 4 | 2 passing, 2 failing |

## Sweep

Round 1/1: coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED for encode_lr). Manual audit: valid 2-operand path, R-type unpack, ABI alias, extra operand, nonzero offset, arity/FP, and non-Mem slot 1 all exercised. Closed: tier round spent; remaining documented gaps are the two filed bugs.
