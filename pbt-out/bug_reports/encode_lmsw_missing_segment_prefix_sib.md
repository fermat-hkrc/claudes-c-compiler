# Bug: encode_lmsw omits segment override prefix on SIB memory operands

**Law:** For every SIB memory operand carrying a segment override, `encode_lmsw` must emit the corresponding segment prefix byte before the `0F 01 /6` opcode, matching llvm-mc i686 and `emit_segment_prefix`.
**Impact:** Segment-qualified SIB forms of LMSW (e.g. `lmsw %es:(%eax,%ecx,4)`) drop the override and address default DS — same root cause as the base+disp segment bug (missing `emit_segment_prefix`).
**Function:** encode_lmsw
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:214
**Detected by:** Differential — llvm-mc i686 (encode_lmsw_diff_segment_sib)
**Minimal input:** `lmsw %es:(%eax,%eax,1)` → SUT `[0f, 01, 34, 00]`, llvm-mc `[26, 0f, 01, 34, 00]`
**Expected:** `[0x26, 0x0f, 0x01, 0x34, 0x00]`
**Actual:** `[0x0f, 0x01, 0x34, 0x00]`
**Severity:** high
**Root cause:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:214-216` — memory arm never calls `self.emit_segment_prefix(mem)` before opcode (same statement as B1 base+disp case).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:214`
```rust
            Operand::Memory(mem) => {
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.encode_modrm_mem(6, mem)
            }
```
**Suggested fix:** Emit the segment prefix before the opcode.
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
cargo test --lib encode_lmsw_diff_segment_sib -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `[15, 1, 52, 0]`,
 right: `[38, 15, 1, 52, 0]`: seg+SIB diff for `lmsw %es:(%eax,%eax,1)`: SUT=[0f, 01, 34, 00] llvm-mc=[26, 0f, 01, 34, 00]
minimal failing input: seg = "es", base = "eax", index = "eax", scale = 1, disp = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_lmsw_pbt.rs (encode_lmsw_diff_segment_sib / test_encode_lmsw_regression_missing_es_prefix)
