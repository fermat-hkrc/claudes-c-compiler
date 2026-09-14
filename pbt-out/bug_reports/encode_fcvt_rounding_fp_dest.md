# Bug: encode_fcvt_rounding encodes FP dest as integer W-form
**Law:** Integer FCVT* dest must be a GPR (Wd|Xd). An FP dest (`fcvtzs s0, s0`) is a different instruction (Advanced SIMD scalar FCVTZS, encoding 0x5e...). GP / Q / V / B sources are also invalid for this form (llvm-mc rejects them).
**Impact:** Public dispatch (`encoder/mod.rs:440-453`) sends scalar `fcvtzs s0, s0` to this helper, which emits integer `fcvtzs w0, s0` (0x1e380000) instead of SIMD-scalar 0x5ea0b800. `fcvtzs w0, x1` similarly encodes as `fcvtzs w0, s1`. Valid SIMD-scalar assembly and invalid GP-source assembly both become the wrong integer conversion.
**Function:** encode_fcvt_rounding
**Detected by:** Negative/Error Contract
**Minimal input:** `[Reg("s0"), Reg("s0")]` with rmode=0b11 opcode=0b000 (shrunk; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Err (this helper's job is integer W/X dest; SIMD-scalar is a different encoder)
**Actual:** Ok(Word(0x1e380000)) — is_64bit_reg("s0") is false so sf=0, source 's' so ftype=00
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fcvt_rounding_regression_fp_dest
