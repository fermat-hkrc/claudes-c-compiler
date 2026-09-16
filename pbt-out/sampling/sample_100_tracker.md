# Random sample of 100 issues (seed=42, drawn 2026-09-15 from #1..#511, excluding stub #1 & dupe #511)

Status values: real / not-a-bug / cannot-reproduce / pending

| # | issue | type | tool | status |
|---|-------|------|------|--------|
| 5 | Bug: encode_adc silently ignores a trailing shift operand | assembler-encoder-validation | data_processing | real |
| 14 | Bug: ADD/SUB silently encodes ROR (and other invalid shift | other | data_processing | real (user-confirmed) |
| 15 | Bug: ADD/SUB with SP/WSP and LSL #N uses shifted-register  | other | data_processing | real (user-confirmed) |
| 17 | Bug: encode_adr silently truncates immediates outside the  | assembler-encoder-validation | load_store | real (user-confirmed; wrap-to-+1048575 correction) |
| 18 | Bug: encode_adr accepts :lo12:/:got: modifiers as a bare A | assembler-encoder-validation | load_store | real (user-confirmed + clang cross-check) |
| 24 | Bug: encode_bic accepts mixed-width GPR operands | assembler-encoder-validation | data_processing | real (user-confirmed + clang cross-check) |
| 25 | Bug: encode_bic masks out-of-range shift amounts instead o | assembler-encoder-validation | data_processing | real (agent-run + clang cross-check) |
| 37 | Bug: encode_bl accepts :lo12: / modifier operands as Call2 | assembler-encoder-validation | compare_branch | real (agent-run + clang cross-check, user-confirmed reference) |
| 38 | Bug: encode_neon_three_diff_narrow silently ignores a four | assembler-encoder-validation | neon | real (agent-run + clang cross-check) |
| 42 | Bug: encode_blr ignores extra operands | assembler-encoder-validation | compare_branch | real (agent-run + clang cross-check) |
| 46 | Bug: encode_br ignores extra operands | assembler-encoder-validation | compare_branch | real (agent-run + clang cross-check) |
| 49 | Bug: encode_br accepts 32-bit W registers as BR Rn | assembler-encoder-validation | compare_branch | real (agent-run + clang cross-check) |
| 51 | Bug: encode_branch rejects immediate PC-offset form `b #im | assembler-encoder-validation | compare_branch | real (agent-run + clang cross-check) |
| 53 | Bug: encode_cbz ignores extra operands | assembler-encoder-validation | compare_branch | real (agent-run + clang cross-check) |
| 54 | Bug: encode_cbz accepts FP/SIMD register names as Rt | assembler-encoder-validation | compare_branch | real (agent-run + clang cross-check) |
| 59 | Bug: encode_ccmp_ccmn truncates out-of-range imm5 and nzcv | assembler-encoder-validation | compare_branch | real (agent-run + clang cross-check) |
| 65 | Bug: encode_cinc accepts mixed x/w register widths | assembler-encoder-validation | compare_branch | real (agent-run + clang cross-check) |
| 73 | Bug: encode_cmn accepts FP/SIMD register names as GPRs | assembler-encoder-validation | compare_branch | real (agent-run + clang cross-check) |
| 81 | Bug: encode_cmp accepts mixed x/w without an extend | assembler-encoder-validation | compare_branch | real (agent-run + clang cross-check) |
| 83 | Bug: encode_cmp accepts XZR/WZR as immediate-form Rn | assembler-encoder-validation | compare_branch | real (agent-run + gcc cross-check) |
| 85 | Bug: encode_cneg ignores extra operands | assembler-encoder-validation | compare_branch | real (agent-run + gcc cross-check) |
| 100 | Bug: encode_csetm encodes SP as XZR | assembler-encoder-validation | compare_branch | real (agent-run + gcc cross-check) |
| 103 | Bug: encode_csinc accepts mixed x/w register widths | assembler-encoder-validation | compare_branch | real (agent-run + gcc cross-check) |
| 109 | Bug: encode_csneg ignores extra operands | assembler-encoder-validation | compare_branch | real (agent-run + gcc cross-check) |
| 112 | Bug: encode_csneg encodes SP as XZR | assembler-encoder-validation | compare_branch | real (agent-run + gcc cross-check) |
| 113 | Bug: encode_div silently ignores a fourth operand | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 114 | Bug: encode_div accepts FP/SIMD register names as GPRs | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 116 | Bug: encode_div accepts SP/WSP as a GPR operand | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 118 | Bug: encode_eon accepts FP/SIMD register names as GPRs | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 121 | Bug: encode_eon masks out-of-range shift amounts instead o | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 127 | Bug: encode_neon_across_long ignores destination register  | assembler-encoder-validation | neon | real (agent-run + gcc cross-check) |
| 137 | Bug: encode_ldur_stur ignores extra operands | assembler-encoder-validation | load_store | real (agent-run + gcc cross-check) |
| 138 | Bug: encode_ldur_stur silently wraps offsets outside signe | assembler-encoder-validation | load_store | real (agent-run + gcc cross-check) |
| 142 | Bug: encode_ldur_stur encodes a V-register Rt as a 64-bit  | assembler-encoder-validation | load_store | real (agent-run + gcc/clang cross-check) |
| 144 | Bug: encode_ldxp_stxp ignores extra operands | assembler-encoder-validation | load_store | real (agent-run + gcc/clang cross-check) |
| 150 | Bug: encode_ldxp_stxp encodes STXP when Ws aliases a sourc | assembler-encoder-validation | load_store | real (warning-class: gas accepts w/ 3 warnings, llvm-mc errors, ccc silent) (agent-run + gcc/clang cross-check) |
| 152 | Bug: encode_ldxp_stxp encodes XZR/WZR as SP when used as t | assembler-encoder-validation | load_store | real (agent-run + gcc cross-check) |
| 174 | Bug: encode_logical accepts SP/WSP in shifted-register for | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 176 | Bug: encode_madd silently ignores a fifth operand | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 178 | Bug: encode_madd accepts mixed X/W register widths | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 183 | Bug: encode_movk accepts non-lsl shifts and illegal shift  | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 185 | Bug: encode_movn ignores extra operands | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 187 | Bug: encode_movn silently truncates immediates outside [0, | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 188 | Bug: encode_movn accepts non-lsl shifts and illegal shift  | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 191 | Bug: encode_movz encodes FP/SIMD register names as GPRs | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 195 | Bug: encode_neon_qshrn ignores extra operands beyond index | assembler-encoder-validation | neon | real (agent-run + gcc cross-check) |
| 196 | Bug: encode_neon_qshrn accepts a GPR/FP dest as if it were | assembler-encoder-validation | neon | real (agent-run + gcc cross-check) |
| 216 | Bug: encode_mvn accepts NEON arrangements other than 8b/16 | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 218 | Bug: encode_mvn encodes SP/WSP as XZR/WZR | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check; #234 panic-class) |
| 231 | Bug: encode_neon_shift_imm discards source arrangement | assembler-encoder-validation | neon | real (agent-run + gcc cross-check; #234 panic-class) |
| 234 | Bug: encode_neon_shift_imm panics or encodes out-of-range  | assembler-encoder-validation | neon | real (agent-run + gcc cross-check; #234 panic-class) |
| 237 | Bug: encode_neon_tbl panics on an empty register list | assembler-encoder-validation | neon | real (panic-class + gcc cross-check) |
| 260 | Bug: encode_sbc accepts FP/SIMD register names as GPRs | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 276 | Bug: encode_smull silently ignores a 4th operand | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 281 | Bug: encode_sxth encodes FP/SIMD register names as GPRs | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 284 | Bug: encode_sxtw ignores extra operands | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 289 | Bug: encode_neon_shift_left_imm ignores extra operands | assembler-encoder-validation | neon | real (agent-run + gcc cross-check) |
| 297 | Bug: encode_umulh ignores extra operands | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 303 | Bug: encode_neon_rbit ignores source arrangement | assembler-encoder-validation | neon | real (agent-run + gcc cross-check) |
| 304 | Bug: encode_neon_rbit encodes GPR/FP prefixes as V registe | assembler-encoder-validation | neon | real (agent-run + gcc cross-check) |
| 310 | Bug: encode_uxtw ignores extra operands | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 311 | Bug: encode_uxtw accepts FP/SIMD register names as GPRs | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 318 | Bug: encode_ldaxr_stlxr encodes SP as ZR in Rt | assembler-encoder-validation | load_store | real (agent-run + gcc cross-check) |
| 323 | Bug: encode_ldaxr_stlxr encodes XZR as SP in the exclusive | assembler-encoder-validation | load_store | real (agent-run + gcc cross-check) |
| 327 | Bug: encode_ldrsw silently truncates out-of-range offsets | assembler-encoder-validation | load_store | real (agent-run + gcc cross-check) |
| 329 | Bug: encode_ldrsw accepts a W register as the memory base | assembler-encoder-validation | load_store | real (agent-run + gcc cross-check) |
| 333 | Bug: encode_ldrsw encodes XZR/X31 as the memory base SP | assembler-encoder-validation | load_store | real (agent-run + gcc cross-check) |
| 334 | Bug: encode_ldtr_sized ignores extra operands | assembler-encoder-validation | load_store | real (agent-run + gcc cross-check) |
| 340 | Bug: encode_ldtr_sized accepts XZR as the memory base | assembler-encoder-validation | load_store | real (agent-run + gcc cross-check) |
| 345 | Bug: encode_prfm accepts a W register as the memory base | assembler-encoder-validation | load_store | real (agent-run + gcc cross-check) |
| 348 | Bug: encode_smulh ignores extra operands | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 351 | Bug: encode_smulh accepts 32-bit W registers | assembler-encoder-validation | data_processing | real (agent-run + gcc cross-check) |
| 359 | Bug: encode_fp_1src encodes mixed S/D (and GPR/SP/QVB) ins | assembler-encoder-validation | fp_scalar | real (agent-run + gcc cross-check) |
| 361 | Bug: encode_int_to_float encodes H dest as ftype=00 (singl | assembler-encoder-validation | fp_scalar | real (differential: gcc 0x1ee30000 vs ccc 0x1e230000) (agent-run + gcc cross-check) |
| 362 | Bug: encode_int_to_float encodes SP/WSP source as ZR | assembler-encoder-validation | fp_scalar | real (agent-run + gcc cross-check) |
| 368 | Bug: encode_fcvt_precision ignores a third operand | assembler-encoder-validation | fp_scalar | real (agent-run + gcc cross-check) |
| 375 | Bug: encode_neon_aes accepts non-V register prefixes | assembler-encoder-validation | neon | real (agent-run + gcc cross-check) |
| 379 | Bug: encode_bfi panics or encodes out-of-range #lsb/#width | assembler-encoder-validation | bitfield | real (panic-class) (agent-run + gcc cross-check) |
| 381 | Bug: encode_bfi treats SP/WSP as ZR | assembler-encoder-validation | bitfield | real (agent-run + gcc cross-check) |
| 390 | Bug: encode_cas accepts mixed W/X Rs and Rt | assembler-encoder-validation | load_store | real (agent-run + gcc cross-check) |
| 392 | Bug: encode_cas accepts SP/WSP as Rs or Rt | assembler-encoder-validation | load_store | real (agent-run + gcc cross-check) |
| 397 | Bug: encode_cls accepts mixed W/X register widths | assembler-encoder-validation | bitfield | real (agent-run + gcc cross-check) |
| 414 | Bug: encode_fp_arith encodes H registers as ftype=00 (sing | assembler-encoder-validation | fp_scalar | real (differential: gcc 0x1ee00800 vs ccc 0x1e200800) (agent-run + gcc cross-check) |
| 415 | Bug: encode_fp_arith encodes mixed S/D (and GPR/SP/QVB) in | assembler-encoder-validation | fp_scalar | real (agent-run + gcc cross-check) |
| 416 | Bug: encode_rbit ignores extra operands | assembler-encoder-validation | bitfield | real (agent-run + gcc cross-check) |
| 426 | Bug: encode_rev32 accepts mixed W/X register widths | assembler-encoder-validation | bitfield | real (agent-run + gcc cross-check) |
| 435 | Bug: encode_sbfiz accepts FP/SIMD registers as GPR | assembler-encoder-validation | bitfield | real (agent-run + gcc cross-check) |
| 444 | Bug: encode_bfm ignores extra operands | assembler-encoder-validation | bitfield | real (agent-run + gcc cross-check) |
| 447 | Bug: encode_bfm accepts mixed W/X registers | assembler-encoder-validation | bitfield | real (agent-run + gcc cross-check) |
| 458 | Bug: encode_sbfx treats SP/WSP as ZR | assembler-encoder-validation | bitfield | real (agent-run + gcc cross-check) |
| 465 | Bug: encode_ubfx accepts FP/SIMD registers as GPR operands | assembler-encoder-validation | bitfield | real (agent-run + gcc cross-check) |
| 475 | Bug: encode_fabs encodes H registers as ftype=00 (single) | assembler-encoder-validation | fp_scalar | real (differential) (agent-run + gcc cross-check) |
| 481 | Bug: encode_fneg encodes H registers as ftype=00 (single) | assembler-encoder-validation | fp_scalar | real (differential) (agent-run + gcc cross-check) |
| 489 | Bug: encode_neon_dup accepts wrong-width and non-GPR sourc | assembler-encoder-validation | neon | real (agent-run + gcc cross-check) |
| 491 | Bug: encode_ldrs ignores operands beyond the second | assembler-encoder-validation | load_store | real (agent-run + gcc cross-check) |
| 494 | Bug: encode_ldrs accepts SP/WSP as the destination | assembler-encoder-validation | load_store | real (agent-run + gcc cross-check) |
| 497 | Bug: encode_ldrs encodes pre/post-index when Rt==Rn (Rn!=S | assembler-encoder-validation | load_store | real (warning-class: gcc accepts w/ warning, ccc silent) (agent-run + gcc cross-check) |
| 498 | Bug: encode_ldrs accepts XZR as the memory base | assembler-encoder-validation | load_store | real (agent-run + gcc cross-check) |
| 501 | Bug: SIMD/FP register accepted as ldNr base | other | neon | real (agent-run + gcc cross-check) |
| 507 | Bug: XZR and x31 are accepted as ldNr base (encoded as SP) | assembler-encoder-validation | neon | real (agent-run + gcc cross-check) |

Progress: **100/100 verified — 100 real, 0 false positives, 0 not-reproducible**

> **Superseded by the full run:** all 509 issues later verified live → [full_verification_tracker.md](../full_verification_tracker.md) — 506 real (99.4%), 3 false positives (#30/#119/#247, outside this sample).