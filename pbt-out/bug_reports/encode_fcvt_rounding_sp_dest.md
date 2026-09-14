# Bug: encode_fcvt_rounding encodes SP/WSP dest as ZR
**Law:** Integer FCVT* dest is Wd|Xd using WZR/XZR at register 31, never SP/WSP. llvm-mc / gas reject `fcvtzs wsp, s0` and `fcvtzs sp, s0`.
**Impact:** `fcvtzs wsp, s0` is encoded as `fcvtzs wzr, s0` (and `sp` as `xzr`) because parse_reg_num maps sp/wsp to 31 and the encoder never checks the SP alias. Invalid GNU-style assembly assembles to a different, well-formed instruction.
**Function:** encode_fcvt_rounding
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("wsp"), Reg("s0")]` with rmode=0b11 opcode=0b000 (shrunk; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) for FCVTZS WZR, S0
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fcvt_rounding_regression_sp_dest
