# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_fnmadd_fnmsub, English)
> Coverage evidence: file-level (symbol presence) — coverage_gaps had no LLVM profraw; C++ reporter listed unrelated binaries and claimed NOT LINKED. The cargo test binary executed encode_fnmadd_fnmsub (7 KATs + 9 properties).
> Files: 11/11 scanned | Functions: 307 total | PBT candidates: 167 | This campaign tested: encode_fnmadd_fnmsub
> Effort tier: standard | Sweep round 1/1 spent (manual arm audit + encode_fnmadd_fnmsub_neg_invalid_name)

## Summary

| Metric | Value |
|--------|-------|
| Target | encode_fnmadd_fnmsub |
| Properties | 9 (6 passing / 3 failing) |
| KAT | 7 passing |
| Regression witnesses | 5 failing |
| Bugs | 3 |
| Sweep | encode_fnmadd_fnmsub_neg_invalid_name passing |

## This campaign

| Function | Source File | Tested | Result |
|----------|-------------|--------|--------|
| encode_fnmadd_fnmsub | fp_scalar.rs | yes | 6 pass / 3 fail |

## Sweep

coverage_gaps: no .gcda/.profraw; file-level evidence claimed NOT LINKED against unrelated C++ binaries. Manual audit of the 16-line body: arity, extra operand, wrong types, H ftype, S/D ftype, o0/o1, nonreg, invalid name all have properties. Closed: every documented behavior has a property; tier round spent.
