# Bug: encode_verw omits segment-override prefix on SIB memory operands
**Law:** For any SIB memory operand with a segment override, VERW encoding must emit the corresponding prefix byte before the 0F 00 opcode.
**Impact:** Same as missing-prefix on base-only forms: segment-relative VERW with index/scale targets the wrong segment.
**Function:** encode_verw
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:129
**Detected by:** Differential — llvm-mc i686 (VERW 0F 00 /5), segment+SIB strengthening property
**Minimal input:** `verw %es:(%eax,%eax,1)`
**Expected:** `[0x26, 0x0f, 0x00, 0x2c, 0x00]`
**Actual:** `[0x0f, 0x00, 0x2c, 0x00]`
**Severity:** high
**Root cause:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:129-132` — memory arm never calls `emit_segment_prefix` (same defect as base-only form).
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
cargo test --lib encode_verw_diff_segment_sib -- --test-threads=1
```
**Raw output:**
```text
seg+SIB diff for `verw %es:(%eax,%eax,1)`: SUT=[0f, 00, 2c, 00] llvm-mc=[26, 0f, 00, 2c, 00]
minimal failing input: seg = "es", base = "eax", index = "eax", scale = 1, disp = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_verw_pbt.rs (encode_verw_diff_segment_sib / test_encode_verw_regression_missing_fs_prefix)
