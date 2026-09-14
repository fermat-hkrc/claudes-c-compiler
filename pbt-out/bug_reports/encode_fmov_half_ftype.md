# Bug: encode_fmov encodes H registers as ftype=00 (single)
**Law:** ARM ARM FMOV (register) uses ftype=11 for half-precision (Hn) (FEAT_FP16). llvm-mc `-mattr=+fullfp16` encodes `fmov h0, h0` as 0x1ee04000. gas `-march=armv8.2-a+fp16` agrees (`fmov h0, h1` = 0x1ee04020). If the SUT accepts H registers (parse_reg_num maps h0-h31; is_fp_reg includes 'h'), the word must use ftype=11, not silently encode as single.
**Impact:** `fmov h0, h0` is assembled as `fmov s0, s0` (ftype=00). Half-precision moves silently operate on the wrong register view. `fmov h0, w0` is assembled as `fmov s0, w0` (0x1e270000) instead of 0x1ee70000.
**Function:** encode_fmov
**Detected by:** Differential — llvm-mc -triple=aarch64 -mattr=+fullfp16 -show-encoding
**Minimal input:** `[Reg("h0"), Reg("h0")]` (shrunk `fmov h0, h0`; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Word(0x1ee04000) — ftype=11
**Actual:** Word(0x1e204000) — ftype=00 because `rd_lower.starts_with('d') || rm_lower.starts_with('d')` is the only ftype check
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fmov_regression_half_ftype
