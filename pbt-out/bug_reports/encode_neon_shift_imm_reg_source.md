# Bug: encode_neon_shift_imm accepts a bare GPR/FP/V register as Vn
**Law:** Vector USHR requires `Vn.<T>` (a RegArrangement). A bare `Operand::Reg` source is not a NEON arranged operand and must be rejected, matching llvm-mc / GNU as.
**Impact:** `ushr v0.8b, x0, #1` (and w/d/s/q/h/b/v names) encodes using `parse_reg_num` of the bare name as Rn, producing a 32-bit word instead of an assembler error.
**Function:** encode_neon_shift_imm
**Detected by:** Negative/Error Contract (4e) — encode_neon_shift_imm_neg_reg_source
**Minimal input:** `encode_neon_shift_imm([v0.8b, Reg("x0"), #1], true)`
**Expected:** `Err` (source is not Vn.T)
**Actual:** `Ok(EncodeResult::Word(_))` — `get_neon_reg` accepts `Operand::Reg` (`neon.rs:14-18`) and source arrangement is discarded (`neon.rs:379`)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs::encode_neon_shift_imm_pbt::test_encode_neon_shift_imm_regression_reg_source
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1` (proptest replay; not a test-isolation defect)
