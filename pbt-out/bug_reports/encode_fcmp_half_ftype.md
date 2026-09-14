# Bug: encode_fcmp encodes H registers as ftype=00 (single)
**Law:** ARM ARM Floating-point compare uses ftype=11 for half-precision (Hn) (FEAT_FP16). llvm-mc `-mattr=+fullfp16` encodes `fcmp h0, h0` as 0x1ee02000. If the SUT accepts H registers (`parse_reg_num` maps h0-h31), the word must use ftype=11, not silently encode as single.
**Impact:** `fcmp h0, h0` is assembled as `fcmp s0, s0` (ftype=00). Half-precision compare silently operates on the wrong register view. The #0.0 form is likewise `0x1e202008` instead of `0x1ee02008`.
**Function:** encode_fcmp
**Detected by:** Differential — llvm-mc -triple=aarch64 -mattr=+fullfp16 -show-encoding
**Minimal input:** `[Reg("h0"), Reg("h0")]` (shrunk `rn=0, rm=0, zero=false`; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Word(0x1ee02000) — ftype=11
**Actual:** Word(0x1e202000) — ftype=00 because `rn_name.starts_with('d')` is the only ftype check
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fcmp_regression_half_ftype
