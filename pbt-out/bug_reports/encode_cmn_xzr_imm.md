# Bug: encode_cmn accepts XZR/WZR as immediate-form Rn
**Law:** CMN immediate-form Rn is Xn|SP / Wn|WSP; register 31 in that form is SP/WSP. encode_cmn([Reg("xzr"|"wzr"), Imm(imm)]) must be Err.
**Impact:** `cmn xzr, #0` encodes as `cmn sp, #0` (ADDS XZR, SP, #0). That is a different instruction (compares SP, not XZR) and diverges from gas/llvm-mc, which reject XZR/WZR as CMN-immediate Rn.
**Function:** encode_cmn
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("xzr"), Imm(0)]  (`cmn xzr, #0`)
**Expected:** Err
**Actual:** Ok(Word(0xb10003ff)) — the same encoding as `cmn sp, #0`. WZR similarly encodes as `cmn wsp, #0`.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cmn_pbt::test_encode_cmn_regression_xzr_imm
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cmn_neg -- --test-threads=1 reproduced encode_cmn_neg_wrong_reg (shrunk CE kind=0, n=0, imm=0).
