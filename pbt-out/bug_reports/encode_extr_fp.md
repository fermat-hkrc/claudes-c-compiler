# Bug: encode_extr accepts FP/SIMD registers as GPRs
**Law:** EXTR is a GPR instruction. FP/SIMD prefixes (d/s/q/v/h/b) in Rd, Rn, or Rm must return Err (llvm-mc: invalid operand).
**Impact:** `extr d0, x1, x2, #0` is encoded as if X0 were the destination (parse_reg_num accepts 'd' prefix and is_64bit_reg is false for 'd', so sf=0). Malformed FP assembly is assembled as a 32-bit EXTR.
**Function:** encode_extr
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("d0"), Reg("x1"), Reg("x2"), Imm(0)]  (which=0, prefix=d, n=0)
**Expected:** Err
**Actual:** Ok(Word). parse_reg_num accepts d/s/q/v/h/b prefixes; encode_extr does not call is_fp_reg.
**Severity:** medium
**Fix:** Reject FP/SIMD register names in Rd/Rn/Rm.
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_extr_regression_fp
**Serial reconfirmation:** reproduced with cargo test --lib encode_extr -- --test-threads=1
