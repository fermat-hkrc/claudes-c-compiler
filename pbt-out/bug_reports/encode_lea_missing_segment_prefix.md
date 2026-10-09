# Bug: encode_lea omits segment-override prefix on memory source

**Law:** For every valid i686 LEA with a segment override on the memory operand, the encoder must emit the corresponding segment-override prefix (ES=0x26, CS=0x2E, SS=0x36, DS=0x3E, FS=0x64, GS=0x65) before opcode 0x8D, matching llvm-mc and `core.rs::emit_segment_prefix`.
**Impact:** Any AT&T form such as `leal %es:(%eax), %ebx` assembles without the override byte, so the machine code differs from the intended encoding and from gas/llvm-mc. Callers that rely on segment-relative address materialization get wrong bytes in the object file.
**Function:** encode_lea
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:332
**Detected by:** Differential — llvm-mc i686 (encode_lea_diff_llvm_mc_segment / KAT es+fs)
**Minimal input:** `leal %es:(%eax), %eax` (ops: Memory{segment:es, base:eax}, Register eax)
**Expected:** `[0x26, 0x8d, 0x00]`
**Actual:** `[0x8d, 0x00]`
**Severity:** medium
**Root cause:** `gp_integer.rs:338-340` pushes 0x8D and encodes ModR/M without calling `self.emit_segment_prefix(mem)` (available at `core.rs:31-42`). Same defect class as encode_invlpg / encode_prefetch / encode_mov_mem_reg (partial).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:338`
```rust
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                self.bytes.push(0x8D);
                self.encode_modrm_mem(dst_num, mem)
            }
```
**Suggested fix:** Emit the segment override before the opcode.
```rust
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad dst register")?;
                self.emit_segment_prefix(mem);
                self.bytes.push(0x8D);
                self.encode_modrm_mem(dst_num, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_lea_regression_missing_es_prefix -- --test-threads=1
cargo test --lib encode_lea_diff_llvm_mc_segment -- --test-threads=1
```
**Raw output:**
```text
assertion `left == right` failed: SUT must emit ES override 0x26 before 8D
  left: [141, 24]
 right: [38, 141, 24]

segment diff `leal %es:(%eax), %eax`: sut=[8d, 00] mc=[26, 8d, 00]
minimal failing input: seg = "es", base = "eax", disp = 0, di = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_lea_pbt.rs (encode_lea_regression_missing_es_prefix)
