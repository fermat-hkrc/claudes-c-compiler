# Bug: encode_fmov encodes incompatible register classes instead of rejecting
**Law:** ARM FMOV (register) requires matching Sd,Sn or Dd,Dn (or Hd,Hn). ARM FMOV (general) requires matching widths Sd/Wn or Dd/Xn (or Hd/Wn). Mixed S/D, size-mismatched GP/FP (Dd,Wn / Sd,Xn / Xd,Sn / Wd,Dn), Q/V/B scalar, and two GP registers are not valid FMOV operands and must be rejected. llvm-mc/gas reject `fmov s0, d1`, `fmov d0, w1`, and `fmov q0, s0`.
**Impact:** `fmov q0, s0` is assembled as FP-to-FP with ftype=00 (S). `fmov s0, d1` is assembled as double (is_double = dest-or-src starts with 'd'). `fmov d0, w1` is assembled as `fmov d0, x1` (sf taken only from the FP register). Invalid GNU-style assembly becomes the wrong instruction.
**Function:** encode_fmov
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("q0"), Reg("s0")]` (shrunk; serial reconfirm with PBT_TEST_JOBS=1). Also `s0,d1` and `d0,w1`.
**Expected:** Err
**Actual:** Ok(Word) — is_fp_reg treats q/v/h/b as FP; mixed S/D uses OR of d-prefix; GP-FP path ignores GP width
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fmov_regression_mixed_sd, test_encode_fmov_regression_size_mismatch_d_w, test_encode_fmov_regression_qvb
