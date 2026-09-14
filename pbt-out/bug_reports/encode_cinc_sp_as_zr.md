# Bug: encode_cinc encodes SP/WSP as XZR/WZR
**Law:** CINC Rd/Rn are Wt/Xt with register 31 = XZR/WZR, never SP/WSP; encode_cinc([Reg("sp"|"wsp"), ...]) must be Err.
**Impact:** `cinc sp, x0, eq` assembles as `cinc xzr, x0, eq`. That is a different instruction (writes XZR, not SP) and diverges from gas/llvm-mc, which reject SP.
**Function:** encode_cinc
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("sp"), Reg("x0"), Cond("eq")]  (`cinc sp, x0, eq`)
**Expected:** Err
**Actual:** Ok(Word) with Rd=31 (XZR) — parse_reg_num maps "sp"/"wsp" to 31 and get_reg does not reject SP.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cinc_pbt::test_encode_cinc_regression_sp
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cinc_pbt -- --test-threads=1 reproduced encode_cinc_neg_wrong_reg (shrunk CE kind=0 n=0).
