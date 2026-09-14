# Bug: encode_cmp accepts XZR/WZR as immediate-form Rn
**Law:** CMP immediate-form Rn is Xn|SP / Wn|WSP; register 31 in that form is SP/WSP. encode_cmp([Reg("xzr"|"wzr"), Imm(imm)]) must be Err.
**Impact:** `cmp xzr, #0` encodes as `cmp sp, #0` (SUBS XZR, SP, #0). That is a different instruction (compares SP, not XZR) and diverges from gas/llvm-mc, which reject XZR/WZR as CMP-immediate Rn.
**Function:** encode_cmp
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("xzr"), Imm(0)]  (`cmp xzr, #0`)
**Expected:** Err
**Actual:** Ok(Word(0xf10003ff)) — the same encoding as `cmp sp, #0`. WZR similarly encodes as `cmp wsp, #0`.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cmp_pbt::test_encode_cmp_regression_xzr_imm
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cmp_neg_ -- --test-threads=1 reproduced encode_cmp_neg_wrong_reg (shrunk CE kind=0, n=0, imm=0).
