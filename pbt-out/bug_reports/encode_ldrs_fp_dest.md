# Bug: encode_ldrs accepts SIMD/FP destination registers
**Law:** LDRSB/LDRSH destination is a GPR (Wt/Xt), never D/S/Q/H/B/V.
**Impact:** `ldrsb d0, [x1]` encodes a GPR load into register 0 instead of being rejected.
**Function:** encode_ldrs
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** size=0, rt=0, simd='d' → `[Reg("d0"), Mem { base: "x1", offset: 0 }]`
**Expected:** Err
**Actual:** Ok(Word(0x39c00020)) — parse_reg_num accepts d/s/q/v/h/b prefixes
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::test_encode_ldrs_regression_fp_dest
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrs_neg_fp_dest -- --test-threads=1`
