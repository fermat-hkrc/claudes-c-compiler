# PBT Coverage Status

> Last updated: 2026-10-07 (campaign: encode_vid_v, English)
> Files: 16/16 scanned | Functions: 383 total | PBT candidates: 233 | This campaign tested: encode_vid_v

## Summary

| Metric | Value |
|--------|-------|
| Total source files | 16 |
| Files scanned | 16 / 16 |
| Total functions (all files) | 383 |
| PBT candidates (from FUNCTION_INDEX) | 233 |
| This campaign target | encode_vid_v (vector.rs:182) |
| This campaign properties | 6 (4 passing, 2 failing) |
| Coverage evidence | file-level (symbol presence) — no .gcda/.profraw; C++ reporter listed encode_vid_v NOT LINKED; function executed under cargo test --lib encode_vid_v |

## This campaign

| Function | Source | Tested | Result |
|----------|--------|--------|--------|
| encode_vid_v | vector.rs | yes | 4 pass / 2 fail (extra, v0.t) |

Sweep (standard, 1 round): documented 1-op / format / isolation / arity paths have passing properties; extra / v0.t have failing properties with bug reports. No further documented branch without a property.

## Oracle Type Distribution (this campaign)

| Oracle Type | Count | Status |
|-------------|-------|--------|
| differential | 2 | 1 passing, 1 failing |
| algebraic.invariant | 1 | passing |
| algebraic.metamorphic | 1 | passing |
| negative_error | 2 | 1 passing, 1 failing |
