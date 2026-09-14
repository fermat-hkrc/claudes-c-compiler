# Bug: encode_fmadd_fmsub encodes H registers as ftype=00 (single)
**Law:** ARM ARM Floating-point data-processing (3 source) uses ftype=11 for half-precision (Hn) (FEAT_FP16). llvm-mc `-mattr=+fullfp16` encodes `fmadd h0, h0, h0, h0` as 0x1fc00000 and `fmadd h0, h1, h2, h3` as 0x1fc20c20. If the SUT accepts H registers (parse_reg_num maps h0-h31), the word must use ftype=11, not silently encode as single.
**Impact:** `fmadd h0, h0, h0, h0` is assembled as `fmadd s0, s0, s0, s0` (ftype=00). Half-precision fused multiply-add silently operates on the wrong register view. gas without fp16 rejects the instruction; it never means "treat H as S".
**Function:** encode_fmadd_fmsub
**Detected by:** Differential — llvm-mc -triple=aarch64 -mattr=+fullfp16 -show-encoding
**Minimal input:** `[Reg("h0"), Reg("h0"), Reg("h0"), Reg("h0")]` with is_sub=false (shrunk `fmadd h0, h0, h0, h0`; serial reconfirm with PBT_TEST_JOBS=1). Also `fmadd h0, h1, h2, h3`: SUT 0x1f020c20 vs llvm-mc 0x1fc20c20.
**Expected:** Word(0x1fc00000) — ftype=11
**Actual:** Word(0x1f000000) — ftype=00 because `rd_name.starts_with('d')` is the only ftype check
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fmadd_fmsub_regression_half_ftype
