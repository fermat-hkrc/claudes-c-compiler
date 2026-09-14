# Bug: encode_neon_shift_imm panics or encodes out-of-range USHR shift
**Law:** ARM Advanced SIMD USHR shift must be an integer in range [1, esize] (8 for .8b/.16b, 16 for .4h/.8h, 32 for .2s/.4s, 64 for .2d). Inputs outside that range must be rejected as Err, matching llvm-mc / GNU as. The encoder must not panic.
**Impact:** A negative immediate (e.g. `#-1`) panics in debug with `attempt to subtract with overflow` at `neon.rs:390` (`16 - shift as u32`). Shift 0 and esize+1 do not panic: they wrap/mask immh:immb, often producing immh=0000 (a different encoding group, not a shift-by-immediate). Invalid assembly either crashes the assembler or silently emits a wrong word.
**Function:** encode_neon_shift_imm
**Detected by:** Negative/Error Contract (4e) — encode_neon_shift_imm_neg_shift_oob
**Minimal input:** `encode_neon_shift_imm([v0.8b, v0.8b, Imm(-1)], true)` corresponding to `ushr v0.8b, v0.8b, #-1`. Related: `#0` and `#9` for .8b.
**Expected:** `Err` (llvm-mc: "immediate must be an integer in range [1, 8]")
**Actual:** panic `attempt to subtract with overflow` for Imm(-1); `Ok(Word)` for Imm(0)/Imm(9) via `(16 - shift as u32) & 0xF` with no range check (`neon.rs:389-394`)
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_shift_imm_pbt::test_encode_neon_shift_imm_regression_shift_oob
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1` (not a test-isolation defect)
