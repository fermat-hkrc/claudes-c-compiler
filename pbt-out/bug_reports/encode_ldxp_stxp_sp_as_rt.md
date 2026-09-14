# Bug: encode_ldxp_stxp encodes SP/WSP as ZR in Rt/Rt2/Ws
**Law:** Exclusive-pair Rt/Rt2 are Wt/Xt with register 31 = ZR, never SP/WSP. STXP Ws is Wt with 31 = WZR, never SP/WSP. llvm-mc rejects `stxp w0, sp, w0, [x0]`.
**Impact:** `parse_reg_num` maps `sp`/`wsp` to 31, so `stxp w0, sp, w0, [x0]` encodes as `stxp w0, wzr, w0, [x0]` (and SP as Rt is treated as 64-bit, forcing sz=1). Silent mis-assembly.
**Function:** encode_ldxp_stxp
**Detected by:** Negative/Error Contract
**Minimal input:** n=0, rn=0, is_load=false, acqrel=false, is_64=false, kind=0 — `stxp w0, sp, w0, [x0]`
**Expected:** Err
**Actual:** Ok(Word(0xc81f003f)) encoding SP as XZR/WZR
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_ldxp_stxp_pbt::test_encode_ldxp_stxp_regression_sp_as_rt
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / RUST_TEST_THREADS=1
