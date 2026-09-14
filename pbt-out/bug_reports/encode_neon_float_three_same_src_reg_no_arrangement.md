# Bug: encode_neon_float_three_same accepts a source without arrangement T
**Law:** Vector FP three-same sources must be `Vn.T` / `Vm.T` (RegArrangement). A bare register (`Operand::Reg`) is invalid; llvm-mc rejects `fadd v0.2s, v0, v0.2s`.
**Impact:** A missing arrangement on Vn or Vm is discarded (`let (rn, _) = get_neon_reg(...)`) and the instruction is encoded using dest T, producing a well-formed word for invalid assembly.
**Function:** encode_neon_float_three_same
**Detected by:** Negative/Error Contract
**Minimal input:** `fadd v0.2s, v0, v0.2s` — `[RegArrangement(v0,2s), Reg(v0), RegArrangement(v0,2s)]`, U=0, size_hi=0, opcode=0b11010
**Expected:** Err (source needs arrangement T)
**Actual:** Ok(Word) of the 2s form. Serial reconfirm: `PBT_TEST_JOBS=1` reproduced.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_float_three_same_pbt::test_encode_neon_float_three_same_regression_src_reg_no_arrangement
