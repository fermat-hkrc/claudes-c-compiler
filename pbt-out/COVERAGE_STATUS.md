# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_branch_instr)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated binaries). Execution evidence is `cargo test --lib encode_branch_instr` (KAT + 1000-case proptest).
> This campaign: encode_branch_instr (base.rs) — 1/1 in-scope candidate tested.

## Summary

| Metric | Value |
|--------|-------|
| Scope | src/backend/riscv/assembler/encoder/base.rs#encode_branch_instr |
| Requested symbol | encode_branch (unresolved; tested encode_branch_instr) |
| PBT properties | 9 |
| Passing / Failing | 6 / 3 |
| Bugs | 3 |
| Tier | standard |

## File Coverage

| Source File | Funcs | Candidates this campaign | Tested | Status |
|-------------|-------|--------------------------|--------|--------|
| base.rs | 17 | 1 (encode_branch_instr) | 1 | covered |
