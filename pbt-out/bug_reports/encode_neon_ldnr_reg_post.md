# Bug: Register post-index `[Xn], Xm` is ignored
**Law:** README.md:235 lists ld2r/ld3r/ld4r with post-index. gas/llvm-mc accept `ld2r {v0.8b, v1.8b}, [x1], x2` = 0x0de2c020. The parser leaves this as Mem + Reg (it only merges Mem+Imm into MemPostIndex).
**Impact:** Register post-index is encoded as the no-offset form, so the writeback register is dropped and the instruction does not increment the base.
**Function:** encode_neon_ldnr
**Detected by:** Differential vs llvm-mc
**Minimal input:** encode_neon_ldnr([RegList({v0.8b, v1.8b}), Mem{x1,0}, Reg("x2")], 2) → 0x0d40d020 (no-offset, wrong S). llvm-mc 0x0de2c020.
**Expected:** 0x0de2c020 (L=1, Rm=2)
**Actual:** 0x0d40d020 — third operand ignored; always Rm=11111 only for MemPostIndex.
**Severity:** high
**Regression test:** src/backend/arm/assembler/encoder/neon.rs test_encode_neon_ldnr_regression_reg_post
