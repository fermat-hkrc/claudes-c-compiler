# Coverage Status — encode_bnez campaign

**Coverage evidence:** file-level (symbol presence / cargo execution) — not line-level.
- `coverage_gaps` found no `.gcda`/`.profraw` (C++ reporter; Rust cargo run is not instrumented for that tool).
- Tool listed `encode_bnez` as NOT LINKED against unrelated C++ OH pbt binaries — false negative for this Rust campaign.
- Execution evidence: `cargo test --lib encode_bnez` ran 14 tests against `encode_bnez` (12 proptest + KAT + regression); 12 passed, 2 failed (expected extra-operand bug).

| Metric | Value |
|--------|-------|
| Target function | encode_bnez (pseudo.rs:291) |
| Properties | 12 (11 passing, 1 failing) |
| KAT / regression | 1 / 1 |
| Bugs | 1 (extra operand ignored) |
| Sweep rounds completed | 1/1 (standard) |
| Sweep close reason | documented behaviors have properties; line-level data unavailable |

## Untested in-scope branches
(none documented) — encode_bnez body is straight-line with `?` error paths; all exercised.
