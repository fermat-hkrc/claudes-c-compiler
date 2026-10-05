# PBT Coverage Status

> Last updated: 2026-10-05 (campaign: encode_neon_scalar_addp)
> Coverage evidence: file-level (symbol presence) — coverage_gaps had no .gcda/.profraw; C++ reporter listed unrelated binaries and claimed NOT LINKED. Cargo tests executed the production symbol.
> Files: 10/10 scanned | Functions: 289 total | PBT candidates: 145 | This campaign tested encode_neon_scalar_addp (7 passing / 2 failing properties)

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 10 |
| Files scanned | 10 / 10 (100%) |
| Total functions (all files) | 289 |
| PBT candidates (from FUNCTION_INDEX) | 145 |
| This campaign target | encode_neon_scalar_addp |
| This campaign properties | 9 (7 passing, 2 failing) |
| Coverage evidence | file-level (symbol presence) |

## File Coverage

| Source File | Notes |
|-------------|-------|
| neon.rs | encode_neon_scalar_addp exercised by cargo test --lib encode_neon_scalar_addp |

## Sweep

Round 1/1: coverage_gaps had no LLVM profraw. Manual arm audit of documented behaviors (valid encoding, Rd/Rn isolation, ARM fields, arity, extra, dest class, source arrangement, nonreg, alt-spellings). Closed: tier round spent.
