# Bug: encode_ldrs accepts illegal register-offset shift/extend
**Law:** LDRSB allows only amount #0; LDRSH allows #0 or #1. uxtx is not a valid extend spelling (use lsl/sxtx). llvm-mc rejects `ldrsb Xt, [Xn, Xm, lsl #1]`.
**Impact:** `ldrsb w0, [x0, x0, lsl #1]` encodes with S=1 (Word 0x38e07800) instead of Err. For byte loads S=1 is architecturally shift 0, so the assembler accepts assembly gas would reject.
**Function:** encode_ldrs
**Detected by:** Negative/Error Contract (4e)
**Minimal input:** `[Reg("w0"), MemRegOffset { base: "x0", index: "x0", extend: Some("lsl"), shift: Some(1) }]`, size=0
**Expected:** Err
**Actual:** Ok(Word(0x38e07800)) — S bit is `shift_amount > 0` with no scale check; uxtx maps to option 011
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/load_store.rs::test_encode_ldrs_regression_bad_shift
**Serial reconfirmation:** reproduced with `PBT_TEST_JOBS=1 cargo test --lib encode_ldrs_neg_bad_extend -- --test-threads=1`
