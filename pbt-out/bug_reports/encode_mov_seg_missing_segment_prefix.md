# Bug: encode_mov_seg omits memory segment-override prefix
**Law:** Memory forms of MOV to/from a segment register must emit the segment override prefix (0x26/0x2E/0x36/0x3E/0x64/0x65) before the 8C/8E opcode whenever the memory operand carries a segment, matching llvm-mc and `emit_segment_prefix`.
**Impact:** Any AT&T form such as `movw %ds, %es:(%eax)` or `movw %fs:4(%esi), %gs` assembles without the override, so the instruction addresses the wrong segment at run time.
**Function:** encode_mov_seg
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:309
**Detected by:** Differential — llvm-mc i686 (encode_mov_seg_diff_mem_segment)
**Minimal input:** `movw %es:(%eax), %es` (store=false, sreg=es, mseg=es, base=eax, disp=0)
**Expected:** `[0x26, 0x8e, 0x00]`
**Actual:** `[0x8e, 0x00]`
**Severity:** high
**Root cause:** system.rs:309-319 memory arms push 0x8C/0x8E then call `encode_modrm_mem` without `self.emit_segment_prefix(mem)` (core.rs:31-42). Same defect class as encode_lmsw/invlpg/smsw/system_table.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:309`
```rust
            // mov %sreg, mem
            (Operand::Register(src), Operand::Memory(mem)) if is_segment_reg(&src.name) => {
                let sr = seg_num(&src.name).ok_or("bad segment register")?;
                self.bytes.push(0x8C);
                self.encode_modrm_mem(sr, mem)
            }
            // mov mem, %sreg
            (Operand::Memory(mem), Operand::Register(dst)) if is_segment_reg(&dst.name) => {
                let sr = seg_num(&dst.name).ok_or("bad segment register")?;
                self.bytes.push(0x8E);
                self.encode_modrm_mem(sr, mem)
            }
```
**Suggested fix:** Call `emit_segment_prefix` before the opcode on both memory arms.
```rust
            (Operand::Register(src), Operand::Memory(mem)) if is_segment_reg(&src.name) => {
                let sr = seg_num(&src.name).ok_or("bad segment register")?;
                self.emit_segment_prefix(mem);
                self.bytes.push(0x8C);
                self.encode_modrm_mem(sr, mem)
            }
            (Operand::Memory(mem), Operand::Register(dst)) if is_segment_reg(&dst.name) => {
                let sr = seg_num(&dst.name).ok_or("bad segment register")?;
                self.emit_segment_prefix(mem);
                self.bytes.push(0x8E);
                self.encode_modrm_mem(sr, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_mov_seg_regression_es_segment_prefix -- --test-threads=1
```
**Raw output:**
```text
assertion `left == right` failed: movw %ds, %es:(%eax) must be [26, 8c, 18]
  left: [140, 24]
 right: [38, 140, 24]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_mov_seg_pbt.rs (test_encode_mov_seg_regression_es_segment_prefix)
