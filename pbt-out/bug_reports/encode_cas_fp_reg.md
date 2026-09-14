# Bug: encode_cas accepts FP/SIMD registers as Rs/Rt
**Law:** CAS Rs/Rt must be integer W/X registers. llvm-mc/gas reject `cas s0, s1, [x2]` (and d/q/v/h/b).
**Impact:** parse_reg_num accepts s/d/q/v/h/b prefixes, so `cas s0, s1, [x2]` encodes as 32-bit CAS w0, w1, [x2]. Silent wrong register class.
**Function:** encode_cas
**Detected by:** Negative/Error Contract
**Minimal input:** `cas s0, s1, [x2]`
**Expected:** Err
**Actual:** Ok(Word) encoding of `cas w0, w1, [x2]`
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::encode_cas_pbt::test_encode_cas_regression_fp_reg
**Serial reconfirm:** reproduced with PBT_TEST_JOBS=1 / --test-threads=1 (encode_cas_neg_mixed_fp_xbyte covers FP kinds; shrunk witness was mixed W/X)
