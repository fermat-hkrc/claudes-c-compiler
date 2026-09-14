# Bug: encode_fmadd_fmsub encodes mixed S/D (and GPR/SP/QVB) instead of rejecting
**Law:** ARM ARM FMADD/FMSUB syntax is `<Sd>, <Sn>, <Sm>, <Sa>` or `<Dd>, <Dn>, <Dm>, <Da>` (same precision). Mixed S/D, GPR (Xn/Wn/XZR/WZR/LR), SP/WSP, and Q/V/B registers are not valid scalar FP 3-source operands and must be rejected. llvm-mc/gas reject `fmadd d0, s0, s0, s0`.
**Impact:** `fmadd d0, s0, s0, s0` is assembled as double-precision FMADD d0, d0, d0, d0 (ftype taken only from dest; source types are ignored). GPR/SP/QVB names are similarly accepted via parse_reg_num and encoded as S/D. Invalid GNU-style assembly becomes the wrong instruction.
**Function:** encode_fmadd_fmsub
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("d0"), Reg("s0"), Reg("s0"), Reg("s0")]` with is_sub=false (shrunk; serial reconfirm with PBT_TEST_JOBS=1). Also `[Reg("x0"), Reg("s1"), Reg("s2"), Reg("s3")]` and `[Reg("s0"), Reg("s1"), Reg("s2"), Reg("sp")]`.
**Expected:** Err
**Actual:** Ok(Word) — dest `d0` sets ftype=01; sources `s0` are parsed only for their numbers
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fmadd_fmsub_regression_mixed_sd (also test_encode_fmadd_fmsub_regression_gpr, test_encode_fmadd_fmsub_regression_sp)
