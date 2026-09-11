# Bug: encode_adr treats SP as XZR
**Law:** ARM ADR Rd is Xd; register 31 is XZR, not SP. llvm-mc rejects `adr sp, #0`. encode_adr must Err when Rd is sp.
**Impact:** `adr sp, #-1048576` is assembled as `adr xzr, #-1048576` (encoding 0x1080001f). The object file writes XZR instead of addressing the stack pointer.
**Function:** encode_adr
**Detected by:** Algebraic — Negative/Error Contract (4e)
**Minimal input:** operands = [Reg("sp"), Imm(-1048576)]
**Expected:** Err (invalid operand; SP is not a valid ADR register)
**Actual:** Ok(Word(0x1080001f)) — same encoding as `adr xzr, #-1048576`
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_adr_pbt::test_encode_adr_regression_sp
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 (cargo test --lib encode_adr_neg_sp -- --test-threads=1)
