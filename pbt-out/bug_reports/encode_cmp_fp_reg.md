# Bug: encode_cmp accepts FP/SIMD register names as GPRs
**Law:** CMP takes GPR/SP operands only. encode_cmp([Reg("d0"|"s0"|"q0"|"v0"|"h0"|"b0"), ...]) must be Err.
**Impact:** `cmp d0, #0` is encoded as `cmp x0, #0` because parse_reg_num accepts d/s/q/v/h/b prefixes. That is a different instruction and diverges from gas/llvm-mc, which reject FP/SIMD CMP.
**Function:** encode_cmp
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("d0"), Imm(0)]  (`cmp d0, #0`)
**Expected:** Err
**Actual:** Ok(Word) encoding `cmp x0, #0` (parse_reg_num("d0") = 0, treated as 64-bit).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cmp_pbt::test_encode_cmp_regression_fp_reg
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cmp_pbt -- --test-threads=1 reproduced test_encode_cmp_regression_fp_reg.
