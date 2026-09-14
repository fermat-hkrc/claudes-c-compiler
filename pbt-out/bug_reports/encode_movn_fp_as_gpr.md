# Bug: encode_movn encodes FP/SIMD register names as GPRs
**Law:** MOVN operands are general-purpose registers (Xn/Wn/XZR/WZR). FP/SIMD names (d/s/q/v/h/b) must be rejected.
**Impact:** `movn d0, #0` is accepted and encoded as `movn w0, #0` (parse_reg_num accepts prefix `d` and is_64bit_reg is false), so SIMD names silently retarget a GPR.
**Function:** encode_movn
**Detected by:** Negative/Error Contract — FP/SIMD as Rd
**Minimal input:** `movn d0, #0` (operands `[Reg("d0"), Imm(0)]`)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) encoding 32-bit MOVN with Rd=0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_movn_pbt::test_encode_movn_regression_fp
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 (same shrunk witness `fp="d0", imm=0`)
