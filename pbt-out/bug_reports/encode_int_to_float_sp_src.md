# Bug: encode_int_to_float encodes SP/WSP source as ZR
**Law:** Integer SCVTF/UCVTF source must be Wn|Xn (register 31 is WZR/XZR). llvm-mc rejects `scvtf s0, sp` and `scvtf s0, wsp` ("invalid operand").
**Impact:** `scvtf s0, wsp` is assembled as `scvtf s0, wzr`; `scvtf d0, sp` as `scvtf d0, xzr` (sf=1 because is_64bit_reg("sp") is true). Stack-pointer conversion silently becomes a zero-register conversion.
**Function:** encode_int_to_float
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("s0"), Reg("wsp")]` with is_signed=true (shrunk is_64=false, rd=0, dest_d=false; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps sp/wsp to 31, which is ZR for this instruction class
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_int_to_float_regression_sp_src
