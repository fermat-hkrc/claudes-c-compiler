# Bug: encode_ldur_stur encodes LR as a 32-bit W30 load/store
**Law:** `lr` is the architectural alias of X30. llvm-mc accepts `ldur lr, [x0]` as `ldur x30, [x0]`. Sibling `is_64bit_reg` (encoder/mod.rs:151-154) and `encode_ldr_str_auto` (load_store.rs:17) both treat `lr` as 64-bit. encode_ldur_stur must encode `lr` as Xt register 30.
**Impact:** `ldur lr, [x0]` encodes as `ldur w30, [x0]` because size is taken from `reg_name.starts_with('x')` and `"lr"` does not start with `x`. Silent 32-bit vs 64-bit mismatch on a valid, commonly used alias.
**Function:** encode_ldur_stur
**Detected by:** Differential (llvm-mc) / algebraic (must equal x30 encoding)
**Minimal input:** rn=0, offset=-256, is_load=false, unpriv=false — `stur lr, [x0, #-256]`
**Expected:** Ok(Word) equal to `stur x30, [x0, #-256]` (0xf810001e)
**Actual:** Ok(Word) equal to `stur w30, [x0, #-256]` (0xb810001e)
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldur_stur_pbt::test_encode_ldur_stur_regression_lr_as_x30
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
