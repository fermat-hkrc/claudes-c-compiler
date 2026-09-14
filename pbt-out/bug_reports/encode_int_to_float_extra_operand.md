# Bug: encode_int_to_float ignores extra operands
**Law:** Integer SCVTF/UCVTF takes exactly two registers (Sd|Dd, Wn|Xn); a third operand must be rejected. A `#fbits` third operand is the distinct fixed-point encoding (bit21=0), which this function does not implement.
**Impact:** The assembler silently encodes `scvtf s0, w1, x0` (and extra Imm/Shift/RegArrangement, including `#8` which gas/llvm-mc treat as fixed-point `0x1e02e020`) as the two-operand integer form `0x1e220020`, so invalid or different-form GNU-style assembly produces the wrong machine-code word.
**Function:** encode_int_to_float
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("s0"), Reg("w0"), Reg("x0")]` with is_signed=true (shrunk; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — the function only checks `operands.len() < 2`, so operands beyond index 1 are ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_int_to_float_regression_extra_operand
