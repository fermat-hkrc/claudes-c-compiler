# Bug: encode_neon_qshrn truncates i64 shift via `as u32`
**Law:** The SQSHRN shift immediate is the assembler immediate in [1, dest element size]. An i64 Imm outside that range must be rejected even if its low 32 bits look like a valid shift.
**Impact:** `Imm(4294967297)` (`1 + 2^32`) encodes as shift `#1`. An out-of-range immediate is silently rewritten to a different legal shift, producing the wrong instruction.
**Function:** encode_neon_qshrn
**Detected by:** Algebraic — Negative/Error Contract (4e) (coverage sweep of `get_imm as u32`)
**Minimal input:** rd=0, rn=0, ta=8h, tb=8b, Imm(4294967297), u_bit=0, is_rounding=false, is_high=false
**Expected:** Err (immediate not in [1, 8])
**Actual:** Ok(Word) — `let shift = get_imm(operands, 2)? as u32` yields 1, which passes the source-size range check
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::test_encode_neon_qshrn_regression_shift_i64_trunc
**Serial reconfirm:** PBT_TEST_JOBS=1 `cargo test --lib encode_neon_qshrn_neg_shift_i64_trunc -- --test-threads=1` reproduced the failure.

Doc evidence for the law:
- llvm-mc: shift must be an integer in range [1, dest_esize]
- ARM ARM shift is UInt of the encoded immediate, not a truncated i64
- Parser Operand::Imm is i64 (parser.rs:24), so values above 2^32 are representable
