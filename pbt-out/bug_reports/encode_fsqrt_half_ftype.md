# Bug: encode_fsqrt encodes H registers as ftype=00 (single)
**Law:** ARM ARM Floating-point data-processing (1 source) uses ftype=11 for half-precision (Hn) (FEAT_FP16). llvm-mc `-mattr=+fullfp16` encodes `fsqrt h0, h0` as 0x1ee1c000. If the SUT accepts H registers (parse_reg_num maps h0-h31), the word must use ftype=11, not silently encode as single.
**Impact:** `fsqrt h0, h0` is assembled as `fsqrt s0, s0` (ftype=00). Half-precision square root silently operates on the wrong register view. gas without fp16 rejects the instruction; it never means "treat H as S".
**Function:** encode_fsqrt
**Detected by:** Differential — llvm-mc -triple=aarch64 -mattr=+fullfp16 -show-encoding
**Minimal input:** `[Reg("h0"), Reg("h0")]` (shrunk `fsqrt h0, h0`; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Word(0x1ee1c000) — ftype=11
**Actual:** Word(0x1e21c000) — ftype=00 because `rd_name.starts_with('d')` is the only ftype check
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fsqrt_regression_half_ftype
