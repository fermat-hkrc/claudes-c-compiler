# Bug: encode_bics accepts SP/WSP as register 31
**Law:** BICS shifted-register form uses XZR/WZR for register 31, never SP/WSP. `bics sp, ...` / `bics wsp, ...` must be rejected.
**Impact:** `bics wsp, w0, w0` encodes as `bics wzr, w0, w0`. A stack-pointer destination is silently rewritten to the zero register, corrupting flag-setting bitwise-clear intended for SP.
**Function:** encode_bics
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `bics wsp, w0, w0` (which=0, is_64=false, kind=0)
**Expected:** Err (llvm-mc: "invalid operand for instruction")
**Actual:** Ok(Word) — `parse_reg_num` maps `sp`/`wsp` to 31, the same encoding as XZR/WZR.
**Severity:** medium
**Regression test:** `test_encode_bics_regression_sp` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_bics_neg_sp_fp -- --test-threads=1`
