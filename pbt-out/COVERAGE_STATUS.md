# PBT Coverage Status

> Last updated: 2026-10-07 (campaign: encode_vmv_v_i, English)
> Files: 16/16 scanned | Functions: 383 total | PBT candidates: 232 | This campaign tested: encode_vmv_v_i

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 16 |
| Files scanned | 16 / 16 |
| Total functions (all files) | 383 |
| PBT candidates (from FUNCTION_INDEX) | 232 |
| This campaign target | encode_vmv_v_i (vector.rs:172) |
| This campaign properties | 8 (5 passing, 3 failing) |
| Coverage evidence | file-level (symbol presence) — no .gcda/.profraw; C++ reporter listed encode_vmv_v_i NOT LINKED; function executed under cargo test --lib encode_vmv_v_i |

## This campaign

| Function | Source | Tested | Result |
|----------|--------|--------|--------|
| encode_vmv_v_i | vector.rs | yes | 5 pass / 3 fail (extra, v0.t, imm-oob) |

Sweep (standard, 1 round): documented 2-op / format / isolation / simm5 / arity paths have passing properties; extra / v0.t / imm-oob have failing properties with bug reports. No further documented branch without a property.

## Oracle Type Distribution (this campaign)

| Oracle Type | Count | Status |
|-------------|-------|--------|
| differential | 1 | passing |
| algebraic.invariant | 2 | passing |
| algebraic.metamorphic | 1 | passing |
| negative_error | 4 | 1 passing, 3 failing |
