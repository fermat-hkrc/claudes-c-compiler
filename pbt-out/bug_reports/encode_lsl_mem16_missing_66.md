# Bug: encode_lsl memory form never emits 0x66 for 16-bit destination
**Law:** `LSL r16, m16` requires the 0x66 operand-size override before 0F 03, same as the register form when the destination is a 16-bit GP register.
**Impact:** Forms like `lsl (%eax), %bx` assemble without 0x66 and are decoded as 32-bit LSL into BX's enclosing register, corrupting the upper half of EBX and disagreeing with every standard assembler.
**Function:** encode_lsl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:160
**Detected by:** Differential — llvm-mc i686 (encode_lsl_diff_mem_base_disp / encode_lsl_diff_sib / encode_lsl_diff_edges)
**Minimal input:** `lsl (%eax), %bx` → ops = [Memory(eax), Register(bx)]
**Expected:** `[0x66, 0x0f, 0x03, 0x18]`
**Actual:** `[0x0f, 0x03, 0x18]`
**Severity:** high
**Root cause:** system.rs:160-164 memory arm emits 0F 03 + ModR/M only; unlike the register arm it never inspects destination width for a 0x66 prefix.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:160`
```rust
(Operand::Memory(mem), Operand::Register(dst)) => {
    let dst_num = reg_num(&dst.name).ok_or("bad register")?;
    self.bytes.extend_from_slice(&[0x0F, 0x03]);
    self.encode_modrm_mem(dst_num, mem)
}
```
**Suggested fix:**
```rust
(Operand::Memory(mem), Operand::Register(dst)) => {
    let dst_num = reg_num(&dst.name).ok_or("bad register")?;
    let is_16 = matches!(dst.name.as_str(), "ax"|"bx"|"cx"|"dx"|"si"|"di"|"sp"|"bp");
    self.emit_segment_prefix(mem);
    if is_16 {
        self.bytes.push(0x66);
    }
    self.bytes.extend_from_slice(&[0x0F, 0x03]);
    self.encode_modrm_mem(dst_num, mem)
}
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_lsl_regression_mem16_missing_66 -- --test-threads=1
```
**Raw output:**
```text
assertion `left == right` failed: lsl (%eax), %bx must be [66, 0f, 03, 18], got [0f, 03, 18]
  left: [15, 3, 24]
 right: [102, 15, 3, 24]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_lsl_pbt.rs
