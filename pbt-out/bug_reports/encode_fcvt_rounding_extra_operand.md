# Bug: encode_fcvt_rounding ignores extra operands
**Law:** Integer FCVT* (FCVTZS/ZU/AS/AU/NS/NU/MS/MU/PS/PU) takes exactly two registers (Wd|Xd, Sn|Dn); a third operand must be rejected. A `#fbits` third operand is the distinct fixed-point encoding (bit21=0), which this function does not implement.
**Impact:** The assembler silently encodes `fcvtzs w0, s1, x0` (and extra Imm/Shift/RegArrangement, including `#1`/`#32` which gas/llvm-mc treat as fixed-point) as the two-operand integer form, so invalid or different-form GNU-style assembly produces the wrong machine-code word. llvm-mc rejects a non-immediate extra and encodes `#fbits` as a different instruction.
**Function:** encode_fcvt_rounding
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("w0"), Reg("s0"), Reg("x0")]` with rmode=0b11 opcode=0b000 (shrunk; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err
**Actual:** Ok(Word) — the function only checks `operands.len() < 2`, so operands beyond index 1 are ignored
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fcvt_rounding_regression_extra_operand
