# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_cond_branch, English, standard)
> Coverage evidence: file-level (symbol presence) — coverage_gaps had no LLVM profraw / .gcda; C++ reporter listed unrelated binaries and claimed NOT LINKED. Manual audit of encode_cond_branch (14-line body) plus sweep properties.

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_cond_branch |
| Source | compare_branch.rs:197 |
| Properties | 11 (8 passing, 3 failing) |
| Bugs | 3 |
| Sweep | 1/1 (neg_bad_operand passing, symbol_misclassified passing, neg_modifier failing/filed) |

## Summary

| Metric | Value |
|--------|-------|
| Total source files (index) | 11 |
| PBT candidates (FUNCTION_INDEX) | 181 |
| Tested this campaign | encode_cond_branch |
| Coverage evidence | file-level (symbol presence) |

## File Coverage

| Source File | Notes |
|-------------|-------|
| compare_branch.rs | encode_cond_branch exercised via cargo test --lib encode_cond_branch (sibling encode_cond_branch_pbt.rs) |
