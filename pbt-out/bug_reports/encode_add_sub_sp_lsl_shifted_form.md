# Bug: ADD/SUB with SP/WSP and LSL #N uses shifted-register form (XZR) instead of extended-register form
**Law:** When Rd or Rn is SP/WSP, AArch64 ADD/SUB register form must use the extended-register encoding (bit 21 = 1) so register 31 means SP, not XZR. `LSL #N` with N in 0..=4 is the documented assembler alias for UXTX/UXTW #N. llvm-mc encodes `add w0, wsp, w0, lsl #1` as `0x0b2047e0` (extended, UXTW #1).
**Impact:** `add w0, wsp, w0, lsl #1` is encoded as ADD W0, WZR, W0, LSL #1 (`0x0b0007e0`). The stack pointer is replaced by the zero register; generated code computes the wrong value.
**Function:** encode_add_sub
**Detected by:** Differential vs llvm-mc (5)
**Minimal input:** operands = [Reg("w0"), Reg("wsp"), Reg("w0"), Shift { kind: "lsl", amount: 1 }], is_sub=false, set_flags=false
**Expected:** Ok(Word(0x0b2047e0)) — extended register, option=UXTW, imm3=1, Rn=31
**Actual:** Ok(Word(0x0b0007e0)) — shifted register (the SP special case only fires when `operands.len() <= 3`, so a following LSL operand skips it)
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_add_sub_regression_sp_lsl_extended
**Serial reconfirmation:** reproduced with `cargo test --lib encode_add_sub_diff_extended_and_sp -- --test-threads=1`
