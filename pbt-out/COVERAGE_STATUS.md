# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_lui, English, standard)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; C++ reporter listed unrelated binaries and claimed encode_lui NOT LINKED. Manual audit of the 22-line body.
> Files: base.rs scanned | Functions in scope: encode_lui | PBT candidates: 1 | Tested: 1 (8 passing / 4 failing properties)

## Summary

| Metric | Value |
|--------|-------|
| Campaign target | encode_lui |
| Source file | src/backend/riscv/assembler/encoder/base.rs |
| PBT candidates (this campaign) | 1 |
| Tested | 1 / 1 |
| Properties | 12 (8 passing, 4 failing) |
| Bugs | 4 |
| Sweep | 1/1 (arity, FP dest, bad operand 1, %hi addend) |
| FUNCTION_INDEX after merge | 12 files, 324 functions, 183 candidates |

## File Coverage

| Source File | Funcs | Candidates | Tested | Coverage | Status |
|-------------|-------|------------|--------|----------|--------|
| base.rs | 17 | 1 | 1 | encode_lui covered | this campaign |

## Recommended Focus

Fix the four failing encode_lui properties (imm oob, extra operand, bad modifier, %hi addend) before adding new tests.
