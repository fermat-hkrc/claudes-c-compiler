# Bug: encode_umulh accepts 32-bit W registers
**Law:** UMULH has no 32-bit form; all three operands must be 64-bit X registers (Xd, Xn, Xm).
**Impact:** `umulh w0, w0, w0` (and mixed W/X) is encoded as if the names were X registers with the same numbers, producing a 64-bit multiply-high encoding that gas / llvm-mc reject as "invalid operand".
**Function:** encode_umulh
**Detected by:** Negative/Error Contract
**Minimal input:** rd=0, rn=0, rm=0, rd64=false, rn64=false, rm64=false — `umulh w0, w0, w0` (serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — encode_umulh discards is_64 from get_reg (`let (rd, _) = get_reg(...)`)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_umulh_regression_wrong_width
