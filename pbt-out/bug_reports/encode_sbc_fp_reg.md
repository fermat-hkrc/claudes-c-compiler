# Bug: encode_sbc accepts FP/SIMD register names as GPRs
**Law:** ARM SBC Rd/Rn/Rm are general-purpose registers. FP/SIMD names (d/s/q/v/h/b) must return Err, matching llvm-mc which rejects `sbc d0, x1, x2`.
**Impact:** `sbc d0, x1, x2` is assembled as `sbc w0, w1, w2` (sf taken from Rd; d0 parses as register 0). Vector/FP operands silently become integer SBC.
**Function:** encode_sbc
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("d0"), Reg("x1"), Reg("x2")], set_flags = false
**Expected:** Err (FP/SIMD register is not a valid SBC operand)
**Actual:** Ok(Word(0x5a020020)) — parse_reg_num accepts prefix d and number 0; is_64bit_reg is false for d0 so sf=0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_sbc_pbt::test_encode_sbc_regression_fp_reg
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_sbc_neg -- --test-threads=1)
