# Bug: encode_smsw omits segment override prefix on memory operands

**Law:** For every memory operand carrying a segment override, `encode_smsw` must emit the corresponding segment prefix byte (ES=0x26, CS=0x2E, SS=0x36, DS=0x3E, FS=0x64, GS=0x65) before the `0F 01 /4` opcode, matching llvm-mc i686 and the in-tree `emit_segment_prefix` helper.
**Impact:** Any AT&T `smsw %fs:…` / `smsw %es:…` (etc.) assembles to bytes that address the wrong segment (default DS), silently corrupting machine code for privileged SMSW sequences that intentionally touch non-DS memory.
**Function:** encode_smsw
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:241
**Detected by:** Differential — llvm-mc i686 (encode_smsw_diff_segment / encode_smsw_diff_segment_sib)
**Minimal input:** `smsw %es:(%eax)` → SUT `[0f, 01, 20]`, llvm-mc `[26, 0f, 01, 20]`
**Expected:** `[0x26, 0x0f, 0x01, 0x20]`
**Actual:** `[0x0f, 0x01, 0x20]`
**Severity:** high
**Root cause:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:241-244` — the memory arm extends the opcode then calls `encode_modrm_mem` without first calling `self.emit_segment_prefix(mem)`. Same defect class as encode_lmsw / encode_invlpg / encode_verw / encode_prefetch / encode_system_table.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:241`
```rust
            Operand::Memory(mem) => {
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.encode_modrm_mem(4, mem)
            }
```
**Suggested fix:** Emit the segment prefix before the opcode, matching the x86-64 sibling's `emit_rex_rm` path and i686 `core.rs:emit_segment_prefix`.
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.encode_modrm_mem(4, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_smsw_diff_segment -- --test-threads=1
cargo test --lib test_encode_smsw_regression_missing_es_prefix -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `[15, 1, 32]`,
 right: `[38, 15, 1, 32]`: segment prefix diff for `smsw %es:(%eax)`: SUT=[0f, 01, 20] llvm-mc=[26, 0f, 01, 20]
minimal failing input: seg = "es", base = "eax", disp = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_smsw_pbt.rs (test_encode_smsw_regression_missing_es_prefix, test_encode_smsw_regression_missing_fs_prefix)
