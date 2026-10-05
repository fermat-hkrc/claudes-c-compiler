# PBT Coverage Status

> Last updated: 2026-10-05 (campaign: encode_neon_add_sub, English)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (build tree not instrumented); C++ reporter listed unrelated binaries and claimed encode_neon_add_sub NOT LINKED. Cargo lib tests executed the symbol (KAT + 9 properties).
> Files: 10/10 scanned | Functions: 121/289 PBT candidates | encode_neon_add_sub: 6 pass / 3 fail

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 121 |
| Tested this campaign | encode_neon_add_sub (9 properties) |
| Pass / Fail this campaign | 6 / 3 |
| Coverage evidence | file-level (symbol presence) |
| Sweep | 1/1 closed (manual arm audit + alt-spellings + nonreg) |

## This campaign

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| encode_neon_add_sub | neon.rs | encode_neon_add_sub_pbt.rs | cargo test --lib encode_neon_add_sub | 6 passing / 3 failing; 3 bugs |

## Oracle Type Distribution (this campaign)

| Oracle Type | Count |
|-------------|-------|
| differential | 2 |
| algebraic.metamorphic | 1 |
| algebraic.invariant | 1 |
| negative_error | 5 |
