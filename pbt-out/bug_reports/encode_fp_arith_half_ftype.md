# Bug: encode_fp_arith encodes H registers as ftype=00 (single)
**Law:** ARM ARM Floating-point data-processing (2 source) uses ftype=11 for half-precision (Hn) (FEAT_FP16). llvm-mc `-mattr=+fullfp16` encodes `fmul h0, h0, h0` as 0x1ee00800 and `fadd h0, h1, h2` as 0x1ee22820. If the SUT accepts H registers (parse_reg_num maps h0-h31), the word must use ftype=11, not silently encode as single.
**Impact:** `fmul h0, h0, h0` is assembled as `fmul s0, s0, s0` (ftype=00). Half-precision arithmetic silently operates on the wrong register view. gas without fp16 rejects the instruction; it never means "treat H as S".
**Function:** encode_fp_arith
**Detected by:** Differential — llvm-mc -triple=aarch64 -mattr=+fullfp16 -show-encoding
**Minimal input:** `[Reg("h0"), Reg("h0"), Reg("h0")]` with opcode=0b0000 (shrunk `fmul h0, h0, h0`; serial reconfirm with PBT_TEST_JOBS=1). Also `fadd h0, h1, h2`: SUT 0x1e222820 vs llvm-mc 0x1ee22820.
**Expected:** Word(0x1ee00800) — ftype=11
**Actual:** Word(0x1e200800) — ftype=00 because `rd_name.starts_with('d')` is the only ftype check
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fp_arith_regression_half_ftype
