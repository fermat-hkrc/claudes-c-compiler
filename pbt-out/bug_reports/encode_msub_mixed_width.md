# Bug: encode_msub accepts mixed X/W register widths
**Law:** ARM MSUB uses a single sf bit for all four registers. llvm-mc rejects `msub w0, w0, w0, x0` ("invalid operand"). encode_msub must Err when Rd/Rn/Rm/Ra are not all W or all X.
**Impact:** `msub w0, w0, w0, x0` is assembled as 32-bit MSUB with Ra=0 (sf taken only from Rd). The object file contains a different instruction than the source text — the X/W mismatch is dropped.
**Function:** encode_msub
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("w0"), Reg("w0"), Reg("w0"), Reg("x0")]
**Expected:** Err (mixed-width MSUB is not valid AArch64)
**Actual:** Ok(Word) — sf from Rd only; Rn/Rm/Ra widths discarded by `let (rn, _) = get_reg(...)`. Reproduced serially (`PBT_TEST_JOBS=1`).
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_msub_pbt::test_encode_msub_regression_mixed_width
