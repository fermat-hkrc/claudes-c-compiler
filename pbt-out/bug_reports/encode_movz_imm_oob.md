# Bug: encode_movz silently truncates immediates outside [0, 65535]
**Law:** MOVZ imm16 must be an integer in range [0, 65535]; values outside that range must be rejected.
**Impact:** Out-of-range immediates (negative, 65536+) are masked to 16 bits and encoded as a different value, so the assembler accepts invalid GNU as / ARM ARM input and emits the wrong zero-immediate.
**Function:** encode_movz
**Detected by:** Negative/Error Contract — imm16 range
**Minimal input:** `movz w0, #-1` (operands `[Reg("w0"), Imm(-1)]`)
**Expected:** Err (llvm-mc: "immediate must be an integer in range [0, 65535]")
**Actual:** Ok(Word) encoding imm16=0xFFFF (`(imm as u32) & 0xFFFF`)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_movz_pbt::test_encode_movz_regression_imm_oob
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 (same shrunk witness `rd=0, is_64=false, imm=-1`)
