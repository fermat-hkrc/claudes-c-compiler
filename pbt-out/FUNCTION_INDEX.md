> Total files: 6 | Total functions: 184 | PBT candidates: 40 | Excluded: 144

| Function | Source File | Line | Kind | PBT Candidate | Reason |
|----------|-------------|------|------|---------------|--------|
| encode_mov | data_processing.rs | 6 | function | no | out of campaign scope |
| encode_mov_wide_imm | data_processing.rs | 144 | function | no | out of campaign scope |
| resolve_abs_g_modifier | data_processing.rs | 183 | function | no | out of campaign scope |
| encode_movz | data_processing.rs | 201 | function | yes | - |
| encode_movk | data_processing.rs | 234 | function | yes | - |
| encode_movn | data_processing.rs | 266 | function | yes | - |
| encode_add_sub | data_processing.rs | 291 | function | yes | - |
| encode_logical | data_processing.rs | 455 | function | yes | - |
| encode_bitmask_imm | data_processing.rs | 505 | function | no | out of campaign scope |
| encode_mul | data_processing.rs | 584 | function | no | out of campaign scope |
| encode_madd | data_processing.rs | 598 | function | yes | - |
| encode_msub | data_processing.rs | 608 | function | no | out of campaign scope |
| encode_div | data_processing.rs | 618 | function | yes | - |
| encode_smull | data_processing.rs | 631 | function | no | out of campaign scope |
| encode_umull | data_processing.rs | 642 | function | no | out of campaign scope |
| encode_smaddl | data_processing.rs | 653 | function | no | out of campaign scope |
| encode_umaddl | data_processing.rs | 665 | function | no | out of campaign scope |
| encode_mneg | data_processing.rs | 677 | function | no | out of campaign scope |
| encode_umulh | data_processing.rs | 688 | function | no | out of campaign scope |
| encode_smulh | data_processing.rs | 697 | function | no | out of campaign scope |
| encode_neg | data_processing.rs | 706 | function | no | out of campaign scope |
| encode_negs | data_processing.rs | 727 | function | no | out of campaign scope |
| encode_mvn | data_processing.rs | 748 | function | no | out of campaign scope |
| encode_adc | data_processing.rs | 774 | function | yes | - |
| encode_sbc | data_processing.rs | 784 | function | no | out of campaign scope |
| encode_shift | data_processing.rs | 796 | function | no | out of campaign scope |
| encode_sxtw | data_processing.rs | 855 | function | no | out of campaign scope |
| encode_sxth | data_processing.rs | 863 | function | no | out of campaign scope |
| encode_sxtb | data_processing.rs | 872 | function | no | out of campaign scope |
| encode_uxtw | data_processing.rs | 881 | function | no | out of campaign scope |
| encode_uxth | data_processing.rs | 891 | function | no | out of campaign scope |
| encode_uxtb | data_processing.rs | 900 | function | no | out of campaign scope |
| encode_orn | data_processing.rs | 910 | function | no | out of campaign scope |
| encode_eon | data_processing.rs | 952 | function | yes | - |
| encode_bics | data_processing.rs | 981 | function | yes | - |
| encode_bic | data_processing.rs | 1013 | function | yes | - |
| f64_to_f128_bytes | constants.rs | 48 | function | no | out of campaign scope |
| f64_to_x87_bytes | constants.rs | 103 | function | no | out of campaign scope |
| is_zero | constants.rs | 155 | method | no | out of campaign scope |
| long_double | constants.rs | 168 | method | no | out of campaign scope |
| long_double_with_bytes | constants.rs | 173 | method | no | out of campaign scope |
| long_double_from_i64 | constants.rs | 179 | method | no | out of campaign scope |
| long_double_from_u64 | constants.rs | 186 | method | no | out of campaign scope |
| long_double_from_u128 | constants.rs | 194 | method | no | out of campaign scope |
| long_double_from_i128 | constants.rs | 202 | method | no | out of campaign scope |
| long_double_bytes | constants.rs | 208 | method | no | out of campaign scope |
| x87_bytes | constants.rs | 219 | method | no | out of campaign scope |
| is_one | constants.rs | 235 | method | no | out of campaign scope |
| is_nonzero | constants.rs | 240 | method | no | out of campaign scope |
| to_hash_key | constants.rs | 245 | method | no | out of campaign scope |
| to_f64 | constants.rs | 260 | method | no | out of campaign scope |
| cast_float_to_target | constants.rs | 277 | method | yes | - |
| cast_long_double_to_target | constants.rs | 299 | method | no | out of campaign scope |
| to_i64 | constants.rs | 322 | method | no | out of campaign scope |
| to_i128 | constants.rs | 336 | method | no | out of campaign scope |
| to_u64 | constants.rs | 349 | method | no | out of campaign scope |
| to_usize | constants.rs | 362 | method | no | out of campaign scope |
| to_u32 | constants.rs | 367 | method | no | out of campaign scope |
| push_le_bytes | constants.rs | 377 | method | no | out of campaign scope |
| push_le_bytes_x86 | constants.rs | 410 | method | no | out of campaign scope |
| push_le_bytes_riscv | constants.rs | 425 | method | no | out of campaign scope |
| ptr_int | constants.rs | 438 | method | no | out of campaign scope |
| from_i64 | constants.rs | 452 | method | no | out of campaign scope |
| coerce_to_with_src | constants.rs | 484 | method | no | out of campaign scope |
| coerce_to | constants.rs | 550 | method | no | out of campaign scope |
| bool_normalize | constants.rs | 557 | method | no | out of campaign scope |
| zero | constants.rs | 562 | method | no | out of campaign scope |
| narrowed_to | constants.rs | 579 | method | no | out of campaign scope |
| to_le_bytes | constants.rs | 617 | method | no | out of campaign scope |
| one | constants.rs | 632 | method | no | out of campaign scope |
| classify_cast_with_f128 | cast.rs | 67 | function | yes | - |
| classify_cast | cast.rs | 152 | function | no | out of campaign scope |
| classify_f128_cast_native | cast.rs | 158 | function | no | private helper; campaign is single-symbol |
| classify_float_binop | cast.rs | 208 | function | no | out of campaign scope |
| f128_cmp_libcall | cast.rs | 239 | function | no | out of campaign scope |
| f128_const_halves | cast.rs | 253 | function | no | out of campaign scope |
| encode_ldr_str_auto | load_store.rs | 7 | function | no | out of campaign scope |
| encode_ldr_str | load_store.rs | 33 | function | no | out of campaign scope |
| encode_ldur_stur | load_store.rs | 248 | function | yes | - |
| encode_ldtr_sized | load_store.rs | 292 | function | no | out of campaign scope |
| encode_ldrsw | load_store.rs | 311 | function | no | out of campaign scope |
| encode_ldrs | load_store.rs | 382 | function | no | out of campaign scope |
| encode_ldp_stp | load_store.rs | 452 | function | no | out of campaign scope |
| encode_ldnp_stnp | load_store.rs | 518 | function | no | out of campaign scope |
| encode_ldxr_stxr | load_store.rs | 547 | function | yes | - |
| encode_ldaxr_stlxr | load_store.rs | 573 | function | no | out of campaign scope |
| encode_ldxp_stxp | load_store.rs | 604 | function | yes | - |
| encode_ldar_stlr | load_store.rs | 637 | function | yes | - |
| encode_adrp | load_store.rs | 653 | function | no | out of campaign scope |
| encode_adr | load_store.rs | 693 | function | yes | - |
| encode_prfm | load_store.rs | 726 | function | no | out of campaign scope |
| encode_prfop | load_store.rs | 786 | function | no | out of campaign scope |
| encode_cas | load_store.rs | 814 | function | no | out of campaign scope |
| encode_swp | load_store.rs | 849 | function | no | out of campaign scope |
| encode_ldop | load_store.rs | 882 | function | no | out of campaign scope |
| encode_stop | load_store.rs | 927 | function | no | out of campaign scope |
| get_neon_reg | neon.rs | 7 | function | no | helper; campaign is single-symbol |
| encode_cnt | neon.rs | 23 | function | no | out of campaign scope |
| neon_arr_to_q_size | neon.rs | 44 | function | no | helper; campaign is single-symbol |
| encode_neon_three_same | neon.rs | 65 | function | no | out of campaign scope |
| encode_neon_three_diff | neon.rs | 89 | function | no | out of campaign scope |
| encode_neon_sqshrun | neon.rs | 119 | function | no | out of campaign scope |
| encode_neon_xtl | neon.rs | 163 | function | no | out of campaign scope |
| encode_neon_cmp_zero | neon.rs | 189 | function | no | out of campaign scope |
| encode_neon_two_misc_narrow | neon.rs | 206 | function | no | out of campaign scope |
| encode_neon_elem_long | neon.rs | 235 | function | no | out of campaign scope |
| encode_neon_logical | neon.rs | 297 | function | no | out of campaign scope |
| encode_neon_mul | neon.rs | 323 | function | no | out of campaign scope |
| encode_neon_pmul | neon.rs | 336 | function | no | out of campaign scope |
| encode_neon_mla | neon.rs | 349 | function | no | out of campaign scope |
| encode_neon_mls | neon.rs | 361 | function | no | out of campaign scope |
| encode_neon_shift_imm | neon.rs | 373 | function | no | out of campaign scope |
| encode_neon_ext | neon.rs | 405 | function | no | out of campaign scope |
| encode_neon_addv | neon.rs | 424 | function | no | out of campaign scope |
| encode_neon_across | neon.rs | 445 | function | no | out of campaign scope |
| encode_neon_umov | neon.rs | 461 | function | no | out of campaign scope |
| encode_neon_dup | neon.rs | 491 | function | no | out of campaign scope |
| encode_neon_ins | neon.rs | 549 | function | no | out of campaign scope |
| encode_neon_not | neon.rs | 608 | function | no | out of campaign scope |
| encode_neon_movi | neon.rs | 624 | function | no | out of campaign scope |
| encode_neon_bic | neon.rs | 718 | function | no | out of campaign scope |
| encode_neon_bsl | neon.rs | 735 | function | no | out of campaign scope |
| encode_neon_rev64 | neon.rs | 752 | function | no | out of campaign scope |
| encode_neon_tbl | neon.rs | 768 | function | no | out of campaign scope |
| encode_neon_tbx | neon.rs | 803 | function | no | out of campaign scope |
| encode_neon_ld1r | neon.rs | 832 | function | no | out of campaign scope |
| encode_neon_ld_st_dispatch | neon.rs | 889 | function | no | out of campaign scope |
| encode_neon_ld_st_single | neon.rs | 904 | function | no | out of campaign scope |
| encode_neon_ld_st_multi | neon.rs | 1008 | function | no | out of campaign scope |
| encode_neon_zip_uzp | neon.rs | 1094 | function | no | out of campaign scope |
| encode_neon_eor3 | neon.rs | 1112 | function | no | out of campaign scope |
| encode_neon_pmull | neon.rs | 1128 | function | no | out of campaign scope |
| encode_neon_aes | neon.rs | 1146 | function | no | out of campaign scope |
| encode_neon_add_sub | neon.rs | 1164 | function | no | out of campaign scope |
| encode_neon_ushr | neon.rs | 1179 | function | no | out of campaign scope |
| encode_neon_sshr | neon.rs | 1205 | function | no | out of campaign scope |
| encode_neon_shl | neon.rs | 1231 | function | no | out of campaign scope |
| encode_neon_sli | neon.rs | 1258 | function | yes | - |
| encode_neon_sri | neon.rs | 1285 | function | no | out of campaign scope |
| encode_neon_rbit | neon.rs | 1312 | function | no | out of campaign scope |
| encode_neon_mvni | neon.rs | 1333 | function | no | out of campaign scope |
| encode_neon_float_three_same | neon.rs | 1390 | function | yes | - |
| encode_neon_two_misc | neon.rs | 1407 | function | no | out of campaign scope |
| encode_neon_float_two_misc | neon.rs | 1420 | function | no | out of campaign scope |
| encode_neon_shrn | neon.rs | 1436 | function | no | out of campaign scope |
| encode_neon_shift_right | neon.rs | 1454 | function | no | out of campaign scope |
| encode_neon_shll | neon.rs | 1472 | function | no | out of campaign scope |
| encode_neon_qshrn | neon.rs | 1496 | function | no | out of campaign scope |
| encode_neon_three_diff_narrow | neon.rs | 1514 | function | yes | - |
| encode_neon_ldnr | neon.rs | 1528 | function | no | out of campaign scope |
| encode_neon_float_cmp_zero | neon.rs | 1576 | function | yes | - |
| encode_neon_elem | neon.rs | 1591 | function | no | out of campaign scope |
| encode_neon_float_elem | neon.rs | 1613 | function | no | out of campaign scope |
| encode_neon_fcvtl | neon.rs | 1640 | function | no | out of campaign scope |
| encode_neon_fcvtn | neon.rs | 1652 | function | no | out of campaign scope |
| encode_neon_bitwise_insert | neon.rs | 1667 | function | no | out of campaign scope |
| encode_neon_faddp | neon.rs | 1686 | function | no | out of campaign scope |
| encode_neon_across_long | neon.rs | 1724 | function | yes | - |
| encode_neon_shift_left_imm | neon.rs | 1744 | function | no | out of campaign scope |
| is_neon_scalar_d_reg_op | neon.rs | 1778 | function | no | out of campaign scope |
| encode_neon_scalar_three_same | neon.rs | 1791 | function | no | out of campaign scope |
| encode_neon_scalar_addp | neon.rs | 1802 | function | no | out of campaign scope |
| encode_neon_scalar_two_misc | neon.rs | 1819 | function | no | out of campaign scope |
| encode_neon_scalar_qshrn | neon.rs | 1835 | function | no | out of campaign scope |
| encode_cmp | compare_branch.rs | 6 | function | yes | - |
| encode_cmn | compare_branch.rs | 22 | function | yes | - |
| encode_tst | compare_branch.rs | 37 | function | no | out of campaign scope |
| encode_ccmp_ccmn | compare_branch.rs | 52 | function | yes | - |
| encode_csel | compare_branch.rs | 85 | function | yes | - |
| encode_csinc | compare_branch.rs | 99 | function | yes | - |
| encode_csinv | compare_branch.rs | 113 | function | yes | - |
| encode_csneg | compare_branch.rs | 127 | function | yes | - |
| encode_cset | compare_branch.rs | 141 | function | yes | - |
| encode_csetm | compare_branch.rs | 155 | function | yes | - |
| encode_branch | compare_branch.rs | 171 | function | yes | - |
| encode_bl | compare_branch.rs | 184 | function | yes | - |
| encode_cond_branch | compare_branch.rs | 197 | function | no | out of campaign scope |
| encode_br | compare_branch.rs | 212 | function | yes | - |
| encode_blr | compare_branch.rs | 219 | function | yes | - |
| encode_ret | compare_branch.rs | 226 | function | no | out of campaign scope |
| encode_cbz | compare_branch.rs | 237 | function | yes | - |
| encode_tbz | compare_branch.rs | 254 | function | no | out of campaign scope |
| encode_cneg | compare_branch.rs | 276 | function | yes | - |
| encode_cinc | compare_branch.rs | 293 | function | yes | - |
| encode_cinv | compare_branch.rs | 309 | function | yes | - |
