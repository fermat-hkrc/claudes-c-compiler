# Bug: encode_invlpg drops segment override on SIB memory forms

**Law:** For any valid SIB memory operand carrying a segment override, `encode_invlpg` must emit the segment-override prefix before `0F 01 /7`, matching llvm-mc.
**Impact:** Same as the base+disp segment bug: INVLPG with `%seg:(%base,%index,scale)` targets the wrong address space.
**Function:** encode_invlpg
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:110
**Detected by:** Differential — llvm-mc i686 (encode_invlpg_diff_segment_sib)
**Minimal input:** `invlpg %es:(%eax,%eax,1)`
**Expected:** `[0x26, 0x0f, 0x01, 0x3c, 0x00]`
**Actual:** `[0x0f, 0x01, 0x3c, 0x00]`
**Severity:** high
**Root cause:** `system.rs:116-117` pushes `0F 01` and calls `encode_modrm_mem` without `emit_segment_prefix(mem)` — identical root cause to the base+disp segment bug; SIB is an independent witness.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/system.rs:116`
```rust
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.encode_modrm_mem(7, mem)
```
**Suggested fix:** Call `emit_segment_prefix` before the opcode bytes.
```rust
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&[0x0F, 0x01]);
                self.encode_modrm_mem(7, mem)
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_invlpg_diff_segment_sib -- --test-threads=1
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `[15, 1, 60, 0]`,
 right: `[38, 15, 1, 60, 0]`: seg+SIB diff for `invlpg %es:(%eax,%eax,1)`
minimal failing input: seg = "es", base = "eax", index = "eax", scale = 1, disp = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_invlpg_pbt.rs (`test_encode_invlpg_regression_missing_fs_prefix`)
