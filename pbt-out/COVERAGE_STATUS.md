# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_svc, English, standard)
> Coverage evidence: file-level (symbol presence) — coverage_gaps had no LLVM profraw (C++ reporter listed unrelated binaries and claimed NOT LINKED). Cargo tests executed encode_svc.

## Summary

| Metric | Value |
|--------|-------|
| Campaign target | encode_svc |
| Source file | system.rs |
| Properties | 7 |
| Passing / Failing | 5 / 2 |
| Bugs | 2 |
| KAT | 3 passing |
| Regression witnesses | 3 failing |
| Sweep | 1/1 (manual arm audit; tier round spent) |

## This campaign

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| encode_svc | system.rs | encode_svc_pbt.rs | cargo test --lib encode_svc | 5 passing / 2 failing properties (plus 3 passing KAT + 3 failing regression witnesses; 2 bugs) |

## Oracle Type Distribution (this campaign)

| Oracle Type | Total | Passing | Failing |
|-------------|-------|---------|---------|
| differential | 1 | 1 | 0 |
| algebraic.invariant | 1 | 1 | 0 |
| algebraic.metamorphic | 1 | 1 | 0 |
| negative_error | 4 | 2 | 2 |

## Sweep

coverage_gaps: no .gcda/.profraw; C++ reporter claimed encode_svc NOT LINKED (wrong binaries). File-level evidence: cargo test --lib encode_svc ran the production symbol. Manual arm audit of the 3-line body: get_imm Ok (valid imm16, extra, oob mask) and get_imm Err (empty, wrong kind) are both driven. Closed: tier round spent.
