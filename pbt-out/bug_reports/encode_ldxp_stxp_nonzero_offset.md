# Bug: encode_ldxp_stxp ignores a nonzero exclusive-pair offset
**Law:** llvm-mc: "index must be absent or #0". Exclusive pair addressing is `[Xn|SP]` or `[Xn|SP, #0]` only.
**Impact:** `Mem { base, .. }` discards `offset`, so `stxp w0, w0, w0, [x0, #-1]` encodes as `stxp w0, w0, w0, [x0]`. Silent mis-assembly of an illegal addressing mode. Non-Mem addressing (pre/post/reg-offset/Imm/Symbol) and invalid base names (`foo`, `x32`) correctly return Err.
**Function:** encode_ldxp_stxp
**Detected by:** Negative/Error Contract
**Minimal input:** rt=0, rt2=0, rn=0, ws=0, is_load=false, acqrel=false, is_64=false, offset=-1, shape=0 — `stxp w0, w0, w0, [x0, #-1]`
**Expected:** Err
**Actual:** Ok(Word(0x88200000)) encoding of `stxp w0, w0, w0, [x0]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldxp_stxp_pbt::test_encode_ldxp_stxp_regression_nonzero_offset
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
