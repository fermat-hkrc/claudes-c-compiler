# Bug: encode_fmov encodes SP/WSP as an FP or ZR register
**Law:** FMOV (general) uses WZR/XZR at register 31, never SP/WSP. SP is not an FP register. llvm-mc / gas reject `fmov s0, sp` and `fmov s0, wsp`.
**Impact:** `fmov wsp, s0` (shrunk) is encoded because wsp is treated as a 32-bit GP (parse_reg_num maps wsp to 31) i.e. `fmov wzr, s0`. `fmov s0, sp` is worse: is_fp_reg("sp") is true (prefix 's'), so both operands look like FP registers and the word is `fmov s0, s31`. Invalid GNU-style assembly assembles to a different, well-formed instruction.
**Function:** encode_fmov
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("wsp"), Reg("s0")]` (shrunk is_64=false, n=0, which=0, partner_fp=true; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) for FMOV WZR, S0 (and FMOV S0, S31 when the name is "sp")
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fmov_regression_sp_src
