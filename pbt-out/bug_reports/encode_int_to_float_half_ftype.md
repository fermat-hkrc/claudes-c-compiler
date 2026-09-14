# Bug: encode_int_to_float encodes H dest as ftype=00 (single)
**Law:** ARM ARM Conversion between floating-point and integer uses ftype=11 for half-precision (Hd) dest (FEAT_FP16). llvm-mc `-mattr=+fullfp16` encodes `scvtf h0, w1` as 0x1ee20020 and `ucvtf h0, w0` as 0x1ee30000. The SUT must not emit the single-precision encoding for an H register.
**Impact:** `scvtf h0, w1` is assembled as `scvtf s0, w1` (ftype=00). Half-precision int-to-float conversion silently writes the wrong register view. gas without fp16 rejects the instruction; it never means "treat H as S".
**Function:** encode_int_to_float
**Detected by:** Differential — llvm-mc -triple=aarch64 -mattr=+fullfp16 -show-encoding
**Minimal input:** `[Reg("h0"), Reg("w0")]` with is_signed=false (shrunk `ucvtf h0, w0`; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Word(0x1ee30000) — ftype=11
**Actual:** Word(0x1e230000) — ftype=00 because `dst_name.starts_with('d')` is the only ftype check
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_int_to_float_regression_half_ftype
