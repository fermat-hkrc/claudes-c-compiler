# Bug: encode_bsr_bsf omits segment-override prefix on memory forms
**Law:** Memory forms of BSF/BSR must emit the segment-override prefix (ES=0x26, CS=0x2E, SS=0x36, DS=0x3E, FS=0x64, GS=0x65) before the opcode, matching llvm-mc and Intel SDM segment-override encoding.
**Impact:** Assembler silently drops segment overrides on `bsfl`/`bsrl`/`bsf`/`bsr` memory sources, so code that addresses `%es:`, `%fs:`, etc. is assembled as DS-relative and runs against the wrong segment — silent wrong code generation for any caller using segmented memory operands.
**Function:** encode_bsr_bsf
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1003
**Detected by:** Differential — llvm-mc i686 (encode_bsr_bsf_diff_mem_segment)
**Minimal input:** `bsfl %es:(%eax), %ebx` (also `bsrl %fs:4(%esi), %edi`)
**Expected:** `[0x26, 0x0f, 0xbc, 0x18]`
**Actual:** `[0x0f, 0xbc, 0x18]` (0x26 omitted)
**Severity:** high
**Root cause:** gp_integer.rs:1003-1007 Memory arm emits opcode then `encode_modrm_mem` without calling `emit_segment_prefix(mem)` first (same defect class as encode_bsr_bsf_16 / encode_lmsw / encode_invlpg).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1003`
```rust
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.bytes.extend_from_slice(&opcode);
                self.encode_modrm_mem(dst_num, mem)
            }
```
**Suggested fix:** Call `emit_segment_prefix` before the opcode bytes.
```rust
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&opcode);
                self.encode_modrm_mem(dst_num, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_bsr_bsf_regression_segment_es_missing -- --test-threads=1
```
**Raw output:**
```text
assertion `left == right` failed: regression: bsfl %es:(%eax), %ebx must include 0x26 (got [0f, bc, 18])
  left: [15, 188, 24]
 right: [38, 15, 188, 24]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_bsr_bsf_pbt.rs (test_encode_bsr_bsf_regression_segment_es_missing)
