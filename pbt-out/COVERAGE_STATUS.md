# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_ldnp_stnp, English)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; C++ reporter listed unrelated binaries and claimed NOT LINKED. Cargo tests executed encode_ldnp_stnp directly.
> Files: 11/11 scanned | Functions: 307 | PBT candidates: 178 | This campaign tested: encode_ldnp_stnp (9 properties)

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_ldnp_stnp |
| Properties | 9 (5 passing, 4 failing) |
| KAT | 5 passing |
| Regression witnesses | 9 failing |
| Bugs | 4 |
| Sweep | encode_ldnp_stnp_diff_alt_spellings passing; tier round 1/1 spent |

## Oracle Type Distribution (this campaign)

| Oracle Type | Total | Passing | Failing |
|-------------|-------|---------|---------|
| differential | 3 | 2 | 1 |
| algebraic.invariant | 1 | 1 | 0 |
| algebraic.metamorphic | 1 | 1 | 0 |
| negative_error | 4 | 1 | 3 |

## File Coverage

| Source File | Funcs | Candidates | Tested this campaign | Status |
|-------------|-------|------------|----------------------|--------|
| load_store.rs | 20 | encode_ldnp_stnp | encode_ldnp_stnp | covered |
