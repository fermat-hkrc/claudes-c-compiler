# Bug: encode_sbc encodes SP/WSP as XZR/WZR
**Law:** ARM Add/subtract (with carry) register 31 is XZR/WZR, never SP/WSP. `encode_sbc` with SP or WSP at any operand must return Err, matching llvm-mc which rejects `sbc wsp, w0, w0` / `sbc sp, x0, x1`.
**Impact:** `sbc wsp, w0, w0` is assembled as `sbc wzr, w0, w0`. Using the stack pointer as an SBC operand silently becomes a zero register.
**Function:** encode_sbc
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("wsp"), Reg("w0"), Reg("w0")], set_flags = false
**Expected:** Err (SP/WSP is not a valid SBC operand)
**Actual:** Ok(Word(0x5a00001f)) — parse_reg_num maps wsp to 31, encoded as WZR
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_sbc_pbt::test_encode_sbc_regression_sp
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_sbc_neg -- --test-threads=1)
