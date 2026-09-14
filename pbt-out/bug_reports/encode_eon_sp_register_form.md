# Bug: encode_eon encodes SP/WSP as XZR/WZR
**Law:** ARM ARM EON uses register 31 as XZR/WZR, never SP/WSP. llvm-mc rejects `eon sp, ...` and `eon wsp, ...`.
**Impact:** `eon wsp, w0, w0` encodes as `eon wzr, w0, w0`. A stack-pointer operand is silently rewritten to the zero register.
**Function:** encode_eon
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `eon wsp, w0, w0` (which=0, is_64=false, kind=0)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — parse_reg_num maps sp/wsp to 31, the same encoding as xzr/wzr.
**Severity:** medium
**Regression test:** `test_encode_eon_regression_sp` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_eon_neg_sp_fp -- --test-threads=1`
