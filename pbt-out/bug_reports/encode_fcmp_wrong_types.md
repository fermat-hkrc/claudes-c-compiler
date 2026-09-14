# Bug: encode_fcmp accepts mixed S/D, GPR, Q/V/B, and SP
**Law:** ARM FCMP requires matching `Sn, Sm` or `Dn, Dm` (or `Hn, Hm` with fp16). llvm-mc rejects `fcmp s0, d0`, `fcmp x0, x1`, `fcmp q0, q1`, and `fcmp s0, sp` as "invalid operand for instruction".
**Impact:** Mixed `s0, d0` is encoded as `fcmp s0, s0` (ftype taken only from operand 0). GPR/Q/V/B/SP names are parsed as 5-bit register numbers and packed into the FP compare encoding, producing a well-formed but wrong FCMP.
**Function:** encode_fcmp
**Detected by:** Negative/error contract — llvm-mc "invalid operand"; ARM ARM FCMP Sn/Sm or Dn/Dm
**Minimal input:** `[Reg("s0"), Reg("d0")]` (shrunk mixed S/D; serial reconfirm with PBT_TEST_JOBS=1). Same class also encodes GPR, Q/V/B, and SP/WSP.
**Expected:** Err
**Actual:** Ok(Word(0x1e202000)) — ftype=00, Rm=0, Rn=0 (same bits as `fcmp s0, s0`)
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fcmp_regression_mixed_sd
