# Bug: encode_system_table omits segment-override prefix on SIB memory operands

**Law:** For SGDT/SIDT/LGDT/LIDT with a segment-overridden SIB memory operand, the encoder must emit the override prefix before `0F 01`, matching Intel SDM and llvm-mc i686.
**Impact:** Same as the base+disp segment case: silent wrong encoding when OS/boot code uses e.g. `%es:(%eax,%ecx,4)` with system-table instructions.
**Function:** encode_system_table
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:185
**Detected by:** Differential — llvm-mc i686 (segment+SIB forms); strengthening round of encode_system_table_diff_segment_llvm_mc
**Minimal input:** `sgdt %es:(%eax,%eax,1)`
**Expected:** `[0x26, 0x0f, 0x01, 0x04, 0x00]`
**Actual:** `[0x0f, 0x01, 0x04, 0x00]` (ES override 0x26 missing)
**Severity:** high
**Root cause:** Same statement as B1 — `system.rs:185-187` does not call `emit_segment_prefix(mem)` before the opcode. SIB vs base+disp only changes the trailing ModR/M encoding, not the missing prefix.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:185`
```rust
            Operand::Memory(mem) => {
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.encode_modrm_mem(reg_ext, mem)
            }
```
**Suggested fix:** Call the existing helper before the opcode:
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.encode_modrm_mem(reg_ext, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_system_table_diff_segment_sib -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `[15, 1, 4, 0]`,
 right: `[38, 15, 1, 4, 0]`: seg+SIB diff for `sgdt %es:(%eax,%eax,1)`: SUT=[0f, 01, 04, 00] llvm-mc=[26, 0f, 01, 04, 00]
minimal failing input: mnem = "sgdt", seg = "es", base = "eax", index = "eax", scale = 1, disp = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_system_table_pbt.rs (`test_encode_system_table_regression_missing_es_prefix` covers ES prefix; SIB path covered by this property)
