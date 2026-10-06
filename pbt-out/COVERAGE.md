# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: pbt-test-v2 — rounds 01–15; round 15 = x86 assembler deep sweep, 5 new issues #774-#777/#782)
> Round 15 adds 5 documented red witnesses to the suite (total 49: 44 from rounds 01-08 + 5 from round 15).
> Coverage evidence: file-level (symbol presence) — no .gcda/.profraw; coverage_gaps' passes NOT LINKED list is the known cfg(test) false negative.

## Round 15 — x86 assembler deep (2026-10-06)

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| x86 builtin assembler (parser + encoders) | assembler/parser.rs | src/backend/x86/mod.rs (pbt_asm_deep) | cargo test --lib backend::x86::pbt_asm_deep | FAILING — bugs #774-#777 (double-reference confirmed: GNU as ∧ llvm-mc); valid forms byte-exact vs llvm-mc across 18 template families |
| AVX-512 EVEX forms | assembler/encoder/* | — | — | excluded: llvm-mc default target lacks avx512 (needs matched -mattr both sides) — follow-up |
| ARM NEON encoders | arm/assembler | — | — | EXCLUDED — teammate's surface (#474-#773, 263 issues enumerated 2026-10-06); no NEON forms generated, no duplicate-detection risk |

## Round 16 — ARM assembler deep, non-NEON (2026-10-06)

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| arm builtin assembler (non-NEON general surface) | arm/assembler (parser + encoders) | src/backend/arm/mod.rs (pbt_asm_deep) | cargo test --lib backend::arm::pbt_asm_deep | p1/p2 in flight — validation-gap classes (W/X width mix, imm ranges incl. ldp imm7×8 and movz/movk imm16, shift amounts, W-register as base, addressing-form scales), all double-reference confirmed (llvm-mc + aarch64-linux-gnu-as); NEON excluded (teammate) |
