# Coverage status — encode_imul campaign

**Tier:** standard
**Mode:** incremental (sole target `encode_imul`)
**Evidence level:** file-level (Rust lib tests; line coverage via instrument-coverage when available)

## This campaign

| Metric | Value |
|--------|-------|
| Target function | encode_imul |
| Source | src/backend/i686/assembler/encoder/gp_integer.rs:711 |
| Properties | 12 (10 primary + 2 strengthen) |
| Passing | 2 (neg_arity, bare32_all_forms) + 8 KAT gates |
| Failing | 10 properties + 5 regression witnesses |
| Bugs filed | 4 |
| Generator runs | 1000 cases (proptest) |

## Sweep

- coverage_gaps round 1: change-surface sole target fully property-covered; strengthen added bare-32 differential (pass) and mismatched/non-GP negative (fail → bug).
- Documented behaviors exercised: 1/2/3-op forms, imm8/imm16/imm32 edges, segment overrides, arity rejection, unsupported shapes.

## Untested in this campaign (HARD scope skip)

- All other gp_integer.rs symbols
- `imulb` public dispatch (not present in mod.rs)
