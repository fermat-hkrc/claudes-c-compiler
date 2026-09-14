# Bug: encode_ldur_stur encodes SIMD Rt on LDTR/STTR
**Law:** ARM ARM LDTR/STTR (unprivileged) take Wt/Xt only. llvm-mc rejects `ldtr d0, [x1]` (`invalid operand for instruction`). SIMD unscaled forms are LDUR/STUR (op2=00), not LDTR/STTR (op2=10).
**Impact:** `ldtr d0, [x0]` is encoded as an unprivileged SIMD load (V=1, op2=10), which is not a valid AArch64 instruction. Silent mis-assembly.
**Function:** encode_ldur_stur
**Detected by:** Negative/Error Contract
**Minimal input:** kind=6, n=0 (d0), is_load=true, op2=0b10 — `ldtr d0, [x0]`
**Expected:** Err
**Actual:** Ok(Word) with V=1 and bits [11:10]=10
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldur_stur_pbt::test_encode_ldur_stur_regression_simd_ldtr
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
