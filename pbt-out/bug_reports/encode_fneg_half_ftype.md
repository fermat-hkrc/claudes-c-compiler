# Bug: encode_fneg encodes H registers as ftype=00 (single)
**Law:** ARM ARM Floating-point data-processing (1 source) uses ftype=11 for half-precision (Hn) (FEAT_FP16). llvm-mc `-mattr=+fullfp16` encodes `fneg h0, h0` as 0x1ee14000. If the SUT accepts H registers (parse_reg_num maps h0-h31), the word must use ftype=11, not silently encode as single.
**Impact:** `fneg h0, h0` is assembled as `fneg s0, s0` (ftype=00). Half-precision negate silently operates on the wrong register view. gas without fp16 rejects the instruction; it never means "treat H as S".
**Function:** encode_fneg
**Detected by:** Differential — llvm-mc -triple=aarch64 -mattr=+fullfp16 -show-encoding
**Minimal input:** `[Reg("h0"), Reg("h0")]` (shrunk `fneg h0, h0`; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Word(0x1ee14000) — ftype=11
**Actual:** Word(0x1e214000) — ftype=00 because `rd_name.starts_with('d')` is the only ftype check
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fneg_regression_half_ftype
