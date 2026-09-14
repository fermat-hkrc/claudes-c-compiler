# Bug: encode_cmn encodes SP as Rm as XZR
**Law:** CMN shifted-register Rm is Xm/Wm with register 31 = XZR/WZR; SP as Rm without an extend must be Err.
**Impact:** `cmn x0, sp` assembles as `cmn x0, xzr`. That is a different instruction (adds XZR, not SP) and diverges from gas/llvm-mc, which reject SP as Rm unless an extend is present.
**Function:** encode_cmn
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Reg("sp")]  (`cmn x0, sp`)
**Expected:** Err
**Actual:** Ok(Word) with Rm=31 (XZR) — parse_reg_num maps "sp" to 31 and the shifted-register form treats 31 as XZR.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cmn_pbt::test_encode_cmn_regression_sp_rm
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cmn_neg -- --test-threads=1 reproduced encode_cmn_neg_wrong_reg; SP-as-Rm is kind=6 of the same property.
