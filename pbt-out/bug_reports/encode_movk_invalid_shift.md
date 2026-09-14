# Bug: encode_movk accepts non-lsl shifts and illegal shift amounts
**Law:** MOVK allows only `lsl` with amount in {0,16} (32-bit) or {0,16,32,48} (64-bit); any other shift kind or amount must be rejected.
**Impact:** `lsr`/`asr`/`ror` and non-multiple-of-16 `lsl` amounts are treated as `hw=0` (or `amount/16` with no range check), so invalid assembly is encoded as a different MOVK. Out-of-range `hw` can also spill into opcode bits (`hw << 21`).
**Function:** encode_movk
**Detected by:** Negative/Error Contract — shift kind and amount
**Minimal input:** `movk w0, #0, lsr #0` (operands `[Reg("w0"), Imm(0), Shift { kind: "lsr", amount: 0 }]`)
**Expected:** Err (llvm-mc: "expected 'lsl' with optional integer 0 or 16")
**Actual:** Ok(Word) with hw=0 (non-lsl defaults to 0)
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/data_processing.rs::encode_movk_pbt::test_encode_movk_regression_invalid_shift
**Serial reconfirmation:** reproduced with PBT_TEST_JOBS=1 (same shrunk witness `rd=0, imm=0, is_64=false, kind="lsr", amount=0`)
