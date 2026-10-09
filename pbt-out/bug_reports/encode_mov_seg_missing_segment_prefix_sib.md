# Bug: encode_mov_seg omits segment-override prefix on SIB memory forms
**Law:** SIB memory forms of MOV to/from a segment register must emit the segment override prefix before 8C/8E whenever the memory operand carries a segment, matching llvm-mc and `emit_segment_prefix`.
**Impact:** Forms such as `movw %es:4(%eax,%ecx,4), %gs` assemble without the override; the instruction addresses the wrong segment.
**Function:** encode_mov_seg
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:315
**Detected by:** Differential — llvm-mc i686 (encode_mov_seg_diff_segment_sib)
**Minimal input:** `movw %es:4(%eax,%eax,1), %es`
**Expected:** `[0x26, 0x8e, 0x44, 0x00, 0x04]`
**Actual:** `[0x8e, 0x44, 0x00, 0x04]`
**Severity:** high
**Root cause:** system.rs:315-319 (and 309-313) push 0x8C/0x8E then `encode_modrm_mem` without `self.emit_segment_prefix(mem)`. Same defect as base-only segmented mem (encode_mov_seg_missing_segment_prefix.md).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:315`
```rust
            // mov mem, %sreg
            (Operand::Memory(mem), Operand::Register(dst)) if is_segment_reg(&dst.name) => {
                let sr = seg_num(&dst.name).ok_or("bad segment register")?;
                self.bytes.push(0x8E);
                self.encode_modrm_mem(sr, mem)
            }
```
**Suggested fix:** Call `emit_segment_prefix` before the opcode on both memory arms.
```rust
                self.emit_segment_prefix(mem);
                self.bytes.push(0x8E);
                self.encode_modrm_mem(sr, mem)
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_mov_seg_diff_segment_sib -- --test-threads=1
```
**Raw output:**
```text
left: `[142, 68, 0, 4]`, right: `[38, 142, 68, 0, 4]`: seg+SIB must match llvm-mc for movw %es:4(%eax,%eax,1), %es
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_mov_seg_pbt.rs (encode_mov_seg_diff_segment_sib)
