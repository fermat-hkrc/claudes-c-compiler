# Bug: SQSHRUN ignores destination arrangement Tb
**Law:** ARM ARM mandates Tb/Ta pairs (Q=0: 8B←8H, 4H←4S, 2S←2D; Q=1: 16B←8H, 8H←4S, 4S←2D). A mismatched dest arrangement must be rejected, matching gas/llvm-mc.
**Impact:** `sqshrun v0.4h, v0.8h, #1` is encoded as if dest were `.8b` (Q taken only from the mnemonic's `is_high`). Wrong arrangement in the assembly text is silently dropped.
**Function:** encode_neon_sqshrun
**Detected by:** Negative/Error Contract (5) — llvm-mc rejects mismatched Tb; README.md:1-14 gas-compatible GNU-style text
**Minimal input:** `sqshrun v0.4h, v0.8h, #1` — operands `[RegArrangement(v0, 4h), RegArrangement(v0, 8h), Imm(1)]`, is_rounding=false, is_high=false
**Expected:** Err (llvm-mc: invalid operand; dest Tb=4h is not 8b)
**Actual:** Ok(Word). Body binds dest as `let (rd, _arr_d) = get_neon_reg(operands, 0)?` and never consults `_arr_d`.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::test_encode_neon_sqshrun_regression_mismatched_dest_tb
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_neon_sqshrun -- --test-threads=1)
