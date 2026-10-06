# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_jalr, English)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw; C++ reporter listed unrelated binaries and claimed encode_jalr NOT LINKED. Execution evidence is cargo test --lib encode_jalr (7 passing + 3 failing properties plus KAT/regression).

## Summary

| Metric | Value |
|--------|-------|
| Campaign target | encode_jalr |
| Source file | base.rs |
| Properties | 10 |
| Passing / Failing | 7 / 3 |
| Bugs | 3 |
| Tier | standard |
| Sweep | 1/1 spent (manual arm audit) |

## File Coverage (this campaign)

| Source File | Funcs targeted | Properties | Status |
|-------------|----------------|------------|--------|
| base.rs | encode_jalr | 10 (7 pass / 3 fail) | covered |

## Untested documented arms

(none remaining — 1-op Mem and MemSymbol were the sweep targets and are filed as bugs)
