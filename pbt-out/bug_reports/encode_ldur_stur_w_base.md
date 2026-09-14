# Bug: encode_ldur_stur accepts a W register (or XZR/WZR/WSP) as the memory base
**Law:** ARM ARM Rn is Xn|SP; a 32-bit W base, WSP, XZR, or WZR must be rejected. Register 31 as base is SP, never XZR.
**Impact:** `ldur x0, [w0]` encodes as `ldur x0, [x0]` because parse_reg_num drops the width prefix. llvm-mc rejects `[w0]` as an invalid operand. Same path treats `[xzr]` as `[sp]`.
**Function:** encode_ldur_stur
**Detected by:** Negative/Error Contract
**Minimal input:** kind=2, n=0, offset in [-256,255] — `ldur x0, [w0]`
**Expected:** Err
**Actual:** Ok(Word) encoding of `ldur x0, [x0]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldur_stur_pbt::test_encode_ldur_stur_regression_w_base
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
