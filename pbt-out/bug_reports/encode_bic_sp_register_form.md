# Bug: encode_bic register form accepts SP/WSP as a GPR
**Law:** In BIC (shifted register), register 31 is XZR/WZR, never SP/WSP. `bic sp, ...` / `bic wsp, ...` must be rejected.
**Impact:** `bic wsp, w0, w0` is encoded as `bic wzr, w0, w0`. A programmer or codegen that meant the stack pointer gets a write to the zero register instead, with no assembler diagnostic.
**Function:** encode_bic
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `bic wsp, w0, w0` (which=0, is_64=false)
**Expected:** Err (llvm-mc: "expected compatible register or logical immediate")
**Actual:** Ok(Word) — `parse_reg_num("wsp")` returns 31 and `is_64bit_reg("wsp")` is false, so the instruction is encoded as 32-bit BIC with Rd=31 (WZR).
**Severity:** medium
**Regression test:** `test_encode_bic_regression_sp` in src/backend/arm/assembler/encoder/data_processing.rs
**Serial reconfirm:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_bic_neg_sp_fp_regform -- --test-threads=1`
