# Bug: encode_bics ignores a trailing non-shift 4th operand
**Law:** BICS takes three registers and an optional shift. A 4th operand that is not a valid shift must be rejected.
**Impact:** `bics x0, x1, x2, x3` encodes as `bics x0, x1, x2`. Extra operands in compiler output or hand-written asm are dropped, so a parse/arity bug is silently assembled.
**Function:** encode_bics
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `bics w0, w0, w0, w0` (is_64=false, rd=rn=rm=0, which=0 extra=Reg)
**Expected:** Err (llvm-mc: "expected 'lsl', 'lsr' or 'asr' with optional integer in range [0, 63]")
**Actual:** Ok(Word) — operand 3 is inspected only with `if let Some(Operand::Shift { ... })`; any other 4th operand is ignored.
**Severity:** low
**Regression test:** `test_encode_bics_regression_extra_operand` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_bics_neg_extra_operand -- --test-threads=1`
