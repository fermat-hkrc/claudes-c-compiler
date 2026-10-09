# Bug: encode_smsw omits segment override prefix on SIB memory operands

**Law:** For every SIB memory operand carrying a segment override, `encode_smsw` must emit the corresponding segment prefix byte before the `0F 01 /4` opcode, matching llvm-mc i686 and `emit_segment_prefix`.
**Impact:** Segmented SMSW SIB forms assemble to default-DS addressing, silently wrong machine code.
**Function:** encode_smsw
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:241
**Detected by:** Differential — llvm-mc i686 (encode_smsw_diff_segment_sib)
**Minimal input:** `smsw %es:(%eax,%eax,1)` → SUT `[0f, 01, 24, 00]`, llvm-mc `[26, 0f, 01, 24, 00]`
**Expected:** `[0x26, 0x0f, 0x01, 0x24, 0x00]`
**Actual:** `[0x0f, 0x01, 0x24, 0x00]`
**Severity:** high
**Root cause:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:241-244` — memory arm never calls `self.emit_segment_prefix(mem)` (same root cause as base+disp segment bug).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:241`
```rust
            Operand::Memory(mem) => {
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.encode_modrm_mem(4, mem)
            }
```
**Suggested fix:** Emit the segment prefix before the opcode.
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
cargo test --lib encode_smsw_diff_segment_sib -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `[15, 1, 36, 0]`,
 right: `[38, 15, 1, 36, 0]`: seg+SIB diff for `smsw %es:(%eax,%eax,1)`: SUT=[0f, 01, 24, 00] llvm-mc=[26, 0f, 01, 24, 00]
minimal failing input: seg = "es", base = "eax", index = "eax", scale = 1, disp = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_smsw_pbt.rs (encode_smsw_diff_segment_sib)
