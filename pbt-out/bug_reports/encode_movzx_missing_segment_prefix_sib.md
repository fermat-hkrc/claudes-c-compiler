# Bug: encode_movzx omits segment-override prefixes on SIB mem→reg forms
**Law:** ∀ seg ∈ {es,cs,ss,ds,fs,gs}, valid movzx SIB mem→reg. SUT bytes equal llvm-mc including the segment-override prefix before optional 0x66 and 0F B6/B7
**Impact:** Same as missing segment prefix on base+disp: SIB forms with segment overrides silently drop the override
**Function:** encode_movzx
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:302
**Detected by:** Differential — llvm-mc i686 (segment + SIB)
**Minimal input:** `movzbl %es:(%eax,%eax,1), %eax`
**Expected:** bytes `[0x26, 0x0f, 0xb6, 0x04, 0x00]`
**Actual:** `[0x0f, 0xb6, 0x04, 0x00]` — no override prefix
**Severity:** high
**Root cause:** Same as base+disp — memory arm never calls emit_segment_prefix before opcode (gp_integer.rs:322-325).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:322`
```rust
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                self.bytes.extend_from_slice(&opcode);
                self.encode_modrm_mem(dst_num, mem)?;
            }
```
**Suggested fix:** Same as encode_movzx_missing_segment_prefix — `self.emit_segment_prefix(mem);` before 0x66/opcode on the memory arm.
```rust
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                self.emit_segment_prefix(mem);
                if dst_size == 2 { self.bytes.push(0x66); }
                self.bytes.extend_from_slice(&opcode);
                self.encode_modrm_mem(dst_num, mem)?;
            }
```
**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_movzx_diff_llvm_mc_segment_sib -- --test-threads=1
```
**Raw output:**
```text
segment+SIB diff `movzbl %es:(%eax,%eax,1), %eax`: sut=[0f, b6, 04, 00] mc=[26, 0f, b6, 04, 00]
minimal failing input: fi = 0, seg = "es", base = "eax", index = "eax", scale = 1, disp = 0, di = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_movzx_pbt.rs (`encode_movzx_diff_llvm_mc_segment_sib`)
