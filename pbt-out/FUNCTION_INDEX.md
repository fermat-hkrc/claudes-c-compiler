> Total files: 3 | Total functions: 76 | PBT candidates: 3 | Excluded: 73

| Function | Source File | Line | Kind | PBT Candidate | Reason |
|----------|-------------|------|------|---------------|--------|
| encode_mov | data_processing.rs | 6 | function | no | out of campaign scope |
| encode_mov_wide_imm | data_processing.rs | 144 | function | no | out of campaign scope |
| resolve_abs_g_modifier | data_processing.rs | 183 | function | no | out of campaign scope |
| encode_movz | data_processing.rs | 201 | function | no | out of campaign scope |
| encode_movk | data_processing.rs | 234 | function | no | out of campaign scope |
| encode_movn | data_processing.rs | 266 | function | no | out of campaign scope |
| encode_add_sub | data_processing.rs | 291 | function | yes | - |
| encode_logical | data_processing.rs | 455 | function | no | out of campaign scope |
| encode_bitmask_imm | data_processing.rs | 505 | function | no | out of campaign scope |
| encode_mul | data_processing.rs | 584 | function | no | out of campaign scope |
| encode_madd | data_processing.rs | 598 | function | no | out of campaign scope |
| encode_msub | data_processing.rs | 608 | function | no | out of campaign scope |
| encode_div | data_processing.rs | 618 | function | no | out of campaign scope |
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
| encode_adc | data_processing.rs | 774 | function | no | out of campaign scope |
| encode_sbc | data_processing.rs | 784 | function | no | out of campaign scope |
| encode_shift | data_processing.rs | 796 | function | no | out of campaign scope |
| encode_sxtw | data_processing.rs | 855 | function | no | out of campaign scope |
| encode_sxth | data_processing.rs | 863 | function | no | out of campaign scope |
| encode_sxtb | data_processing.rs | 872 | function | no | out of campaign scope |
| encode_uxtw | data_processing.rs | 881 | function | no | out of campaign scope |
| encode_uxth | data_processing.rs | 891 | function | no | out of campaign scope |
| encode_uxtb | data_processing.rs | 900 | function | no | out of campaign scope |
| encode_orn | data_processing.rs | 910 | function | no | out of campaign scope |
| encode_eon | data_processing.rs | 952 | function | no | out of campaign scope |
| encode_bics | data_processing.rs | 981 | function | no | out of campaign scope |
| encode_bic | data_processing.rs | 1013 | function | no | out of campaign scope |
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
