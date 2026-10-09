# Bug: encode_alu omits segment-override prefix (metamorphic seg‖bare witness)
**Law:** ∀ ALU op with a memory operand carrying segment S ∈ {es,cs,ss,ds,fs,gs}, the encoding must begin with the corresponding override byte (0x26/0x2E/0x36/0x3E/0x64/0x65), matching llvm-mc and core.rs emit_segment_prefix.
**Impact:** Segmented memory ALU (e.g. TLS via %gs:, %fs:, or explicit %es:) assembles without the override, so the CPU uses the default segment — silent wrong-address access / data corruption for any caller relying on the override.
**Function:** encode_alu
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:505
**Detected by:** Differential — llvm-mc i686; Algebraic — Metamorphic (seg ‖ bare)
**Minimal input:** `addb %es:(%eax), %al` vs bare `addb (%eax), %al` (metamorphic)
**Expected:** `[0x26, 0x02, 0x00]` = `[0x26] ++ bare [0x02, 0x00]`
**Actual:** `[0x02, 0x00]` (identical to bare — prefix omitted)
**Severity:** high
**Root cause:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:505-517` memory arms call `encode_modrm_mem` without `emit_segment_prefix(mem)`; x86-64 sibling does call it before the opcode.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:505`
```rust
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                if size == 2 { self.bytes.push(0x66); }
                self.bytes.push(if size == 1 { 0x02 } else { 0x03 } + alu_op * 8);
                self.encode_modrm_mem(dst_num, mem)
            }
            (Operand::Register(src), Operand::Memory(mem)) => {
                let src_num = reg_num(&src.name).ok_or("bad src register")?;
                if size == 2 { self.bytes.push(0x66); }
                self.bytes.push(if size == 1 { 0x00 } else { 0x01 } + alu_op * 8);
                self.encode_modrm_mem(src_num, mem)
            }
```
**Suggested fix:** Call `self.emit_segment_prefix(mem);` before the 0x66/opcode on every memory arm (m→r, r→m, imm→m, symbol/label mem), matching the x86-64 sibling.
```rust
            (Operand::Register(src), Operand::Memory(mem)) => {
                let src_num = reg_num(&src.name).ok_or("bad src register")?;
                self.emit_segment_prefix(mem);
                if size == 2 { self.bytes.push(0x66); }
                self.bytes.push(if size == 1 { 0x00 } else { 0x01 } + alu_op * 8);
                self.encode_modrm_mem(src_num, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib backend::i686::assembler::encoder::encode_alu_pbt -- --test-threads=1
# narrowed:
cargo test --lib 'backend::i686::assembler::encoder::encode_alu_pbt::encode_alu_metamorphic_segment_prefix' -- --test-threads=1 --nocapture
```
**Raw output:**
```text
assertion `left == right` failed: regression: segmented ALU must match llvm-mc (got [01, 18])
  left: [1, 24]
 right: [38, 1, 24]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_alu_pbt.rs (encode_alu_metamorphic_segment_prefix)
