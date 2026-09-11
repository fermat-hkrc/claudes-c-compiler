# Bug: encode_neon_three_diff_narrow ignores destination arrangement Tb
**Law:** ARM ARM ADDHN/RADDHN/SUBHN/RSUBHN require Vd.Tb paired with Vn.Ta: (8H→8B / 16B for *2), (4S→4H / 8H), (2D→2S / 4S). A mismatched Tb must be rejected, matching llvm-mc / GNU as.
**Impact:** `addhn2 v0.8b, v0.8h, v0.8h` is encoded as ADDHN2 (Q=1, size=00) as if the dest were `v0.16b`. Invalid assembly is silently accepted and the written arrangement is ignored.
**Function:** encode_neon_three_diff_narrow
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `addhn2 v0.8b, v0.8h, v0.8h` (rd=rn=rm=0, Ta=8h, Tb=8b, is_high=true, U=0, opcode=0b0100)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — dest arrangement is discarded (`let (rd, _) = get_neon_reg(operands, 0)`); Q comes only from `is_high`.
**Severity:** medium
**Regression test:** `test_encode_neon_three_diff_narrow_regression_mismatched_dest_tb` in src/backend/arm/assembler/encoder/neon.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_neon_three_diff_narrow_dest_tb_must_match -- --test-threads=1`
