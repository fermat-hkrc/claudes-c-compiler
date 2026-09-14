# Bug: encode_madd accepts mixed X/W register widths
**Law:** ARM MADD uses a single sf bit for all four registers. llvm-mc rejects `madd w0, w0, w0, x0` ("invalid operand"). encode_madd must Err when Rd/Rn/Rm/Ra are not all W or all X.
**Impact:** `madd w0, w0, w0, x0` is assembled as 32-bit MADD with Ra=0 (sf taken only from Rd). The object file contains a different instruction than the source text — the X/W mismatch is dropped.
**Function:** encode_madd
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("w0"), Reg("w0"), Reg("w0"), Reg("x0")]
**Expected:** Err (mixed-width MADD is not valid AArch64)
**Actual:** Ok(Word) — sf from Rd only; Rn/Rm/Ra widths discarded by `let (rn, _) = get_reg(...)`. Reproduced serially (`PBT_TEST_JOBS=1`).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_madd_pbt::test_encode_madd_regression_mixed_width
