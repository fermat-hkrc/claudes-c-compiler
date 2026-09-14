# Bug: encode_sbc silently ignores a trailing shift operand
**Law:** ARM SBC has no shifted-register form; `encode_sbc([Rd, Rn, Rm, Shift{...}], _)` must return Err, matching llvm-mc / GNU as which reject `sbc Rd, Rn, Rm, lsl #N`.
**Impact:** Source text `sbc w0, w0, w0, lsl #0` is assembled as unshifted `sbc w0, w0, w0` instead of being rejected. The assembled object silently disagrees with the written instruction.
**Function:** encode_sbc
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("w0"), Reg("w0"), Reg("w0"), Shift { kind: "lsl", amount: 0 }], set_flags = false
**Expected:** Err (invalid operand; SBC has no shift field; bits 15:10 are fixed 000000)
**Actual:** Ok(Word) — extra operand ignored; encodes as `sbc w0, w0, w0` (0x5a000000)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_sbc_pbt::test_encode_sbc_regression_extra_shift
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_sbc_neg -- --test-threads=1)
