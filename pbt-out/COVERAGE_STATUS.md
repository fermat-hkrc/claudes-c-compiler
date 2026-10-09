# PBT Coverage Status

> Last updated: 2026-10-09 (campaign: encode_push16)
> Coverage evidence: file-level (symbol presence via cargo lib test) — no Rust .profraw/.gcda from this host build; coverage_gaps reported NOT LINKED against unrelated OH binaries (ignored). Real execution evidence: `cargo test --lib encode_push16` ran encode_push16 via InstructionEncoder::encode("pushw", …) (8 pass / 12 fail including KAT/regression).

## This campaign target

| Function | Source | Tested | Result |
|----------|--------|--------|--------|
| encode_push16 | gp_integer.rs:382 | yes | 5 props passing, 4 failing (3 bugs) |

## Documented behaviors vs properties

| Behavior | Property | Status |
|----------|----------|--------|
| Imm integer form (66 6A / 66 68) | encode_push16_diff_imm, encode_push16_invariant_imm_form | passing |
| Imm8 metamorphic vs pushl | encode_push16_metamorphic_imm8_vs_pushl | passing |
| Arity ≠ 1 → Err | encode_push16_neg_arity | passing |
| r32/r8 rejected | encode_push16_neg_wrong_width_gp | passing |
| r16 short form | encode_push16_diff_r16 | failing (bug) |
| Sreg forms | encode_push16_diff_sreg | failing (bug) |
| Memory FF /6 | encode_push16_diff_mem | failing (bug) |
| Segmented memory | encode_push16_diff_mem_segment | failing (same bug) |

## Contract-surface sweep

- Round 1 (standard tier): coverage_gaps → no line-level data; file-level cargo evidence shows encode_push16 exercised.
- Uncovered documented arms still absent in SUT body: Register r16, Sreg, Memory, Symbol imm — already targeted by failing properties; no additional properties owed beyond the three filed bugs.
- Close reason: tier sweep round complete; every documented llvm-mc pushw form has a property.
