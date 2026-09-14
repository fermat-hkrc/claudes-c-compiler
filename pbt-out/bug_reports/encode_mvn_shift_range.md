# Bug: encode_mvn accepts out-of-range shift amounts
**Law:** ARM ARM Logical (shifted register) imm6 is 0..31 when sf=0 (imm6<5>==1 is UNALLOCATED) and 0..63 when sf=1. Amounts outside that range must be rejected.
**Impact:** `mvn w0, w0, lsl #32` is accepted; amount is masked with 0x3F so #32 encodes as imm6=32 (UNALLOCATED encoding). llvm-mc/gas reject it.
**Function:** encode_mvn
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("w0"), Reg("w0"), Shift{kind:"lsl", amount:32}]` (is_64=false)
**Expected:** Err
**Actual:** Ok(Word) with imm6 = amount & 0x3F
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_mvn_regression_shift_range
**Serial reconfirm:** PBT_TEST_JOBS=1, `--test-threads=1` — reproduced
