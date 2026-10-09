# Bug: encode_pop metamorphic segment strip fails (missing prefix)
**Law:** For every segmented memory POP, the encoding must start with the segment-override prefix and, after stripping prefixes, equal the bare memory POP encoding.
**Impact:** Same defect as encode_pop_missing_segment_prefix: segmented POP drops the override, so far/TLS addressing is wrong. Reported separately because the metamorphic property is an independent oracle witness of the same root cause.
**Function:** encode_pop
**Source location:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:424
**Detected by:** Algebraic — Metamorphic (segmented = prefix ‖ bare)
**Minimal input:** seg=es, base=eax, disp=0 (`popl %es:(%eax)`)
**Expected:** with_seg = `[0x26, 0x8f, 0x00]`; strip → `[0x8f, 0x00]` = bare
**Actual:** with_seg = `[0x8f, 0x00]` (no 0x26)
**Severity:** high
**Root cause:** `gp_integer.rs:424-427` does not call `emit_segment_prefix(mem)` before opcode 0x8F (same as b1).
**Offending code:** `/home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/gp_integer.rs:424`
```rust
            Operand::Memory(mem) => {
                // pop m32: 0x8F /0
                self.bytes.push(0x8F);
                self.encode_modrm_mem(0, mem)
            }
```
**Suggested fix:** Call `emit_segment_prefix` before the opcode.
```rust
            Operand::Memory(mem) => {
                self.emit_segment_prefix(mem);
                self.bytes.push(0x8F);
                self.encode_modrm_mem(0, mem)
            }
```

**Reproduction:**
```bash
cd /home/toan/github/claudes-c-compiler
cargo test --lib encode_pop_meta_segment_stripped_eq_bare -- --test-threads=1
```
**Raw output:**
```text
Test failed: segmented pop must start with 0x26 for %es:, got [8f, 00]
minimal failing input: seg = "es", base = "eax", disp = 0
```
**Regression test:** /home/toan/github/claudes-c-compiler/src/backend/i686/assembler/encoder/encode_pop_pbt.rs (encode_pop_meta_segment_stripped_eq_bare)
