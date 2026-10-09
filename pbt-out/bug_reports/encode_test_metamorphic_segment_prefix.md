# Bug: encode_test Imm→Mem breaks metamorphic seg‖bare
**Law:** Segmented Imm→Mem encoding must equal seg_prefix_byte ‖ bare Imm→Mem bytes.
**Impact:** Segmented TEST imm forms omit the override prefix (same root cause as missing segment prefix on Imm→Mem).
**Function:** encode_test
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:645
**Detected by:** Algebraic — Metamorphic (seg‖bare)
**Minimal input:** `testb $0, %es:(%eax)`
**Expected:** `[0x26, 0xf6, 0x00, 0x00]`
**Actual:** `[0xf6, 0x00, 0x00]`
**Severity:** high
**Root cause:** Imm→Mem arm at gp_integer.rs:689 never calls `emit_segment_prefix(mem)` (same as b2).
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
**Suggested fix:** Call emit_segment_prefix before opcode bytes:
```rust
self.emit_segment_prefix(mem);
if size == 2 { self.bytes.push(0x66); }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_test_metamorphic_segment_prefix -- --test-threads=1
```
**Raw output:**
```text
metamorphic seg||bare for testb $0 %es:(%eax): got [f6, 00, 00] expect [26, f6, 00, 00]
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_test_pbt.rs
