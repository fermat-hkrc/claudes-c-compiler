# Bug: encode_imul memory forms omit segment override prefix
**Law:** Memory operands with a segment override must emit the corresponding prefix byte (es=0x26, cs=0x2E, ss=0x36, ds=0x3E, fs=0x64, gs=0x65) before the opcode, as `emit_segment_prefix` documents and llvm-mc does.
**Impact:** Segmented addressing (`%es:(%eax)`, `%fs:…`) assembles without the prefix, so the instruction reads/writes the wrong segment — silent wrong code for any caller using segment overrides on IMUL.
**Function:** encode_imul
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:723
**Detected by:** Differential + metamorphic (encode_imul_diff_mem_reg, encode_imul_diff_imm_mem_reg, encode_imul_metamorphic_seg_prefix)
**Minimal input:** `imull %es:(%eax), %ebx` → SUT `[0f,af,18]` vs llvm-mc `[26,0f,af,18]`; `imull $5, %es:(%eax), %ebx` → SUT `[6b,18,05]` vs `[26,6b,18,05]`
**Expected:** segment byte ‖ bare encoding
**Actual:** bare encoding only (no 0x26)
**Severity:** high
**Root cause:** gp_integer.rs:723-726 and 761-771 call `encode_modrm_mem` without first calling `self.emit_segment_prefix(mem)` (core.rs:31-42). Same defect class as encode_test/lea/invlpg memory arms.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:723`
```rust
(Operand::Memory(mem), Operand::Register(dst)) => {
    let dst_num = reg_num(&dst.name).ok_or("bad register")?;
    self.bytes.extend_from_slice(&[0x0F, 0xAF]);
    self.encode_modrm_mem(dst_num, mem)
}
```
**Suggested fix:** Call `emit_segment_prefix` before the opcode on both memory arms:
```rust
(Operand::Memory(mem), Operand::Register(dst)) => {
    let dst_num = reg_num(&dst.name).ok_or("bad register")?;
    self.emit_segment_prefix(mem);
    if size == 2 { self.bytes.push(0x66); }
    self.bytes.extend_from_slice(&[0x0F, 0xAF]);
    self.encode_modrm_mem(dst_num, mem)
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_imul_regression_segment_mem_reg -- --test-threads=1
cargo test --lib encode_imul_regression_seg_imm_mem_reg -- --test-threads=1
```
**Raw output:**
```text
assertion `left == right` failed: Mem→Reg must emit segment prefix; sut=[0f, af, 18] mc=[26, 0f, af, 18]
assertion `left == right` failed: 3-op Imm,Mem,Reg must emit segment prefix; sut=[6b, 18, 05] mc=[26, 6b, 18, 05]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_imul_pbt.rs (encode_imul_regression_segment_mem_reg, encode_imul_regression_seg_imm_mem_reg)
