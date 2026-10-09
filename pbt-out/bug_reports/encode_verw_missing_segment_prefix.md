# Bug: encode_verw omits segment-override prefix on memory operands
**Law:** For any memory operand with a segment override, VERW encoding must emit the corresponding prefix byte (26/2E/36/3E/64/65) before the 0F 00 opcode, matching Intel SDM and llvm-mc.
**Impact:** Assembler output for `verw %fs:(%reg)` / `%gs:` / `%es:` etc. is missing the override; the CPU uses the default data segment, so segment-relative descriptor checks target the wrong address space.
**Function:** encode_verw
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:129
**Detected by:** Differential — llvm-mc i686 (VERW 0F 00 /5)
**Minimal input:** `verw %es:(%eax)` (also `%fs:(%eax)`)
**Expected:** `[0x26, 0x0f, 0x00, 0x28]` (ES); `[0x64, 0x0f, 0x00, 0x28]` (FS)
**Actual:** `[0x0f, 0x00, 0x28]` (no prefix)
**Severity:** high
**Root cause:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:129-132` — memory arm extends opcode then calls `encode_modrm_mem` without `emit_segment_prefix(mem)` (helper exists at core.rs:31-42; x86-64 sibling calls `emit_rex_rm` which covers segment).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:129`
```rust
            Operand::Memory(mem) => {
                self.bytes.extend_from_slice(&[0x0F, 0x00]);
                self.encode_modrm_mem(5, mem)
            }
```
**Suggested fix:** Call `emit_segment_prefix` before the opcode bytes.
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&[0x0F, 0x00]);
                self.encode_modrm_mem(5, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib test_encode_verw_regression_missing_fs_prefix -- --test-threads=1
```
**Raw output:**
```text
assertion `left == right` failed: verw %fs:(%eax) must be [64, 0f, 00, 28], got [0f, 00, 28]
  left: [15, 0, 40]
 right: [100, 15, 0, 40]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_verw_pbt.rs (test_encode_verw_regression_missing_fs_prefix)
