# Bug: encode_cmp accepts mixed x/w without an extend
**Law:** CMP shifted-register form requires same-width GPR pair (Xn,Xm or Wn,Wm). Mixed `cmp xN, wM` without an extend must be Err.
**Impact:** `cmp x0, w0` is accepted and encoded as a 64-bit CMP (sf taken from prepended XZR). llvm-mc rejects mixed width without sxtw/uxtw. Silent wrong-width compare.
**Function:** encode_cmp
**Detected by:** Negative/Error Contract
**Minimal input:** [Reg("x0"), Reg("w0")]  (`cmp x0, w0`)
**Expected:** Err
**Actual:** Ok(Word) — encode_add_sub takes sf only from Rd (the prepended ZR) and never checks Rm width.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/compare_branch.rs::encode_cmp_pbt::test_encode_cmp_regression_mixed_width
**Serial reconfirmation:** PBT_TEST_JOBS=1 cargo test --lib encode_cmp_pbt -- --test-threads=1 reproduced test_encode_cmp_regression_mixed_width.
