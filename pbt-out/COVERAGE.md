# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: pbt-test-v2 — rounds 01–15; round 15 = x86 assembler deep sweep per user request, 4 new issues #774-#777)
> Final suite before round 15: 608 passed / 44 failed (all documented red witnesses, rounds 01-08 bugs) / 11 ignored. Round 15 adds 4 more documented red witnesses (#774-#777).

## Round 15 — x86 assembler deep (2026-10-06)

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| x86 builtin assembler (parser + encoders) | assembler/parser.rs | src/backend/x86/mod.rs (pbt_asm_deep) | cargo test --lib backend::x86::pbt_asm_deep | FAILING — bugs #774-#777 (double-reference confirmed: GNU as ∧ llvm-mc); valid forms byte-exact vs llvm-mc across 18 template families |
| AVX-512 EVEX forms | assembler/encoder/* | — | — | excluded: llvm-mc default target lacks avx512 (needs matched -mattr both sides) — follow-up |
| ARM NEON encoders | arm/assembler | — | — | EXCLUDED — teammate's surface (#474-#773, 263 issues enumerated 2026-10-06); no NEON forms generated, no duplicate-detection risk |
