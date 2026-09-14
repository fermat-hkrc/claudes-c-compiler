# Bug: encode_ldar_stlr accepts a W register as the memory base
**Law:** ARM ARM Rn is Xn|SP; a 32-bit W base (or WSP/WZR/XZR) must be rejected. Offset must be absent or #0.
**Impact:** `stlr w0, [w0]` encodes as `stlr w0, [x0]` because parse_reg_num drops the width prefix. llvm-mc rejects `[w0]` as an invalid operand. Same path also treats `[xzr]` as `[sp]` and ignores a nonzero offset (`[x0, #8]` encodes as `[x0]`).
**Function:** encode_ldar_stlr
**Detected by:** Negative/Error Contract
**Minimal input:** rt=0, is_load=false, variant=0, is_64=false, kind=0, wn=0 — `stlr w0, [w0]`
**Expected:** Err
**Actual:** Ok(Word) encoding of `stlr w0, [x0]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldar_stlr_pbt::test_encode_ldar_stlr_regression_w_base
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
