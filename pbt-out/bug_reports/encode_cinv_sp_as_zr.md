# Bug: encode_cinv encodes SP/WSP as XZR/WZR
**Law:** CINV Rd/Rn are Wt/Xt with register 31 = XZR/WZR, never SP/WSP; encode_cinv([Reg("sp"|"wsp"), ...]) must be Err.
**Impact:** `cinv sp, x0, eq` assembles as `cinv xzr, x0, eq`. That is a different instruction (writes XZR, not SP) and diverges from gas/llvm-mc, which reject SP.
**Function:** encode_cinv
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("sp"), Reg("x0"), Cond("eq")]  (`cinv sp, x0, eq`)
**Expected:** Err
**Actual:** Ok(Word) with Rd=31 (XZR) — parse_reg_num maps "sp"/"wsp" to 31 and get_reg does not reject SP.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cinv_pbt::test_encode_cinv_regression_sp
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cinv_neg -- --test-threads=1 reproduced encode_cinv_neg_wrong_reg (shrunk CE kind=0 n=0).
