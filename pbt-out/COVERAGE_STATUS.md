# Coverage Status — encode_movsx campaign

**Tier:** standard
**Coverage evidence:** file-level (symbol presence) — `coverage_gaps` reported no .gcda/.profraw (Rust cargo host build) and falsely listed encode_movsx as NOT LINKED against unrelated OH test binaries. Real execution evidence: `cargo test --lib encode_movsx -- --test-threads=1` links and runs production `InstructionEncoder::encode` → `encode_movsx` (gp_integer.rs:272) for KAT + proptest properties (1000 cases each).

## This campaign

| Metric | Value |
|--------|-------|
| Target function | encode_movsx |
| Properties | 11 (7 passing, 4 failing — segment + segment_sib share one bug) |
| Bugs filed | 3 |
| Generator runs | 1000 per proptest property |
| Strengthen rounds | 1 (segment+SIB differential added) |
| Sweep rounds | 1 (coverage_gaps + cargo execution) |

## Untested documented branches under HARD scope
(none remaining — arity/src_size error paths exercised; mem and RR arms both hit; segment/width/non-GP contracts covered by failing properties)
