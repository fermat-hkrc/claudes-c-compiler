# Bug: encode_smulh accepts WZR (32-bit form of XZR)
**Law:** SMULH has no 32-bit form; register 31 must be XZR, not WZR.
**Impact:** `smulh wzr, x0, x0` is encoded as XZR number 31. gas / llvm-mc reject WZR as "invalid operand". Same root cause as W-register acceptance (`is_64` discarded); this is the register-31 bound the 0..30 W-width generator missed.
**Function:** encode_smulh
**Detected by:** Negative/Error Contract (contract-surface sweep)
**Minimal input:** which=0, a=0, b=0 — `smulh wzr, x0, x0` (serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — parse_reg_num maps wzr to 31 and encode_smulh discards is_64
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::test_encode_smulh_regression_wzr
