# Bug: `[Xn, #imm]` (non-zero unsigned offset) is accepted as no-offset ldNr
**Law:** gas/llvm-mc reject `ld2r {v0.8b, v1.8b}, [x1, #4]` and even `[x1, #0]`. Only `[Xn]` or post-index `[Xn], #imm` / `[Xn], Xm` are valid.
**Impact:** A memory operand with an offset is encoded as the no-offset form, dropping the offset.
**Function:** encode_neon_ldnr
**Detected by:** Negative/error contract
**Minimal input:** encode_neon_ldnr([RegList({v0.8b, v1.8b}), Mem{base:"x1", offset:4}], 2) returns Ok.
**Expected:** Err
**Actual:** Ok — match is `Operand::Mem { base, .. }` (neon.rs:1558), offset ignored.
**Severity:** medium
**Regression test:** src/backend/arm/assembler/encoder/neon.rs test_encode_neon_ldnr_regression_mem_offset
