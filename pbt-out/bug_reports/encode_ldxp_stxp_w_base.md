# Bug: encode_ldxp_stxp accepts a W register as the memory base
**Law:** ARM ARM Rn is Xn|SP; a 32-bit W base (or WSP) must be rejected.
**Impact:** `ldxp w0, w1, [w0]` encodes as `ldxp w0, w1, [x0]` because parse_reg_num drops the width prefix. llvm-mc rejects `[w0]` as an invalid operand.
**Function:** encode_ldxp_stxp
**Detected by:** Negative/Error Contract
**Minimal input:** `ldxp w0, w1, [w0]` (kind=2 in encode_ldxp_stxp_neg_invalid_rt_base)
**Expected:** Err
**Actual:** Ok(Word) encoding of `ldxp w0, w1, [x0]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldxp_stxp_pbt::test_encode_ldxp_stxp_regression_w_base
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
