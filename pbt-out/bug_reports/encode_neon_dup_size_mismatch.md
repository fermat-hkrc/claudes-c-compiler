# Bug: encode_neon_dup ignores dest arrangement vs element-size mismatch
**Law:** ARM ARM DUP (element) requires dest T to match the source element size: 8B/16B↔B, 4H/8H↔H, 2S/4S↔S, 2D↔D. llvm-mc rejects `dup v0.8b, v0.h[0]` as an invalid operand.
**Impact:** Dest T is used only for the Q bit; elem_size independently encodes imm5. `dup v0.8b, v0.h[0]` is therefore encoded as the halfword form with Q=0 — i.e. `dup v0.4h, v0.h[0]` — a well-formed encoding of a different instruction.
**Function:** encode_neon_dup
**Detected by:** Negative/Error Contract (5)
**Minimal input:** operands = [RegArrangement(v0, 8b), RegLane(v0, h, 0)]  — `dup v0.8b, v0.h[0]`
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) encoding DUP with Q=0 and imm5 halfword size (same bits as `dup v0.4h, v0.h[0]`). Serial reconfirmation: `PBT_TEST_JOBS=1 cargo test --lib encode_neon_dup_neg_size -- --test-threads=1` still fails.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::test_encode_neon_dup_regression_size_mismatch
