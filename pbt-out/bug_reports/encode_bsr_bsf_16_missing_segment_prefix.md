# Bug: encode_bsr_bsf_16 omits segment-override prefix on memory forms

**Law:** For AT&T `bsfw`/`bsrw` with a segmented memory source, the emitted bytes must match llvm-mc `-triple=i686` — segment override (e.g. `0x26` for ES, `0x64` for FS) then operand-size `0x66`, then `0F BC/BD /r` + ModR/M.
**Impact:** Any `bsfw`/`bsrw` whose source uses `%es:`, `%fs:`, etc. assembles without the override, so the CPU reads from the wrong segment — silent wrong-address loads in kernel/boot code that relies on segment overrides.
**Function:** encode_bsr_bsf_16
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:353
**Detected by:** Differential — llvm-mc i686 (encode_bsr_bsf_16_diff_mem_segment)
**Minimal input:** `bsfw %es:(%eax), %bx`
**Expected:** `[0x26, 0x66, 0x0f, 0xbc, 0x18]`
**Actual:** `[0x66, 0x0f, 0xbc, 0x18]` (ES override `0x26` missing)
**Severity:** high
**Root cause:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:362-375` — pushes `0x66` then opcode then `encode_modrm_mem` without calling `emit_segment_prefix(mem)` first (same class as encode_lmsw / encode_invlpg / encode_pop16 memory).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:362`
```rust
        self.bytes.push(0x66); // 16-bit operand size prefix
        match (&ops[0], &ops[1]) {
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&opcode);
                self.bytes.push(self.modrm(3, dst_num, src_num));
                Ok(())
            }
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&opcode);
                self.encode_modrm_mem(dst_num, mem)
            }
```
**Suggested fix:** Call `emit_segment_prefix` on the memory arm **before** the shared `0x66` push (segment must precede operand-size override per llvm-mc / Intel SDM).
```rust
        match (&ops[0], &ops[1]) {
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                if reg_size(&src.name) != 2 || reg_size(&dst.name) != 2 {
                    return Err(format!("unsupported {} operands", mnemonic));
                }
                self.bytes.push(0x66);
                self.bytes.extend_from_slice(&opcode);
                self.bytes.push(self.modrm(3, dst_num, src_num));
                Ok(())
            }
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                if reg_size(&dst.name) != 2 {
                    return Err(format!("unsupported {} operands", mnemonic));
                }
                self.emit_segment_prefix(mem); // BEFORE 0x66
                self.bytes.push(0x66);
                self.bytes.extend_from_slice(&opcode);
                self.encode_modrm_mem(dst_num, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_bsr_bsf_16_regression_segment_es_missing -- --test-threads=1
```
**Raw output:**
```text
assertion `left == right` failed: regression: bsfw %es:(%eax), %bx must include 0x26 (got [66, 0f, bc, 18])
  left: [102, 15, 188, 24]
 right: [38, 102, 15, 188, 24]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_bsr_bsf_16_pbt.rs (test_encode_bsr_bsf_16_regression_segment_es_missing)
