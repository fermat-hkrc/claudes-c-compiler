# Bug: encode_ldur_stur encodes a V-register Rt as a 64-bit SIMD (D) load/store
**Law:** LDUR/STUR SIMD Rt is Bt/Ht/St/Dt/Qt. llvm-mc rejects `ldur v0, [x1]` (`invalid operand for instruction`). A V name without arrangement is not a valid unscaled SIMD operand.
**Impact:** `ldur v0, [x0]` is classified as FP (`is_fp_reg` matches prefix `v`) then falls through the q/d/s/h/b chain to size=11, opc=01 — the encoding of `ldur d0, [x0]`. Silent wrong-register-class encoding.
**Function:** encode_ldur_stur
**Detected by:** Negative/Error Contract
**Minimal input:** kind=7, n=0 — `ldur v0, [x0]`
**Expected:** Err
**Actual:** Ok(Word) encoding of `ldur d0, [x0]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldur_stur_pbt::test_encode_ldur_stur_regression_v_reg
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
