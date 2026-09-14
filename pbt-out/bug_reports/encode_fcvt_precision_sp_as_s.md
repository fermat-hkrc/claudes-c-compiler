# Bug: encode_fcvt_precision treats SP as an S register
**Law:** Scalar FCVT operands are Sd|Dd|Hd and Sn|Dn|Hn. llvm-mc rejects `fcvt sp, d0` and `fcvt s0, sp` as "invalid operand for instruction". SP/WSP are stack pointers, not FP registers.
**Impact:** `fcvt sp, d0` is assembled as `fcvt s31, d0` because parse_reg_num maps "sp" to 31 and dest_name.chars().next()=='s' selects opc=00. A mistyped SP operand silently becomes S31.
**Function:** encode_fcvt_precision
**Detected by:** Negative/error contract — llvm-mc "invalid operand"; ARM ARM FCVT S/D/H only
**Minimal input:** `[Reg("sp"), Reg("s0")]` (property shrink); deterministic witness `[Reg("sp"), Reg("d0")]` (serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) encoding SP as S31
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fcvt_precision_regression_sp_as_s and test_encode_fcvt_precision_regression_sp_src
