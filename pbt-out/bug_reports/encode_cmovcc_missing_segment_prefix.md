# Bug: encode_cmovcc omits memory segment-override prefix
**Law:** Memory-form CMOVcc must emit the segment override byte (ES=0x26, CS=0x2E, SS=0x36, DS=0x3E, FS=0x64, GS=0x65) before the 0F 40+cc opcode whenever the memory operand carries a segment, matching Intel SDM and llvm-mc `-triple=i686`.
**Impact:** Any AT&T `cmov*` with `%es:`/`%fs:`/… memory source assembles to the wrong machine code (missing prefix). Runtime reads/writes the default segment instead of the requested one — silent data corruption for FS/GS TLS-style accesses and ES/DS overrides.
**Function:** encode_cmovcc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1128
**Detected by:** Differential — llvm-mc i686 (encode_cmovcc_diff_segment_llvm_mc)
**Minimal input:** `cmove %es:(%eax), %ebx` → SUT `[0f, 44, 18]`; llvm-mc `[26, 0f, 44, 18]`
**Expected:** `[0x26, 0x0f, 0x44, 0x18]`
**Actual:** `[0x0f, 0x44, 0x18]`
**Severity:** high
**Root cause:** `gp_integer.rs:1128-1133` memory arm never calls `emit_segment_prefix(mem)` before emitting the opcode (same defect class as encode_setcc/smsw/invlpg).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1128`
```rust
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                if is_16bit { self.bytes.push(0x66); }
                self.bytes.extend_from_slice(&[0x0F, 0x40 + cc]);
                self.encode_modrm_mem(dst_num, mem)
            }
```
**Suggested fix:** Call `emit_segment_prefix` before the opcode (and before optional 0x66 so segment precedes operand-size, matching llvm-mc `0x64, 0x66, …` for `cmove %fs:(%eax), %bx`):
```rust
            (Operand::Memory(mem), Operand::Register(dst)) => {
                let dst_num = reg_num(&dst.name).ok_or("bad register")?;
                self.emit_segment_prefix(mem);
                if is_16bit { self.bytes.push(0x66); }
                self.bytes.extend_from_slice(&[0x0F, 0x40 + cc]);
                self.encode_modrm_mem(dst_num, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_cmovcc_regression_missing_es_prefix -- --test-threads=1 --nocapture
```
**Raw output:**
```text
assertion `left == right` failed: cmove %es:(%eax), %ebx must be [26, 0f, 44, 18], got [0f, 44, 18]
  left: [15, 68, 24]
 right: [38, 15, 68, 24]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_cmovcc_pbt.rs (test_encode_cmovcc_regression_missing_es_prefix, test_encode_cmovcc_regression_missing_fs_prefix)
