# Bug: explicit lsl #12 ADD/SUB immediate masks overflow instead of rejecting
**Law:** ADD/SUB (immediate) `imm12` is a 12-bit field. With explicit `lsl #12`, the unshifted immediate must be in 0..=4095. llvm-mc: `add w0, w1, #4097, lsl #12` → "integer in range [0, 4095]". The SUT comment itself says the immediate "must fit in 12 bits".
**Impact:** `add Rd, Rn, #4097, lsl #12` is encoded as `add Rd, Rn, #1, lsl #12` (4097 & 0xFFF = 1), a silent wrong immediate.
**Function:** encode_add_sub
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("w0"), Reg("w1"), Imm(4097), Shift { kind: "lsl", amount: 12 }], is_sub=false, set_flags=false
**Expected:** Err (immediate does not fit in add/sub imm12 encoding)
**Actual:** Ok(Word) via `((imm_val as u32) & 0xFFF, 1u32)` in the explicit-shift arm
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_add_sub_regression_imm12_lsl12_overflow
**Serial reconfirmation:** reproduced with `cargo test --lib encode_add_sub_neg_imm_out_of_range -- --test-threads=1`
