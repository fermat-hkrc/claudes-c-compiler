# Bug: encode_rev32 accepts mixed W/X register widths
**Law:** Both REV32 operands must be 64-bit GPRs. Mixed W/X must be rejected.
**Impact:** `rev32 x0, w0` encodes as `rev32 x0, x0`. Silent operand-width mismatch; llvm-mc/gas reject it.
**Function:** encode_rev32
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("x0"), Reg("w0")]` (rd=0, rn=0, rd64=true, rn64=false)
**Expected:** Err (llvm-mc: invalid operand)
**Actual:** Ok(Word(0xdac00800)) — is_64 from get_reg is discarded
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_rev32_regression_mixed_width
**Serial reconfirm:** PBT_TEST_JOBS=1 RUST_TEST_THREADS=1 reproduced
