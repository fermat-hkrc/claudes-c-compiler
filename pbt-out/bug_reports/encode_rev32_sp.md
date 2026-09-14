# Bug: encode_rev32 accepts SP/WSP as register 31
**Law:** REV32 register 31 is XZR, not SP/WSP. SP is not a valid operand.
**Impact:** `rev32 wsp, x0` / `rev32 sp, x0` encode as `rev32 xzr, x0`. Wrong instruction emitted; gas/llvm-mc reject SP.
**Function:** encode_rev32
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("wsp"), Reg("x0")]` (which=0, sp64=false, other=0)
**Expected:** Err (llvm-mc: invalid operand; ARM ARM register 31 is ZR not SP)
**Actual:** Ok(Word) — parse_reg_num maps sp/wsp to 31
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/bitfield.rs::test_encode_rev32_regression_sp
**Serial reconfirm:** PBT_TEST_JOBS=1 RUST_TEST_THREADS=1 reproduced
