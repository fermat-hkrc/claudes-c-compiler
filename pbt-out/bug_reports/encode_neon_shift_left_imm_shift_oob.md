# Bug: encode_neon_shift_left_imm does not reject out-of-range shift
**Law:** ARM ARM SQSHL/UQSHL (immediate) and llvm-mc require shift in `[0, esize(T)-1]` (e.g. `[0, 7]` for `.8b`). Out-of-range immediates must be `Err`, not a panic or a silently wrapped encoding.
**Impact:** Negative `#-1` debug-panics (`attempt to add with overflow` at `esize + shift` after `as u32`). Shift equal to esize (`#8` on `.8b`) encodes as a different element size (`immh:immb = 16` looks like a 16-bit lane). i64 values above `u32::MAX` truncate via `as u32` (e.g. `1<<32` becomes `#0`). llvm-mc rejects all of these (`immediate must be an integer in range [0, 7]`).
**Function:** encode_neon_shift_left_imm
**Detected by:** Negative/Error Contract
**Minimal input:** `[RegArrangement(v0, 8b), RegArrangement(v0, 8b), Imm(-1)]` with `u=0`, `opcode=0b01110` (asm `sqshl v0.8b, v0.8b, #-1`). Also `Imm(8)` and `Imm(4294967296)`.
**Expected:** `Err(...)`
**Actual:** debug panic `attempt to add with overflow` at neon.rs:1768 for `Imm(-1)`; `Ok(Word(...))` with wrapped `immh:immb` for `Imm(8)`
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_shift_left_imm_pbt::test_encode_neon_shift_left_imm_regression_negative_shift (also shift_eq_esize, shift_i64_trunc)
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1` / `--test-threads=1`
