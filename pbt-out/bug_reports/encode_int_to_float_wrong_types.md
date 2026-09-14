# Bug: encode_int_to_float encodes GP dest / FP source as integer SCVTF
**Law:** Integer SCVTF/UCVTF dest must be an FP register (Sd|Dd) and source a GPR (Wn|Xn). An FP source (`scvtf s0, s1`) is a different instruction (Advanced SIMD scalar SCVTF, encoding 0x5e21d820). GP dest (`scvtf w0, w0`) and Q/V/B operands are invalid (llvm-mc rejects them).
**Impact:** Public dispatch (`encoder/mod.rs:454-459`) sends scalar `scvtf s0, s1` to this helper, which emits integer `scvtf s0, w1` (0x1e220020) instead of SIMD-scalar 0x5e21d820. `scvtf w0, w0` similarly encodes as `scvtf s0, w0`. Valid SIMD-scalar assembly and invalid GP-dest assembly both become the wrong integer conversion.
**Function:** encode_int_to_float
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("w0"), Reg("w0")]` with is_signed=true (shrunk; serial reconfirm with PBT_TEST_JOBS=1). Also `[Reg("s0"), Reg("s1")]`.
**Expected:** Err (this helper's job is integer W/X source; SIMD-scalar is a different encoder)
**Actual:** Ok(Word(0x1e220000)) — dest "w0" does not start with 'd' so ftype=00; is_64bit_reg("w0") is false so sf=0
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_int_to_float_regression_fp_src
