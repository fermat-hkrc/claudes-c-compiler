# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_vsetvli)
> Files: 16 scanned | Functions: 383 total | PBT candidates: 222 | This campaign tested: encode_vsetvli
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; encode_vsetvli NOT LINKED in C++ reporter binaries (Rust cargo tests are not those binaries)

## Summary

| Metric | Value |
|--------|-------|
| Total source files (FUNCTION_INDEX) | 16 |
| This campaign scope | src/backend/riscv/assembler/encoder/vector.rs |
| PBT candidates in scope | 1 (encode_vsetvli) |
| **Tested this campaign** | **1 / 1** |
| Pass / Fail properties | 6 passing / 3 failing (plus 1 sweep FP passing) |
| Bugs | 3 |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| vector.rs | 16 | 1 | 1 | 100% of in-scope candidate | encode_vsetvli tested |

## This campaign

| Function | Source | Test file | Result |
|----------|--------|-----------|--------|
| encode_vsetvli | vector.rs | encode_vsetvli_pbt.rs | 6 passing / 3 failing; 3 bugs |
