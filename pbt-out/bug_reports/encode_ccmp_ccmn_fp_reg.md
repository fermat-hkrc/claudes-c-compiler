# Bug: encode_ccmp_ccmn accepts FP/SIMD register names as GPRs
**Law:** CCMP/CCMN Rn/Rm are GPR Wt/Xt only; encode_ccmp_ccmn([Reg("d0"|"s0"|"q0"|"v0"|"h0"|"b0"), ...]) must be Err.
**Impact:** `ccmp d0, #0, #0, eq` assembles as a 32-bit GPR CCMP of w0 (parse_reg_num accepts d/s/q/v/h/b prefixes; is_64bit_reg is false for those names). That is a different instruction and diverges from gas/llvm-mc, which reject FP/SIMD Rn.
**Function:** encode_ccmp_ccmn
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("d0"), Imm(0), Imm(0), Cond("eq")], is_ccmp=true  (`ccmp d0, #0, #0, eq`)
**Expected:** Err
**Actual:** Ok(Word) encoding `ccmp w0, #0, #0, eq` (sf=0, Rn=0).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_ccmp_ccmn_pbt::test_encode_ccmp_ccmn_regression_fp_reg
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_ccmp_ccmn_neg_wrong_reg_class -- --test-threads=1 reproduced SP as the shrunk CE of the same property; the d0 regression fails in isolation.
