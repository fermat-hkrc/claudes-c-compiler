# Bug: encode_movsx omits segment-override prefixes on SIB memory forms
**Law:** ∀ form ∈ {movsbl,movsbw,movswl}, seg ∈ {es,cs,ss,ds,fs,gs}, SIB mem, dst. SUT bytes equal llvm-mc including the segment-override prefix before optional 0x66 and 0F BE/BF
**Impact:** Same as plain base+disp: segment overrides on SIB addresses are silently dropped, so kernel/OS AT&T with `%es:(%eax,%ecx,4)` reads the wrong segment
**Function:** encode_movsx
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:272
**Detected by:** Differential — llvm-mc i686 (segment + SIB strengthen round)
**Minimal input:** `movsbl %es:(%eax,%eax,1), %eax`
**Expected:** bytes `[0x26, 0x0f, 0xbe, 0x04, 0x00]`
**Actual:** `[0x0f, 0xbe, 0x04, 0x00]` — no override prefix
**Severity:** high
**Root cause:** Same as missing segment on base+disp: gp_integer.rs:292-295 memory arm never calls `emit_segment_prefix` before opcode/ModRM (SIB is produced inside encode_modrm_mem with no prefix path).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:292`
```rust
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                self.bytes.extend_from_slice(&opcode);
                self.encode_modrm_mem(dst_num, mem)?;
            }
```
**Suggested fix:** Emit segment prefix before opcode on the memory arm (covers base, SIB, and abs):
```rust
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&opcode);
                self.encode_modrm_mem(dst_num, mem)?;
            }
```
**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_movsx_diff_llvm_mc_segment_sib -- --test-threads=1
```
**Raw output:**
```text
segment+SIB diff `movsbl %es:(%eax,%eax,1), %eax`: sut=[0f, be, 04, 00] mc=[26, 0f, be, 04, 00]
minimal failing input: fi = 0, seg = "es", base = "eax", index = "eax", scale = 1, disp = 0, di = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_movsx_pbt.rs (`encode_movsx_diff_llvm_mc_segment_sib`)
