# Bug: encode_orn encodes SP/WSP as XZR/WZR
**Law:** ARM ARM Logical (shifted register) register 31 is XZR/WZR, never SP/WSP. llvm-mc rejects `orn wsp, w0, w0` and `orn sp, x1, x2`.
**Impact:** `orn wsp, w0, w0` assembles as `orn wzr, w0, w0`. Stack-pointer operands are silently rewritten to the zero register.
**Function:** encode_orn
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `orn wsp, w0, w0` (which=0, is_64=false, kind=0)
**Expected:** Err (llvm-mc: "expected compatible register or logical immediate")
**Actual:** Ok(Word) — parse_reg_num maps sp/wsp to 31, the same encoding as xzr/wzr.
**Severity:** medium
**Regression test:** `test_encode_orn_regression_sp` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_orn_neg -- --test-threads=1`
