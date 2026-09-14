# Bug: encode_movz accepts non-lsl shifts and out-of-set lsl amounts
**Law:** MOVZ optional shift must be `lsl` with integer 0 or 16 (W) or 0, 16, 32 or 48 (X); any other kind or amount must be rejected.
**Impact:** `lsr`/`asr`/`ror` default to hw=0; non-canonical `lsl` amounts are integer-divided by 16 with no range check, so invalid GNU as is encoded as a different (often lsl #0) MOVZ.
**Function:** encode_movz
**Detected by:** Negative/Error Contract — shift kind/amount
**Minimal input:** `movz w0, #0, lsr #0` (operands `[Reg("w0"), Imm(0), Shift { kind: "lsr", amount: 0 }]`)
**Expected:** Err (llvm-mc: "expected 'lsl' with optional integer 0 or 16")
**Actual:** Ok(Word) encoding hw=0 (non-lsl branch sets hw=0)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_movz_pbt::test_encode_movz_regression_invalid_shift
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 (same shrunk witness `rd=0, imm=0, is_64=false, kind="lsr", amount=0`)
