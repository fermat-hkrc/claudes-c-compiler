# Bug: encode_fcvt_rounding encodes H source as ftype=00 (single)
**Law:** ARM ARM Conversion between floating-point and integer uses ftype=11 for half-precision (Hn) source (FEAT_FP16). llvm-mc `-mattr=+fullfp16` encodes `fcvtzs w0, h1` as 0x1ef80020. The SUT must not emit the single-precision encoding for an H register.
**Impact:** `fcvtzs w0, h1` is assembled as `fcvtzs w0, s1` (ftype=00). Half-precision float-to-int conversion silently operates on the wrong register view. gas without fp16 rejects the instruction; it never means "treat H as S".
**Function:** encode_fcvt_rounding
**Detected by:** Differential — llvm-mc -triple=aarch64 -mattr=+fullfp16 -show-encoding
**Minimal input:** `[Reg("w0"), Reg("h0")]` with rmode=0b11 opcode=0b000 (shrunk `fcvtzs w0, h0`; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Word(0x1ef80000) — ftype=11
**Actual:** Word(0x1e380000) — ftype=00 because `src_name.starts_with('d')` is the only ftype check
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fcvt_rounding_regression_half_ftype
