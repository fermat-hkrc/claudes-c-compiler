# Bug: encode_fneg encodes mixed S/D (and GPR/SP/QVB) instead of rejecting
**Law:** ARM ARM FNEG syntax is `<Sd>, <Sn>` or `<Dd>, <Dn>` (same precision; also `<Hd>, <Hn>` with FEAT_FP16). Mixed S/D, GPR (Xn/Wn/XZR/WZR/LR), SP/WSP, and Q/V/B registers are not valid scalar FNEG operands and must be rejected. llvm-mc/gas reject `fneg s0, d0`.
**Impact:** `fneg s0, d0` is assembled as single-precision FNEG s0, s0 (ftype taken only from dest; source type is ignored). GPR/SP/QVB names are similarly accepted via parse_reg_num and encoded as S/D. Invalid GNU-style assembly becomes the wrong instruction.
**Function:** encode_fneg
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("s0"), Reg("d0")]` (shrunk; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — dest `s0` sets ftype=00; source `d0` is parsed only for its number (0)
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fneg_regression_mixed_sd
