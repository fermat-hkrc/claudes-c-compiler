# Bug: encode_movsx omits all segment-override prefixes
**Law:** ∀ seg ∈ {es,cs,ss,ds,fs,gs}, valid movsx mem→reg. SUT bytes equal llvm-mc including the segment-override prefix (26/2E/36/3E/64/65) before optional 0x66 and 0F BE/BF
**Impact:** Assembler silently drops segment overrides on forms like `movsbl %es:(%eax), %ecx` and `movsbw %fs:(%ebx), %dx`, producing wrong machine code that reads the wrong segment — a silent correctness bug in OS/kernel-style AT&T asm
**Function:** encode_movsx
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:272
**Detected by:** Differential — llvm-mc i686 (segment override forms)
**Minimal input:** `movsbl %es:(%eax), %eax` (also `movsbl %es:(%eax), %ecx`, movsbw/movswl with any of the six segments)
**Expected:** bytes `[0x26, 0x0f, 0xbe, 0x00]` for movsbl %es:(%eax), %eax
**Actual:** `[0x0f, 0xbe, 0x00]` — no override prefix at all (including fs/gs)
**Severity:** high
**Root cause:** gp_integer.rs:278-295 optionally pushes 0x66, extends opcode, then calls encode_modrm_mem without ever calling `emit_segment_prefix` (core.rs:31-42). Memory segment field is ignored entirely.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:278`
```rust
        if dst_size == 2 { self.bytes.push(0x66); }

        let opcode = match src_size {
            1 => vec![0x0F, 0xBE],
            2 => vec![0x0F, 0xBF],
            _ => return Err(format!("unsupported movsx src size: {}", src_size)),
        };

        match (&ops[0], &ops[1]) {
            (Operand::Register(src), Operand::Register(dst)) => {
                let src_num = reg_num(&src.name).ok_or("bad src register")?;
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                self.bytes.extend_from_slice(&opcode);
                self.bytes.push(self.modrm(3, dst_num, src_num));
            }
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                self.bytes.extend_from_slice(&opcode);
                self.encode_modrm_mem(dst_num, mem)?;
            }
```
**Suggested fix:** Emit the shared segment prefix before the operand-size override / opcode on the memory arm:
```rust
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                self.emit_segment_prefix(mem);
                if dst_size == 2 { self.bytes.push(0x66); }
                self.bytes.extend_from_slice(&opcode);
                self.encode_modrm_mem(dst_num, mem)?;
            }
```
(Move the unconditional `if dst_size == 2` 0x66 into each arm, or emit segment before the existing 0x66 push so order is seg → 66 → 0F BE/BF.)
**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_movsx_diff_llvm_mc_segment -- --test-threads=1
cargo test --lib encode_movsx_regression_es_segment_prefix -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `[15, 190, 0]`,
 right: `[38, 15, 190, 0]`: segment diff `movsbl %es:(%eax), %eax`: sut=[0f, be, 00] mc=[26, 0f, be, 00]
minimal failing input: fi = 0, seg = "es", base = "eax", disp = 0, di = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_movsx_pbt.rs (`encode_movsx_regression_es_segment_prefix`)
