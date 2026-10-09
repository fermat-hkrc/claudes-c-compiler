# Bug: encode_lsl omits segment-override prefix on memory form
**Law:** For LSL with a segmented memory source, the assembler must emit the segment override (0x26/0x2E/0x36/0x3E/0x64/0x65) before the 0F 03 opcode, matching Intel/AT&T encoding and llvm-mc.
**Impact:** Any `lsl %fs:…` / `%gs:…` / `%es:…` form assembles to the wrong machine code (default DS), so kernel/boot code that loads a segment limit through a non-DS selector reads the wrong descriptor and can fault or use a wrong limit.
**Function:** encode_lsl
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:160
**Detected by:** Differential — llvm-mc i686 (encode_lsl_diff_segment_prefix)
**Minimal input:** `lsl %es:(%eax), %ebx` → ops = [Memory(es:eax), Register(ebx)]
**Expected:** `[0x26, 0x0f, 0x03, 0x18]`
**Actual:** `[0x0f, 0x03, 0x18]`
**Severity:** high
**Root cause:** system.rs:160-164 memory arm never calls `emit_segment_prefix(mem)` before emitting the opcode, so `mem.segment` is ignored.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:160`
```rust
(Operand::Memory(mem), Operand::Register(dst)) => {
    let dst_num = reg_num(&dst.name).ok_or("bad register")?;
    self.bytes.extend_from_slice(&[0x0F, 0x03]);
    self.encode_modrm_mem(dst_num, mem)
}
```
**Suggested fix:** Emit the segment override (and keep osize handling) before the opcode:
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
cargo test --lib test_encode_lsl_regression_missing_es_prefix -- --test-threads=1
```
**Raw output:**
```text
assertion `left == right` failed: lsl %es:(%eax), %ebx must be [26, 0f, 03, 18], got [0f, 03, 18]
  left: [15, 3, 24]
 right: [38, 15, 3, 24]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_lsl_pbt.rs
