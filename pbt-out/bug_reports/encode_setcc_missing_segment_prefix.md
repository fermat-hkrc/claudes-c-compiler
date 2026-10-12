# Bug: encode_setcc omits segment-override prefix on memory operands

**Law:** For any SETcc mnemonic and any memory operand carrying a segment override, the emitted bytes must begin with the Intel segment-override prefix (ES=0x26, CS=0x2E, SS=0x36, DS=0x3E, FS=0x64, GS=0x65) before the 0F 90+cc opcode, matching llvm-mc `-triple=i686` and sibling i686 encoders that call `emit_segment_prefix`.
**Impact:** Assembler output for `sete %es:(%eax)`, `setne %fs:8(%esi)`, etc. is missing the override byte, so the instruction reads/writes the wrong segment at run time when those forms are assembled.
**Function:** encode_setcc
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1089
**Detected by:** Differential — llvm-mc i686 (encode_setcc_diff_segment_llvm_mc / encode_setcc_diff_segment_sib)
**Minimal input:** `sete %es:(%eax)` (also `sete %fs:8(%esi,%edi,4)`)
**Expected:** `[0x26, 0x0f, 0x94, 0x00]`
**Actual:** `[0x0f, 0x94, 0x00]`
**Severity:** high
**Root cause:** `gp_integer.rs:1089-1091` memory arm extends `0F 90+cc` then calls `encode_modrm_mem` without first calling `self.emit_segment_prefix(mem)` (defined at `core.rs:31`). Same defect class as encode_smsw / encode_invlpg / encode_prefetch.
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:1089`
```rust
            Operand::Memory(mem) => {
                self.bytes.extend_from_slice(&[0x0F, 0x90 + cc]);
                self.encode_modrm_mem(0, mem)
            }
```
**Suggested fix:** Emit the segment override before the opcode bytes.
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.extend_from_slice(&[0x0F, 0x90 + cc]);
                self.encode_modrm_mem(0, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_setcc_diff_segment_llvm_mc -- --test-threads=1 --exact
cargo test --lib test_encode_setcc_regression_missing_es_prefix -- --test-threads=1 --exact
```
**Raw output:**
```text
Test failed: assertion failed: `(left == right)`
  left: `[15, 148, 0]`,
 right: `[38, 15, 148, 0]`: segment prefix diff for `sete %es:(%eax)`: SUT=[0f, 94, 00] llvm-mc=[26, 0f, 94, 00]
minimal failing input: m = "sete", seg = "es", base = "eax", disp = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_setcc_pbt.rs (`test_encode_setcc_regression_missing_es_prefix`, `test_encode_setcc_regression_missing_fs_prefix`)
