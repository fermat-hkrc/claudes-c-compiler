# Bug: encode_rev accepts FP/SIMD register names as GPRs
**Law:** Scalar REV operands are W/X GPRs. llvm-mc rejects `rev d0, x1`. Vector byte-reverse uses rev16/rev32/rev64 (Advanced SIMD two-register miscellaneous), not the `rev` mnemonic.
**Impact:** `rev d0, x1` is encoded as 32-bit `rev w0, w1` (parse_reg_num strips the `d` prefix), silently assembling the wrong instruction class.
**Function:** encode_rev
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("d0"), Reg("x1")]` (serial reconfirm: PBT_TEST_JOBS=1)
**Expected:** `Err(_)`
**Actual:** `Ok(Word(...))` — `parse_reg_num` accepts d/s/q/v/h/b prefixes; `is_64bit_reg("d0")` is false so sf=0, opc=000010
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs `test_encode_rev_regression_fp`
