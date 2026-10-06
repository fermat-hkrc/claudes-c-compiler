# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_brk, English)
> Files: 11/11 scanned (100%) | Functions: 156/307 total PBT candidates | Tested this campaign: encode_brk | Coverage evidence: file-level (symbol presence)

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 11 |
| Files scanned | 11 / 11 (100%) |
| Total functions (all files) | 307 |
| PBT candidates (from FUNCTION_INDEX) | 156 |
| This campaign target | encode_brk |
| This campaign properties | 7 (5 passing, 2 failing) |
| Coverage evidence | file-level (symbol presence) — coverage_gaps had no LLVM profraw; C++ reporter listed unrelated binaries and claimed NOT LINKED. Cargo tests executed encode_brk. |

## This campaign (encode_brk)

| Function | Source | Test file | Result |
|----------|--------|-----------|--------|
| encode_brk | system.rs | encode_brk_pbt.rs | 5 passing / 2 failing (extra operand, oob imm) |

Manual arm audit (sweep round 1/1): valid imm16, ARM layout bits[31:21]/[20:5]/[4:0], imm isolation, empty operands, wrong kind, extra operands, oob imm. All documented behaviors have a property. Closed: tier round spent.

## File Coverage (this campaign)

| Source File | Target | Tested | Status |
|-------------|--------|--------|--------|
| system.rs | encode_brk | yes | covered (cargo test --lib encode_brk) |
