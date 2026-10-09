# Bug: encode_lmsw omits segment override prefix on memory operands

**Law:** For every memory operand carrying a segment override, `encode_lmsw` must emit the corresponding segment prefix byte (ES=0x26, CS=0x2E, SS=0x36, DS=0x3E, FS=0x64, GS=0x65) before the `0F 01 /6` opcode, matching llvm-mc i686 and the in-tree `emit_segment_prefix` helper.
**Impact:** Any AT&T `lmsw %fs:…` / `lmsw %es:…` (etc.) assembles to bytes that address the wrong segment (default DS), silently corrupting machine code for privileged LMSW sequences that intentionally touch non-DS memory.
**Function:** encode_lmsw
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:214
**Detected by:** Differential — llvm-mc i686 (encode_lmsw_diff_segment / encode_lmsw_diff_segment_sib)
**Minimal input:** `lmsw %es:(%eax)` → SUT `[0f, 01, 30]`, llvm-mc `[26, 0f, 01, 30]`
**Expected:** `[0x26, 0x0f, 0x01, 0x30]`
**Actual:** `[0x0f, 0x01, 0x30]`
**Severity:** high
**Root cause:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:214-216` — the memory arm extends the opcode then calls `encode_modrm_mem` without first calling `self.emit_segment_prefix(mem)`. Same defect class as encode_invlpg / encode_verw / encode_prefetch / encode_system_table.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:214`
```rust
            Operand::Memory(mem) => {
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.encode_modrm_mem(6, mem)
            }
```
**Suggested fix:** Emit the segment prefix before the opcode, matching the x86-64 sibling's `emit_rex_rm` path and i686 `core.rs:emit_segment_prefix`.
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.encode_modrm_mem(6, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_lmsw_diff_segment -- --test-threads=1
cargo test --lib test_encode_lmsw_regression_missing_es_prefix -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `[15, 1, 48]`,
 right: `[38, 15, 1, 48]`: segment prefix diff for `lmsw %es:(%eax)`: SUT=[0f, 01, 30] llvm-mc=[26, 0f, 01, 30]
minimal failing input: seg = "es", base = "eax", disp = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_lmsw_pbt.rs (test_encode_lmsw_regression_missing_es_prefix, test_encode_lmsw_regression_missing_fs_prefix)
