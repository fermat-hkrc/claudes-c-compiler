# Bug: encode_cmn accepts FP/SIMD register names as GPRs
**Law:** CMN Rn/Rm are GPR Wt/Xt (or SP/WSP in the immediate / extended forms); encode_cmn([Reg("d0"|"s0"|"q0"|"v0"|"h0"|"b0"), ...]) must be Err.
**Impact:** `cmn d0, #0` assembles as `cmn x0, #0` (parse_reg_num accepts d/s/q/v/h/b prefixes; is_32bit_reg is false for those names so XZR is prepended). That is a different instruction and diverges from gas/llvm-mc, which reject FP/SIMD registers.
**Function:** encode_cmn
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("d0"), Imm(0)]  (`cmn d0, #0`)
**Expected:** Err
**Actual:** Ok(Word) encoding `cmn x0, #0` (sf=1, Rn=0, Rd=31).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cmn_pbt::test_encode_cmn_regression_fp_reg
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cmn_neg -- --test-threads=1 reproduced encode_cmn_neg_wrong_reg (shrunk CE is XZR-imm); the d0 regression fails in isolation.
