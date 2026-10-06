# PBT Coverage Status

> Last updated: 2026-10-06 (campaign: encode_tst, English, standard)
> Coverage evidence: file-level (symbol presence) — coverage_gaps reported no .gcda/.profraw (C++ reporter listed unrelated host binaries and claimed encode_tst NOT LINKED). Cargo `cargo test --lib encode_tst` executed the production symbol (6 KATs matched llvm-mc). Sweep round 1/1: manual arm audit of the 12-line body.

## This campaign

| Metric | Value |
|--------|-------|
| Target | encode_tst |
| Source | compare_branch.rs:37 |
| Properties | 13 (8 passing, 5 failing) + 1 retired |
| Bugs | 5 medium |
| Sweep | 1/1 spent — invalid-name and invalid-imm passing; extra/sp/mixed/fp/shift-oor failing |

## encode_tst documented behaviors vs properties

| Behavior | Property | Status |
|----------|----------|--------|
| Valid TST Rn, Rm{, shift} vs llvm-mc | encode_tst_diff_valid_reg | passing |
| Valid TST Rn, #bitmask vs llvm-mc | encode_tst_diff_valid_imm | passing |
| ANDS Rd=31 opc=11 field layout | encode_tst_arm_fields | passing |
| Alias encode_logical([ZR]++ops, 0b11) | encode_tst_meta_vs_ands | passing |
| Rn/Rm/imm6/sf isolation | encode_tst_metamorphic_fields | passing |
| Arity 0..1 Err | encode_tst_neg_arity | passing |
| Extra operand Err | encode_tst_neg_extra_operand | failing (bug) |
| SP/WSP Err | encode_tst_neg_sp | failing (bug) |
| Mixed W/X Err | encode_tst_neg_mixed_width | failing (bug) |
| FP/SIMD Err | encode_tst_neg_fp_reg | failing (bug) |
| Invalid name Err | encode_tst_neg_invalid_name | passing (sweep) |
| Non-bitmask imm Err | encode_tst_neg_invalid_imm | passing (sweep) |
| Shift amount OOR Err | encode_tst_neg_shift_oor | failing (bug, sweep) |
