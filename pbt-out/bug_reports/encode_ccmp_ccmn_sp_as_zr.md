# Bug: encode_ccmp_ccmn encodes SP/WSP as XZR/WZR
**Law:** CCMP/CCMN Rn is Wt/Xt with register 31 = XZR/WZR, never SP/WSP; encode_ccmp_ccmn([Reg("sp"|"wsp"), ...]) must be Err.
**Impact:** `ccmn sp, #0, #0, eq` assembles as `ccmn xzr, #0, #0, eq`. That is a different instruction (compares XZR, not SP) and diverges from gas/llvm-mc, which reject SP.
**Function:** encode_ccmp_ccmn
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("sp"), Imm(0), Imm(0), Cond("eq")], is_ccmp=false  (`ccmn sp, #0, #0, eq`)
**Expected:** Err
**Actual:** Ok(Word) with Rn=31 (XZR) — parse_reg_num maps "sp"/"wsp" to 31 and get_reg does not reject SP.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_ccmp_ccmn_pbt::test_encode_ccmp_ccmn_regression_sp
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_ccmp_ccmn_neg_wrong_reg_class -- --test-threads=1 reproduced the failure.
