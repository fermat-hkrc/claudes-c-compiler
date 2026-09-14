# Bug: encode_ccmp_ccmn truncates out-of-range imm5 and nzcv instead of rejecting them
**Law:** For CCMP/CCMN immediate form, imm5 must be in [0, 31] and nzcv in [0, 15]; values outside those ranges must be Err (llvm-mc / ARM ARM 5-bit and 4-bit unsigned fields).
**Impact:** `ccmp x0, #-1, #0, eq` and `ccmp x0, #0, #16, eq` assemble to a different instruction (`#31` / `#0`) instead of being rejected. That silently changes the compared immediate or the NZCV flags written on condition-fail, diverging from gas.
**Function:** encode_ccmp_ccmn
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Imm(-1), Imm(0), Cond("eq")], is_ccmp=false  (`ccmn x0, #-1, #0, eq`); also Imm(16) for nzcv (`ccmp x0, #0, #16, eq`) and Imm(32) for imm5
**Expected:** Err
**Actual:** Ok(Word) — imm5 is stored as `(*imm5 as u32 & 0x1F)` and nzcv as `(*nzcv as u32 & 0xF)`, so #-1 encodes as #31 and #16 encodes as #0.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_ccmp_ccmn_pbt::test_encode_ccmp_ccmn_regression_imm5_oor and test_encode_ccmp_ccmn_regression_nzcv_oor
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_ccmp_ccmn_neg_imm5_nzcv_oor -- --test-threads=1 reproduced the failure.
