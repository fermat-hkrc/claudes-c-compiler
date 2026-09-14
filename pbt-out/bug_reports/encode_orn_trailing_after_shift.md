# Bug: encode_orn ignores a trailing operand after a valid shift
**Law:** ORN grammar is Rd, Rn, Rm [, shift #imm]. A 5th operand after a valid shift must be rejected. llvm-mc rejects extra tokens after the shift.
**Impact:** `orn w0, w0, w0, lsl #1, x0` encodes as `orn w0, w0, w0, lsl #1`. Extra operands after the shift are dropped.
**Function:** encode_orn
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `orn w0, w0, w0, lsl #1, w0` (rd=rn=rm=0, is_64=false, extra=Reg)
**Expected:** Err (llvm-mc rejects trailing tokens after a valid shift)
**Actual:** Ok(Word) — only operands[0..3] are read; operand 4 is never inspected.
**Severity:** low
**Regression test:** `test_encode_orn_regression_trailing_after_shift` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** first parallel run failed with the same shrunk witness; encode_orn is a pure function (no shared state).
