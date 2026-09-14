# PBT Coverage Status

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
| encode_ubfm | bitfield.rs | bitfield.rs | cargo test --lib | 8 passing / 5 failing properties (plus 6 passing KAT + 5 failing regression witnesses; 5 bugs; sweep invalid-name/nonreg/alt-spellings passing, fp/mixed/extra/sp/immr-imms failing) |
| encode_sbfx | bitfield.rs | bitfield.rs | cargo test --lib | 8 passing / 5 failing properties (plus 6 passing KAT + 5 failing regression witnesses; 5 bugs; sweep invalid-name/nonreg/alt-spellings passing, fp/mixed/extra/sp/lsb-width failing) |
| encode_sbfm | bitfield.rs | bitfield.rs | cargo test --lib | 7 passing / 5 failing properties (plus 6 passing KAT + 5 failing regression witnesses; 5 bugs; sweep invalid-name/nonreg/alt-spellings passing, fp/mixed/extra/sp/immr failing) |
| encode_sbfiz | bitfield.rs | bitfield.rs | cargo test --lib | 8 passing / 5 failing properties (plus 6 passing KAT + 5 failing regression witnesses; 5 bugs; sweep invalid-name/nonreg/alt-spellings passing) |
| encode_shift | gp_integer.rs | gp_integer.rs | cargo test --lib | 9 passing / 3 failing properties (plus 6 KAT + 4 failing regression witnesses; 3 bugs) |
| encode_add_sub | data_processing.rs | data_processing.rs | cargo test --lib | 7 passing / 7 failing properties (plus KAT + 7 regression witnesses) |
| cast_float_to_target | constants.rs | constants.rs | cargo test --lib | 6 passing / 3 failing properties (plus 2 regression witnesses) |
| classify_cast_with_f128 | cast.rs | cast.rs | cargo test --lib | 8 passing / 3 failing properties (plus 3 regression witnesses) |
| encode_adc | data_processing.rs | data_processing.rs | cargo test --lib | 6 passing / 4 failing properties (plus KAT + 4 regression witnesses) |
| encode_adr | load_store.rs | load_store.rs | cargo test --lib | 6 passing / 6 failing properties (plus 2 KAT + 6 regression witnesses) |
| encode_bic | data_processing.rs | data_processing.rs | cargo test --lib | 8 passing / 7 failing properties (plus 3 KAT + 7 regression witnesses) |
| encode_neon_three_diff_narrow | neon.rs | neon.rs | cargo test --lib | 8 passing / 4 failing properties (plus 1 KAT + 4 regression witnesses) |
| encode_bics | data_processing.rs | data_processing.rs | cargo test --lib | 5 passing / 5 failing properties (plus 2 KAT + 7 regression witnesses) |
| encode_bl | compare_branch.rs | compare_branch.rs | cargo test --lib | 6 passing / 3 failing properties (plus 2 failing KAT + 1 passing KAT + 3 regression witnesses) |
| encode_blr | compare_branch.rs | compare_branch.rs | cargo test --lib | 6 passing / 3 failing properties (plus 2 passing KAT + 4 failing regression witnesses) |
| encode_br | compare_branch.rs | compare_branch.rs | cargo test --lib | 6 passing / 3 failing properties (plus 2 passing KAT + 4 failing regression witnesses) |
| encode_branch | compare_branch.rs | compare_branch.rs | cargo test --lib | 6 passing / 3 failing properties (plus 1 passing KAT + 2 failing KAT + 3 failing regression witnesses) |
| encode_cbz | compare_branch.rs | compare_branch.rs | cargo test --lib | 7 passing / 3 failing properties (plus 1 passing KAT + 3 failing KAT + 4 failing regression witnesses) |
| encode_ccmp_ccmn | compare_branch.rs | compare_branch.rs | cargo test --lib | 8 passing / 4 failing properties (plus 3 passing KAT + 6 failing regression witnesses) |
| encode_cinc | compare_branch.rs | compare_branch.rs | cargo test --lib | 8 passing / 3 failing properties (plus 3 passing KAT + 6 failing regression witnesses) |
| encode_cinv | compare_branch.rs | compare_branch.rs | cargo test --lib | 8 passing / 3 failing properties (plus 3 passing KAT + 6 failing regression witnesses) |
| encode_cmn | compare_branch.rs | compare_branch.rs | cargo test --lib | 8 passing / 3 failing properties (plus 7 passing KAT + 7 failing regression witnesses) |
| encode_cmp | compare_branch.rs | compare_branch.rs | cargo test --lib | 8 passing / 3 failing properties (plus 7 passing KAT + 7 failing regression witnesses) |
| encode_cneg | compare_branch.rs | compare_branch.rs | cargo test --lib | 7 passing / 3 failing properties (plus 3 passing KAT + 6 failing regression witnesses) |
| encode_csel | compare_branch.rs | compare_branch.rs | cargo test --lib | 8 passing / 2 failing properties (plus 3 passing KAT + 4 failing regression witnesses) |
| encode_cset | compare_branch.rs | compare_branch.rs | cargo test --lib | 8 passing / 3 failing properties (plus 3 passing KAT + 5 failing regression witnesses) |
| encode_csetm | compare_branch.rs | compare_branch.rs | cargo test --lib | 8 passing / 3 failing negative-contract properties (4 bugs: extra operand, AL/NV, SP-as-ZR, FP-as-GPR; 3 KAT pass; 5 regression witnesses fail) |
| encode_csinc | compare_branch.rs | compare_branch.rs | cargo test --lib | 8 passing / 2 failing properties (4 bugs: extra operand, SP-as-ZR, mixed width, FP-as-GPR; 4 KAT pass; 4 regression witnesses fail) |
| encode_csinv | compare_branch.rs | compare_branch.rs | cargo test --lib | 8 passing / 2 failing properties (4 bugs: extra operand, SP-as-ZR, mixed width, FP-as-GPR; 4 KAT pass; 4 regression witnesses fail) |
| encode_csneg | compare_branch.rs | compare_branch.rs | cargo test --lib | 8 passing / 2 failing properties (4 bugs: extra operand, SP-as-ZR, mixed width, FP-as-GPR; 4 KAT pass; 4 regression witnesses fail) |
| encode_div | data_processing.rs | data_processing.rs | cargo test --lib | 6 passing / 4 failing properties (plus 2 passing KAT + 4 failing regression witnesses) |
| encode_eon | data_processing.rs | data_processing.rs | cargo test --lib | 5 passing / 6 failing properties (plus 2 passing KAT + 7 failing regression witnesses) |
| encode_ldar_stlr | load_store.rs | load_store.rs | cargo test --lib | 5 passing / 3 failing properties (plus 6 passing KAT + 3 failing regression witnesses) |
| encode_neon_across_long | neon.rs | neon.rs | cargo test --lib | 5 passing / 3 failing properties (plus 3 passing KAT + 3 failing regression witnesses) |
| encode_neon_float_cmp_zero | neon.rs | neon.rs | cargo test --lib | 6 passing / 3 failing properties (plus 3 passing KAT + 3 failing regression witnesses) |
| encode_neon_sli | neon.rs | neon.rs | cargo test --lib | 5 passing / 3 failing properties (plus 2 passing KAT + 6 failing regression witnesses; 4 bugs) |
| encode_ldur_stur | load_store.rs | load_store.rs | cargo test --lib | 6 passing / 4 failing properties (plus 3 passing KAT + 8 failing regression witnesses; 7 bugs) |
| encode_ldxp_stxp | load_store.rs | load_store.rs | cargo test --lib | 5 passing / 4 failing properties (plus 7 passing KAT + 9 failing regression witnesses; 9 bugs) |
| encode_neon_float_three_same | neon.rs | neon.rs | cargo test --lib | 7 passing / 4 failing properties (plus 2 passing KAT + 4 failing regression witnesses; 4 bugs) |
| encode_ldxr_stxr | load_store.rs | load_store.rs | cargo test --lib | 3 passing / 4 failing properties (plus 8 passing KAT + 9 failing regression witnesses; 9 bugs) |
| encode_logical | data_processing.rs | data_processing.rs | cargo test --lib | 10 passing / 1 failing property group (9 bugs; plus 2 KAT + 9 regression witnesses) |
| encode_madd | data_processing.rs | data_processing.rs | cargo test --lib | 8 passing / 4 failing properties (plus 3 passing KAT + 4 failing regression witnesses; 4 bugs) |
| encode_movk | data_processing.rs | data_processing.rs | cargo test --lib | 9 passing / 5 failing properties (plus 3 passing KAT + 5 failing regression witnesses; 5 bugs) |
| encode_movn | data_processing.rs | data_processing.rs | cargo test --lib | 7 passing / 5 failing properties (plus 3 passing KAT + 5 failing regression witnesses; 5 bugs) |
| encode_movz | data_processing.rs | data_processing.rs | cargo test --lib | 8 passing / 5 failing properties (plus 1 passing KAT + 5 failing regression witnesses; 5 bugs) |
| encode_neon_qshrn | neon.rs | neon.rs | cargo test --lib | 6 passing / 4 failing properties (plus 2 passing KAT + 5 failing regression witnesses; 5 bugs) |
| encode_msub | data_processing.rs | data_processing.rs | cargo test --lib | 8 passing / 4 failing properties (plus 3 passing KAT + 4 failing regression witnesses; 4 bugs) |
| encode_mul | data_processing.rs | data_processing.rs | cargo test --lib | 9 passing / 6 failing properties (plus 4 passing KAT + 6 failing regression witnesses; 6 bugs) |
| encode_mvn | data_processing.rs | data_processing.rs | cargo test --lib | 7 passing / 10 failing properties (plus 4 passing KAT + 10 failing regression witnesses; 9 bugs) |
| encode_neon_shift_right | neon.rs | neon.rs | cargo test --lib | 6 passing / 4 failing properties (plus 1 passing KAT + 4 failing regression witnesses; 4 bugs) |
| encode_neg | pseudo.rs | pseudo.rs | cargo test --lib | 8 passing / 1 failing properties (plus 1 passing KAT + 1 failing regression witness; 1 bug) |
| encode_negs | data_processing.rs | data_processing.rs | cargo test --lib | 7 passing / 7 failing properties (plus 3 passing KAT + 7 failing regression witnesses; 6 bugs) |
| encode_neon_shift_imm | neon.rs | neon.rs | cargo test --lib | 4 passing / 5 failing properties (plus 1 passing KAT + 5 failing regression witnesses; 5 bugs) |
| encode_neon_tbl | neon.rs | neon.rs | cargo test --lib | 4 passing / 5 failing properties (plus 1 passing KAT + 10 failing regression witnesses; 10 bugs) |
| encode_orn | data_processing.rs | data_processing.rs | cargo test --lib | 9 passing / 4 failing properties (plus 4 passing KAT + 10 failing regression witnesses; 10 bugs) |
| encode_ret | compare_branch.rs | compare_branch.rs | cargo test --lib | 6 passing / 4 failing properties (plus 3 passing KAT + 4 failing regression witnesses; 4 bugs) |
| encode_sbc | data_processing.rs | data_processing.rs | cargo test --lib | 9 passing / 4 failing properties (plus 3 passing KAT + 4 failing regression witnesses; 4 bugs) |
| encode_neon_shll | neon.rs | neon.rs | cargo test --lib | 6 passing / 5 failing properties (plus 2 passing KAT + 6 failing regression witnesses; 5 bugs) |
| encode_neon_sqshrun | neon.rs | neon.rs | cargo test --lib | 6 passing / 5 failing properties (plus 2 passing KAT + 5 failing regression witnesses; 5 bugs) |
| encode_smull | data_processing.rs | data_processing.rs | cargo test --lib | 6 passing / 3 failing properties (plus 4 passing KAT + 4 failing regression witnesses; 4 bugs; sweep alt-spellings passing) |
| encode_sxth | data_processing.rs | data_processing.rs | cargo test --lib | 7 passing / 4 failing properties (plus 4 passing KAT + 4 failing regression witnesses; 4 bugs; sweep alt-spellings/nonreg/invalid-name passing) |
| encode_sxtw | data_processing.rs | data_processing.rs | cargo test --lib | 7 passing / 4 failing properties (plus 4 passing KAT + 4 failing regression witnesses; 4 bugs; sweep alt-spellings/nonreg/invalid-name passing) |
| encode_neon_shift_left_imm | neon.rs | neon.rs | cargo test --lib | 5 passing / 5 failing properties (plus 2 passing KAT + 7 failing regression witnesses; 5 bugs; sweep non-v prefix and src-Reg) |
| encode_umaddl | data_processing.rs | data_processing.rs | cargo test --lib | 8 passing / 4 failing properties (plus 4 passing KAT + 4 failing regression witnesses; 4 bugs; sweep alt-spellings/nonreg/invalid-name passing) |
| encode_umulh | data_processing.rs | data_processing.rs | cargo test --lib | 7 passing / 4 failing properties (plus 4 passing KAT + 4 failing regression witnesses; 4 bugs; sweep nonreg/invalid-name passing, fp failing) |
| encode_neon_rbit | neon.rs | neon.rs | cargo test --lib | 8 passing / 5 failing properties (plus 3 passing KAT + 5 failing regression witnesses; 5 bugs) |
| encode_umull | data_processing.rs | data_processing.rs | cargo test --lib | 4 failing / 8 passing properties (plus 4 passing KAT + 4 failing regression witnesses; 4 bugs; sweep nonreg/invalid-name passing) |
| encode_uxtw | data_processing.rs | data_processing.rs | cargo test --lib | 3 passing / 8 failing properties (plus 4 failing KAT + 5 failing regression witnesses; 5 bugs; sweep nonreg/invalid-name passing, alt-spellings failing) |
| encode_ldaxr_stlxr | load_store.rs | load_store.rs | cargo test --lib | 5 passing / 4 failing properties (plus 8 passing KAT + 9 failing regression witnesses; 9 bugs; sweep alt-spellings/mem-index passing) |
| encode_ldrsw | load_store.rs | load_store.rs | cargo test --lib | 8 passing / 4 failing properties (plus 5 passing KAT + 11 failing regression witnesses; 10 bugs; sweep alt-spellings/bad-extend passing, literal failing) |
| encode_ldtr_sized | load_store.rs | load_store.rs | cargo test --lib | 6 passing / 4 failing properties (plus 5 passing KAT + 7 failing regression witnesses; 7 bugs; sweep bad-form/w31-alias passing) |
| encode_prfm | load_store.rs | load_store.rs | cargo test --lib | 6 passing / 5 failing properties (plus 5 passing KAT + 1 failing KAT + 9 failing regression witnesses; 7 bugs; sweep bad-prfop/name/literal passing, w-index/bad-shift failing) |
| encode_smulh | data_processing.rs | data_processing.rs | cargo test --lib | 8 passing / 5 failing properties (plus 4 passing KAT + 5 failing regression witnesses; 4 bugs; sweep wzr failing, nonreg/invalid-name passing) |
| encode_fcvt_rounding | fp_scalar.rs | fp_scalar.rs | cargo test --lib | 6 passing / 4 failing properties (plus 7 passing KAT + 4 failing regression witnesses; 4 bugs; sweep nonreg/invalid-name passing) |
| encode_fp_1src | fp_scalar.rs | fp_scalar.rs | cargo test --lib | 5 passing / 3 failing properties (plus 7 passing KAT + 3 failing regression witnesses; 3 bugs; sweep nonreg/invalid-name passing) |
| encode_int_to_float | fp_scalar.rs | fp_scalar.rs | cargo test --lib | 6 passing / 4 failing properties (plus 8 passing KAT + 4 failing regression witnesses; 4 bugs; sweep nonreg/invalid-name passing) |
| encode_fcmp | fp_scalar.rs | fp_scalar.rs | cargo test --lib | 7 passing / 4 failing properties (plus 8 passing KAT + 4 failing regression witnesses; 4 bugs; sweep nonzero-imm/nonreg/invalid-name passing) |
| encode_fcvt_precision | fp_scalar.rs | fp_scalar.rs | cargo test --lib | 6 passing / 3 failing properties (plus 8 passing KAT + 6 failing regression witnesses; 3 bugs; sweep gpr/qvb/wsp passing, SP dest+src failing) |
| encode_neon_aes | neon.rs | neon.rs | cargo test --lib | 4 passing / 4 failing properties (plus 2 passing KAT + 1 passing isolated invalid-name + 6 failing regression witnesses; 6 bugs; sweep nonreg-src passing, WSP dest failing) |
| encode_bfi | bitfield.rs | bitfield.rs | cargo test --lib | 8 passing / 5 failing properties (plus 6 passing KAT + 5 failing regression witnesses; 5 bugs; sweep invalid-name/nonreg/alt-spellings passing) |
| encode_bfxil | bitfield.rs | bitfield.rs | cargo test --lib | 8 passing / 5 failing properties (plus 6 passing KAT + 5 failing regression witnesses; 5 bugs; sweep invalid-name/nonreg/alt-spellings passing) |
| encode_cas | load_store.rs | load_store.rs | cargo test --lib | 6 passing / 4 failing properties (plus 8 passing KAT + 8 failing regression witnesses; 8 bugs; sweep invalid-name/alt-spellings passing) |
| encode_cls | bitfield.rs | bitfield.rs | cargo test --lib | 7 passing / 4 failing properties (plus 6 passing KAT + 4 failing regression witnesses; 4 bugs; sweep invalid-name/nonreg/alt-spellings passing) |
| encode_clz | bitfield.rs | bitfield.rs | cargo test --lib | 7 passing / 4 failing properties (plus 6 passing KAT + 4 failing regression witnesses; 4 bugs; sweep invalid-name/nonreg/alt-spellings passing) |
| encode_extr | bitfield.rs | bitfield.rs | cargo test --lib | 8 passing / 5 failing properties (plus 8 passing KAT + 5 failing regression witnesses; 5 bugs; sweep invalid-name/nonreg/alt-spellings passing) |
| encode_fmov | fp_scalar.rs | fp_scalar.rs | cargo test --lib | 6 passing / 5 failing properties (plus 9 passing KAT + 7 failing regression witnesses; 5 bugs; sweep nonreg/invalid-name passing, V.D[1] failing) |
| encode_fp_arith | fp_scalar.rs | fp_scalar.rs | cargo test --lib | 6 passing / 3 failing properties (plus 8 passing KAT + 5 failing regression witnesses; 3 bugs; sweep invalid-name passing) |
| encode_rbit | bitfield.rs | bitfield.rs | cargo test --lib | 8 passing / 4 failing properties (plus 6 passing KAT + 4 failing regression witnesses; 4 bugs; sweep invalid-name/nonreg/alt-spellings/neon passing) |
| encode_rev | bitfield.rs | bitfield.rs | cargo test --lib | 7 passing / 4 failing properties (plus 6 passing KAT + 4 failing regression witnesses; 4 bugs; sweep invalid-name/nonreg/alt-spellings passing) |
| encode_rev16 | bitfield.rs | bitfield.rs | cargo test --lib | 7 passing / 4 failing properties (plus 6 passing KAT + 4 failing regression witnesses; 4 bugs; sweep invalid-name/nonreg/alt-spellings passing) |
| encode_rev32 | bitfield.rs | bitfield.rs | cargo test --lib | 8 passing / 6 failing properties (plus 8 passing KAT + 6 failing regression witnesses; 6 bugs; sweep invalid-name/nonreg/alt-spellings passing, mixed/fp/neon-invalid-arr failing) |
| encode_ubfiz | bitfield.rs | bitfield.rs | cargo test --lib | 8 passing / 5 failing properties (plus 6 passing KAT + 5 failing regression witnesses; 5 bugs; sweep invalid-name/nonreg/alt-spellings passing) |
| encode_bfm | bitfield.rs | bitfield.rs | cargo test --lib | 7 passing / 5 failing properties (plus 6 passing KAT + 5 failing regression witnesses; 5 bugs; sweep invalid-name/nonreg/alt-spellings passing, fp/mixed/extra/sp/immr failing) |
