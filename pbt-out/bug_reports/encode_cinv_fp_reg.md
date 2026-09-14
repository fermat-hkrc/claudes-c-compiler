# Bug: encode_cinv accepts FP/SIMD register names as GPRs
**Law:** CINV Rd/Rn are GPR Wt/Xt only; encode_cinv([Reg("d0"|"s0"|"q0"|"v0"|"h0"|"b0"), ...]) must be Err.
**Impact:** `cinv d0, d0, eq` assembles as a 32-bit GPR CINV of w0 (parse_reg_num accepts d/s/q/v/h/b prefixes; is_64bit_reg is false for those names). That is a different instruction and diverges from gas/llvm-mc, which reject FP/SIMD registers.
**Function:** encode_cinv
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("d0"), Reg("d0"), Cond("eq")]  (`cinv d0, d0, eq`)
**Expected:** Err
**Actual:** Ok(Word) encoding `cinv w0, w0, eq` (sf=0, Rd=0, Rn=0).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cinv_pbt::test_encode_cinv_regression_fp_reg
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cinv_neg -- --test-threads=1 reproduced encode_cinv_neg_wrong_reg (shrunk CE is SP); the d0 regression fails in isolation.
