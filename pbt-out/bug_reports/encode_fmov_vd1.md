# Bug: encode_fmov does not encode FMOV Xd, Vn.D[1] / FMOV Vd.D[1], Xn
**Law:** ARM FMOV (general) includes the top-half 64-bit SIMD scalar form: FMOV <Xd>, <Vn>.D[1] and FMOV <Vd>.D[1], <Xn> (sf=1, ftype=10, rmode=01, opcode 110/111). llvm-mc and gas encode `fmov x0, v0.d[1]` as 0x9eae0000. README.md:11 claims gas-compatible assembly. Operand::RegLane is produced by the parser and dispatched through encode_instruction to encode_fmov.
**Impact:** `fmov x0, v0.d[1]` is rejected with "fmov needs register operands" instead of emitting the architectural encoding. Valid GNU-style assembly that gas accepts cannot be assembled.
**Function:** encode_fmov
**Detected by:** Differential — llvm-mc -triple=aarch64 -show-encoding
**Minimal input:** `[Reg("x0"), RegLane { reg: "v0", elem_size: "d", index: 1 }]` (shrunk rd=0, rn=0, to_vec=false; serial reconfirm with PBT_TEST_JOBS=1)
**Expected:** Word matching llvm-mc (`fmov x0, v0.d[1]` = 0x9eae0000)
**Actual:** Err("fmov needs register operands") — only Operand::Reg is matched
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/fp_scalar.rs::test_encode_fmov_regression_vd1
