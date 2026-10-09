# Bug: encode_test Imm→Mem omits segment override prefix
**Law:** Memory operands with a segment override must emit the corresponding prefix byte (es=0x26, cs=0x2E, ss=0x36, ds=0x3E, fs=0x64, gs=0x65) before the TEST opcode, per core.rs emit_segment_prefix and llvm-mc.
**Impact:** `testl $imm, %es:(%eax)` assembles without 0x26, producing wrong machine code that touches DS instead of ES (silent wrong-address bug for any segmented TEST imm form).
**Function:** encode_test
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:689
**Detected by:** Differential — llvm-mc i686 (segment Imm→Mem); Algebraic — metamorphic seg‖bare
**Minimal input:** `testb $5, %es:(%eax)` / `testl $5, %es:(%eax)`
**Expected:** `[0x26, 0xf6, 0x00, 0x05]` (testb) / `[0x26, 0xf7, 0x00, 0x05, 0x00, 0x00, 0x00]` (testl)
**Actual:** `[0xf6, 0x00, 0x05]` / `[0xf7, 0x00, 0x05, 0x00, 0x00, 0x00]` (prefix omitted)
**Severity:** high
**Root cause:** Imm→Mem arm at gp_integer.rs:689-706 never calls `emit_segment_prefix(mem)` before pushing F6/F7 (same defect class as encode_alu/invlpg/lmsw; x86-64 sibling does call it).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:689`
```rust
            (Operand::Immediate(ImmediateValue::Integer(val)), Operand::Memory(mem)) => {
                let val = *val;
                if size == 2 { self.bytes.push(0x66); }
                if size == 1 {
                    self.bytes.push(0xF6);
                } else {
                    self.bytes.push(0xF7);
                }
                self.encode_modrm_mem(0, mem)?;
```
**Suggested fix:** Call emit_segment_prefix before the size override / opcode:
```rust
            (Operand::Immediate(ImmediateValue::Integer(val)), Operand::Memory(mem)) => {
                let val = *val;
                self.emit_segment_prefix(mem);
                if size == 2 { self.bytes.push(0x66); }
                if size == 1 {
                    self.bytes.push(0xF6);
                } else {
                    self.bytes.push(0xF7);
                }
                self.encode_modrm_mem(0, mem)?;
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_test_regression_es_segment_imm_mem -- --test-threads=1
```
**Raw output:**
```text
assertion `left == right` failed: regression: segmented Imm→Mem TEST must match llvm-mc (got [f7, 00, 05, 00, 00, 00])
  left: [247, 0, 5, 0, 0, 0]
 right: [38, 247, 0, 5, 0, 0, 0]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_test_pbt.rs (encode_test_regression_es_segment_imm_mem)
