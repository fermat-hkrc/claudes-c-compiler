# PBT Coverage Status

| Function | Source file | Test file | Test target | Notes |
|----------|-------------|-----------|-------------|-------|
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
