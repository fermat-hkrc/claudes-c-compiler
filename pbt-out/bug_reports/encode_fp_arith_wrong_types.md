# Bug: encode_fp_arith encodes mixed S/D (and GPR/SP/QVB) instead of rejecting
**Law:** ARM ARM FADD/FSUB/FMUL/FDIV/FMAX/FMIN/FMAXNM/FMINNM syntax is `<Sd>, <Sn>, <Sm>` or `<Dd>, <Dn>, <Dm>` (same precision). Mixed S/D, GPR (Xn/Wn/XZR/WZR/LR), SP/WSP, and Q/V/B registers are not valid scalar FP 2-source operands and must be rejected. llvm-mc/gas reject `fadd d0, s0, s0`.
**Impact:** `fadd d0, s0, s0` is assembled as double-precision FADD d0, d0, d0 (ftype taken only from dest; source types are ignored). GPR/SP/QVB names are similarly accepted via parse_reg_num and encoded as S/D. Invalid GNU-style assembly becomes the wrong instruction.
**Function:** encode_fp_arith
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("d0"), Reg("s0"), Reg("s0")]` with opcode=0b0010 (shrunk; serial reconfirm with PBT_TEST_JOBS=1). Also `[Reg("x0"), Reg("s1"), Reg("s2")]` and `[Reg("sp"), Reg("s1"), Reg("s2")]`.
**Expected:** Err
**Actual:** Ok(Word) — dest `d0` sets ftype=01; sources `s0` are parsed only for their numbers
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fp_arith_regression_mixed_sd (also test_encode_fp_arith_regression_gpr, test_encode_fp_arith_regression_sp)
